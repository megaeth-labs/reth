use revm_utils::time_utils::{convert_cycles_to_ns_f64, instant::Instant};

/// ExecuteTxsRecord
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteTxsRecord {
    start_record: Instant,
    sub_record: Instant,

    /// time of execute_and_verify_receipt.
    total: u64,
    /// time of transact.
    transact: u64,
    /// time of commit changes.
    commit_changes: u64,
    /// time of add receipt.
    add_receipt: u64,
    /// time of apply_post_block_changes.
    apply_post_block_changes: u64,
    /// time of verify_receipt.
    verify_receipt: u64,
}

impl ExecuteTxsRecord {
    /// Start record.
    fn start_record(&mut self) {
        self.start_record = Instant::now();
    }
    /// Start sub record.
    fn start_sub_recorder(&mut self) {
        self.sub_record = Instant::now();
    }

    /// Add time of transact.
    fn transact_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.transact = self.transact.checked_add(cycles).expect("overflow");
    }
    /// Add time of commit changes.
    fn commit_changes_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.commit_changes = self.commit_changes.checked_add(cycles).expect("overflow");
    }
    /// Add time of add receipt.
    fn add_receipt_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.add_receipt = self.add_receipt.checked_add(cycles).expect("overflow");
    }
    /// Add time of apply_post_block_changes.
    fn apply_post_block_changes_record(&mut self) {
        let (cycles, _) = self.record_sub_time();
        self.apply_post_block_changes =
            self.apply_post_block_changes.checked_add(cycles).expect("overflow");
    }
    /// Add time of verify_receipt.
    fn verify_receipt_record(&mut self) {
        let (cycles, now) = self.record_sub_time();
        self.verify_receipt = self.verify_receipt.checked_add(cycles).expect("overflow");
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
    /// Return apply_post_block_changes.
    pub fn apply_post_block_changes(&self) -> u64 {
        self.apply_post_block_changes
    }
    /// Return verify_receipt.
    pub fn verify_receipt(&self) -> u64 {
        self.verify_receipt
    }
    /// TODO: This function needs to be deleted later on.
    pub fn print(&self) {
        self.printline("total", self.total);
        self.printline("transact", self.transact);
        self.printline("commit_changes", self.commit_changes);
        self.printline("add receipt", self.add_receipt);
        self.printline("apply_post_block_changes", self.apply_post_block_changes);
        self.printline("verify_receipt", self.verify_receipt);
    }

    fn printline(&self, cat: &'static str, cycles: u64) {
        println!("{:?}: {:?} ns", cat, convert_cycles_to_ns_f64(cycles));
    }
}

// TODO: This variable needs to be merged into a large structural variable later.
static mut METRIC_EXECUTE_TX_RECORD: Option<ExecuteTxsRecord> = None;

#[ctor::ctor]
unsafe fn init() {
    METRIC_EXECUTE_TX_RECORD = Some(ExecuteTxsRecord::default());
}

/// start execute_tx record.
pub fn start_execute_tx_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .start_record();
    }
}

/// start execute_tx sub record.
pub fn start_execute_tx_sub_recorder() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .start_sub_recorder();
    }
}

/// transact record
pub fn transact_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .transact_record();
    }
}

/// commit_changes_record
pub fn commit_changes_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .commit_changes_record();
    }
}

/// add_receipt_record
pub fn add_receipt_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .add_receipt_record();
    }
}

/// apply_post_block_changes_record
pub fn apply_post_block_changes_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .apply_post_block_changes_record()
    }
}

/// verify_receipt_record
pub fn verify_receipt_record() {
    unsafe {
        METRIC_EXECUTE_TX_RECORD
            .as_mut()
            .expect("Metric recorder should not empty!")
            .verify_receipt_record();
    }
}

/// get_execute_tx_record
pub fn get_execute_tx_record() -> ExecuteTxsRecord {
    unsafe { METRIC_EXECUTE_TX_RECORD.as_mut().expect("Metric recorder should not empty!").clone() }
}

/// Record for apply_post_block_changes
pub struct ApplyPostBlockChangesRecord;

impl ApplyPostBlockChangesRecord {
    /// Return ApplyPostBlockChangesRecord.
    pub fn new() -> Self {
        start_execute_tx_sub_recorder();
        ApplyPostBlockChangesRecord
    }
}

impl Drop for ApplyPostBlockChangesRecord {
    fn drop(&mut self) {
        apply_post_block_changes_record();
    }
}
