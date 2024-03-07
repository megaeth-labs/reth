//! This module is used to support recording the overhead of various parts
//! of the execute_inner function in execution stage.
use revm_utils::{metrics::types::TransactTime, time_utils::instant::Instant};

/// This structure is used to record all overhead information.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionDurationRecord {
    // Total time recorder.
    pub(crate) total_recorder: Instant,
    // General time recorder.
    pub(crate) time_recorder: Instant,
    // Time of execute inner.
    pub(crate) total: u64,
    // Time of get_block_td.
    pub(crate) block_td: u64,
    // Time of block_with_senders.
    pub(crate) block_with_senders: u64,
    // Record of txs execution(execute_and_verify_receipt).
    pub(crate) execution: ExecuteTxsRecord,
    // Record of write to db
    pub(crate) write_to_db: WriteToDbRecord,
}

// The following functions are used to record overhead.
impl ExecutionDurationRecord {
    define_start_functions!(start_total_record, total_recorder);
    define_start_functions!(start_time_record, time_recorder);
    define_record_time_function!(add_total_duration, total, total_recorder);
    define_record_time_function!(add_block_td_duration, block_td, time_recorder);
    define_record_time_function!(
        add_block_with_senders_duration,
        block_with_senders,
        time_recorder
    );
}

/// This structure is used to support in-depth measurement of function execute_and_verify_receipt
/// in stage execution.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteTxsRecord {
    /// Record the starting time of function execute_and_verify_receipt.
    start_record: Instant,
    /// Record the start time of each subfunction.
    sub_record: Instant,
    /// Time of execute_and_verify_receipt.
    pub(crate) total: u64,
    /// Time of transact.
    pub(crate) transact: u64,
    /// Time of revm's transact.
    pub(crate) revm_transact: TransactTime,
    /// Time of commit changes.
    pub(crate) commit_changes: u64,
    /// Time of add receipt.
    pub(crate) add_receipt: u64,
    /// Time of apply_post_execution_state_change.
    pub(crate) apply_post_execution_state_change: u64,
    /// Time of merge_transactions.
    pub(crate) merge_transactions: u64,
    /// Time of verify_receipt.
    pub(crate) verify_receipt: u64,
    /// Time of save_receipts.
    pub(crate) save_receipts: u64,
}

impl ExecuteTxsRecord {
    define_start_functions!(start_record, start_record);
    define_start_functions!(start_sub_record, sub_record);

    define_record_with_elapsed_time_function!(commit_changes_record, commit_changes, sub_record);
    define_record_with_elapsed_time_function!(add_receipt_record, add_receipt, sub_record);
    define_record_with_elapsed_time_function!(
        apply_post_execution_state_change_record,
        apply_post_execution_state_change,
        sub_record
    );
    define_record_with_elapsed_time_function!(
        merge_transactions_record,
        merge_transactions,
        sub_record
    );
    define_record_with_elapsed_time_function!(verify_receipt_record, verify_receipt, sub_record);
    define_record_with_elapsed_time_function!(
        save_receipts_record_inner,
        save_receipts,
        sub_record
    );
    define_record_with_elapsed_time_function!(transact_record_inner, transact, sub_record);

    /// Add time of transact, which include revm's transact.
    pub(super) fn transact_record(&mut self) {
        self.transact_record_inner();
        let revm_transact = revm_utils::metrics::get_transact_time();
        self.revm_transact.update(&revm_transact);
    }

    /// Add time of save_receipts.
    pub(super) fn save_receipts_record(&mut self) {
        let now = self.save_receipts_record_inner();
        self.record_total_time(now);
    }

    /// Record total time.
    fn record_total_time(&mut self, now: Instant) {
        let cycles = now.checked_cycles_since(self.start_record).unwrap_or(0);
        self.total = self.total.checked_add(cycles).expect("overflow");
    }
}

/// This structure is used to record all the metrics of write_to_db, including
/// the time spent writing and the amount of data written.
#[derive(Debug, Clone, Copy, Default)]
pub struct WriteToDbRecord {
    /// Record the starting time of function write_to_db.
    start_record: Instant,
    /// Record the start time of each subfunction.
    sub_record: Instant,
    /// Record the start time of each put or upsert.
    write_start_record: Instant,

    /// Time of write_to_db.
    pub(crate) total: u64,

    /// Time of write storage changes in StateReverts.
    pub(crate) revert_storage_time: u64,
    /// Data size of write storage changes in StateReverts.
    pub(crate) revert_storage_size: usize,
    /// Time of append_dup when write storage changes in StateReverts.
    pub(crate) revert_storage_append_time: u64,
    /// Time of write account changes in StateReverts.
    pub(crate) revert_account_time: u64,
    /// Data size of write account changes in StateReverts.
    pub(crate) revert_account_size: usize,
    /// Time of append_dup when write account changes in StateReverts.
    pub(crate) revert_account_append_time: u64,

    /// Time of write receipts.
    pub(crate) write_receipts_time: u64,
    /// Data size of write receipts.
    pub(crate) write_receipts_size: usize,
    /// Time of append when write receipts.
    pub(crate) receipts_append_time: u64,

    /// Time of sort in StateChanges's write_to_db.
    pub(crate) sort_time: u64,
    /// Time of write account in StateChanges.
    pub(crate) state_account_time: u64,
    /// Data size of write account in StateChanges.
    pub(crate) state_account_size: usize,
    /// Time of upsert when write account changes in StateChanges.
    pub(crate) state_account_upsert_time: u64,

    /// Time of write bytecode in StateChanges.
    pub(crate) state_bytecode_time: u64,
    /// Data size of write bytecode in StateChanges.
    pub(crate) state_bytecode_size: usize,
    /// Time of upsert when write bytecode in StateChanges.
    pub(crate) state_bytecode_upsert_time: u64,

    /// Time of write storage in StateChanges.
    pub(crate) state_storage_time: u64,
    /// Data size of write storage in StateChanges.
    pub(crate) state_storage_size: usize,
    /// Time of upsert when write storage in StateChanges.
    pub(crate) state_storage_upsert_time: u64,
}

impl WriteToDbRecord {
    define_start_functions!(start_record, start_record);
    define_start_functions!(start_sub_record, sub_record);
    define_start_functions!(start_write_record, write_start_record);

    define_record_size_function!(record_revert_storage_size, revert_storage_size);
    define_record_size_function!(record_revert_account_size, revert_account_size);
    define_record_size_function!(record_write_receipts_size, write_receipts_size);
    define_record_size_function!(record_state_account_size, state_account_size);
    define_record_size_function!(record_state_bytecode_size, state_bytecode_size);
    define_record_size_function!(record_state_storage_size, state_storage_size);

    define_record_with_elapsed_time_function!(
        record_revert_storage_time,
        revert_storage_time,
        sub_record
    );
    define_record_with_elapsed_time_function!(
        record_revert_account_time,
        revert_account_time,
        sub_record
    );
    define_record_with_elapsed_time_function!(
        record_write_receipts_time,
        write_receipts_time,
        sub_record
    );
    define_record_with_elapsed_time_function!(record_sort_time, sort_time, sub_record);
    define_record_with_elapsed_time_function!(
        record_state_account_time,
        state_account_time,
        sub_record
    );
    define_record_with_elapsed_time_function!(
        record_state_bytecode_time,
        state_bytecode_time,
        sub_record
    );
    define_record_with_elapsed_time_function!(
        record_state_storage_time_inner,
        state_storage_time,
        sub_record
    );

    define_record_with_elapsed_time_function!(
        record_revert_storage_append_time,
        revert_storage_append_time,
        write_start_record
    );
    define_record_with_elapsed_time_function!(
        record_revert_account_append_time,
        revert_account_append_time,
        write_start_record
    );
    define_record_with_elapsed_time_function!(
        record_receipts_append_time,
        receipts_append_time,
        write_start_record
    );
    define_record_with_elapsed_time_function!(
        record_state_account_upsert_time,
        state_account_upsert_time,
        write_start_record
    );
    define_record_with_elapsed_time_function!(
        record_state_bytecode_upsert_time,
        state_bytecode_upsert_time,
        write_start_record
    );
    define_record_with_elapsed_time_function!(
        record_state_storage_upsert_time,
        state_storage_upsert_time,
        write_start_record
    );

    /// Record time of write storage in StateChanges.
    pub(super) fn record_state_storage_time(&mut self) {
        let now = self.record_state_storage_time_inner();
        self.record_total_time(now);
    }
    /// Record total time.
    fn record_total_time(&mut self, now: Instant) {
        let cycles = now.checked_cycles_since(self.start_record).unwrap_or(0);
        self.total = self.total.checked_add(cycles).expect("overflow");
    }
}
