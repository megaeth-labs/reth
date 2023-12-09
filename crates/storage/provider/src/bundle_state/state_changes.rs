use rayon::slice::ParallelSliceMut;
use reth_db::{
    cursor::{DbCursorRO, DbCursorRW, DbDupCursorRO, DbDupCursorRW},
    tables,
    transaction::{DbTx, DbTxMut},
};
use reth_interfaces::db::DatabaseError;
use reth_primitives::{revm::compat::into_reth_acc, Bytecode, StorageEntry, U256};
use revm::db::states::{PlainStorageChangeset, StateChangeset};

/// A change to the state of the world.
#[derive(Debug, Default)]
pub struct StateChanges(pub StateChangeset);

impl From<StateChangeset> for StateChanges {
    fn from(revm: StateChangeset) -> Self {
        Self(revm)
    }
}

impl StateChanges {
    /// Write the bundle state to the database.
    pub fn write_to_db<TX: DbTxMut + DbTx>(mut self, tx: &TX) -> Result<(), DatabaseError> {
        // sort all entries so they can be written to database in more performant way.
        // and take smaller memory footprint.
        self.0.accounts.par_sort_by_key(|a| a.0);
        self.0.storage.par_sort_by_key(|a| a.address);
        self.0.contracts.par_sort_by_key(|a| a.0);
        #[cfg(feature = "enable_execution_duration_record")]
        perf_metrics::record_sort_time();

        // Write new account state
        tracing::trace!(target: "provider::bundle_state", len = self.0.accounts.len(), "Writing new account state");
        let mut accounts_cursor = tx.cursor_write::<tables::PlainAccountState>()?;
        // write account to database.
        for (address, account) in self.0.accounts.into_iter() {
            if let Some(account) = account {
                tracing::trace!(target: "provider::bundle_state", ?address, "Updating plain state account");
                #[cfg(feature = "enable_execution_duration_record")]
                // sizeof(Address) + sizeof(Account) = 100
                let _record = perf_metrics::StateAccountWrite::new(100usize);
                accounts_cursor.upsert(address, into_reth_acc(account))?;
            } else if accounts_cursor.seek_exact(address)?.is_some() {
                tracing::trace!(target: "provider::bundle_state", ?address, "Deleting plain state account");
                accounts_cursor.delete_current()?;
            }
        }
        #[cfg(feature = "enable_execution_duration_record")]
        perf_metrics::record_state_account_time();

        // Write bytecode
        tracing::trace!(target: "provider::bundle_state", len = self.0.contracts.len(), "Writing bytecodes");
        let mut bytecodes_cursor = tx.cursor_write::<tables::Bytecodes>()?;
        for (hash, bytecode) in self.0.contracts.into_iter() {
            #[cfg(feature = "enable_execution_duration_record")]
            // size_of_val(hash) + size_of::<Bytecode>() = 88
            let _record =
                perf_metrics::StateBytecodeWrite::new(88usize + bytecode.bytecode.0.len());
            bytecodes_cursor.upsert(hash, Bytecode(bytecode))?;
        }
        #[cfg(feature = "enable_execution_duration_record")]
        perf_metrics::record_state_bytecode_time();

        // Write new storage state and wipe storage if needed.
        tracing::trace!(target: "provider::bundle_state", len = self.0.storage.len(), "Writing new storage state");
        let mut storages_cursor = tx.cursor_dup_write::<tables::PlainStorageState>()?;
        for PlainStorageChangeset { address, wipe_storage, storage } in self.0.storage.into_iter() {
            // Wiping of storage.
            if wipe_storage && storages_cursor.seek_exact(address)?.is_some() {
                storages_cursor.delete_current_duplicates()?;
            }
            // cast storages to B256.
            let mut storage = storage
                .into_iter()
                .map(|(k, value)| StorageEntry { key: k.into(), value })
                .collect::<Vec<_>>();
            // sort storage slots by key.
            storage.par_sort_unstable_by_key(|a| a.key);

            for entry in storage.into_iter() {
                tracing::trace!(target: "provider::bundle_state", ?address, ?entry.key, "Updating plain state storage");
                if let Some(db_entry) = storages_cursor.seek_by_key_subkey(address, entry.key)? {
                    if db_entry.key == entry.key {
                        storages_cursor.delete_current()?;
                    }
                }

                if entry.value != U256::ZERO {
                    #[cfg(feature = "enable_execution_duration_record")]
                    // sizeof(Address) + sizeof(StorageEntry) = 84
                    let _record = perf_metrics::StateStorageWrite::new(84usize);
                    storages_cursor.upsert(address, entry)?;
                }
            }
        }
        #[cfg(feature = "enable_execution_duration_record")]
        perf_metrics::record_state_storage_time();
        Ok(())
    }
}
