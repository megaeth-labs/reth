//! This module provides a metric to measure reth.
// #[cfg(feature = "enable_execution_duration_record")]
// pub use super::duration::ExecuteTxsRecord;
#[cfg(feature = "enable_execution_duration_record")]
use super::duration::ExecutionDurationRecord;
#[cfg(feature = "enable_tps_gas_record")]
pub use super::tps_gas::TpsAndGasMessage;
#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsGasRecord;
#[cfg(feature = "enable_state_root_record")]
use crate::state_root::StateRootRecord;
#[cfg(feature = "enable_state_root_record")]
use crate::StateRootUpdateRecord;
#[cfg(feature = "enable_state_root_record")]
use alloy_trie::utils::{metric::Keccak256Record, TreeNode};
#[cfg(feature = "enable_cache_record")]
use revm_utils::metrics::types::CacheDbRecord;
#[cfg(feature = "enable_opcode_metrics")]
use revm_utils::metrics::types::OpcodeRecord;
use tokio::sync::mpsc::UnboundedSender;

pub use super::execute_measure::execute_inner::*;
#[cfg(feature = "enable_opcode_metrics")]
pub use super::execute_measure::revm_measure::*;
#[cfg(feature = "enable_execution_duration_record")]
pub use super::execute_measure::{execute_txs::*, write_to_db::*};

/// Alias type for metric producers to use.
pub type MetricEventsSender = UnboundedSender<MetricEvent>;

/// Collection of metric events.
#[derive(Clone, Copy, Debug)]
pub enum MetricEvent {
    /// Duration record of function execute_inner.
    #[cfg(feature = "enable_execution_duration_record")]
    ExecutionStageTime {
        /// Current block_number.
        block_number: u64,
        /// excution duration record.
        record: ExecutionDurationRecord,
    },
    /// Amount of txs and gas in a block.
    #[cfg(feature = "enable_tps_gas_record")]
    BlockTpsAndGas {
        /// Current block_number.
        block_number: u64,
        /// tps and gas record.
        record: TpsAndGasMessage,
    },
    /// Opcode record in revm.
    #[cfg(feature = "enable_opcode_metrics")]
    OpcodeInfo {
        /// Current block_number.
        block_number: u64,
        /// opcode record in revm.
        record: OpcodeRecord,
    },
    /// CacheDB metric record.
    #[cfg(feature = "enable_cache_record")]
    CacheDbInfo {
        /// Current block_number.
        block_number: u64,
        /// cache db size.
        size: usize,
        /// cache db record.
        record: CacheDbRecord,
    },
    #[cfg(feature = "enable_state_root_record")]
    StateRootUpdate { record: StateRootUpdateRecord },
    #[cfg(feature = "enable_state_root_record")]
    StateRootRecordUpdate { record: StateRootRecord },
    #[cfg(feature = "enable_state_root_record")]
    StateRootUpdatePrint {},
}

/// This structure is used to facilitate all metric operations in reth's performance test.
#[derive(Default)]
pub struct PerfMetric {
    /// Record the time consumption of each function in execution stage.
    #[cfg(feature = "enable_execution_duration_record")]
    pub(crate) duration_record: ExecutionDurationRecord,
    /// Record tps and gas.
    #[cfg(feature = "enable_tps_gas_record")]
    pub(crate) tps_gas_record: TpsGasRecord,
    /// Record cache hits, number of accesses, and memory usage.
    #[cfg(feature = "enable_cache_record")]
    pub(crate) cachedb_record: CacheDbRecord,
    /// Record information on instruction execution.
    #[cfg(feature = "enable_opcode_metrics")]
    pub(crate) op_record: OpcodeRecord,
    ///
    #[cfg(feature = "enable_state_root_record")]
    state_root_update_record: StateRootUpdateRecord,

    #[cfg(feature = "enable_state_root_record")]
    state_root_record: StateRootRecord,

    /// A channel for sending recorded indicator information to the dashboard for display.
    pub(crate) events_tx: Option<MetricEventsSender>,

    /// Used to record the current block_number.
    pub(crate) block_number: u64,
}

static mut METRIC_RECORDER: Option<PerfMetric> = None;

#[ctor::ctor]
fn init() {
    unsafe {
        METRIC_RECORDER = Some(PerfMetric::default());
    }
}

pub fn set_metric_event_sender(events_tx: MetricEventsSender) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.events_tx = Some(events_tx);
    }
}

pub(crate) fn recorder<'a>() -> &'a mut PerfMetric {
    unsafe { METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!") }
}

// *************************************************************************************************
// The functions in the following range is for the feature enable_state_root_record.
//
// *************************************************************************************************

#[cfg(feature = "enable_state_root_record")]
pub fn reset_state_root_update_record(block_number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record = StateRootUpdateRecord::default();
        _record.state_root_update_record.set_block_number(block_number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn set_hashswitch(switch: bool) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.set_hashswitch(switch);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn set_dbswitch(switch: bool) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.set_dbswitch(switch);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn record_changes(account_number: u64, contract_account_number: u64, storage_number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_account_changes(account_number);
        _record.state_root_update_record.add_contract_account_changes(contract_account_number);
        _record.state_root_update_record.add_storage_changes(storage_number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn record_mpt_updates_to_db(number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_mpt_updates_to_db(number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_current(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_current_count(count);
            _record.state_root_update_record.add_db_current(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_seek(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_seek_count(count);
            _record.state_root_update_record.add_db_seek(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_next(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_next_count(count);
            _record.state_root_update_record.add_db_next(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_seek_exact(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_seek_exact_count(count);
            _record.state_root_update_record.add_db_seek_exact(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_seek_by_sub_key(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_seek_by_sub_key_count(count);
            _record.state_root_update_record.add_db_seek_by_sub_key(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_next_dup_val(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_next_dup_val_count(count);
            _record.state_root_update_record.add_db_next_dup_val(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_account_trie_seek(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_account_trie_seek_count(count);
            _record.state_root_update_record.add_db_account_trie_seek(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_account_trie_seek_exact(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_account_trie_seek_exact_count(count);
            _record.state_root_update_record.add_db_account_trie_seek_exact(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_account_trie_current(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_account_trie_current_count(count);
            _record.state_root_update_record.add_db_account_trie_current(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_storage_trie_seek(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_account_trie_seek_count(count);
            _record.state_root_update_record.add_db_account_trie_seek(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_storage_trie_seek_by_subkey(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_storage_trie_seek_by_key_subkey_count(count);
            _record.state_root_update_record.add_db_storage_trie_seek_by_key_subkey(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_storage_trie_current(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_dbswith_set() {
            _record.state_root_update_record.add_db_read_time(count, time_cycles);
            _record.state_root_update_record.add_db_storage_trie_current_count(count);
            _record.state_root_update_record.add_db_storage_trie_current(time_cycles);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_hash_account_cursor_seek_hit_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_db_hash_account_cursor_seek_hit_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_db_hash_storage_cursor_seek_hit_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_db_hash_storage_cursor_seek_hit_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_calculate(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_calculate(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_record_calculate(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_record_calculate(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_before_loop(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_before_loop(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_loop_begin(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_loop_begin(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_total_time(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_total_count(count);
        _record.state_root_update_record.add_state_try_next_stat_total_time(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_skip_branch_node_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_skip_branch_node_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_leaf_miss_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_leaf_miss_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_leaf_hit_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_leaf_hit_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_walk_next_unprocessed_key_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record
            .state_root_update_record
            .add_state_try_next_stat_walk_next_unprocessed_key_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_walk_advance_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_walk_advance_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next_stat_loop_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next_stat_loop_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_total_count_time(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_total_count(count);
        _record.state_root_update_record.add_storage_try_next_stat_total_time(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_skip_branch_node_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_skip_branch_node_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_leaf_miss_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_leaf_miss_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_leaf_hit_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_leaf_hit_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_walk_next_unprocessed_key_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record
            .state_root_update_record
            .add_storage_try_next_stat_walk_next_unprocessed_key_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_walk_advance_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_walk_advance_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next_stat_loop_count(count: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next_stat_loop_count(count);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_try_next(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_try_next(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_try_next(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_try_next(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_add_branch(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_add_branch_count(count);
        _record.state_root_update_record.add_state_add_branch(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_cal_storage_root_and_add_leaf(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_cal_storage_root_and_add_leaf(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_after_cal_storage_root(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_after_cal_storage_root(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_add_leaf(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_add_leaf_count(count);
        _record.state_root_update_record.add_state_add_leaf(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_add_root(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_add_root_count(count);
        _record.state_root_update_record.add_state_add_root(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_state_add_after_loop(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_add_after_loop(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_calculate(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_calculate(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_record_calculate(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_record_calculate(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_before_loop(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_before_loop(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_loop_begin(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_loop_begin(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_add_branch(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_add_branch_count(count);
        _record.state_root_update_record.add_storage_add_branch(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_add_leaf(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_add_leaf_count(count);
        _record.state_root_update_record.add_storage_add_leaf(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_add_root(count: u64, time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_add_root_count(count);
        _record.state_root_update_record.add_storage_add_root(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_add_after_loop(time_cycles: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_add_after_loop(time_cycles);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn record_keccak256(record: Keccak256Record) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_hashswith_set() {
            _record.state_root_update_record.add_keccak256_record(record);
        }
    }
}

// #[cfg(feature = "enable_state_root_record")]
// pub fn add_keccak256_execution(count: u64, time: u64) {
//     unsafe {
//         let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
//         if _record.state_root_update_record.is_hashswith_set() {
//             _record.state_root_update_record.add_keccak256_execution(count, time);
//         }
//     }
// }

#[cfg(feature = "enable_state_root_record")]
pub(crate) fn record_state_write_to_db(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_write_to_db(time);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub(crate) fn record_hashed_state(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_hashed_state(time);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub(crate) fn record_hashed_state_slow(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_hash_state_slow(time);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub(crate) fn record_construct_prefix_sets(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_construct_prefix_sets(time);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub(crate) fn record_flush(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_flush(time);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn record_total_txs(txs: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_total_txs(txs);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn record_total_keys(total_key: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_total_keys(total_key);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_account_mpt_info(tree_node: TreeNode, delete_branch: u64, update_branch: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_account_mpt_info(
            tree_node,
            delete_branch,
            update_branch,
        );
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_mpt_add_node_number(number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_mpt_add_node_number(number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_mpt_empty_hash_number(number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_mpt_empty_hash_number(number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_mpt_not_empty_hash_number(number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_mpt_not_empty_hash_number(number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_mpt_delete_branch_number(number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_mpt_delete_branch_number(number);
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_storage_mpt_info(tree_node: TreeNode, delete_branch: u64, update_branch: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_storage_mpt_info(
            tree_node,
            delete_branch,
            update_branch,
        );
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn send_state_root_update_message() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_state_root_record")]
        {
            let _ =
                _record.events_tx.as_mut().expect("No sender").send(MetricEvent::StateRootUpdate {
                    record: _record.state_root_update_record,
                });

            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::StateRootRecordUpdate { record: _record.state_root_record });

            // println!(
            //     "\nmac-at-seek-exact:{:?},{:?}\n\n",
            //     _record.state_root_record.db_read.at_seek_exact_count,
            //     _record.state_root_record.db_read.at_seek_exact_time
            // );
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn send_state_root_update_print_message() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_state_root_record")]
        {
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::StateRootUpdatePrint {});
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
macro_rules! addFunction {
    ($function_name:ident | [$($field_name:ident)|+] | [$($var_name:ident)|+]) => {
        pub fn $function_name($($var_name: u64),*) {
            unsafe {
                let _record =  $crate::metrics::metric::METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
                let tmp = &mut _record.$($field_name).*;

                $(
                    tmp.$var_name = tmp.$var_name.checked_add($var_name).expect("overflow");
                )*
            }
        }
    };
}

#[cfg(feature = "enable_state_root_record")]
pub mod state_root {
    pub mod common {
        pub fn set_block_number(block_number: u64) {
            unsafe {
                let recorder = crate::metrics::metric::METRIC_RECORDER
                    .as_mut()
                    .expect("Metric recorder should not empty!");
                recorder.state_root_record.block_number = block_number;
            }
        }

        addFunction!(add_update_keys | [state_root_record] | [account_changes | storage_changes]);
        addFunction!(add_total_txs_count | [state_root_record] | [total_txs_count]);
        addFunction!(add_total_keys_count | [state_root_record] | [total_keys_count]);
        addFunction!(add_account_changes_count | [state_root_record] | [account_changes]);
        addFunction!(add_storage_changes_count | [state_root_record] | [storage_changes]);
        addFunction!(add_construct_prefix_sets_time | [state_root_record] | [construct_prefix_sets_time]);
        addFunction!(add_state_write_to_db_time | [state_root_record] | [state_write_to_db_time]);
        addFunction!(add_hashed_state_write_time | [state_root_record] | [hashed_state_write_time]);
        addFunction!(add_hash_state_slow_time | [state_root_record] | [hash_state_slow_time]);
        addFunction!(add_flush_time | [state_root_record] | [flush_time]);
    }
    pub mod try_next {
        addFunction!(
            add_state_count_and_time |
                [state_root_record | state_calculate_record | try_next_stat] |
                [total_count | total_time]
        );
        addFunction!(
            add_state_skip_branch_node_count |
                [state_root_record | state_calculate_record | try_next_stat] |
                [skip_branch_node_count]
        );
        addFunction!(
            add_state_leaf_miss_count |
                [state_root_record | state_calculate_record | try_next_stat] |
                [leaf_miss_count]
        );
        addFunction!(
            add_state_leaf_hit_count |
                [state_root_record | state_calculate_record | try_next_stat] |
                [leaf_hit_count]
        );

        addFunction!(
            add_storage_count_and_time |
                [state_root_record | storage_calculate_record | try_next_stat] |
                [total_count | total_time]
        );
        addFunction!(
            add_storage_skip_branch_node_count |
                [state_root_record | storage_calculate_record | try_next_stat] |
                [skip_branch_node_count]
        );

        addFunction!(
            add_storage_leaf_miss_count |
                [state_root_record | storage_calculate_record | try_next_stat] |
                [leaf_miss_count]
        );
        addFunction!(
            add_storage_leaf_hit_count |
                [state_root_record | storage_calculate_record | try_next_stat] |
                [leaf_hit_count]
        );
    }

    pub mod caculate {
        addFunction!(
            add_state_calculate_time | [state_root_record | state_calculate_record] | [total_time]
        );
        addFunction!(
            add_state_before_loop_time |
                [state_root_record | state_calculate_record] |
                [before_loop_time]
        );
        addFunction!(
            add_state_loop_begin_time |
                [state_root_record | state_calculate_record] |
                [loop_begin_time]
        );

        addFunction!(
            add_state_add_branch |
                [state_root_record | state_calculate_record] |
                [add_branch_count | add_branch_time]
        );

        addFunction!(
            add_state_cal_storage_root_and_add_leaf_time |
                [state_root_record | state_calculate_record] |
                [cal_storage_root_and_add_leaf_time]
        );

        addFunction!(
            add_state_after_cal_storage_root_time |
                [state_root_record | state_calculate_record] |
                [after_cal_storage_root_time]
        );

        addFunction!(
            add_state_add_leaf |
                [state_root_record | state_calculate_record] |
                [add_leaf_count | add_leaf_time]
        );

        addFunction!(
            add_state_add_root |
                [state_root_record | state_calculate_record] |
                [add_root_count | add_root_time]
        );
        addFunction!(
            add_state_after_add_root_time |
                [state_root_record | state_calculate_record] |
                [after_add_root_time]
        );

        addFunction!(
            add_storage_calculate_time |
                [state_root_record | storage_calculate_record] |
                [total_time]
        );
        addFunction!(
            add_storage_before_loop_time |
                [state_root_record | storage_calculate_record] |
                [before_loop_time]
        );
        addFunction!(
            add_storage_loop_begin_time |
                [state_root_record | storage_calculate_record] |
                [loop_begin_time]
        );

        addFunction!(
            add_storage_add_branch |
                [state_root_record | storage_calculate_record] |
                [add_branch_count | add_branch_time]
        );

        addFunction!(
            add_storage_add_leaf |
                [state_root_record | storage_calculate_record] |
                [add_leaf_count | add_leaf_time]
        );

        addFunction!(
            add_storage_add_root |
                [state_root_record | storage_calculate_record] |
                [add_root_count | add_root_time]
        );
        addFunction!(
            add_storage_after_add_root_time |
                [state_root_record | storage_calculate_record] |
                [after_add_root_time]
        );
    }

    pub mod hash {
        pub fn add_keccak256(record: alloy_trie::Keccak256Record) {
            unsafe {
                let recorder = crate::metrics::metric::METRIC_RECORDER
                    .as_mut()
                    .expect("Metric recorder should not empty!");
                recorder.state_root_record.keccak256_record.add_other(record);
            }
        }
    }

    pub mod mpt {
        pub fn add_state_mpt_info(
            tree_node: alloy_trie::TreeNode,
            delete_branch: u64,
            update_branch: u64,
        ) {
            unsafe {
                let recorder = crate::metrics::metric::METRIC_RECORDER
                    .as_mut()
                    .expect("Metric recorder should not empty!");
                recorder.state_root_record.state_mpt.add_node(tree_node);
                recorder.state_root_record.state_mpt.add_updates(delete_branch, update_branch);
            }
        }

        pub fn add_storage_mpt_info(
            tree_node: alloy_trie::TreeNode,
            delete_branch: u64,
            update_branch: u64,
        ) {
            unsafe {
                let recorder = crate::metrics::metric::METRIC_RECORDER
                    .as_mut()
                    .expect("Metric recorder should not empty!");
                recorder.state_root_record.storage_mpt.add_node(tree_node);
                recorder.state_root_record.storage_mpt.add_updates(delete_branch, update_branch);
            }
        }
    }

    pub mod db {
        addFunction!(add_current | [state_root_record | db_read] | [current_count | current_time]);
        addFunction!(add_seek | [state_root_record | db_read] | [seek_count | seek_time]);
        addFunction!(add_next | [state_root_record | db_read] | [next_count | next_time]);
        addFunction!(
            add_seek_exact | [state_root_record | db_read] | [seek_exact_count | seek_exact_time]
        );
        addFunction!(
            add_seek_by_sub_key |
                [state_root_record | db_read] |
                [seek_by_sub_key_count | seek_by_sub_key_time]
        );
        addFunction!(
            add_next_dup_val |
                [state_root_record | db_read] |
                [next_dup_val_count | next_dup_val_time]
        );
        addFunction!(add_at_seek | [state_root_record | db_read] | [at_seek_count | at_seek_time]);
        addFunction!(
            add_at_seek_exact |
                [state_root_record | db_read] |
                [at_seek_exact_count | at_seek_exact_time]
        );
        addFunction!(
            add_at_current | [state_root_record | db_read] | [at_current_count | at_current_time]
        );
        addFunction!(
            add_st_seek_by_subkey |
                [state_root_record | db_read] |
                [st_seek_by_subkey_count | st_seek_by_subkey_time]
        );
        addFunction!(
            add_st_current | [state_root_record | db_read] | [st_current_count | st_current_time]
        );

        pub fn record_distribution(time_in_ns: f64) {
            unsafe {
                let recorder = crate::metrics::metric::METRIC_RECORDER
                    .as_mut()
                    .expect("Metric recorder should not empty!");
                recorder.state_root_record.db_read_distribution.record(time_in_ns);
            }
        }
    }

    // type AddFun = fn(u64);
    // pub struct DBRecorder {
    //     pub(crate) count_fun: AddFun,
    //     pub(crate) time_fun: AddFun,

    //     start: revm_utils::time_utils::instant::Instant,
    // }

    // impl DBRecorder {
    //     pub fn new(count_fun: AddFun, time_fun: AddFun) -> Self {
    //         Self { count_fun, time_fun, start: Instant::now() }
    //     }
    // }

    // impl Drop for DBRecorder {
    //     fn drop(&mut self) {
    //         (self.count_fun)(1);
    //         let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
    //         (self.time_fun)(time_cycles);

    //         let time_ns = convert_cycles_to_ns_f64(time_cycles);
    //         crate::metrics::metric::state_root::db::record_distribution(time_ns);
    //     }
    // }

    // pub struct CountAndTimeRecorder {
    //     pub(crate) count_fun: AddFun,
    //     pub(crate) time_fun: AddFun,

    //     start: revm_utils::time_utils::instant::Instant,
    // }

    // impl CountAndTimeRecorder {
    //     pub fn new(count_fun: AddFun, time_fun: AddFun) -> Self {
    //         Self { count_fun, time_fun, start: Instant::now() }
    //     }
    // }

    // impl Drop for CountAndTimeRecorder {
    //     fn drop(&mut self) {
    //         (self.count_fun)(1);
    //         let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
    //         (self.time_fun)(time_cycles);
    //     }
    // }

    // pub struct TimeRecorder {
    //     pub(crate) time_fun: AddFun,

    //     start: revm_utils::time_utils::instant::Instant,
    // }

    // impl TimeRecorder {
    //     pub fn new(time_fun: AddFun) -> Self {
    //         Self { time_fun, start: Instant::now() }
    //     }
    // }

    // impl Drop for TimeRecorder {
    //     fn drop(&mut self) {
    //         let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
    //         (self.time_fun)(time_cycles);
    //     }
    // }

    // pub struct DBSwither {}
    // impl DBSwither {
    //     pub fn new() -> Self {
    //         unsafe {
    //             let recorder = crate::metrics::metric::METRIC_RECORDER
    //                 .as_mut()
    //                 .expect("Metric recorder should not empty!");
    //             recorder.stete_root_record.db_switch = true;
    //         }
    //         Self {}
    //     }
    // }
    // impl Drop for DBSwither {
    //     fn drop(&mut self) {
    //         unsafe {
    //             let recorder = crate::metrics::metric::METRIC_RECORDER
    //                 .as_mut()
    //                 .expect("Metric recorder should not empty!");
    //             recorder.stete_root_record.db_switch = false;
    //         }
    //     }
    // }
}

// *************************************************************************************************
//                              functions for the feature enable_state_root_record end
// *************************************************************************************************
