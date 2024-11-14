// generator.rs
//! This module contains the implementation of the generator for the DBBench workload.

mod evm_cfg;
mod evm_provider;

use reth_chainspec::MAINNET;
use reth_consensus::test_utils::TestConsensus;
use reth_execution_types::{BlockExecutionOutput, Chain, ExecutionOutcome};
use reth_primitives::{BlockWithSenders, SealedBlockWithSenders};
use reth_prune_types::PruneModes;

use reth_db::{init_db, open_db_read_only, tables, Database, DatabaseEnv, PlainAccountState};

use reth_evm::execute::{BatchExecutor, BlockExecutionError, BlockExecutionInput, BlockExecutorProvider, Executor};
use reth_revm::database::{EvmStateProvider, StateProviderDatabase};

use reth_evm_ethereum::execute::EthExecutorProvider;
use reth_provider::{BlockReader, DatabaseProviderFactory, HeaderProvider, LatestStateProviderRef, OriginalValuesKnown, ProviderError, ProviderFactory, ProviderResult, StateWriter, StaticFileProviderFactory, TransactionVariant};

use crate::evm_cfg::BenchEvmConfig;
use reth_blockchain_tree::{
    metrics::MakeCanonicalDurationsRecorder, BlockchainTree, BlockchainTreeConfig, TreeExternals,
};

use alloy_primitives::{Address, BlockNumber, StorageKey, StorageValue, B256};
use clap::{arg, command, Parser};
use reth_db::{
    database_metrics::DatabaseMetrics,
    mdbx::{DatabaseArguments, MaxReadTransactionDuration},
    models::ClientVersion,
};
use reth_db_common::init::init_genesis;
use reth_primitives::revm_primitives::ruint::aliases::U256;
use reth_provider::providers::StaticFileProvider;
use reth_revm::db::BundleAccount;
use reth_tasks::TokioTaskExecutor;
use reth_trie::{HashedPostState, HashedStorage};
use std::{
    collections::HashMap,
    net::SocketAddrV4,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};
use revm_primitives::Account;
use tokio::{
    sync::{mpsc, mpsc::Sender},
    task, time,
};
use tracing::info;
use reth_blockchain_tree::error::CanonicalError;
use reth_db::cursor::DbCursorRO;
use reth_db::transaction::DbTx;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Bytecode;
use reth_provider::writer::UnifiedStorageWriter;
use crate::evm_provider::BenchStateProviderDatabase;

/// Represents the configuration for the data generator.
#[derive(Parser, Debug, Clone, Default)]
#[command(name = "benchmark")]
pub struct BenchConfig {
    /// The starting block number for the data generator.
    #[arg(short, long, default_value = "0")]
    pub start_block_number: BlockNumber,

    /// The target block number for the data generator.
    #[arg(short, long, default_value = "1000000")]
    pub end_block_number: BlockNumber,

    /// The source file path for the data generator stream.
    #[arg(long, short = 'i', default_value = "/dev/shm/tmp")]
    pub input_file_path: PathBuf,

    /// The source file path for the data generator stream.
    #[arg(long, short = 'o', default_value = "/tmp/output")]
    pub output_file_path: PathBuf,

    /// metric job name
    #[arg(long, default_value = "192.168.0.203:9091")]
    pub metric_push_addr: String,

    /// metric job name
    #[arg(long, default_value = "db_bench_generator")]
    pub metric_job_name: String,

    /// metric instance name
    #[arg(long, default_value = "instance0")]
    pub metric_instance_name: String,

    /// merge transactions size
    #[arg(short, long, default_value = "false")]
    pub merge_by_tx: bool,

    /// merge transactions size
    #[arg(short, long, default_value = "10000")]
    pub merge_txs: u32,

    /// pipeline count
    #[arg(short, long, default_value = "10")]
    pub pipeline: usize,

    /// merge transactions size
    #[arg(short, long, default_value = "1000000000")]
    pub block_interval: f64,

    /// merge transactions size
    #[arg(short, long, default_value = "false")]
    pub build_salt_trie: bool,

    /// merge transactions size
    #[arg(short, long, default_value = "false")]
    pub dump_salt_key: bool,
}

fn merge_block(
    from: SealedBlockWithSenders,
    mut to: SealedBlockWithSenders,
) -> SealedBlockWithSenders {
    to.senders.extend(from.senders.clone());
    to.block.body.extend(from.block.body.clone());
    to.block.ommers.extend(from.ommers.clone());

    if let (Some(ref mut to_withdrawals), Some(from_withdrawals)) =
        (&mut to.block.withdrawals, &from.block.withdrawals)
    {
        to_withdrawals.0.extend(from_withdrawals.0.clone());
    } else if to.block.withdrawals.is_none() {
        to.block.withdrawals = from.block.withdrawals.clone();
    }

    if let (Some(ref mut to_requests), Some(from_requests)) =
        (&mut to.block.requests, &from.block.requests)
    {
        to_requests.0.extend(from_requests.0.clone());
    } else if to.block.requests.is_none() {
        to.block.requests = from.block.requests.clone();
    }

    to.header = from.header.clone();
    to
}

pub fn open_mainnet_db_ro<DB: Database + Clone>(
    db_path: &Path,
) -> ProviderFactory<Arc<DatabaseEnv>> {
    let db = open_db_read_only(
        db_path.join("db").as_path(),
        DatabaseArguments::new(ClientVersion::default())
            .with_exclusive(Some(true))
            .with_max_read_transaction_duration(Some(MaxReadTransactionDuration::Unbounded)),
    )
    .unwrap();

    ProviderFactory::new(
        Arc::new(db),
        MAINNET.clone(),
        StaticFileProvider::read_write(db_path.join("static_files")).unwrap(),
    )
}

/// Create a provider factory for testing
pub fn open_mainnet_db_rw<DB: Database + Clone>(
    db_path: &Path,
) -> ProviderFactory<Arc<DatabaseEnv>> {
    let db = init_db(
        db_path.join("db"),
        DatabaseArguments::new(ClientVersion::default())
            .with_exclusive(Some(true))
            .with_max_read_transaction_duration(Some(MaxReadTransactionDuration::Unbounded)),
    )
    .unwrap();

    ProviderFactory::new(
        Arc::new(db),
        MAINNET.clone(),
        StaticFileProvider::read_write(db_path.join("static_files")).unwrap(),
    )
}

fn read_mainnet(
    config: BenchConfig,
    tx: Sender<Vec<(U256, BlockWithSenders)>>,
    start: BlockNumber,
    factory: ProviderFactory<Arc<DatabaseEnv>>
) {
    let max_block = config.end_block_number;
    let factory_clone = factory.clone();
    tokio::spawn(async move {
        let src_provider = factory_clone.database_provider_ro().unwrap();
        let mut blocks: Vec<(U256, BlockWithSenders)> = Vec::new();
        let mut total_merge_tx = 0;
        for block_number in start..=max_block {
            info!("current block number:{}", block_number);
            let td = src_provider
                .header_td_by_number(block_number)
                .unwrap()
                .ok_or_else(|| ProviderError::HeaderNotFound(block_number.into()))
                .unwrap();

            let block = src_provider
                .block_with_senders(block_number.into(), TransactionVariant::NoHash)
                .unwrap()
                .ok_or_else(|| ProviderError::HeaderNotFound(block_number.into()))
                .unwrap();

            let current_len = block.body.len();
            blocks.push((td, block));
            total_merge_tx += current_len;
            if (config.merge_by_tx && total_merge_tx < config.merge_txs as usize) {
                continue;
            }
            if tx.send(blocks).await.is_err() {
                println!("Receiver dropped, exiting data fetch loop");
                break;
            }
            blocks = Vec::new();
            total_merge_tx = 0;
        }
        drop(tx);
    });
}

async fn run_bench(config: BenchConfig) {
    let mut start_block = config.start_block_number;

    let mut output_factory =
        open_mainnet_db_rw::<Arc<DatabaseEnv>>(config.output_file_path.as_path());
    let input_factory =
        open_mainnet_db_ro::<Arc<DatabaseEnv>>(&config.input_file_path);

    if start_block == 0 {
        init_genesis(output_factory.clone()).expect("TODO: init genesis failed");
        start_block += 1;
    }

    let consensus = Arc::new(TestConsensus::default());
    let executor_factory = EthExecutorProvider::mainnet();
    let tree_config = BlockchainTreeConfig::new(1, 2, 3, 2);
    let mut tree = BlockchainTree::new(
        TreeExternals::new(output_factory.clone(), consensus, executor_factory),
        tree_config,
        PruneModes::default(),
    )
        .unwrap();

    let (tx, mut rx) = mpsc::channel(100);
    read_mainnet(config.clone(), tx, start_block, input_factory.clone());
    let executor_provider = EthExecutorProvider::new(MAINNET.clone(), EthEvmConfig::default());

    let input_read_ro = input_factory.provider().unwrap();
    while let Some(blocks) = rx.recv().await {
        let output_read_ro = output_factory.provider().unwrap();
        let db = BenchStateProviderDatabase::new(
            LatestStateProviderRef::new(
                input_read_ro.tx_ref(),
                input_factory.static_file_provider()
            ),
            LatestStateProviderRef::new(
                output_read_ro.tx_ref(),
                output_read_ro.static_file_provider().clone(),
            ),
        );
        let mut executor = executor_provider.batch_executor(db);
        let mut to_seal_blocks = Vec::new();
        for (td, block) in blocks {
            to_seal_blocks.push(block.clone());
            let result = executor.execute_and_verify_one((&block, td).into());
            match result  {
                Ok(_) => {}
                Err(err) => {
                    println!("block execute failed:{}", err);
                }
            };
        }
        info!("process current block number:{}", to_seal_blocks.last().unwrap().number);
        let ExecutionOutcome { bundle, receipts, requests, first_block } = executor.finalize();
        let state = ExecutionOutcome::new(bundle, receipts, first_block, requests);
        let blocks = to_seal_blocks.into_iter().map(|block| {
            let hash = block.header.hash_slow();
            block.seal(hash)
        });
        drop(output_read_ro);
        let chain = Chain::new(blocks, state, None);
        let mut recorder = MakeCanonicalDurationsRecorder::default();
        let result = tree.commit_canonical_to_database(chain, &mut recorder);
        match result {
            Ok(_) => {}
            Err(err) => {
                println!("block commit failed:{}", err);
            }
        }
    }

    println!("~~~finished~~~");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .with_max_level(tracing::Level::INFO)
        .init();
    let conf = BenchConfig::parse();
    run_bench(conf).await;
}
