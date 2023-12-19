use alloy_trie::utils::{metric::Keccak256Record, TreeNode};
use revm_utils::metrics::types::TimeDistributionStats;

#[derive(Clone, Copy, Default, Debug)]
pub struct TryNextRecord {
    /// total execution count of try_next function
    pub(crate) total_count: u64,
    /// total execution time of try_next function
    pub(crate) total_time: u64,
    /// count of the branch node walker advance to but can skip.
    pub(crate) skip_branch_node_count: u64,
    /// count of boundary reading leaf
    pub(crate) leaf_miss_count: u64,
    /// count of work leaf
    pub(crate) leaf_hit_count: u64,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct CaculateRecord {
    pub(crate) total_time: u64,
    pub(crate) before_loop_time: u64,
    pub(crate) try_next_stat: TryNextRecord,
    pub(crate) add_branch_count: u64,
    pub(crate) add_branch_time: u64,
    pub(crate) cal_storage_root_and_add_leaf_time: u64,
    pub(crate) after_cal_storage_root_time: u64,
    pub(crate) add_leaf_count: u64,
    pub(crate) add_leaf_time: u64,
    pub(crate) add_root_count: u64,
    pub(crate) add_root_time: u64,
    pub(crate) after_add_root_time: u64,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct DBReadRecord {
    pub(crate) seek_count: u64,
    pub(crate) seek_time: u64,
    pub(crate) next_count: u64,
    pub(crate) next_time: u64,
    pub(crate) seek_exact_count: u64,
    pub(crate) seek_exact_time: u64,
    pub(crate) seek_by_sub_key_count: u64,
    pub(crate) seek_by_sub_key_time: u64,
    pub(crate) next_dup_val_count: u64,
    pub(crate) next_dup_val_time: u64,
    pub(crate) at_seek_count: u64,
    pub(crate) at_seek_time: u64,
    pub(crate) at_seek_exact_count: u64,
    pub(crate) at_seek_exact_time: u64,
    pub(crate) at_current_count: u64,
    pub(crate) at_current_time: u64,
    pub(crate) st_seek_by_subkey_count: u64,
    pub(crate) st_seek_by_subkey_time: u64,
    pub(crate) st_current_count: u64,
    pub(crate) st_current_time: u64,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct UpdateKeys {
    pub(crate) account_trie_count: u64,
    pub(crate) storage_trie_count: u64,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct StateRootRecord {
    pub(crate) start_block_number: u64,
    pub(crate) end_block_number: u64,
    /// total number of tx
    pub(crate) total_txs_count: u64,
    pub(crate) update_keys: UpdateKeys,
    pub(crate) total_time: u64,
    pub(crate) block_td_time: u64,
    pub(crate) block_with_senders_time: u64,
    pub(crate) execute_and_verify_receipt_time: u64,
    pub(crate) construct_prefix_sets_time: u64,
    pub(crate) state_write_to_db_time: u64,
    pub(crate) hashed_state_write_time: u64,
    pub(crate) hash_state_slow_time: u64,
    pub(crate) flush_time: u64,
    pub(crate) db_read: DBReadRecord,
    pub(crate) db_read_distribution: TimeDistributionStats,
    pub(crate) state_calculate_record: CaculateRecord,
    pub(crate) storage_calculate_record: CaculateRecord,
    pub(crate) keccak256_record: Keccak256Record,
    pub(crate) state_trie_info: TreeNode,
    pub(crate) storage_trie_info: TreeNode,
    pub(crate) delete_branch_count: u64,
}
