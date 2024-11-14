//! This example shows how to implement a node with a custom EVM

use alloy_primitives::Bytes;
use reth_chainspec::{ChainSpec, Head};
use reth_evm::{ConfigureEvm, ConfigureEvmEnv};
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives::{Address, Header, TransactionSigned, U256};
use reth_primitives::revm_primitives::{AnalysisKind, CfgEnvWithHandlerCfg, Env, TxEnv};
use reth_revm::{Database, Evm, EvmBuilder};

/// Custom EVM configuration
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub(crate) struct BenchEvmConfig;

impl ConfigureEvmEnv for BenchEvmConfig {
    fn fill_tx_env(&self, tx_env: &mut TxEnv, transaction: &TransactionSigned, sender: Address) {
        EthEvmConfig::default().fill_tx_env(tx_env, transaction, sender);
        tx_env.nonce = None;
    }

    fn fill_tx_env_system_contract_call(
        &self,
        env: &mut Env,
        caller: Address,
        contract: Address,
        data: Bytes,
    ) {
        EthEvmConfig::default().fill_tx_env_system_contract_call(env, caller, contract, data)
    }

    fn fill_cfg_env(
        &self,
        cfg_env: &mut CfgEnvWithHandlerCfg,
        chain_spec: &ChainSpec,
        header: &Header,
        total_difficulty: U256,
    ) {
        let spec_id = reth_evm_ethereum::revm_spec(
            chain_spec,
            &Head {
                number: header.number,
                timestamp: header.timestamp,
                difficulty: header.difficulty,
                total_difficulty,
                hash: Default::default(),
            },
        );

        cfg_env.chain_id = chain_spec.chain().id();
        cfg_env.perf_analyse_created_bytecodes = AnalysisKind::Analyse;
        cfg_env.handler_cfg.spec_id = spec_id;

        // (Lary) solve the balance check bench bugs.
        // cfg_env.disable_balance_check = true;
        // cfg_env.disable_block_gas_limit = true;
    }
}

impl ConfigureEvm for BenchEvmConfig {
    type DefaultExternalContext<'a> = ();

    fn evm<DB: Database>(&self, db: DB) -> Evm<'_, Self::DefaultExternalContext<'_>, DB> {
        EvmBuilder::default().with_db(db).build()
    }

    fn default_external_context<'a>(&self) -> Self::DefaultExternalContext<'a> {}
}
