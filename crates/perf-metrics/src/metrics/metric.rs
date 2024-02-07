//! This module provides a metric to measure the execution stage.

#[cfg(feature = "enable_execution_duration_record")]
use super::duration::ExecutionDurationRecord;
#[cfg(feature = "enable_execute_measure")]
use super::execute_tx::ExecuteTxsRecord;
#[cfg(feature = "enable_db_speed_record")]
use super::speed::DatabaseOperationRecord;
#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsGasRecord;
#[cfg(feature = "enable_write_to_db_measure")]
use super::write_to_db::WriteToDbRecord;
#[cfg(feature = "enable_state_root_record")]
use crate::{StateRootUpdateRecord, TreeNode};
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
        /// current block_number.
        block_number: u64,
        /// excution duration record.
        record: ExecutionDurationRecord,
    },
    /// Amount of txs and gas in a block.
    #[cfg(feature = "enable_tps_gas_record")]
    BlockTpsAndGas {
        /// tps and gas record.
        record: TpsGasRecord,
    },
    /// Tps and gas record switch.
    #[cfg(feature = "enable_tps_gas_record")]
    BlockTpsAndGasSwitch {
        /// current block_number.
        block_number: u64,
        /// true: start tps and gas record.
        /// false: stop tps and gas record.
        switch: bool,
    },
    /// Db speed metric record.
    #[cfg(feature = "enable_db_speed_record")]
    DBSpeedInfo {
        /// db speed record.
        record: DatabaseOperationRecord,
    },
    /// Opcode record in revm.
    #[cfg(feature = "enable_opcode_metrics")]
    OpcodeInfo {
        /// opcode record in revm.
        record: OpcodeRecord,
    },
    /// CacheDB metric record.
    #[cfg(feature = "enable_cache_record")]
    CacheDbInfo {
        /// current block_number.
        block_number: u64,
        /// cache db size.
        size: usize,
        /// cache db record.
        record: CacheDbRecord,
    },
    /// Measure execute_tx one deeper level info.
    #[cfg(feature = "enable_execute_measure")]
    ExecuteTxsInfo {
        /// execute txs record.
        record: ExecuteTxsRecord,
    },
    /// Measure write_to_db one deeper level info.
    #[cfg(feature = "enable_write_to_db_measure")]
    WriteToDbInfo {
        /// write_to_db record.
        record: WriteToDbRecord,
    },
    #[cfg(feature = "enable_state_root_record")]
    StateRootUpdate { record: StateRootUpdateRecord },
    #[cfg(feature = "enable_state_root_record")]
    StateRootUpdatePrint {},
}

/// This structure is used to support all metric operations of
/// execution stage.
#[derive(Default)]
struct ExecutionStageMetric {
    /// Record the time consumption of each function in execution stage.
    #[cfg(feature = "enable_execution_duration_record")]
    duration_record: ExecutionDurationRecord,
    /// Record the rate of reading and writing to the database.
    #[cfg(feature = "enable_db_speed_record")]
    speed_record: DatabaseOperationRecord,
    /// Record tps and gas.
    #[cfg(feature = "enable_tps_gas_record")]
    tps_gas_record: TpsGasRecord,
    /// Record cache hits, number of accesses, and memory usage.
    #[cfg(feature = "enable_cache_record")]
    cachedb_record: CacheDbRecord,
    /// Record information on instruction execution.
    #[cfg(feature = "enable_opcode_metrics")]
    op_record: OpcodeRecord,
    /// Record information on in-depth measurement of function execute_and_verify_receipt.
    #[cfg(feature = "enable_execute_measure")]
    execute_tx_record: ExecuteTxsRecord,
    /// Record information on in-depth measurement of function write_to_db.
    #[cfg(feature = "enable_write_to_db_measure")]
    write_to_db_record: WriteToDbRecord,
    ///
    #[cfg(feature = "enable_state_root_record")]
    state_root_update_record: StateRootUpdateRecord,

    /// A channel for sending recorded indicator information to the dashboard for display.
    events_tx: Option<MetricEventsSender>,

    /// Used to record the current block_number.
    block_number: u64,
}

static mut METRIC_RECORDER: Option<ExecutionStageMetric> = None;

#[ctor::ctor]
fn init() {
    unsafe {
        METRIC_RECORDER = Some(ExecutionStageMetric::default());
    }
}

pub fn set_metric_event_sender(events_tx: MetricEventsSender) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.events_tx = Some(events_tx);
    }
}

// *************************************************************************************************
//
// The functions in the following range should be called in the execute_inner function of execution
// stage.
//
// *************************************************************************************************
pub fn start_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        // record duration
        #[cfg(feature = "enable_execution_duration_record")]
        _record.duration_record.start_total_record();
    }
}

pub fn record_before_loop() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_tps_gas_record")]
        let _ = _record.events_tx.as_mut().expect("No sender").send(
            MetricEvent::BlockTpsAndGasSwitch { block_number: _record.block_number, switch: true },
        );
    }
}

pub fn record_before_td(block_number: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        _record.duration_record.start_time_record();

        #[cfg(feature = "enable_db_speed_record")]
        crate::db_metric::start_db_record();

        _record.block_number = block_number;
    }
}

pub fn record_after_td() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        {
            _record.duration_record.add_block_td_duration();
            _record.duration_record.start_time_record();
        }

        #[cfg(feature = "enable_db_speed_record")]
        {
            let (size, time, _, _) = crate::db_metric::get_db_record();
            _record.speed_record.header_td_record(size, time);
            crate::db_metric::start_db_record();
        }
    }
}

pub fn record_after_block_with_senders() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        {
            _record.duration_record.add_block_with_senders_duration();
            _record.duration_record.start_time_record();
        }

        #[cfg(feature = "enable_db_speed_record")]
        {
            let (size, time, _, _) = crate::db_metric::get_db_record();
            _record.speed_record.block_with_senders_record(size, time);
        }
    }
}

pub fn record_after_execute() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        _record.duration_record.add_execute_tx_duration();
    }
}

pub fn record_after_get_tps(_block_number: u64, _txs: u64, _gas: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_tps_gas_record")]
        {
            _record.tps_gas_record.record(_block_number, _txs as u128, _gas as u128);
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::BlockTpsAndGas { record: _record.tps_gas_record });
        }
    }
}

pub fn record_after_loop() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        _record.duration_record.start_time_record();

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

pub fn record_after_take_output_state() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        #[cfg(feature = "enable_tps_gas_record")]
        let _ = _record.events_tx.as_mut().expect("No sender").send(
            MetricEvent::BlockTpsAndGasSwitch { block_number: _record.block_number, switch: false },
        );

        #[cfg(feature = "enable_execution_duration_record")]
        {
            _record.duration_record.add_take_output_state_duration();
            _record.duration_record.start_time_record();
        }

        #[cfg(feature = "enable_db_speed_record")]
        crate::db_metric::start_db_record();
    }
}

pub fn record_at_end(_cachedb_size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        {
            _record.duration_record.add_write_to_db_duration();
            _record.duration_record.add_total_duration();
            let _ = _record.events_tx.as_mut().expect("No sender").send(
                MetricEvent::ExecutionStageTime {
                    block_number: _record.block_number,
                    record: _record.duration_record,
                },
            );
        }

        #[cfg(feature = "enable_db_speed_record")]
        {
            let (_, _, size, time) = crate::db_metric::get_db_record();
            _record.speed_record.write_to_db_record(size, time);
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::DBSpeedInfo { record: _record.speed_record });
        }

        #[cfg(feature = "enable_cache_record")]
        {
            let cachedb_record = revm_utils::metrics::get_cache_record();
            _record.cachedb_record.update(&cachedb_record);
            let _ = _record.events_tx.as_mut().expect("No sender").send(MetricEvent::CacheDbInfo {
                block_number: _record.block_number,
                size: _cachedb_size,
                record: _record.cachedb_record,
            });
        }

        #[cfg(feature = "enable_opcode_metrics")]
        {
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::OpcodeInfo { record: _record.op_record });
        }

        #[cfg(feature = "enable_execute_measure")]
        {
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::ExecuteTxsInfo { record: _record.execute_tx_record });
        }
        #[cfg(feature = "enable_write_to_db_measure")]
        {
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::WriteToDbInfo { record: _record.write_to_db_record });
        }
    }
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
        unsafe {
            let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
            _record.op_record.update(&mut op_record);
        }
    }
}

/// start execute_tx record.
#[cfg(feature = "enable_execute_measure")]
pub fn start_execute_tx_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.start_record();
    }
}

/// start execute_tx sub record.
#[cfg(feature = "enable_execute_measure")]
pub fn start_execute_tx_sub_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.start_sub_record();
    }
}

/// transact record
#[cfg(feature = "enable_execute_measure")]
pub fn transact_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.transact_record();
    }
}

/// commit_changes_record
#[cfg(feature = "enable_execute_measure")]
pub fn commit_changes_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.commit_changes_record();
    }
}

/// add_receipt_record
#[cfg(feature = "enable_execute_measure")]
pub fn add_receipt_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.add_receipt_record();
    }
}

/// apply_post_execution_state_change_record
#[cfg(feature = "enable_execute_measure")]
pub fn apply_post_execution_state_change_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.apply_post_execution_state_change_record()
    }
}

/// merge_transactions_record
#[cfg(feature = "enable_execute_measure")]
pub fn merge_transactions_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.merge_transactions_record()
    }
}

/// verify_receipt_record
#[cfg(feature = "enable_execute_measure")]
pub fn verify_receipt_record() {
    unsafe {
        let _record: &mut ExecutionStageMetric =
            METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.verify_receipt_record();
    }
}

/// save_receipts_record
#[cfg(feature = "enable_execute_measure")]
pub fn save_receipts_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.save_receipts_record()
    }
}

/// get_execute_tx_record
#[cfg(feature = "enable_execute_measure")]
pub fn get_execute_tx_record() -> ExecuteTxsRecord {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record
    }
}

/// Record for verfity_and_save_receipts
#[cfg(feature = "enable_execute_measure")]
pub struct VerifyAndSaveReceiptsRecord;

#[cfg(feature = "enable_execute_measure")]
impl VerifyAndSaveReceiptsRecord {
    /// Return VerifyAndSaveReceiptsRecord
    pub fn new() -> Self {
        verify_receipt_record();
        VerifyAndSaveReceiptsRecord
    }
}

#[cfg(feature = "enable_execute_measure")]
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
#[cfg(feature = "enable_write_to_db_measure")]
pub fn start_write_to_db_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.start_record();
    }
}

/// start write_to_db sub record.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn start_write_to_db_sub_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.start_sub_record();
    }
}

/// start write_to_db write record.
#[cfg(feature = "enable_write_to_db_measure")]
fn start_write_to_db_write_record() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.start_write_record();
    }
}

/// Record data size of write storage changes in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_revert_storage_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_storage_size(size);
    }
}

/// Record time of write storage append time in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_revert_storage_append_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_storage_append_time();
    }
}

/// Encapsulate this structure to record write_storage in revert state in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct RevertsStorageWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl RevertsStorageWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for RevertsStorageWrite {
    fn drop(&mut self) {
        record_revert_storage_append_time();
        record_revert_storage_size(self.0);
    }
}

/// Record time of write storage changes in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_revert_storage_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_storage_time();
    }
}

/// Record data size of write account changes in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_revert_account_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_account_size(size);
    }
}

/// Record time of write account append time in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_revert_account_append_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_account_append_time();
    }
}

/// Encapsulate this structure to record write_account in revert state in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct RevertsAccountWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl RevertsAccountWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for RevertsAccountWrite {
    fn drop(&mut self) {
        record_revert_account_append_time();
        record_revert_account_size(self.0);
    }
}

/// Record time of write account changes in StateReverts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_revert_account_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_revert_account_time();
    }
}

/// Record data size of write receipts in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_write_receipts_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_write_receipts_size(size);
    }
}

/// Record time of write receipts append in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_receipts_append_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_receipts_append_time();
    }
}

/// Encapsulate this structure to record write receipts in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct ReceiptsWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl ReceiptsWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for ReceiptsWrite {
    fn drop(&mut self) {
        record_receipts_append_time();
        record_write_receipts_size(self.0);
    }
}

/// Record time of write receipts  in BundleStateWithReceipts's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_write_receipts_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_write_receipts_time();
    }
}

/// Record time of sort in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_sort_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_sort_time();
    }
}

/// Record data size of write account in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_account_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_account_size(size);
    }
}

/// Record time of write account upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_account_upsert_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_account_upsert_time();
    }
}

/// Encapsulate this structure to record write_account in state changes in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct StateAccountWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl StateAccountWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for StateAccountWrite {
    fn drop(&mut self) {
        record_state_account_upsert_time();
        record_state_account_size(self.0);
    }
}

/// Record time of write account in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_state_account_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_account_time();
    }
}

/// Record data size of write bytecode in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_bytecode_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_bytecode_size(size);
    }
}

/// Record time of write bytecode upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_bytecode_upsert_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_bytecode_upsert_time();
    }
}

/// Encapsulate this structure to record write_bytecode in state changes in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct StateBytecodeWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl StateBytecodeWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for StateBytecodeWrite {
    fn drop(&mut self) {
        record_state_bytecode_upsert_time();
        record_state_bytecode_size(self.0);
    }
}

/// Record time of write bytecode in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_state_bytecode_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_bytecode_time();
    }
}

/// Record data size of write storage in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_storage_size(size: usize) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_storage_size(size);
    }
}

/// Record time of write storage upsert in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
fn record_state_storage_upsert_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_storage_upsert_time();
    }
}

/// Encapsulate this structure to record write_storage in state changes in a RAII manner.
#[cfg(feature = "enable_write_to_db_measure")]
pub struct StateStorageWrite(usize);

#[cfg(feature = "enable_write_to_db_measure")]
impl StateStorageWrite {
    pub fn new(size: usize) -> Self {
        start_write_to_db_write_record();
        Self(size)
    }
}

#[cfg(feature = "enable_write_to_db_measure")]
impl Drop for StateStorageWrite {
    fn drop(&mut self) {
        record_state_storage_upsert_time();
        record_state_storage_size(self.0);
    }
}

/// Record time of write storage in StateChanges's write_to_db.
#[cfg(feature = "enable_write_to_db_measure")]
pub fn record_state_storage_time() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.write_to_db_record.record_state_storage_time();
    }
}
// *************************************************************************************************
//                              functions called by write_to_db end
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
pub fn record_keccak256(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_hashswith_set() {
            _record.state_root_update_record.add_keccak256(time);
        }
    }
}

#[cfg(feature = "enable_state_root_record")]
pub fn add_keccak256_execution(count: u64, time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        if _record.state_root_update_record.is_hashswith_set() {
            _record.state_root_update_record.add_keccak256_execution(count, time);
        }
    }
}

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
pub(crate) fn record_state_root_calculator(time: u64) {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.state_root_update_record.add_state_root_calculator(time);
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

// *************************************************************************************************
//                              functions for the feature enable_state_root_record end
// *************************************************************************************************
