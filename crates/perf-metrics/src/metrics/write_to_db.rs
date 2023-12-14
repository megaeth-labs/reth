//! This module is used to measure the function write_to_db. When measuring
//! function write_to_db, the main measures are: (1) its total time
//! consumption and the time consumption of each sub function; (2) The
//! average write rate and the write rate of each sub function.
use revm_utils::time_utils::instant::Instant;

/// This structure is used to record all the metrics of write_to_db, including
/// the time spent writing and the amount of data written.
#[derive(Debug, Clone, Copy, Default)]
pub struct WriteToDbRecord {
    /// Record the starting time of function write_to_db.
    start_record: Instant,
    /// Record the start time of each subfunction.
    sub_record: Instant,

    /// Time of write_to_db.
    total: u64,

    /// Time of write storage changes in StateReverts.
    revert_storage_time: u64,
    /// Data size of write storage changes in StateReverts.
    revert_storage_size: usize,
    /// Time of write account changes in StateReverts.
    revert_account_time: u64,
    /// Data size of write account changes in StateReverts.
    revert_account_size: usize,

    /// Time of write receipts.
    write_receipts_time: u64,
    /// Data size of write receipts.
    write_receipts_size: usize,

    /// Time of sort in StateChanges's write_to_db.
    sort_time: u64,
    /// Time of write account in StateChanges.
    state_account_time: u64,
    /// Data size of write account in StateChanges.
    state_account_size: usize,
    /// Time of write bytecode in StateChanges.
    state_bytecode_time: u64,
    /// Data size of write bytecode in StateChanges.
    state_bytecode_size: usize,
    /// Time of write storage in StateChanges.
    state_storage_time: u64,
    /// Data size of write storage in StateChanges.
    state_storage_size: usize,
}

impl WriteToDbRecord {
    /// Start record.
    pub(super) fn start_record(&mut self) {
        self.start_record = Instant::now();
    }
    /// Start sub record.
    pub(super) fn start_sub_record(&mut self) {
        self.sub_record = Instant::now();
    }
    /// Record data size of write storage changes in StateReverts.
    pub(super) fn record_revert_storage_size(&mut self, size: usize) {
        self.revert_storage_size = self.revert_storage_size.checked_add(size).expect("overflow");
    }
    /// Record time of write storage changes in StateReverts.
    pub(super) fn record_revert_storage_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.revert_storage_time = self.revert_storage_time.checked_add(cycles).expect("overflow");
    }
    /// Record data size of write account changes in StateReverts.
    pub(super) fn record_revert_account_size(&mut self, size: usize) {
        self.revert_account_size = self.revert_account_size.checked_add(size).expect("overflow");
    }
    /// Record time of write account changes in StateReverts.
    pub(super) fn record_revert_account_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.revert_account_time = self.revert_account_time.checked_add(cycles).expect("overflow");
    }
    /// Record data size of write receipts.
    pub(super) fn record_write_receipts_size(&mut self, size: usize) {
        self.write_receipts_size = self.write_receipts_size.checked_add(size).expect("overflow");
    }
    /// Record time of write receipts.
    pub(super) fn record_write_receipts_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.write_receipts_time = self.write_receipts_time.checked_add(cycles).expect("overflow");
    }
    /// Record time of sort in StateChanges.
    pub(super) fn record_sort_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.sort_time = self.sort_time.checked_add(cycles).expect("overflow");
    }
    /// Record data size of write account in StateChanges.
    pub(super) fn record_state_account_size(&mut self, size: usize) {
        self.state_account_size = self.state_account_size.checked_add(size).expect("overflow");
    }
    /// Record time of write account in StateChanges.
    pub(super) fn record_state_account_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.state_account_time = self.state_account_time.checked_add(cycles).expect("overflow");
    }
    /// Record data size of write bytecode in StateChanges.
    pub(super) fn record_state_bytecode_size(&mut self, size: usize) {
        self.state_bytecode_size = self.state_bytecode_size.checked_add(size).expect("overflow");
    }
    /// Record time of write bytecode in StateChanges.
    pub(super) fn record_state_bytecode_time(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.state_bytecode_time = self.state_bytecode_time.checked_add(cycles).expect("overflow");
    }
    /// Record data size of write storage in StateChanges.
    pub(super) fn record_state_storage_size(&mut self, size: usize) {
        self.state_storage_size = self.state_storage_size.checked_add(size).expect("overflow");
    }
    /// Record time of write storage in StateChanges.
    pub(super) fn record_state_storage_time(&mut self) {
        let (cycles, now) = self.record_sub_time();
        self.state_storage_time = self.state_storage_time.checked_add(cycles).expect("overflow");
        self.record_total_time(now);
    }
    /// Record total time.
    fn record_total_time(&mut self, now: Instant) {
        let cycles = now.checked_cycles_since(self.start_record).unwrap_or(0);
        self.total = self.total.checked_add(cycles).expect("overflow");
    }
    /// Record time of sub function.
    fn record_sub_time(&mut self) -> (u64, Instant) {
        let now = Instant::now();
        let cycles = now.checked_cycles_since(self.sub_record).unwrap_or(0);
        self.sub_record = now;
        (cycles, now)
    }
}

impl WriteToDbRecord {
    /// Return total time of write_to_db.
    pub fn total_time(&self) -> u64 {
        self.total
    }
    /// Return time of write storage changes in StateReverts.
    pub fn revert_storage_time(&self) -> u64 {
        self.revert_storage_time
    }
    /// Return data size of write storage changes in StateReverts.
    pub fn revert_storage_size(&self) -> usize {
        self.revert_storage_size
    }
    /// Return time of write account changes in StateReverts.
    pub fn revert_account_time(&self) -> u64 {
        self.revert_account_time
    }
    /// Return data size of write account changes in StateReverts.
    pub fn revert_account_size(&self) -> usize {
        self.revert_account_size
    }
    /// Return time of write receipts.
    pub fn write_receipts_time(&self) -> u64 {
        self.write_receipts_time
    }
    /// Return data size of write receipts.
    pub fn write_receipts_size(&self) -> usize {
        self.write_receipts_size
    }
    /// Return time of sort in StateChanges's write_to_db.
    pub fn sort_time(&self) -> u64 {
        self.sort_time
    }
    /// Return time of write account in StateChanges.
    pub fn state_account_time(&self) -> u64 {
        self.state_account_time
    }
    /// Return data size of write account in StateChanges.
    pub fn state_account_size(&self) -> usize {
        self.state_account_size
    }
    /// Return time of write bytecode in StateChanges.
    pub fn state_bytecode_time(&self) -> u64 {
        self.state_bytecode_time
    }
    /// Return data size of write bytecode in StateChanges.
    pub fn state_bytecode_size(&self) -> usize {
        self.state_bytecode_size
    }
    /// Return time of write storage in StateChanges.
    pub fn state_storage_time(&self) -> u64 {
        self.state_storage_time
    }
    /// Return data size of write storage in StateChanges.
    pub fn state_storage_size(&self) -> usize {
        self.state_storage_size
    }
}
