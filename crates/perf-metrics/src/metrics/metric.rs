//! This module provides a metric to measure reth.
#[cfg(feature = "enable_execution_duration_record")]
use super::duration::{ExecuteTxsRecord, ExecutionDurationRecord};
#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::{TpsAndGasMessage, TpsGasRecord};
#[cfg(feature = "enable_cache_record")]
use revm_utils::metrics::types::CacheDbRecord;
#[cfg(feature = "enable_opcode_metrics")]
use revm_utils::metrics::types::OpcodeRecord;

use tokio::sync::mpsc::UnboundedSender;

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
}

/// This structure is used to facilitate all metric operations in reth's performance test.
#[derive(Default)]
struct PerfMetric {
    /// Record the time consumption of each function in execution stage.
    #[cfg(feature = "enable_execution_duration_record")]
    duration_record: ExecutionDurationRecord,
    /// Record tps and gas.
    #[cfg(feature = "enable_tps_gas_record")]
    tps_gas_record: TpsGasRecord,
    /// Record cache hits, number of accesses, and memory usage.
    #[cfg(feature = "enable_cache_record")]
    cachedb_record: CacheDbRecord,
    /// Record information on instruction execution.
    #[cfg(feature = "enable_opcode_metrics")]
    op_record: OpcodeRecord,

    /// A channel for sending recorded indicator information to the dashboard for display.
    events_tx: Option<MetricEventsSender>,

    /// Used to record the current block_number.
    block_number: u64,
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

fn recorder<'a>() -> &'a mut PerfMetric {
    unsafe { METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!") }
}

// *************************************************************************************************
//
// The functions in the following range should be called in the execute_inner function of execution
// stage.
//
// *************************************************************************************************
pub fn start_record() {
    #[cfg(feature = "enable_execution_duration_record")]
    recorder().duration_record.start_total_record();
}

pub fn record_before_loop() {
    #[cfg(feature = "enable_tps_gas_record")]
    let _ = recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::BlockTpsAndGas {
        block_number: recorder().block_number,
        record: TpsAndGasMessage::Switch(true),
    });
}

pub fn record_before_td(block_number: u64) {
    #[cfg(feature = "enable_execution_duration_record")]
    recorder().duration_record.start_time_record();

    recorder().block_number = block_number;
}

pub fn record_after_td() {
    #[cfg(feature = "enable_execution_duration_record")]
    {
        recorder().duration_record.add_block_td_duration();
        recorder().duration_record.start_time_record();
    }
}

pub fn record_after_block_with_senders() {
    #[cfg(feature = "enable_execution_duration_record")]
    {
        recorder().duration_record.add_block_with_senders_duration();
        recorder().duration_record.start_time_record();
    }
}

pub fn record_after_get_tps(_block_number: u64, _txs: u64, _gas: u64) {
    #[cfg(feature = "enable_tps_gas_record")]
    {
        recorder().tps_gas_record.record(_block_number, _txs as u128, _gas as u128);
        let _ =
            recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::BlockTpsAndGas {
                block_number: recorder().block_number,
                record: TpsAndGasMessage::Record(recorder().tps_gas_record),
            });
    }
}

pub fn record_after_take_output_state() {
    #[cfg(feature = "enable_tps_gas_record")]
    let _ = recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::BlockTpsAndGas {
        block_number: recorder().block_number,
        record: TpsAndGasMessage::Switch(false),
    });

    #[cfg(feature = "enable_execution_duration_record")]
    recorder().duration_record.start_time_record();
}

pub fn record_at_end(_cachedb_size: usize) {
    #[cfg(feature = "enable_execution_duration_record")]
    {
        recorder().duration_record.add_total_duration();
        let _ = recorder().events_tx.as_mut().expect("No sender").send(
            MetricEvent::ExecutionStageTime {
                block_number: recorder().block_number,
                record: recorder().duration_record,
            },
        );
    }

    #[cfg(feature = "enable_cache_record")]
    {
        let cachedb_record = revm_utils::metrics::get_cache_record();
        recorder().cachedb_record.update(&cachedb_record);
        let _ = recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::CacheDbInfo {
            block_number: recorder().block_number,
            size: _cachedb_size,
            record: recorder().cachedb_record,
        });
    }

    #[cfg(feature = "enable_opcode_metrics")]
    let _ = recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::OpcodeInfo {
        block_number: recorder().block_number,
        record: recorder().op_record,
    });
}
// *************************************************************************************************
//                            functions called by execute_inner end
// *************************************************************************************************

// *************************************************************************************************
//
// The functions in the following range should be called in executor.
//
// *************************************************************************************************

/// After each transaction is executed, the execution status of instructions is counted and then
/// updated to the global metric recorder. This function will be called in executor.
#[cfg(feature = "enable_opcode_metrics")]
pub fn record_opcode() {
    let mut op_record = revm_utils::metrics::get_op_record();
    if op_record.not_empty() {
        recorder().op_record.update(&mut op_record);
    }
}

/// start execute_tx record.
#[cfg(feature = "enable_execution_duration_record")]
pub fn start_execute_tx_record() {
    recorder().duration_record.execution.start_record();
}

/// start execute_tx sub record.
#[cfg(feature = "enable_execution_duration_record")]
pub fn start_execute_tx_sub_record() {
    recorder().duration_record.execution.start_sub_record();
}

/// transact record
#[cfg(feature = "enable_execution_duration_record")]
pub fn transact_record() {
    recorder().duration_record.execution.transact_record();
}

/// commit_changes_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn commit_changes_record() {
    recorder().duration_record.execution.commit_changes_record();
}

/// add_receipt_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn add_receipt_record() {
    recorder().duration_record.execution.add_receipt_record();
}

/// apply_post_execution_state_change_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn apply_post_execution_state_change_record() {
    recorder().duration_record.execution.apply_post_execution_state_change_record();
}

/// merge_transactions_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn merge_transactions_record() {
    recorder().duration_record.execution.merge_transactions_record();
}

/// verify_receipt_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn verify_receipt_record() {
    recorder().duration_record.execution.verify_receipt_record();
}

/// save_receipts_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn save_receipts_record() {
    recorder().duration_record.execution.save_receipts_record();
}

/// get_execute_tx_record
#[cfg(feature = "enable_execution_duration_record")]
pub fn get_execute_tx_record() -> ExecuteTxsRecord {
    recorder().duration_record.execution
}

/// Record for verfity_and_save_receipts
#[cfg(feature = "enable_execution_duration_record")]
pub struct VerifyAndSaveReceiptsRecord;

#[cfg(feature = "enable_execution_duration_record")]
impl VerifyAndSaveReceiptsRecord {
    /// Return VerifyAndSaveReceiptsRecord
    pub fn new() -> Self {
        verify_receipt_record();
        VerifyAndSaveReceiptsRecord
    }
}

#[cfg(feature = "enable_execution_duration_record")]
impl Drop for VerifyAndSaveReceiptsRecord {
    fn drop(&mut self) {
        save_receipts_record();
    }
}
// *************************************************************************************************
//                              functions called by executor end
// *************************************************************************************************

// *************************************************************************************************
//
// The function within this range will be used to measure write_to_db and will be called in
// write_to_db.
//
// *************************************************************************************************
/// start write_to_db record.
#[cfg(feature = "enable_execution_duration_record")]
pub fn start_write_to_db_record() {
    recorder().duration_record.write_to_db.start_record();
}

/// start write_to_db sub record.
#[cfg(feature = "enable_execution_duration_record")]
pub fn start_write_to_db_sub_record() {
    recorder().duration_record.write_to_db.start_sub_record();
}

/// start write_to_db write record.
#[cfg(feature = "enable_execution_duration_record")]
fn start_write_to_db_write_record() {
    recorder().duration_record.write_to_db.start_write_record();
}

/// Record data size of write storage changes in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_revert_storage_size(size: usize) {
    recorder().duration_record.write_to_db.record_revert_storage_size(size);
}

/// Record time of write storage append time in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_revert_storage_append_time() {
    recorder().duration_record.write_to_db.record_revert_storage_append_time();
}

// Encapsulate this structure to record write_storage in revert state in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    RevertsStorageWrite,
    start_write_to_db_write_record,
    record_revert_storage_append_time,
    record_revert_storage_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write storage changes in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_revert_storage_time() {
    recorder().duration_record.write_to_db.record_revert_storage_time();
}

/// Record data size of write account changes in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_revert_account_size(size: usize) {
    recorder().duration_record.write_to_db.record_revert_account_size(size);
}

/// Record time of write account append time in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_revert_account_append_time() {
    recorder().duration_record.write_to_db.record_revert_account_append_time();
}

// Encapsulate this structure to record write_account in revert state in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    RevertsAccountWrite,
    start_write_to_db_write_record,
    record_revert_account_append_time,
    record_revert_account_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write account changes in StateReverts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_revert_account_time() {
    recorder().duration_record.write_to_db.record_revert_account_time();
}

/// Record data size of write receipts in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_write_receipts_size(size: usize) {
    recorder().duration_record.write_to_db.record_write_receipts_size(size);
}

/// Record time of write receipts append in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_receipts_append_time() {
    recorder().duration_record.write_to_db.record_receipts_append_time();
}

// Encapsulate this structure to record write receipts in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    ReceiptsWrite,
    start_write_to_db_write_record,
    record_receipts_append_time,
    record_write_receipts_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write receipts  in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_write_receipts_time() {
    recorder().duration_record.write_to_db.record_write_receipts_time();
}

/// Record time of sort in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_sort_time() {
    recorder().duration_record.write_to_db.record_sort_time();
}

/// Record data size of write account in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_account_size(size: usize) {
    recorder().duration_record.write_to_db.record_state_account_size(size);
}

/// Record time of write account upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_account_upsert_time() {
    recorder().duration_record.write_to_db.record_state_account_upsert_time();
}

// Encapsulate this structure to record write_account in state changes in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    StateAccountWrite,
    start_write_to_db_write_record,
    record_state_account_upsert_time,
    record_state_account_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write account in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_state_account_time() {
    recorder().duration_record.write_to_db.record_state_account_time();
}

/// Record data size of write bytecode in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_bytecode_size(size: usize) {
    recorder().duration_record.write_to_db.record_state_bytecode_size(size);
}

/// Record time of write bytecode upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_bytecode_upsert_time() {
    recorder().duration_record.write_to_db.record_state_bytecode_upsert_time();
}

// Encapsulate this structure to record write_bytecode in state changes in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    StateBytecodeWrite,
    start_write_to_db_write_record,
    record_state_bytecode_upsert_time,
    record_state_bytecode_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write bytecode in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_state_bytecode_time() {
    recorder().duration_record.write_to_db.record_state_bytecode_time();
}

/// Record data size of write storage in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_storage_size(size: usize) {
    recorder().duration_record.write_to_db.record_state_storage_size(size);
}

/// Record time of write storage upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
fn record_state_storage_upsert_time() {
    recorder().duration_record.write_to_db.record_state_storage_upsert_time();
}

// Encapsulate this structure to record write_storage in state changes in a RAII manner.
#[cfg(feature = "enable_execution_duration_record")]
impl_write_macro!(
    StateStorageWrite,
    start_write_to_db_write_record,
    record_state_storage_upsert_time,
    record_state_storage_size,
    record_receipts_append_time,
    record_write_receipts_size
);

/// Record time of write storage in StateChanges's write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub fn record_state_storage_time() {
    recorder().duration_record.write_to_db.record_state_storage_time();
}
// *************************************************************************************************
//                              functions called by write_to_db end
// *************************************************************************************************
