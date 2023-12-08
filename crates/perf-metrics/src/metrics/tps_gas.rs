#[derive(Debug, Default, Copy, Clone)]
pub struct TpsGasRecord {
    block_number: u64,
    txs: u128,
    gas: u128,
}

impl TpsGasRecord {
    pub(crate) fn record(&mut self, block_number: u64, txs: u128, gas: u128) {
        self.block_number = block_number;
        self.txs = self.txs.checked_add(txs).expect("overflow");
        self.gas = self.gas.checked_add(gas).expect("overflow");
    }

    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    pub fn txs(&self) -> u128 {
        self.txs
    }

    pub fn gas(&self) -> u128 {
        self.gas
    }
}
