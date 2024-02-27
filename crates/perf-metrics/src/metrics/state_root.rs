//! This module is used to support in-depth measurement of state root update.
use super::time_distribution_stats::TimeDistributionStats;
use alloy_trie::utils::{metric::Keccak256Record, TreeNode};
use revm_utils::time_utils::{convert_cycles_to_ns_f64, instant::Instant};

#[derive(Debug, Clone, Copy, Default)]
pub struct DBRead {
    pub(crate) current_count: u64,
    pub(crate) current: u64,
    pub(crate) seek_count: u64,
    pub(crate) seek: u64,
    pub(crate) next_count: u64,
    pub(crate) next: u64,
    pub(crate) seek_exact_count: u64,
    pub(crate) seek_exact: u64,
    pub(crate) seek_by_sub_key_count: u64,
    pub(crate) seek_by_sub_key: u64,
    pub(crate) next_dup_val_count: u64,
    pub(crate) next_dup_val: u64,

    pub(crate) account_trie_seek_count: u64,
    pub(crate) account_trie_seek: u64,
    pub(crate) account_trie_seek_exact_count: u64,
    pub(crate) account_trie_seek_exact: u64,
    pub(crate) account_trie_current_count: u64,
    pub(crate) account_trie_current: u64,
    pub(crate) storage_trie_seek_by_subkey_count: u64,
    pub(crate) storage_trie_seek_by_subkey: u64,
    pub(crate) storage_trie_current_count: u64,
    pub(crate) storage_trie_current: u64,

    pub(crate) hash_account_cursor_seek_hit_count: u64,
    pub(crate) hash_storage_cursor_seek_hit_count: u64,
}

impl DBRead {
    pub fn add_other(&mut self, other: Self) {
        self.add_current_count(other.current_count);
        self.add_current(other.current);
        self.add_seek_count(other.seek_count);
        self.add_seek(other.seek);
        self.add_next_count(other.next_count);
        self.add_next(other.next);
        self.add_seek_exact_count(other.seek_exact_count);
        self.add_seek_exact(other.seek_exact);
        self.add_seek_by_sub_key_count(other.seek_by_sub_key_count);
        self.add_seek_by_sub_key(other.seek_by_sub_key);
        self.add_next_dup_val_count(other.next_dup_val_count);
        self.add_next_dup_val(other.next_dup_val);

        self.add_account_trie_seek_count(other.account_trie_seek_count);
        self.add_account_trie_seek(other.account_trie_seek);
        self.add_account_trie_seek_exact_count(other.account_trie_seek_exact_count);
        self.add_account_trie_seek_exact(other.account_trie_seek_exact);
        self.add_account_trie_current_count(other.account_trie_current_count);
        self.add_account_trie_current(other.account_trie_current);
        self.add_storage_trie_seek_by_key_subkey_count(other.storage_trie_seek_by_subkey_count);
        self.add_storage_trie_seek_by_key_subkey(other.storage_trie_seek_by_subkey);
        self.add_storage_trie_current_count(other.storage_trie_current_count);
        self.add_storage_trie_current(other.storage_trie_current);

        self.add_hash_account_cursor_seek_hit_count(other.hash_account_cursor_seek_hit_count);
        self.add_hash_storage_cursor_seek_hit_count(other.hash_storage_cursor_seek_hit_count);
    }

    pub(crate) fn add_current_count(&mut self, count: u64) {
        self.current_count = self.current_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_current(&mut self, time_cycles: u64) {
        self.current = self.current.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_seek_count(&mut self, count: u64) {
        self.seek_count = self.seek_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_seek(&mut self, time_cycles: u64) {
        self.seek = self.seek.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_next_count(&mut self, count: u64) {
        self.next_count = self.next_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_next(&mut self, time_cycles: u64) {
        self.next = self.next.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_seek_exact_count(&mut self, count: u64) {
        self.seek_exact_count = self.seek_exact_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_seek_exact(&mut self, time_cycles: u64) {
        self.seek_exact = self.seek_exact.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_seek_by_sub_key_count(&mut self, count: u64) {
        self.seek_by_sub_key_count =
            self.seek_by_sub_key_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_seek_by_sub_key(&mut self, time_cycles: u64) {
        self.seek_by_sub_key = self.seek_by_sub_key.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_next_dup_val_count(&mut self, count: u64) {
        self.next_dup_val_count = self.next_dup_val_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_next_dup_val(&mut self, time_cycles: u64) {
        self.next_dup_val = self.next_dup_val.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_account_trie_seek_count(&mut self, count: u64) {
        self.account_trie_seek_count =
            self.account_trie_seek_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_account_trie_seek(&mut self, time_cycles: u64) {
        self.account_trie_seek = self.account_trie_seek.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_account_trie_seek_exact_count(&mut self, count: u64) {
        self.account_trie_seek_exact_count =
            self.account_trie_seek_exact_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_account_trie_seek_exact(&mut self, time_cycles: u64) {
        self.account_trie_seek_exact =
            self.account_trie_seek_exact.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_account_trie_current_count(&mut self, count: u64) {
        self.account_trie_current_count =
            self.account_trie_current_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_account_trie_current(&mut self, time_cycles: u64) {
        self.account_trie_current =
            self.account_trie_current.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_storage_trie_seek_by_key_subkey_count(&mut self, count: u64) {
        self.storage_trie_seek_by_subkey_count =
            self.storage_trie_seek_by_subkey_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_storage_trie_seek_by_key_subkey(&mut self, time_cycles: u64) {
        self.storage_trie_seek_by_subkey =
            self.storage_trie_seek_by_subkey.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_storage_trie_current_count(&mut self, count: u64) {
        self.storage_trie_current_count =
            self.storage_trie_current_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_storage_trie_current(&mut self, time_cycles: u64) {
        self.storage_trie_current =
            self.storage_trie_current.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_hash_account_cursor_seek_hit_count(&mut self, count: u64) {
        self.hash_account_cursor_seek_hit_count =
            self.hash_account_cursor_seek_hit_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_hash_storage_cursor_seek_hit_count(&mut self, count: u64) {
        self.hash_storage_cursor_seek_hit_count =
            self.hash_storage_cursor_seek_hit_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn hash_account_table_count(&self) -> u64 {
        self.current_count
            .checked_add(self.next_count)
            .and_then(|x| x.checked_add(self.seek_count))
            .expect("overflow")
    }

    pub(crate) fn hash_account_table_time(&self) -> u64 {
        self.current
            .checked_add(self.next)
            .and_then(|x| x.checked_add(self.seek))
            .expect("overflow")
    }

    pub(crate) fn hash_storage_table_count(&self) -> u64 {
        self.seek_exact_count
            .checked_add(self.seek_by_sub_key_count)
            .and_then(|x| x.checked_add(self.next_dup_val_count))
            .expect("overflow")
    }

    pub(crate) fn hash_storage_table_time(&self) -> u64 {
        self.seek_exact
            .checked_add(self.seek_by_sub_key)
            .and_then(|x| x.checked_add(self.next_dup_val))
            .expect("overflow")
    }

    pub(crate) fn account_trie_table_count(&self) -> u64 {
        self.account_trie_seek_count
            .checked_add(self.account_trie_seek_exact_count)
            .and_then(|x| x.checked_add(self.account_trie_current_count))
            .expect("overflow")
    }

    pub(crate) fn account_trie_table_time(&self) -> u64 {
        self.account_trie_seek
            .checked_add(self.account_trie_seek_exact)
            .and_then(|x| x.checked_add(self.account_trie_current))
            .expect("overflow")
    }

    pub(crate) fn storage_trie_table_count(&self) -> u64 {
        self.storage_trie_seek_by_subkey_count
            .checked_add(self.storage_trie_current_count)
            .expect("overflow")
    }

    pub(crate) fn storage_trie_table_time(&self) -> u64 {
        self.storage_trie_seek_by_subkey.checked_add(self.storage_trie_current).expect("overflow")
    }

    pub(crate) fn leaf_table_count(&self) -> u64 {
        self.current_count
            .checked_add(self.next_count)
            .and_then(|x| x.checked_add(self.next_dup_val_count))
            .and_then(|x| x.checked_add(self.seek_count))
            .and_then(|x| x.checked_add(self.seek_by_sub_key_count))
            .and_then(|x| x.checked_add(self.seek_exact_count))
            .expect("overflow")
    }

    pub(crate) fn leaf_table_time(&self) -> u64 {
        self.current
            .checked_add(self.next)
            .and_then(|x| x.checked_add(self.next_dup_val))
            .and_then(|x| x.checked_add(self.seek))
            .and_then(|x| x.checked_add(self.seek_by_sub_key))
            .and_then(|x| x.checked_add(self.seek_exact))
            .expect("overflow")
    }

    pub(crate) fn branch_table_count(&self) -> u64 {
        self.account_trie_seek_count
            .checked_add(self.account_trie_seek_exact_count)
            .and_then(|x| x.checked_add(self.account_trie_current_count))
            .and_then(|x| x.checked_add(self.storage_trie_seek_by_subkey_count))
            .and_then(|x| x.checked_add(self.storage_trie_current_count))
            .expect("overflow")
    }

    pub(crate) fn branch_table_time(&self) -> u64 {
        self.account_trie_seek
            .checked_add(self.account_trie_seek_exact)
            .and_then(|x| x.checked_add(self.account_trie_current))
            .and_then(|x| x.checked_add(self.storage_trie_seek_by_subkey))
            .and_then(|x| x.checked_add(self.storage_trie_current))
            .expect("overflow")
    }

    pub(crate) fn total_count(&self) -> u64 {
        self.current_count
            .checked_add(self.next_count)
            .and_then(|x| x.checked_add(self.seek_count))
            .and_then(|x| x.checked_add(self.next_dup_val_count))
            .and_then(|x| x.checked_add(self.seek_by_sub_key_count))
            .and_then(|x| x.checked_add(self.seek_exact_count))
            .and_then(|x| x.checked_add(self.account_trie_seek_count))
            .and_then(|x| x.checked_add(self.account_trie_seek_exact_count))
            .and_then(|x| x.checked_add(self.account_trie_current_count))
            .and_then(|x| x.checked_add(self.storage_trie_seek_by_subkey_count))
            .and_then(|x| x.checked_add(self.storage_trie_current_count))
            .expect("overflow")
    }

    pub(crate) fn total_time(&self) -> u64 {
        self.current
            .checked_add(self.next)
            .and_then(|x| x.checked_add(self.seek))
            .and_then(|x| x.checked_add(self.next_dup_val))
            .and_then(|x| x.checked_add(self.seek_by_sub_key))
            .and_then(|x| x.checked_add(self.seek_exact))
            .and_then(|x| x.checked_add(self.account_trie_seek))
            .and_then(|x| x.checked_add(self.account_trie_seek_exact))
            .and_then(|x| x.checked_add(self.account_trie_current))
            .and_then(|x| x.checked_add(self.storage_trie_seek_by_subkey))
            .and_then(|x| x.checked_add(self.storage_trie_current))
            .expect("overflow")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TryNextStat {
    pub(crate) total_count: u64,
    pub(crate) total_time: u64,
    pub(crate) skip_branch_node_count: u64,
    pub(crate) leaf_miss_count: u64,
    pub(crate) leaf_hit_count: u64,
    pub(crate) walk_next_unprocessed_key_count: u64,
    pub(crate) walk_advance_count: u64,
    pub(crate) loop_count: u64,
}

impl TryNextStat {
    pub(crate) fn add_other(&mut self, other: Self) {
        self.add_total_count(other.total_count);
        self.add_total_time(other.total_time);
        self.add_skip_branch_node_count(other.skip_branch_node_count);
        self.add_leaf_miss_count(other.leaf_miss_count);
        self.add_leaf_hit_count(other.leaf_hit_count);
        self.add_walk_next_unprocessed_key_count(other.walk_next_unprocessed_key_count);
        self.add_walk_advance_count(other.walk_advance_count);
        self.add_loop_count(other.loop_count);
    }

    pub(crate) fn add_total_count(&mut self, count: u64) {
        self.total_count = self.total_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_total_time(&mut self, time_cycles: u64) {
        self.total_time = self.total_time.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_skip_branch_node_count(&mut self, count: u64) {
        self.skip_branch_node_count =
            self.skip_branch_node_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_leaf_miss_count(&mut self, count: u64) {
        self.leaf_miss_count = self.leaf_miss_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_leaf_hit_count(&mut self, count: u64) {
        self.leaf_hit_count = self.leaf_hit_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_walk_next_unprocessed_key_count(&mut self, count: u64) {
        self.walk_next_unprocessed_key_count =
            self.walk_next_unprocessed_key_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_walk_advance_count(&mut self, count: u64) {
        self.walk_advance_count = self.walk_advance_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_loop_count(&mut self, count: u64) {
        self.loop_count = self.loop_count.checked_add(count).expect("overflow");
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CaculateStat {
    pub(crate) total_time: u64,
    pub(crate) before_loop: u64,
    pub(crate) loop_begin: u64,
    pub(crate) try_next_stat: TryNextStat,
    pub(crate) add_branch_count: u64,
    pub(crate) add_branch: u64,
    pub(crate) cal_storage_root_and_add_leaf: u64,
    pub(crate) after_cal_storage_root: u64,
    pub(crate) add_leaf_count: u64,
    pub(crate) add_leaf: u64,
    pub(crate) add_root_count: u64,
    pub(crate) add_root: u64,
    pub(crate) after_add_root: u64,
}

impl CaculateStat {
    pub(crate) fn add_other(&mut self, other: Self) {
        self.add_total_time(other.total_time);
        self.add_before_loop(other.before_loop);
        self.add_loop_begin(other.loop_begin);
        self.try_next_stat.add_other(other.try_next_stat);
        self.add_add_branch_count(other.add_branch_count);
        self.add_add_branch(other.add_branch);
        self.add_cal_storage_root_and_add_leaf(other.cal_storage_root_and_add_leaf);
        self.add_after_cal_storage_root(other.after_cal_storage_root);
        self.add_add_leaf_count(other.add_leaf_count);
        self.add_add_leaf(other.add_leaf);
        self.add_add_root_count(other.add_root_count);
        self.add_add_root(other.add_root);
        self.add_after_loop(other.after_add_root);
    }
    pub(crate) fn add_total_time(&mut self, time_cycles: u64) {
        self.total_time = self.total_time.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_before_loop(&mut self, time_cycles: u64) {
        self.before_loop = self.before_loop.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_loop_begin(&mut self, time_cycles: u64) {
        self.loop_begin = self.loop_begin.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_try_next_stat_count(&mut self, count: u64) {
        self.try_next_stat.add_total_count(count);
    }

    pub(crate) fn add_try_next_stat_time(&mut self, time_cycles: u64) {
        self.try_next_stat.add_total_time(time_cycles);
    }

    pub(crate) fn add_try_next_stat_skip_branch_node_count(&mut self, count: u64) {
        self.try_next_stat.add_skip_branch_node_count(count);
    }

    pub(crate) fn add_try_next_stat_leaf_miss_count(&mut self, count: u64) {
        self.try_next_stat.add_leaf_miss_count(count);
    }

    pub(crate) fn add_try_next_stat_leaf_hit_count(&mut self, count: u64) {
        self.try_next_stat.add_leaf_hit_count(count);
    }

    pub(crate) fn add_try_next_stat_walk_next_unprocessed_key_count(&mut self, count: u64) {
        self.try_next_stat.add_walk_next_unprocessed_key_count(count);
    }

    pub(crate) fn add_try_next_stat_walk_advance_count(&mut self, count: u64) {
        self.try_next_stat.add_walk_advance_count(count);
    }

    pub(crate) fn add_try_next_stat_loop_count(&mut self, count: u64) {
        self.try_next_stat.add_loop_count(count);
    }

    pub(crate) fn add_add_branch_count(&mut self, count: u64) {
        self.add_branch_count = self.add_branch_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_add_branch(&mut self, time_cycles: u64) {
        self.add_branch = self.add_branch.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_cal_storage_root_and_add_leaf(&mut self, time_cycles: u64) {
        self.cal_storage_root_and_add_leaf =
            self.cal_storage_root_and_add_leaf.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_after_cal_storage_root(&mut self, time_cycles: u64) {
        self.after_cal_storage_root =
            self.after_cal_storage_root.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_add_leaf_count(&mut self, count: u64) {
        self.add_leaf_count = self.add_leaf_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_add_leaf(&mut self, time_cycles: u64) {
        self.add_leaf = self.add_leaf.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_add_root_count(&mut self, count: u64) {
        self.add_root_count = self.add_root_count.checked_add(count).expect("overflow");
    }

    pub(crate) fn add_add_root(&mut self, time_cycles: u64) {
        self.add_root = self.add_root.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_after_loop(&mut self, time_cycles: u64) {
        self.after_add_root = self.after_add_root.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn state_misc_time(&self) -> u64 {
        self.total_time
            .checked_sub(self.before_loop)
            .and_then(|x| x.checked_sub(self.try_next_stat.total_time))
            .and_then(|x| x.checked_sub(self.add_branch))
            .and_then(|x| x.checked_sub(self.cal_storage_root_and_add_leaf))
            .and_then(|x| x.checked_sub(self.add_root))
            .and_then(|x| x.checked_sub(self.after_add_root))
            .expect("overflow")
    }

    pub(crate) fn storage_misc_time(&self) -> u64 {
        self.total_time
            .checked_sub(self.before_loop)
            .and_then(|x| x.checked_sub(self.try_next_stat.total_time))
            .and_then(|x| x.checked_sub(self.add_branch))
            .and_then(|x| x.checked_sub(self.add_leaf))
            .and_then(|x| x.checked_sub(self.add_root))
            .and_then(|x| x.checked_sub(self.after_add_root))
            .expect("overflow")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MPTStat {
    root_node: TreeNode,
    update_branch_number: u64,
    delete_branch_number: u64,
}

impl MPTStat {
    pub fn add(&mut self, other: MPTStat) {
        self.root_node.add(&other.root_node);
        self.add_updates(other.delete_branch_number, other.update_branch_number);
    }

    pub fn add_nodes(&mut self, other: TreeNode) {
        self.root_node.add(&other);
    }

    pub fn add_updates(&mut self, delete_branch_nodes: u64, update_branch_nodes: u64) {
        self.delete_branch_number =
            self.delete_branch_number.checked_add(delete_branch_nodes).expect("overflow");
        self.update_branch_number =
            self.update_branch_number.checked_add(update_branch_nodes).expect("overflow");
    }
}

/// StateRootUpdateRecord
#[derive(Debug, Clone, Copy, Default)]
pub struct StateRootUpdateRecord {
    block_number: u64,

    hash_switch: bool,
    db_switch: bool,

    /// total number of tx
    total_txs: u64,
    /// total number of account changes
    account_changes: u64,
    contract_account_changes: u64,
    /// total number of storage changes
    storage_changes: u64,
    mpt_updates_to_db: u64,

    state_write_to_db: u64,

    hashed_state_write: u64,
    hash_state_slow: u64,
    construct_prefix_sets: u64,

    state_calculate: u64,
    state_try_next: u64,
    storage_calculate: u64,
    storage_try_next: u64,

    // state_db_time: DBRead,
    // storage_db_time: DBRead,
    db_read: DBRead,
    state_calculate_record: CaculateStat,
    storage_calculate_record: CaculateStat,

    flush: u64,

    /// Time of the exetion of function keccak256.
    keccak256: u64,
    // keccak256_execution: Keccak256Record,
    keccak256_record: Keccak256Record,

    state_trie_db_read_distribution: TimeDistributionStats,

    /// number of keys updated.
    total_keys: u64,

    account_mpt: MPTStat,
    storage_mpt: MPTStat,

    mpt_delete_branch_number: u64,
    mpt_add_node_number: u64,
    storage_mpt_empty_hash_number: u64,
    storage_mpt_not_empty_hash_number: u64,
}

impl StateRootUpdateRecord {
    pub(crate) fn add(&mut self, other: StateRootUpdateRecord) {
        self.set_block_number(other.block_number);

        self.add_account_changes(other.account_changes);
        self.add_contract_account_changes(other.contract_account_changes);
        self.add_storage_changes(other.storage_changes);
        self.add_mpt_updates_to_db(other.mpt_updates_to_db);
        self.add_state_write_to_db(other.state_write_to_db);
        self.add_hashed_state(other.hashed_state_write);
        self.add_hash_state_slow(other.hash_state_slow);
        self.add_construct_prefix_sets(other.construct_prefix_sets);
        self.add_flush(other.flush);
        self.db_read.add_other(other.db_read);
        self.state_calculate_record.add_other(other.state_calculate_record);
        self.storage_calculate_record.add_other(other.storage_calculate_record);
        self.add_state_calculate(other.state_calculate);
        self.add_state_try_next(other.state_try_next);
        self.add_storage_calculate(other.storage_calculate);
        self.add_storage_try_next(other.storage_try_next);

        self.add_keccak256(other.keccak256);

        // self.keccak256_execution.add_other(other.keccak256_execution);
        self.keccak256_record.add_other(other.keccak256_record);
        self.state_trie_db_read_distribution.update(&other.state_trie_db_read_distribution);

        self.add_total_txs(other.total_txs);
        self.add_total_keys(other.total_keys);

        self.account_mpt.add(other.account_mpt);
        self.storage_mpt.add(other.storage_mpt);

        self.add_mpt_delete_branch_number(other.mpt_delete_branch_number);
        self.add_mpt_add_node_number(other.mpt_add_node_number);
        self.add_storage_mpt_empty_hash_number(other.storage_mpt_empty_hash_number);
        self.add_storage_mpt_not_empty_hash_number(other.storage_mpt_not_empty_hash_number);
    }

    pub(crate) fn set_block_number(&mut self, block_number: u64) {
        self.block_number = block_number;
    }

    pub(crate) fn block_number(&self) -> u64 {
        self.block_number
    }

    pub(crate) fn total_txs(&self) -> u64 {
        self.total_txs
    }

    pub(crate) fn mpt_updates_to_db(&self) -> u64 {
        self.mpt_updates_to_db
    }

    //
    pub(crate) fn total_update_keys(&self) -> u64 {
        self.account_changes.checked_add(self.storage_changes).expect("overflow")
    }

    // avage update keys of one tx
    pub(crate) fn tx_avg_update_keys(&self) -> f64 {
        if 0 == self.total_txs {
            return 0 as f64
        }
        self.total_update_keys() as f64 / self.total_txs as f64
    }

    // average leaf depth in account mpt and storage mpt
    pub(crate) fn mpt_avg_leaf_depth(&self) -> f64 {
        let leaf_number = self.mpt_leaf_number();
        if 0 == leaf_number {
            return 0 as f64
        }

        self.mpt_leaf_depth() as f64 / leaf_number as f64
    }

    // total leaf depth in account mpt and storage mpt div update keys number
    pub(crate) fn keys_update_leaf_avg_depth(&self) -> f64 {
        let keys_number = self.total_update_keys();
        if 0 == keys_number {
            return 0 as f64
        }

        self.mpt_leaf_depth() as f64 / keys_number as f64
    }

    pub(crate) fn keys_update_mpt_avg_dept(&self) -> f64 {
        let total_update_keys = self.total_update_keys();
        if 0 == total_update_keys {
            return 0 as f64
        }

        self.mpt_nodes_depth() as f64 / total_update_keys as f64
    }

    // total node depth of account mpt and storage mpt
    pub(crate) fn mpt_nodes_depth(&self) -> u64 {
        self.account_mpt.root_node.state_node.total_depth
    }

    // average node depth of account mpt and storage mpt
    pub(crate) fn mpt_nodes_avg_depth(&self) -> f64 {
        let number = self.mpt_nodes_number();
        if 0 == number {
            return 0 as f64
        }
        self.mpt_nodes_depth() as f64 / number as f64
    }

    // total node number of account mpt and storage mpt
    pub(crate) fn mpt_nodes_number(&self) -> u64 {
        self.account_mpt.root_node.state_node.total_number
    }

    // total leaf depth of account mpt and storage mpt
    pub(crate) fn mpt_leaf_depth(&self) -> u64 {
        self.account_mpt.root_node.state_node.leaf_depth
    }

    // total leaf number of account mpt and storage mpt
    pub(crate) fn mpt_leaf_number(&self) -> u64 {
        self.account_mpt.root_node.state_node.leaf_number
    }

    pub(crate) fn mpt_updated_branch_nodes(&self) -> u64 {
        self.account_mpt
            .update_branch_number
            .checked_add(self.storage_mpt.update_branch_number)
            .expect("overflow")
    }

    pub(crate) fn mpt_delete_branch_nodes(&self) -> u64 {
        self.account_mpt
            .delete_branch_number
            .checked_add(self.storage_mpt.delete_branch_number)
            .expect("overflow")
    }

    pub(crate) fn account_mpt_updated_branch_nodes(&self) -> u64 {
        self.account_mpt.update_branch_number
    }
    pub(crate) fn account_mpt_delete_branch_nodes(&self) -> u64 {
        self.account_mpt.delete_branch_number
    }

    pub(crate) fn storage_mpt_updated_branch_nodes(&self) -> u64 {
        self.storage_mpt.update_branch_number
    }
    pub(crate) fn storage_mpt_delete_branch_nodes(&self) -> u64 {
        self.storage_mpt.delete_branch_number
    }

    // total node number of account mpt
    pub(crate) fn account_mpt_node_number(&self) -> u64 {
        self.account_mpt.root_node.trie_node.total_number
    }

    // total node depth of account mpt
    pub(crate) fn account_mpt_nodes_depth(&self) -> u64 {
        self.account_mpt.root_node.trie_node.total_depth
    }

    // avg node depth of account mpt
    pub(crate) fn account_mpt_avg_nodes_depth(&self) -> f64 {
        let number = self.account_mpt_node_number();
        if 0 == number {
            return 0 as f64
        }
        self.account_mpt_nodes_depth() as f64 / number as f64
    }

    // total leaf number of account mpt
    pub(crate) fn account_mpt_leaf_number(&self) -> u64 {
        self.account_mpt.root_node.trie_node.leaf_number
    }

    // total leaf depth of account mpt
    pub(crate) fn account_mpt_leaf_depth(&self) -> u64 {
        self.account_mpt.root_node.trie_node.leaf_depth
    }

    // avg leaf depth of account mpt
    pub(crate) fn account_mpt_avg_leaf_depth(&self) -> f64 {
        let number = self.account_mpt_leaf_number();
        if 0 == number {
            return 0 as f64
        }
        self.account_mpt_leaf_depth() as f64 / number as f64
    }

    // total node number of storage mpt
    pub(crate) fn storage_mpt_nodes_number(&self) -> u64 {
        self.storage_mpt.root_node.trie_node.total_number
    }

    // total node depth of storage mpt
    pub(crate) fn storage_mpt_nodes_depth(&self) -> u64 {
        self.storage_mpt.root_node.trie_node.total_depth
    }

    // avg leaf depth of account mpt
    pub(crate) fn storage_mpt_avg_nodes_depth(&self) -> f64 {
        let number = self.storage_mpt_nodes_number();
        if 0 == number {
            return 0 as f64
        }
        self.storage_mpt_nodes_depth() as f64 / number as f64
    }

    // total leaf number of storage mpt
    pub(crate) fn storage_mpt_leaf_number(&self) -> u64 {
        self.storage_mpt.root_node.trie_node.leaf_number
    }

    // total leaf depth of storage mpt
    pub(crate) fn storage_mpt_leaf_depth(&self) -> u64 {
        self.storage_mpt.root_node.trie_node.leaf_depth
    }

    // avg leaf depth of account mpt
    pub(crate) fn storage_mpt_avg_leaf_depth(&self) -> f64 {
        let number = self.storage_mpt_leaf_number();
        if 0 == number {
            return 0 as f64
        }
        self.storage_mpt_leaf_depth() as f64 / number as f64
    }

    pub(crate) fn account_changes(&self) -> u64 {
        self.account_changes
    }

    pub(crate) fn contract_account_changes(&self) -> u64 {
        self.contract_account_changes
    }

    pub(crate) fn storage_changes(&self) -> u64 {
        self.storage_changes
    }

    pub(crate) fn hashed_state_write(&self) -> u64 {
        self.hashed_state_write
    }

    pub(crate) fn state_write_to_db(&self) -> u64 {
        self.state_write_to_db
    }

    pub(crate) fn hash_state_slow(&self) -> u64 {
        self.hash_state_slow
    }

    pub(crate) fn construct_prefix_sets(&self) -> u64 {
        self.construct_prefix_sets
    }

    pub(crate) fn flush(&self) -> u64 {
        self.flush
    }

    pub(crate) fn db_read(&self) -> &DBRead {
        &self.db_read
    }

    pub(crate) fn state_calculate_record(&self) -> &CaculateStat {
        &self.state_calculate_record
    }

    pub(crate) fn storage_calculate_record(&self) -> &CaculateStat {
        &self.storage_calculate_record
    }

    pub(crate) fn state_calculate(&self) -> u64 {
        self.state_calculate
    }

    pub(crate) fn state_try_next(&self) -> u64 {
        self.state_try_next
    }

    pub(crate) fn storage_calculate(&self) -> u64 {
        self.storage_calculate
    }

    pub(crate) fn storage_try_next(&self) -> u64 {
        self.storage_try_next
    }

    pub(crate) fn keccak256(&self) -> u64 {
        self.keccak256
    }

    pub(crate) fn keccak256_execution_count(&self) -> u64 {
        self.keccak256_record.count
    }

    pub(crate) fn keccak256_execution_time(&self) -> u64 {
        self.keccak256_record.time_cycles
    }

    pub(crate) fn keccak256_avg_execution_time(&self) -> u64 {
        let count = self.keccak256_execution_count();
        if count == 0 {
            return 0
        }

        self.keccak256_execution_time() / count
    }

    pub(crate) fn state_trie_db_read(&self) -> &TimeDistributionStats {
        &self.state_trie_db_read_distribution
    }

    pub(crate) fn keys_read_db_avg_count(&self) -> f64 {
        let total_update_keys = self.total_update_keys();
        if 0 == total_update_keys {
            return 0 as f64
        }

        self.db_read.total_count() as f64 / total_update_keys as f64
    }

    pub(crate) fn keys_read_db_avg_time(&self) -> u64 {
        let total_update_keys = self.total_update_keys();
        if 0 == total_update_keys {
            return 0
        }

        self.db_read.total_time() / total_update_keys
    }

    pub(crate) fn is_hashswith_set(&self) -> bool {
        self.hash_switch
    }

    pub(crate) fn set_hashswitch(&mut self, switch: bool) {
        self.hash_switch = switch;
    }

    pub(crate) fn is_dbswith_set(&self) -> bool {
        self.db_switch
    }

    pub(crate) fn set_dbswitch(&mut self, switch: bool) {
        self.db_switch = switch;
    }

    pub(crate) fn add_account_changes(&mut self, number: u64) {
        self.account_changes = self.account_changes.checked_add(number).expect("overflow");
    }

    pub(crate) fn add_contract_account_changes(&mut self, number: u64) {
        self.contract_account_changes =
            self.contract_account_changes.checked_add(number).expect("overflow");
    }

    pub(crate) fn add_storage_changes(&mut self, number: u64) {
        self.storage_changes = self.storage_changes.checked_add(number).expect("overflow");
    }

    pub(crate) fn add_mpt_updates_to_db(&mut self, number: u64) {
        self.mpt_updates_to_db = self.mpt_updates_to_db.checked_add(number).expect("overflow");
    }

    pub(crate) fn add_state_write_to_db(&mut self, time: u64) {
        self.state_write_to_db = self.state_write_to_db.checked_add(time).expect("overflow");
    }

    pub(crate) fn add_hashed_state(&mut self, time: u64) {
        self.hashed_state_write = self.hashed_state_write.checked_add(time).expect("overflow");
    }

    pub(crate) fn add_hash_state_slow(&mut self, time: u64) {
        self.hash_state_slow = self.hash_state_slow.checked_add(time).expect("overflow");
    }

    pub(crate) fn add_construct_prefix_sets(&mut self, time: u64) {
        self.construct_prefix_sets =
            self.construct_prefix_sets.checked_add(time).expect("overflow");
    }

    pub(crate) fn add_flush(&mut self, time: u64) {
        self.flush = self.flush.checked_add(time).expect("overflow");
    }

    pub(crate) fn add_db_current_count(&mut self, count: u64) {
        self.db_read.add_current_count(count);
    }

    pub(crate) fn add_db_current(&mut self, time_cycles: u64) {
        self.db_read.add_current(time_cycles);
    }

    pub(crate) fn add_db_seek_count(&mut self, count: u64) {
        self.db_read.add_seek_count(count);
    }

    pub(crate) fn add_db_seek(&mut self, time_cycles: u64) {
        self.db_read.add_seek(time_cycles);
    }

    pub(crate) fn add_db_next_count(&mut self, count: u64) {
        self.db_read.add_next_count(count);
    }

    pub(crate) fn add_db_next(&mut self, time_cycles: u64) {
        self.db_read.add_next(time_cycles);
    }

    pub(crate) fn add_db_seek_exact_count(&mut self, count: u64) {
        self.db_read.add_seek_exact_count(count);
    }

    pub(crate) fn add_db_seek_exact(&mut self, time_cycles: u64) {
        self.db_read.add_seek_exact(time_cycles);
    }

    pub(crate) fn add_db_seek_by_sub_key_count(&mut self, count: u64) {
        self.db_read.add_seek_by_sub_key_count(count);
    }

    pub(crate) fn add_db_seek_by_sub_key(&mut self, time_cycles: u64) {
        self.db_read.add_seek_by_sub_key(time_cycles);
    }

    pub(crate) fn add_db_next_dup_val_count(&mut self, count: u64) {
        self.db_read.add_next_dup_val_count(count);
    }

    pub(crate) fn add_db_next_dup_val(&mut self, time_cycles: u64) {
        self.db_read.add_next_dup_val(time_cycles);
    }

    pub(crate) fn add_db_account_trie_seek_count(&mut self, count: u64) {
        self.db_read.add_account_trie_seek_count(count);
    }

    pub(crate) fn add_db_account_trie_seek(&mut self, time_cycles: u64) {
        self.db_read.add_account_trie_seek(time_cycles);
    }

    pub(crate) fn add_db_account_trie_seek_exact_count(&mut self, count: u64) {
        self.db_read.add_account_trie_seek_exact_count(count);
    }

    pub(crate) fn add_db_account_trie_seek_exact(&mut self, time_cycles: u64) {
        self.db_read.add_account_trie_seek_exact(time_cycles);
    }

    pub(crate) fn add_db_account_trie_current_count(&mut self, count: u64) {
        self.db_read.add_account_trie_current_count(count);
    }

    pub(crate) fn add_db_account_trie_current(&mut self, time_cycles: u64) {
        self.db_read.add_account_trie_current(time_cycles);
    }

    pub(crate) fn add_db_storage_trie_seek_by_key_subkey_count(&mut self, count: u64) {
        self.db_read.add_storage_trie_seek_by_key_subkey_count(count);
    }

    pub(crate) fn add_db_storage_trie_seek_by_key_subkey(&mut self, time_cycles: u64) {
        self.db_read.add_storage_trie_seek_by_key_subkey(time_cycles);
    }

    pub(crate) fn add_db_storage_trie_current_count(&mut self, count: u64) {
        self.db_read.add_storage_trie_current_count(count);
    }

    pub(crate) fn add_db_storage_trie_current(&mut self, time_cycles: u64) {
        self.db_read.add_storage_trie_current(time_cycles);
    }

    pub(crate) fn add_db_hash_account_cursor_seek_hit_count(&mut self, count: u64) {
        self.db_read.add_hash_account_cursor_seek_hit_count(count);
    }
    pub(crate) fn add_db_hash_storage_cursor_seek_hit_count(&mut self, count: u64) {
        self.db_read.add_hash_storage_cursor_seek_hit_count(count);
    }

    pub(crate) fn add_state_calculate(&mut self, time_cycles: u64) {
        self.state_calculate = self.state_calculate.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_state_record_calculate(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_total_time(time_cycles);
    }

    pub(crate) fn add_state_before_loop(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_before_loop(time_cycles);
    }

    pub(crate) fn add_state_loop_begin(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_loop_begin(time_cycles);
    }

    pub(crate) fn add_state_try_next_stat_total_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_count(count);
    }

    pub(crate) fn add_state_try_next_stat_total_time(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_try_next_stat_time(time_cycles);
    }

    pub(crate) fn add_state_try_next_stat_skip_branch_node_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_skip_branch_node_count(count);
    }

    pub(crate) fn add_state_try_next_stat_leaf_miss_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_leaf_miss_count(count);
    }

    pub(crate) fn add_state_try_next_stat_leaf_hit_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_leaf_hit_count(count);
    }

    pub(crate) fn add_state_try_next_stat_walk_next_unprocessed_key_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_walk_next_unprocessed_key_count(count);
    }

    pub(crate) fn add_state_try_next_stat_walk_advance_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_walk_advance_count(count);
    }

    pub(crate) fn add_state_try_next_stat_loop_count(&mut self, count: u64) {
        self.state_calculate_record.add_try_next_stat_loop_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_total_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_total_time(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_try_next_stat_time(time_cycles);
    }

    pub(crate) fn add_storage_try_next_stat_skip_branch_node_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_skip_branch_node_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_leaf_miss_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_leaf_miss_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_leaf_hit_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_leaf_hit_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_walk_next_unprocessed_key_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_walk_next_unprocessed_key_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_walk_advance_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_walk_advance_count(count);
    }

    pub(crate) fn add_storage_try_next_stat_loop_count(&mut self, count: u64) {
        self.storage_calculate_record.add_try_next_stat_loop_count(count);
    }

    pub(crate) fn add_state_try_next(&mut self, time_cycles: u64) {
        self.state_try_next = self.state_try_next.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_state_add_branch_count(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_branch_count(time_cycles);
    }

    pub(crate) fn add_state_add_branch(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_branch(time_cycles);
    }

    pub(crate) fn add_state_cal_storage_root_and_add_leaf(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_cal_storage_root_and_add_leaf(time_cycles);
    }

    pub(crate) fn add_state_after_cal_storage_root(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_after_cal_storage_root(time_cycles);
    }

    pub(crate) fn add_state_add_leaf_count(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_leaf_count(time_cycles);
    }

    pub(crate) fn add_state_add_leaf(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_leaf(time_cycles);
    }

    pub(crate) fn add_state_add_root_count(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_root_count(time_cycles);
    }

    pub(crate) fn add_state_add_root(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_add_root(time_cycles);
    }

    pub(crate) fn add_state_add_after_loop(&mut self, time_cycles: u64) {
        self.state_calculate_record.add_after_loop(time_cycles);
    }

    pub(crate) fn add_storage_calculate(&mut self, time_cycles: u64) {
        self.storage_calculate = self.storage_calculate.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_storage_record_calculate(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_total_time(time_cycles);
    }

    pub(crate) fn add_storage_before_loop(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_before_loop(time_cycles);
    }

    pub(crate) fn add_storage_loop_begin(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_loop_begin(time_cycles);
    }

    pub(crate) fn add_storage_try_next(&mut self, time_cycles: u64) {
        self.storage_try_next = self.storage_try_next.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_storage_add_branch_count(&mut self, count: u64) {
        self.storage_calculate_record.add_add_branch_count(count);
    }

    pub(crate) fn add_storage_add_branch(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_add_branch(time_cycles);
    }

    pub(crate) fn add_storage_add_leaf_count(&mut self, count: u64) {
        self.storage_calculate_record.add_add_leaf_count(count);
    }

    pub(crate) fn add_storage_add_leaf(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_add_leaf(time_cycles);
    }

    pub(crate) fn add_storage_add_root_count(&mut self, count: u64) {
        self.storage_calculate_record.add_add_root_count(count);
    }

    pub(crate) fn add_storage_add_root(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_add_root(time_cycles);
    }

    pub(crate) fn add_storage_add_after_loop(&mut self, time_cycles: u64) {
        self.storage_calculate_record.add_after_loop(time_cycles);
    }

    pub(crate) fn add_keccak256(&mut self, time_cycles: u64) {
        self.keccak256 = self.keccak256.checked_add(time_cycles).expect("overflow");
    }

    pub(crate) fn add_keccak256_record(&mut self, other: Keccak256Record) {
        self.keccak256_record.add_other(other);
    }

    // pub(crate) fn add_keccak256_execution(&mut self, count: u64, time: u64) {
    //     self.keccak256_execution.add(count, time);
    // }

    pub(crate) fn add_db_read_time(&mut self, _read_count: u64, time_cycles: u64) {
        let time_ns = convert_cycles_to_ns_f64(time_cycles);

        self.state_trie_db_read_distribution.record(time_ns);
    }

    pub(crate) fn add_total_txs(&mut self, txs: u64) {
        self.total_txs = self.total_txs.checked_add(txs).expect("overflow");
    }

    pub(crate) fn add_total_keys(&mut self, keys: u64) {
        self.total_keys = self.total_keys.checked_add(keys).expect("overflow");
    }

    pub(crate) fn add_account_mpt_info(
        &mut self,
        tree_node: TreeNode,
        delete_branch: u64,
        update_branch: u64,
    ) {
        self.account_mpt.add_nodes(tree_node);
        self.account_mpt.add_updates(delete_branch, update_branch);
    }

    // pub(crate) fn add_old_account_mpt_leaf(&mut self, tree_nodes: TreeNode) {
    //     self.old_account_mpt_leaf.add(&tree_nodes);
    // }

    pub(crate) fn add_storage_mpt_info(
        &mut self,
        tree_node: TreeNode,
        delete_branch: u64,
        update_branch: u64,
    ) {
        self.storage_mpt.add_nodes(tree_node);
        self.storage_mpt.add_updates(delete_branch, update_branch);
    }

    // pub(crate) fn add_old_storage_mpt_leaf(&mut self, tree_node: TreeNode) {
    //     self.old_storage_mpt_leaf.add(&tree_node);
    // }

    pub(crate) fn add_mpt_add_node_number(&mut self, number: u64) {
        self.mpt_add_node_number = self.mpt_add_node_number.checked_add(number).expect("overflow");
    }
    pub(crate) fn mpt_add_node_number(&self) -> u64 {
        self.mpt_add_node_number
    }

    pub(crate) fn add_storage_mpt_empty_hash_number(&mut self, number: u64) {
        self.storage_mpt_empty_hash_number =
            self.storage_mpt_empty_hash_number.checked_add(number).expect("overflow");
    }

    pub(crate) fn storage_mpt_empty_hash_number(&self) -> u64 {
        self.storage_mpt_empty_hash_number
    }

    pub(crate) fn add_storage_mpt_not_empty_hash_number(&mut self, number: u64) {
        self.storage_mpt_not_empty_hash_number =
            self.storage_mpt_not_empty_hash_number.checked_add(number).expect("overflow");
    }

    pub(crate) fn storage_mpt_not_empty_hash_number(&self) -> u64 {
        self.storage_mpt_not_empty_hash_number
    }

    pub(crate) fn add_mpt_delete_branch_number(&mut self, number: u64) {
        self.mpt_delete_branch_number =
            self.mpt_delete_branch_number.checked_add(number).expect("overflow");
    }

    pub(crate) fn mpt_delete_branch_number(&self) -> u64 {
        self.mpt_delete_branch_number
    }
}

#[macro_export]
macro_rules! timeRecorder {
    ($struct_name:ident | $function_name:ident) => {
        pub struct $struct_name {
            start: Instant,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self { start: Instant::now() }
            }
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
                super::metric::$function_name(time_cycles);
            }
        }
    };
}

#[macro_export]
macro_rules! counterRecorder {
    ($struct_name:ident | $function_name:ident) => {
        pub struct $struct_name {
            start: Instant,
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self { start: Instant::now() }
            }
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
                super::metric::$function_name(1, time_cycles);
            }
        }
    };
}
timeRecorder!(StateWriteToDBRecord | record_state_write_to_db);
timeRecorder!(StateCaculateRecord | add_state_record_calculate);
timeRecorder!(StateBeforeLoopRecord | add_state_before_loop);
timeRecorder!(StateLoopBeginRecord | add_state_loop_begin);
timeRecorder!(StateCalStorageRootAndAddLeafRecord | add_state_cal_storage_root_and_add_leaf);
timeRecorder!(StateAfterCalStorageRootRecord | add_state_after_cal_storage_root);
timeRecorder!(StateAfterLoopRecord | add_state_add_after_loop);
timeRecorder!(StorageCalculateRecord | add_storage_record_calculate);
timeRecorder!(StorageBeforeLoopRecord | add_storage_before_loop);
timeRecorder!(StorageLoopBeginRecord | add_storage_loop_begin);
timeRecorder!(StorageAfterLoopRecord | add_storage_add_after_loop);

counterRecorder!(StateTryNextStatRecord | add_state_try_next_stat_total_time);
counterRecorder!(StorageTryNextStatRecord | add_storage_try_next_stat_total_count_time);

counterRecorder!(StateAddBranchRecord | add_state_add_branch);
counterRecorder!(StateAddLeafRecord | add_state_add_leaf);
counterRecorder!(StorageAddBranchRecord | add_storage_add_branch);
counterRecorder!(StorageAddLeafRecord | add_storage_add_leaf);
counterRecorder!(StateAddRootRecord | add_state_add_root);
counterRecorder!(StorageAddRootRecord | add_storage_add_root);

counterRecorder!(DBCurrentRead | add_db_current);
counterRecorder!(DBSeekRead | add_db_seek);
counterRecorder!(DBNextRead | add_db_next);
counterRecorder!(DBSeekExactRead | add_db_seek_exact);
counterRecorder!(DBSeekBySubKeyRead | add_db_seek_by_sub_key);
counterRecorder!(DBNextDupValRead | add_db_next_dup_val);
counterRecorder!(DBAccountTrieSeekRead | add_db_account_trie_seek);
counterRecorder!(DBAccountTrieSeekExactRead | add_db_account_trie_seek_exact);
counterRecorder!(DBAccountTrieCurrentRead | add_db_account_trie_current);
counterRecorder!(DBStorageTrieSeekBySubKeyRead | add_db_storage_trie_seek_by_subkey);
counterRecorder!(DBStorageTrieCurrentRead | add_db_storage_trie_current);

pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self { start: Instant::now() }
    }
}

impl Timer {
    pub fn cycles_since(&self) -> u64 {
        Instant::now().checked_cycles_since(self.start).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FunctionName {
    HashedState,
    HashedStateSlow,
    ConstructPrefixSets,
    Flush,
    StateCalculate,
    StateTryNext,
    StorageCalculate,
    StorageTryNext,
    // Keccak256,
}

pub struct TimeRecorder2 {
    function: FunctionName,
    start: Instant,
}

impl TimeRecorder2 {
    pub fn new(function: FunctionName) -> Self {
        Self { function, start: Instant::now() }
    }
}

impl Drop for TimeRecorder2 {
    fn drop(&mut self) {
        let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();

        match self.function {
            FunctionName::StateTryNext => {
                super::metric::add_state_try_next(time_cycles);
            }
            FunctionName::StorageTryNext => {
                super::metric::add_storage_try_next(time_cycles);
            }
            FunctionName::HashedState => {
                super::metric::record_hashed_state(time_cycles);
            }

            FunctionName::HashedStateSlow => {
                super::metric::record_hashed_state_slow(time_cycles);
            }

            FunctionName::ConstructPrefixSets => {
                super::metric::record_construct_prefix_sets(time_cycles);
            }

            FunctionName::Flush => {
                super::metric::record_flush(time_cycles);
            }

            FunctionName::StateCalculate => {
                super::metric::add_state_calculate(time_cycles);
            }
            FunctionName::StorageCalculate => {
                super::metric::add_storage_calculate(time_cycles);
            } /* FunctionName::Keccak256 => {
               *     super::metric::record_keccak256(time_cycles);
               *     super::metric::add_keccak256_execution(1, time_cycles);
               * } */
        }
    }
}

pub struct HashSwither {}
impl HashSwither {
    pub fn new() -> Self {
        super::metric::set_hashswitch(true);
        Self {}
    }
}

impl Drop for HashSwither {
    fn drop(&mut self) {
        super::metric::set_hashswitch(false);
    }
}

pub struct DBSwither {}
impl DBSwither {
    pub fn new() -> Self {
        super::metric::set_dbswitch(true);
        Self {}
    }
}

impl Drop for DBSwither {
    fn drop(&mut self) {
        super::metric::set_dbswitch(false);
    }
}

// #[derive(Debug, Default, Copy, Clone)]
// pub struct Keccak256Execution {
//     pub count: u64,
//     pub time_cycles: u64,
// }

// impl Keccak256Execution {
//     pub fn new() -> Self {
//         Self { count: 0, time_cycles: 0 }
//     }

//     pub fn add_other(&mut self, other: Self) {
//         self.count = self.count.checked_add(other.count).expect("overflow");
//         self.time_cycles = self.time_cycles.checked_add(other.time_cycles).expect("overvflow");
//     }

//     pub fn add(&mut self, count: u64, time_cycles: u64) {
//         self.count = self.count.checked_add(count).expect("overflow");
//         self.time_cycles = self.time_cycles.checked_add(time_cycles).expect("overflow");
//     }
// }
