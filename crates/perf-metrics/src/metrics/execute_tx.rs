//! This module is used to support in-depth measurement of function execute_and_verify_receipt
//! in stage execution.
use revm_utils::time_utils::{convert_cycles_to_ns_f64, instant::Instant};

/// ExecuteTxsRecord
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteTxsRecord {
    /// Record the starting time of function execute_and_verify_receipt.
    start_record: Instant,
    /// Record the start time of each subfunction.
    sub_record: Instant,
    /// Time of execute_and_verify_receipt.
    total: u64,
    /// Time of transact.
    transact: u64,
    /// Time of commit changes.
    commit_changes: u64,
    /// Time of add receipt.
    add_receipt: u64,
    /// Time of apply_post_execution_state_change.
    apply_post_execution_state_change: u64,
    /// Time of merge_transactions.
    merge_transactions: u64,
    /// Time of verify_receipt.
    verify_receipt: u64,
    /// Time of save_receipts.
    save_receipts: u64,
}

impl ExecuteTxsRecord {
    /// Start record.
    pub(super) fn start_record(&mut self) {
        self.start_record = Instant::now();
    }
    /// Start sub record.
    pub(super) fn start_sub_record(&mut self) {
        self.sub_record = Instant::now();
    }

    /// Add time of transact.
    pub(super) fn transact_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.transact = self.transact.checked_add(cycles).expect("overflow");
    }
    /// Add time of commit changes.
    pub(super) fn commit_changes_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.commit_changes = self.commit_changes.checked_add(cycles).expect("overflow");
    }
    /// Add time of add receipt.
    pub(super) fn add_receipt_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.add_receipt = self.add_receipt.checked_add(cycles).expect("overflow");
    }
    /// Add time of apply_post_execution_state_change.
    pub(super) fn apply_post_execution_state_change_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.apply_post_execution_state_change =
            self.apply_post_execution_state_change.checked_add(cycles).expect("overflow");
    }
    /// Add time of merge_transactions.
    pub(super) fn merge_transactions_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.merge_transactions = self.merge_transactions.checked_add(cycles).expect("overflow");
    }
    /// Add time of verify_receipt.
    pub(super) fn verify_receipt_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.verify_receipt = self.verify_receipt.checked_add(cycles).expect("overflow");
    }
    /// Add time of save_receipts.
    pub(super) fn save_receipts_record(&mut self) {
        let (cycles, now) = self.record_sub_time();
        self.save_receipts = self.save_receipts.checked_add(cycles).expect("overflow");
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

impl ExecuteTxsRecord {
    /// Return total.
    pub fn total(&self) -> u64 {
        self.total
    }
    /// Return transact.
    pub fn transact(&self) -> u64 {
        self.transact
    }
    /// Return commit changes.
    pub fn commit_changes(&self) -> u64 {
        self.commit_changes
    }
    /// Return add_receipt.
    pub fn add_receipt(&self) -> u64 {
        self.add_receipt
    }
    /// Return apply_post_execution_state_change.
    pub fn apply_post_execution_state_change(&self) -> u64 {
        self.apply_post_execution_state_change
    }
    /// Return merge_transactions.
    pub fn merge_transactions(&self) -> u64 {
        self.merge_transactions
    }
    /// Return verify_receipt.
    pub fn verify_receipt(&self) -> u64 {
        self.verify_receipt
    }
    /// Return save_receipts.
    pub fn save_receipts(&self) -> u64 {
        self.save_receipts
    }
    /// TODO: This function needs to be deleted later on.
    pub fn print(&self) {
        self.printline("total", self.total);
        self.printline("transact", self.transact);
        self.printline("commit_changes", self.commit_changes);
        self.printline("add receipt", self.add_receipt);
        self.printline("apply_post_execution_state_change", self.apply_post_execution_state_change);
        self.printline("verify_receipt", self.verify_receipt);
    }

    fn printline(&self, cat: &'static str, cycles: u64) {
        println!("{:?}: {:?} ns", cat, convert_cycles_to_ns_f64(cycles));
    }
}
