//! This module provides a metric to measure the execution stage.

#[cfg(feature = "enable_execution_duration_record")]
use super::duration::ExecutionDurationRecord;
#[cfg(feature = "enable_execute_measure")]
use super::execute_tx::ExecuteTxsRecord;
#[cfg(feature = "enable_db_speed_record")]
use super::speed::DatabaseOperationRecord;
#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsGasRecord;
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

    /// A channel for sending recorded indicator information to the dashboard for display.
    events_tx: Option<MetricEventsSender>,
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
        let _ = _record
            .events_tx
            .as_mut()
            .expect("No sender")
            .send(MetricEvent::BlockTpsAndGasSwitch { switch: true });
    }
}

pub fn record_before_td() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

        #[cfg(feature = "enable_execution_duration_record")]
        _record.duration_record.start_time_record();

        #[cfg(feature = "enable_db_speed_record")]
        crate::db_metric::start_db_record();
    }
}

pub fn record_after_td() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

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
            _record.duration_record.add_fetch_block_duration();
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
    }
}

pub fn record_after_take_output_state() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");

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
            let _ = _record
                .events_tx
                .as_mut()
                .expect("No sender")
                .send(MetricEvent::ExecutionStageTime { record: _record.duration_record });
        }

        #[cfg(feature = "enable_tps_gas_record")]
        let _ = _record
            .events_tx
            .as_mut()
            .expect("No sender")
            .send(MetricEvent::BlockTpsAndGasSwitch { switch: false });

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
pub fn start_execute_tx_sub_recorder() {
    unsafe {
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
        _record.execute_tx_record.start_sub_recorder();
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
        let _record = METRIC_RECORDER.as_mut().expect("Metric recorder should not empty!");
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
