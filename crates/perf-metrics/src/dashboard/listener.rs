use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use crate::metrics::metric::MetricEvent;
use tokio::sync::mpsc::UnboundedReceiver;

#[cfg(feature = "enable_opcode_metrics")]
use super::displayer::RevmMetricTimeDisplayer;

#[cfg(feature = "enable_execution_duration_record")]
use super::displayer::ExecutionDurationDisplayer;

#[cfg(feature = "enable_db_speed_record")]
use super::displayer::DBSpeedDisplayer;

#[cfg(feature = "enable_cache_record")]
use super::displayer::CacheDBRecordDisplayer;

#[cfg(feature = "enable_tps_gas_record")]
use super::displayer::TpsAndGasRecordDisplayer;

#[cfg(feature = "enable_execute_measure")]
use super::displayer::ExecuteTxsDisplayer;

#[cfg(feature = "enable_state_root_record")]
use super::displayer::StateRootUpdateDisplayer;
#[cfg(feature = "enable_write_to_db_measure")]
use super::displayer::WriteToDbDisplayer;

#[derive(Debug)]
pub struct DashboardListener {
    events_rx: UnboundedReceiver<MetricEvent>,

    #[cfg(feature = "enable_opcode_metrics")]
    revm_metric_displayer: RevmMetricTimeDisplayer,

    #[cfg(feature = "enable_execution_duration_record")]
    excution_dureation_displayer: ExecutionDurationDisplayer,

    #[cfg(feature = "enable_db_speed_record")]
    db_speed_displayer: DBSpeedDisplayer,

    #[cfg(feature = "enable_cache_record")]
    cache_db_displayer: CacheDBRecordDisplayer,

    #[cfg(feature = "enable_tps_gas_record")]
    tps_gas_displayer: TpsAndGasRecordDisplayer,

    #[cfg(feature = "enable_execute_measure")]
    execute_txs_displayer: ExecuteTxsDisplayer,

    #[cfg(feature = "enable_write_to_db_measure")]
    write_to_db_displayer: WriteToDbDisplayer,
    #[cfg(feature = "enable_state_root_record")]
    state_root_update_displayer: StateRootUpdateDisplayer,
}

impl DashboardListener {
    /// Creates a new [DashboardListener] with the provided receiver of [MetricEvent].
    pub fn new(events_rx: UnboundedReceiver<MetricEvent>) -> Self {
        Self {
            events_rx,

            #[cfg(feature = "enable_opcode_metrics")]
            revm_metric_displayer: RevmMetricTimeDisplayer::default(),

            #[cfg(feature = "enable_execution_duration_record")]
            excution_dureation_displayer: ExecutionDurationDisplayer::default(),

            #[cfg(feature = "enable_db_speed_record")]
            db_speed_displayer: DBSpeedDisplayer::default(),

            #[cfg(feature = "enable_cache_record")]
            cache_db_displayer: CacheDBRecordDisplayer::default(),

            #[cfg(feature = "enable_tps_gas_record")]
            tps_gas_displayer: TpsAndGasRecordDisplayer::default(),

            #[cfg(feature = "enable_execute_measure")]
            execute_txs_displayer: ExecuteTxsDisplayer::default(),

            #[cfg(feature = "enable_write_to_db_measure")]
            write_to_db_displayer: WriteToDbDisplayer::default(),
            #[cfg(feature = "enable_state_root_record")]
            state_root_update_displayer: StateRootUpdateDisplayer::default(),
        }
    }

    fn handle_event(&mut self, event: MetricEvent) {
        match event {
            #[cfg(feature = "enable_execution_duration_record")]
            MetricEvent::ExecutionStageTime { block_number, record } => {
                self.excution_dureation_displayer.update_excution_duration_record(record);
                self.excution_dureation_displayer.print(block_number);
            }
            #[cfg(feature = "enable_tps_gas_record")]
            MetricEvent::BlockTpsAndGas { record } => {
                self.tps_gas_displayer.update_tps_and_gas(
                    record.block_number(),
                    record.txs(),
                    record.gas(),
                );
            }
            #[cfg(feature = "enable_tps_gas_record")]
            MetricEvent::BlockTpsAndGasSwitch { block_number, switch } => {
                if switch {
                    self.tps_gas_displayer.start_record(block_number);
                } else {
                    self.tps_gas_displayer.stop_record(block_number);
                }
            }
            #[cfg(feature = "enable_db_speed_record")]
            MetricEvent::DBSpeedInfo { record } => {
                self.db_speed_displayer.update_db_speed_record(record);
                self.db_speed_displayer.print();
            }
            #[cfg(feature = "enable_opcode_metrics")]
            MetricEvent::OpcodeInfo { record } => {
                self.revm_metric_displayer.update_opcode_record(record);
                self.revm_metric_displayer.print();
            }
            #[cfg(feature = "enable_cache_record")]
            MetricEvent::CacheDbInfo { block_number, size, record } => {
                self.cache_db_displayer.update_cachedb_record(block_number, size, record);
                self.cache_db_displayer.print();
            }
            #[cfg(feature = "enable_execute_measure")]
            MetricEvent::ExecuteTxsInfo { record } => {
                self.execute_txs_displayer.record(record);
                self.execute_txs_displayer.print();
            }
            #[cfg(feature = "enable_write_to_db_measure")]
            MetricEvent::WriteToDbInfo { record } => {
                self.write_to_db_displayer.record(record);
                self.write_to_db_displayer.print();
            }
            #[cfg(feature = "enable_state_root_record")]
            MetricEvent::StateRootUpdate { record } => {
                self.state_root_update_displayer.record(record);
                // self.state_root_update_displayer.print();
            }
            #[cfg(feature = "enable_state_root_record")]
            MetricEvent::StateRootRecordUpdate { record } => {
                self.state_root_update_displayer.update_record(record);
            }
            #[cfg(feature = "enable_state_root_record")]
            MetricEvent::StateRootUpdatePrint {} => {
                self.state_root_update_displayer.print();
            }
        }
    }
}

impl Future for DashboardListener {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Loop until we drain the `events_rx` channel
        loop {
            let Some(event) = ready!(this.events_rx.poll_recv(cx)) else {
                // Channel has closed
                return Poll::Ready(())
            };

            this.handle_event(event);
        }
    }
}
