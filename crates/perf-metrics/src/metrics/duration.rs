//! This module is used to support recording the overhead of various parts
//! of the execute_inner function in execution stage. It records the overheads of
//! five parts in execute_inner, namely the overheads of fetch_block, executions,
//! process state, write to db, and the total overhead of execute_inner.
use revm_utils::time_utils::instant::Instant;

/// This structure is used to record all overhead information.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionDurationRecord {
    // Total time recorder.
    total_recorder: Instant,
    // General time recorder.
    time_recorder: Instant,
    // Time of execute inner.
    total: u64,
    // Time of fetching blocks(include get_block_td and block_with_senders).
    fetch_block: u64,
    // Time of Revm execution(execute_and_verify_receipt).
    execution: u64,
    // Time of process state(state.extend)
    take_output_state: u64,
    // Time of write to db
    write_to_db: u64,
}

// The following functions are used to record overhead.
impl ExecutionDurationRecord {
    /// Start total time recorder.
    pub(crate) fn start_total_record(&mut self) {
        self.total_recorder = Instant::now();
    }
    /// Start general time recorder.
    pub(crate) fn start_time_record(&mut self) {
        self.time_recorder = Instant::now();
    }
    /// Add time of total.
    pub(crate) fn add_total_duration(&mut self) {
        let cycles = Instant::now().checked_cycles_since(self.total_recorder).expect("overflow");
        self.total = self.total.checked_add(cycles).expect("overflow");
    }
    /// Add time of fetch_block.
    pub(crate) fn add_fetch_block_duration(&mut self) {
        let cycles = Instant::now().checked_cycles_since(self.time_recorder).expect("overflow");
        self.fetch_block = self.fetch_block.checked_add(cycles).expect("overflow");
    }
    /// Add time of Revm execution.
    pub(crate) fn add_execute_tx_duration(&mut self) {
        let cycles = Instant::now().checked_cycles_since(self.time_recorder).expect("overflow");
        self.execution = self.execution.checked_add(cycles).expect("overflow");
    }
    /// Add time of process state
    pub(crate) fn add_take_output_state_duration(&mut self) {
        let cycles = Instant::now().checked_cycles_since(self.time_recorder).expect("overflow");
        self.take_output_state = self.take_output_state.checked_add(cycles).expect("overflow");
    }
    /// Add time of write to db
    pub(crate) fn add_write_to_db_duration(&mut self) {
        let cycles = Instant::now().checked_cycles_since(self.time_recorder).expect("overflow");
        self.write_to_db = self.write_to_db.checked_add(cycles).expect("overflow");
    }
}

// The following functions are used to obtain the recorded results.
impl ExecutionDurationRecord {
    /// Return the overhead of execute_inner.
    pub fn total(&self) -> u64 {
        self.total
    }
    /// Return the overhead of fetching blocks.
    pub fn fetch_block(&self) -> u64 {
        self.fetch_block
    }
    /// Return the overhead of Revm execution.
    pub fn execution(&self) -> u64 {
        self.execution
    }
    /// Return the overhead of process state.
    pub fn take_output_state(&self) -> u64 {
        self.take_output_state
    }
    /// Return the overhead of write to db.
    pub fn write_to_db(&self) -> u64 {
        self.write_to_db
    }
    /// Return the overhead of misc.
    pub fn misc(&self) -> u64 {
        self.total - self.fetch_block - self.execution - self.take_output_state - self.write_to_db
    }
}
