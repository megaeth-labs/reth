//! The functions defined in this file are mainly used to measure the execute stage.

// The functions in execute_inner module should be called in the execute_inner function of
// execution stage.
pub mod execute_inner {
    use crate::metrics::metric::*;
    pub fn start_record() {
        #[cfg(feature = "enable_execution_duration_record")]
        recorder().duration_record.start_total_record();
    }

    pub fn record_before_loop() {
        #[cfg(feature = "enable_tps_gas_record")]
        let _ =
            recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::BlockTpsAndGas {
                block_number: recorder().block_number,
                record: TpsAndGasMessage::Switch(true),
            });
    }

    pub fn record_before_td(block_number: u64) {
        #[cfg(feature = "enable_execution_duration_record")]
        recorder().duration_record.start_time_record();

        recorder().block_number = block_number;
    }

    pub fn record_after_td() {
        #[cfg(feature = "enable_execution_duration_record")]
        {
            recorder().duration_record.add_block_td_duration();
            recorder().duration_record.start_time_record();
        }
    }

    pub fn record_after_block_with_senders() {
        #[cfg(feature = "enable_execution_duration_record")]
        {
            recorder().duration_record.add_block_with_senders_duration();
            recorder().duration_record.start_time_record();
        }
    }

    pub fn record_after_get_tps(_block_number: u64, _txs: u64, _gas: u64) {
        #[cfg(feature = "enable_tps_gas_record")]
        {
            recorder().tps_gas_record.record(_block_number, _txs as u128, _gas as u128);
            let _ = recorder().events_tx.as_mut().expect("No sender").send(
                MetricEvent::BlockTpsAndGas {
                    block_number: recorder().block_number,
                    record: TpsAndGasMessage::Record(recorder().tps_gas_record),
                },
            );
        }
    }

    pub fn record_after_take_output_state() {
        #[cfg(feature = "enable_tps_gas_record")]
        let _ =
            recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::BlockTpsAndGas {
                block_number: recorder().block_number,
                record: TpsAndGasMessage::Switch(false),
            });

        #[cfg(feature = "enable_execution_duration_record")]
        recorder().duration_record.start_time_record();
    }

    pub fn record_at_end(_cachedb_size: usize) {
        #[cfg(feature = "enable_execution_duration_record")]
        {
            recorder().duration_record.add_total_duration();
            let _ = recorder().events_tx.as_mut().expect("No sender").send(
                MetricEvent::ExecutionStageTime {
                    block_number: recorder().block_number,
                    record: recorder().duration_record,
                },
            );
        }

        #[cfg(feature = "enable_cache_record")]
        {
            let cachedb_record = revm_utils::metrics::get_cache_record();
            recorder().cachedb_record.update(&cachedb_record);
            let _ =
                recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::CacheDbInfo {
                    block_number: recorder().block_number,
                    size: _cachedb_size,
                    record: recorder().cachedb_record,
                });
        }

        #[cfg(feature = "enable_opcode_metrics")]
        let _ = recorder().events_tx.as_mut().expect("No sender").send(MetricEvent::OpcodeInfo {
            block_number: recorder().block_number,
            record: recorder().op_record,
        });

        #[cfg(feature = "enable_state_root_record")]
        let _ = recorder()
            .events_tx
            .as_mut()
            .expect("No sender")
            .send(MetricEvent::StateRootUpdatePrint {});
    }
}

// The functions in this module should be called in executor.
#[cfg(feature = "enable_opcode_metrics")]
pub mod revm_measure {
    /// After each transaction is executed, the execution status of instructions is counted and
    /// then updated to the global metric recorder. This function will be called in
    /// executor.
    pub fn record_opcode() {
        let mut op_record = revm_utils::metrics::get_op_record();
        if op_record.not_empty() {
            crate::recorder().op_record.update(&mut op_record);
        }
    }
}

// The functions in this module should be called in executor.
#[cfg(feature = "enable_execution_duration_record")]
pub mod execute_txs {
    use crate::metrics::metric::*;

    /// start execute_tx record.
    pub fn start_execute_tx_record() {
        recorder().duration_record.execution.start_record();
    }

    /// start execute_tx sub record.
    pub fn start_execute_tx_sub_record() {
        recorder().duration_record.execution.start_sub_record();
    }

    /// transact record
    pub fn transact_record() {
        recorder().duration_record.execution.transact_record();
    }

    /// commit_changes_record
    pub fn commit_changes_record() {
        recorder().duration_record.execution.commit_changes_record();
    }

    /// add_receipt_record
    pub fn add_receipt_record() {
        recorder().duration_record.execution.add_receipt_record();
    }

    /// apply_post_execution_state_change_record
    pub fn apply_post_execution_state_change_record() {
        recorder().duration_record.execution.apply_post_execution_state_change_record();
    }

    /// merge_transactions_record
    pub fn merge_transactions_record() {
        recorder().duration_record.execution.merge_transactions_record();
    }

    /// verify_receipt_record
    pub fn verify_receipt_record() {
        recorder().duration_record.execution.verify_receipt_record();
    }

    /// save_receipts_record
    pub fn save_receipts_record() {
        recorder().duration_record.execution.save_receipts_record();
    }

    /// get_execute_tx_record
    pub fn get_execute_tx_record() -> crate::metrics::ExecuteTxsRecord {
        recorder().duration_record.execution
    }

    /// Record for verfity_and_save_receipts
    pub struct VerifyAndSaveReceiptsRecord;

    impl VerifyAndSaveReceiptsRecord {
        /// Return VerifyAndSaveReceiptsRecord
        pub fn new() -> Self {
            verify_receipt_record();
            VerifyAndSaveReceiptsRecord
        }
    }

    impl Drop for VerifyAndSaveReceiptsRecord {
        fn drop(&mut self) {
            save_receipts_record();
        }
    }
}

// The functions in the module will be used to measure write_to_db and will be called in
// write_to_db.
#[cfg(feature = "enable_execution_duration_record")]
pub mod write_to_db {
    use crate::metrics::metric::*;

    /// start write_to_db record.
    pub fn start_write_to_db_record() {
        recorder().duration_record.write_to_db.start_record();
    }

    /// start write_to_db sub record.
    pub fn start_write_to_db_sub_record() {
        recorder().duration_record.write_to_db.start_sub_record();
    }

    /// start write_to_db write record.
    fn start_write_to_db_write_record() {
        recorder().duration_record.write_to_db.start_write_record();
    }

    /// Record data size of write storage changes in StateReverts's write_to_db.
    fn record_revert_storage_size(size: usize) {
        recorder().duration_record.write_to_db.record_revert_storage_size(size);
    }

    /// Record time of write storage append time in StateReverts's write_to_db.
    fn record_revert_storage_append_time() {
        recorder().duration_record.write_to_db.record_revert_storage_append_time();
    }

    // Encapsulate this structure to record write_storage in revert state in a RAII manner.
    impl_write_macro!(
        RevertsStorageWrite,
        start_write_to_db_write_record,
        record_revert_storage_append_time,
        record_revert_storage_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write storage changes in StateReverts's write_to_db.
    pub fn record_revert_storage_time() {
        recorder().duration_record.write_to_db.record_revert_storage_time();
    }

    /// Record data size of write account changes in StateReverts's write_to_db.
    fn record_revert_account_size(size: usize) {
        recorder().duration_record.write_to_db.record_revert_account_size(size);
    }

    /// Record time of write account append time in StateReverts's write_to_db.
    fn record_revert_account_append_time() {
        recorder().duration_record.write_to_db.record_revert_account_append_time();
    }

    // Encapsulate this structure to record write_account in revert state in a RAII manner.
    impl_write_macro!(
        RevertsAccountWrite,
        start_write_to_db_write_record,
        record_revert_account_append_time,
        record_revert_account_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write account changes in StateReverts's write_to_db.
    pub fn record_revert_account_time() {
        recorder().duration_record.write_to_db.record_revert_account_time();
    }

    /// Record data size of write receipts in BundleStateWithReceipts's write_to_db.
    fn record_write_receipts_size(size: usize) {
        recorder().duration_record.write_to_db.record_write_receipts_size(size);
    }

    /// Record time of write receipts append in BundleStateWithReceipts's write_to_db.
    fn record_receipts_append_time() {
        recorder().duration_record.write_to_db.record_receipts_append_time();
    }

    // Encapsulate this structure to record write receipts in a RAII manner.
    impl_write_macro!(
        ReceiptsWrite,
        start_write_to_db_write_record,
        record_receipts_append_time,
        record_write_receipts_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write receipts  in BundleStateWithReceipts's write_to_db.
    pub fn record_write_receipts_time() {
        recorder().duration_record.write_to_db.record_write_receipts_time();
    }

    /// Record time of sort in StateChanges's write_to_db.
    pub fn record_sort_time() {
        recorder().duration_record.write_to_db.record_sort_time();
    }

    /// Record data size of write account in StateChanges's write_to_db.
    fn record_state_account_size(size: usize) {
        recorder().duration_record.write_to_db.record_state_account_size(size);
    }

    /// Record time of write account upsert in StateChanges's write_to_db.
    fn record_state_account_upsert_time() {
        recorder().duration_record.write_to_db.record_state_account_upsert_time();
    }

    // Encapsulate this structure to record write_account in state changes in a RAII manner.
    impl_write_macro!(
        StateAccountWrite,
        start_write_to_db_write_record,
        record_state_account_upsert_time,
        record_state_account_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write account in StateChanges's write_to_db.
    pub fn record_state_account_time() {
        recorder().duration_record.write_to_db.record_state_account_time();
    }

    /// Record data size of write bytecode in StateChanges's write_to_db.
    fn record_state_bytecode_size(size: usize) {
        recorder().duration_record.write_to_db.record_state_bytecode_size(size);
    }

    /// Record time of write bytecode upsert in StateChanges's write_to_db.
    fn record_state_bytecode_upsert_time() {
        recorder().duration_record.write_to_db.record_state_bytecode_upsert_time();
    }

    // Encapsulate this structure to record write_bytecode in state changes in a RAII manner.
    impl_write_macro!(
        StateBytecodeWrite,
        start_write_to_db_write_record,
        record_state_bytecode_upsert_time,
        record_state_bytecode_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write bytecode in StateChanges's write_to_db.
    pub fn record_state_bytecode_time() {
        recorder().duration_record.write_to_db.record_state_bytecode_time();
    }

    /// Record data size of write storage in StateChanges's write_to_db.
    fn record_state_storage_size(size: usize) {
        recorder().duration_record.write_to_db.record_state_storage_size(size);
    }

    /// Record time of write storage upsert in StateChanges's write_to_db.
    fn record_state_storage_upsert_time() {
        recorder().duration_record.write_to_db.record_state_storage_upsert_time();
    }

    // Encapsulate this structure to record write_storage in state changes in a RAII manner.
    impl_write_macro!(
        StateStorageWrite,
        start_write_to_db_write_record,
        record_state_storage_upsert_time,
        record_state_storage_size,
        record_receipts_append_time,
        record_write_receipts_size
    );

    /// Record time of write storage in StateChanges's write_to_db.
    pub fn record_state_storage_time() {
        recorder().duration_record.write_to_db.record_state_storage_time();
    }
}
