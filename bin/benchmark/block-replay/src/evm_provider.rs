use alloy_primitives::{Address, B256};
use revm_primitives::{AccountInfo, Bytecode};
use revm_primitives::db::{Database, DatabaseRef};
use reth_blockchain_tree::error::ProviderError;
use reth_primitives::U256;
use reth_revm::database::EvmStateProvider;

#[derive(Debug, Clone)]
pub struct BenchStateProviderDatabase<DB> {
    pub state_reader: DB,
    pub block_reader: DB,
}

impl<DB> BenchStateProviderDatabase<DB> {
    /// Create new State with generic `StateProvider`.
    pub const fn new(block_reader: DB, state_reader: DB ) -> Self {
        Self{block_reader, state_reader}
    }
}

impl<DB: EvmStateProvider> Database for BenchStateProviderDatabase<DB> {
    type Error = ProviderError;

    /// Retrieves basic account information for a given address.
    ///
    /// Returns `Ok` with `Some(AccountInfo)` if the account exists,
    /// `None` if it doesn't, or an error if encountered.
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        Ok(self.state_reader.basic_account(address)?.map(Into::into))
    }

    /// Retrieves the bytecode associated with a given code hash.
    ///
    /// Returns `Ok` with the bytecode if found, or the default bytecode otherwise.
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        Ok(self.block_reader.bytecode_by_hash(code_hash)?.unwrap_or_default().0)
    }

    /// Retrieves the storage value at a specific index for a given address.
    ///
    /// Returns `Ok` with the storage value, or the default value if not found.
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        Ok(self.state_reader.storage(address, B256::new(index.to_be_bytes()))?.unwrap_or_default())
    }

    /// Retrieves the block hash for a given block number.
    ///
    /// Returns `Ok` with the block hash if found, or the default hash otherwise.
    /// Note: It safely casts the `number` to `u64`.
    fn block_hash(&mut self, number: u64) -> Result<B256, Self::Error> {
        Ok(self.block_reader.block_hash(number)?.unwrap_or_default())
    }
}