use super::{HashedAccountCursor, HashedCursorFactory, HashedStorageCursor};
use reth_db::{
    cursor::{DbCursorRO, DbDupCursorRO},
    tables,
    transaction::DbTx,
};
use reth_primitives::{Account, StorageEntry, B256};

impl<'a, TX: DbTx> HashedCursorFactory for &'a TX {
    type AccountCursor = <TX as DbTx>::Cursor<tables::HashedAccount>;
    type StorageCursor = <TX as DbTx>::DupCursor<tables::HashedStorage>;

    fn hashed_account_cursor(&self) -> Result<Self::AccountCursor, reth_db::DatabaseError> {
        self.cursor_read::<tables::HashedAccount>()
    }

    fn hashed_storage_cursor(&self) -> Result<Self::StorageCursor, reth_db::DatabaseError> {
        self.cursor_dup_read::<tables::HashedStorage>()
    }
}

impl<C> HashedAccountCursor for C
where
    C: DbCursorRO<tables::HashedAccount>,
{
    fn seek(&mut self, key: B256) -> Result<Option<(B256, Account)>, reth_db::DatabaseError> {
        #[cfg(feature = "enable_state_root_record")]
        let _db_seek = perf_metrics::DBSeekRead::default();

        self.seek(key)
    }

    fn next(&mut self) -> Result<Option<(B256, Account)>, reth_db::DatabaseError> {
        #[cfg(feature = "enable_state_root_record")]
        let _db_next = perf_metrics::DBNextRead::default();

        self.next()
    }
}

impl<C> HashedStorageCursor for C
where
    C: DbCursorRO<tables::HashedStorage> + DbDupCursorRO<tables::HashedStorage>,
{
    fn is_storage_empty(&mut self, key: B256) -> Result<bool, reth_db::DatabaseError> {
        let db_entry = {
            #[cfg(feature = "enable_state_root_record")]
            let _db_seek_exact = perf_metrics::DBSeekExactRead::default();

            self.seek_exact(key)?
        };

        Ok(db_entry.is_none())
    }

    fn seek(
        &mut self,
        key: B256,
        subkey: B256,
    ) -> Result<Option<StorageEntry>, reth_db::DatabaseError> {
        #[cfg(feature = "enable_state_root_record")]
        let _db_seek_by_key_subkey = perf_metrics::DBSeekBySubKeyRead::default();

        self.seek_by_key_subkey(key, subkey)
    }

    fn next(&mut self) -> Result<Option<StorageEntry>, reth_db::DatabaseError> {
        #[cfg(feature = "enable_state_root_record")]
        let _db_next_dup_val = perf_metrics::DBNextDupValRead::default();

        self.next_dup_val()
    }
}
