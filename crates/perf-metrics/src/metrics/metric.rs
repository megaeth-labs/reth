//! This module provides a metric to measure reth.
// #[cfg(feature = "enable_execution_duration_record")]
// pub use super::duration::ExecuteTxsRecord;
#[cfg(feature = "enable_execution_duration_record")]
use super::duration::ExecutionDurationRecord;
#[cfg(feature = "enable_tps_gas_record")]
pub use super::tps_gas::TpsAndGasMessage;
#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsGasRecord;
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
