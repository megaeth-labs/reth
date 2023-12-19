use super::{caculate::CaculateRecord, db_read::DBReadRecord, mpt::MPTRecord};
use alloy_trie::utils::metric::Keccak256Record;
use revm_utils::metrics::types::TimeDistributionStats;

#[derive(Clone, Copy, Default, Debug)]
pub struct StateRootRecord {
    pub(crate) block_number: u64,

    // pub(crate) db_switch: bool,
    /// total number of tx
    pub(crate) total_txs_count: u64,
    /// total number of keys updated.
    pub(crate) total_keys_count: u64,

    pub(crate) account_changes: u64,

    // pub(crate) contract_account_changes: u64,
    pub(crate) storage_changes: u64,

    pub(crate) construct_prefix_sets_time: u64,
    
    pub(crate) state_write_to_db_time: u64,

    pub(crate) hashed_state_write_time: u64,

    pub(crate) hash_state_slow_time: u64,

    // pub(crate) state_root_calculator_time: u64,
    pub(crate) flush_time: u64,

    pub(crate) db_read: DBReadRecord,

    pub(crate) db_read_distribution: TimeDistributionStats,

    pub(crate) state_calculate_record: CaculateRecord,

    pub(crate) storage_calculate_record: CaculateRecord,

    pub(crate) keccak256_record: Keccak256Record,

    pub(crate) state_mpt: MPTRecord,

    pub(crate) storage_mpt: MPTRecord,
}
