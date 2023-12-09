//! This module is used to track the total number of transactions
//! and total gas consumed so far.
#[derive(Debug, Default, Copy, Clone)]
pub struct TpsGasRecord {
    pub(crate) block_number: u64,
    pub(crate) txs: u128,
    pub(crate) gas: u128,
}

impl TpsGasRecord {
    pub(crate) fn record(&mut self, block_number: u64, txs: u128, gas: u128) {
        self.block_number = block_number;
        self.txs = self.txs.checked_add(txs).expect("overflow");
        self.gas = self.gas.checked_add(gas).expect("overflow");
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TpsAndGasMessage {
    Switch(bool),
    Record(TpsGasRecord),
}
