use crate::metrics::TpsAndGasMessage;
use revm_utils::time_utils::instant::Instant;
use std::ops::{Div, Mul};

#[derive(Debug, Default)]
pub(crate) struct TpsAndGasDisplayer {
    pre_txs: u128,
    pre_gas: u128,
    last_txs: u128,
    last_gas: u128,
    pre_instant: Instant,
}

impl TpsAndGasDisplayer {
    const N: u64 = 1000;

    fn update_tps_and_gas(&mut self, block_number: u64, txs: u128, gas: u128) {
        if 0 == block_number % Self::N {
            self.print_content(block_number, txs, gas);
        }

        self.last_txs = txs;
        self.last_gas = gas;
    }

    fn start_record(&mut self) {
        self.pre_txs = self.last_txs;
        self.pre_gas = self.last_gas;
        self.pre_instant = Instant::now();
    }

    fn stop_record(&mut self, block_number: u64) {
        self.print_content(block_number, self.last_txs, self.last_gas);
    }

    fn print_content(&mut self, block_number: u64, txs: u128, gas: u128) {
        let now = Instant::now();
        let elapsed_ns = now.checked_nanos_since(self.pre_instant).unwrap_or(0.0);
        let delta_txs = txs - self.pre_txs;
        let delta_gas = gas - self.pre_gas;

        let tps = delta_txs.mul(1000_000_000).div(elapsed_ns as u128);
        let mgas_ps = (delta_gas as f64).mul(1000_000_000 as f64).div(elapsed_ns as f64);

        self.pre_txs = txs;
        self.pre_gas = gas;
        self.pre_instant = now;

        println!("block_number: {:?}, TPS : {:?}", block_number, tps);
        println!("block_number: {:?}, MGas: {:.3}\n", block_number, mgas_ps);
    }

    pub fn print(&mut self, block_number: u64, message: TpsAndGasMessage) {
        match message {
            TpsAndGasMessage::Record(record) => {
                self.update_tps_and_gas(record.block_number, record.txs, record.gas)
            }
            TpsAndGasMessage::Switch(switch) => {
                if switch {
                    self.start_record();
                } else {
                    self.stop_record(block_number);
                }
            }
        }
    }
}
