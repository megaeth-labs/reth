//! [DashboardListener] is used to display various metrics.
use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
))]
use super::commons::*;

use crate::metrics::metric::MetricEvent;
use tokio::sync::mpsc::UnboundedReceiver;

#[cfg(feature = "enable_tps_gas_record")]
use super::tps_gas::TpsAndGasDisplayer;

#[derive(Debug)]
pub struct DashboardListener {
    events_rx: UnboundedReceiver<MetricEvent>,

    #[cfg(feature = "enable_tps_gas_record")]
    tps_gas_displayer: TpsAndGasDisplayer,
}

impl DashboardListener {
    /// Creates a new [DashboardListener] with the provided receiver of [MetricEvent].
    pub fn new(events_rx: UnboundedReceiver<MetricEvent>) -> Self {
        Self {
            events_rx,

            #[cfg(feature = "enable_tps_gas_record")]
            tps_gas_displayer: TpsAndGasDisplayer::default(),
        }
    }

    fn handle_event(&mut self, event: MetricEvent) {
        match event {
            #[cfg(feature = "enable_execution_duration_record")]
            MetricEvent::ExecutionStageTime { block_number, record } => {
                record.print(block_number);
            }
            #[cfg(feature = "enable_tps_gas_record")]
            MetricEvent::BlockTpsAndGas { block_number, record } => {
                self.tps_gas_displayer.print(block_number, record);
            }
            #[cfg(feature = "enable_opcode_metrics")]
            MetricEvent::OpcodeInfo { block_number, record } => {
                record.print(block_number);
            }
            #[cfg(feature = "enable_cache_record")]
            MetricEvent::CacheDbInfo { block_number, size, record } => {
                super::cache::print_state_size(block_number, size);
                record.print(block_number);
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
