//! This module is used to support the display of duration related metrics.
use super::commons::*;
use crate::metrics::{ExecuteTxsRecord, ExecutionDurationRecord, WriteToDbRecord};

const COL_WIDTH_LARGE: usize = 50;
const COL_WIDTH_BIG: usize = 25;
const COL_WIDTH_MIDDLE: usize = 15;

fn print_time(cat: &str, cycles: u64, total: u64) {
    let time = cycles_as_secs(cycles);
    let pct = time / cycles_as_secs(total);

    println!(
        "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}",
        cat,
        time,
        pct * 100.0,
    );
}

impl ExecuteTxsRecord {
    fn print_items(&self, ident: &str, total: u64) {
        let misc = self.total -
            self.transact -
            self.commit_changes -
            self.add_receipt -
            self.apply_post_execution_state_change -
            self.merge_transactions -
            self.verify_receipt -
            self.save_receipts;
        print_time(&(ident.to_owned() + "misc"), misc, total);
        print_time(&(ident.to_owned() + "transact"), self.transact, total);
        print_time(&(ident.to_owned() + "    revm_transact"), self.revm_transact.total, total);
        print_time(
            &(ident.to_owned() + "    preverify_transaction_inner"),
            self.revm_transact.preverify_transaction_inner,
            total,
        );
        print_time(
            &(ident.to_owned() + "    before execute(transact_preverified_inner)"),
            self.revm_transact.transact_preverified_inner.before_execute,
            total,
        );
        print_time(
            &(ident.to_owned() + "    execute(transact_preverified_inner)"),
            self.revm_transact.transact_preverified_inner.execute,
            total,
        );
        print_time(
            &(ident.to_owned() + "    after_execute(transact_preverified_inner)"),
            self.revm_transact.transact_preverified_inner.after_execute,
            total,
        );
        print_time(&(ident.to_owned() + "    handler_end"), self.revm_transact.handle_end, total);
        print_time(&(ident.to_owned() + "commit"), self.commit_changes, total);
        print_time(&(ident.to_owned() + "add_receipt"), self.add_receipt, total);
        print_time(
            &(ident.to_owned() + "apply_post_execution_state_change"),
            self.apply_post_execution_state_change,
            total,
        );
        print_time(&(ident.to_owned() + "merge_transactions"), self.merge_transactions, total);
        print_time(&(ident.to_owned() + "verify_receipt"), self.verify_receipt, total);
        print_time(&(ident.to_owned() + "save receipts"), self.save_receipts, total);
    }
}

impl Print for ExecuteTxsRecord {
    fn print_title(&self) {
        println!(
            "=============================Breakdown of execute txs =========================="
        );
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (s)", "Time (%)",
        );
    }
    fn print_content(&self) {
        print_time("total", self.total, self.total);
        self.print_items("", self.total);
    }
}

fn print_time_and_size(cat: &str, size: Option<usize>, cycles: u64, total_cycles: u64) {
    let total_time = cycles_as_secs(total_cycles);
    let time = cycles_as_secs(cycles);
    let pct = time / total_time;
    let (size_value, rate_value) = match size {
        Some(size) => {
            let size = convert_bytes_to_mega(size);
            let rate = size / time;
            (Some(size), Some(rate))
        }
        None => (None, None),
    };

    println!(
        "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_BIG$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}{: >COL_WIDTH_BIG$.3}",
        cat,
        size_value.unwrap_or(std::f64::NAN),
        time,
        pct * 100.0,
        rate_value.unwrap_or(std::f64::NAN)
    );
}

impl Print for WriteToDbRecord {
    fn print_title(&self) {
        println!("=======================================================Breakdown of write_to_db ==================================================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category",  
            "Size (MB)",   
            "Time (s)",    
            "Time (%)",   
            "Rate (MB/s)"
        );
    }
    fn print_content(&self) {
        let total_size = self.revert_storage_size +
            self.revert_account_size +
            self.write_receipts_size +
            self.state_account_size +
            self.state_bytecode_size +
            self.state_storage_size;

        print_time_and_size("total", Some(total_size), self.total, self.total);
        print_time_and_size(
            "write storage (revert state)",
            Some(self.revert_storage_size),
            self.revert_storage_time,
            self.total,
        );
        print_time_and_size(
            "    write storage iter time (revert state)",
            None,
            self.revert_storage_time - self.revert_storage_append_time,
            self.total,
        );
        print_time_and_size(
            "    write storage append time (revert state)",
            Some(self.revert_storage_size),
            self.revert_storage_append_time,
            self.total,
        );
        print_time_and_size(
            "write account (revert state)",
            Some(self.revert_account_size),
            self.revert_account_time,
            self.total,
        );
        print_time_and_size(
            "    write account iter time (revert state)",
            None,
            self.revert_account_time - self.revert_account_append_time,
            self.total,
        );
        print_time_and_size(
            "    write account append time (revert state)",
            Some(self.revert_account_size),
            self.revert_account_append_time,
            self.total,
        );
        print_time_and_size(
            "write_receipts",
            Some(self.write_receipts_size),
            self.write_receipts_time,
            self.total,
        );
        print_time_and_size(
            "    write receipts iter time",
            None,
            self.write_receipts_time - self.receipts_append_time,
            self.total,
        );
        print_time_and_size(
            "    write receipts append time",
            Some(self.write_receipts_size),
            self.receipts_append_time,
            self.total,
        );
        print_time_and_size("sort state changes", None, self.sort_time, self.total);
        print_time_and_size(
            "write account (state changes)",
            Some(self.state_account_size),
            self.state_account_time,
            self.total,
        );
        print_time_and_size(
            "    write account iter time (state changes)",
            None,
            self.state_account_time - self.state_account_upsert_time,
            self.total,
        );
        print_time_and_size(
            "    write account upsert time (state changes)",
            Some(self.state_account_size),
            self.state_account_upsert_time,
            self.total,
        );
        print_time_and_size(
            "write bytecode (state changes)",
            Some(self.state_bytecode_size),
            self.state_bytecode_time,
            self.total,
        );
        print_time_and_size(
            "    write bytecode iter time (state changes)",
            None,
            self.state_bytecode_time - self.state_bytecode_upsert_time,
            self.total,
        );
        print_time_and_size(
            "    write bytecode upsert time (state changes)",
            Some(self.state_bytecode_size),
            self.state_bytecode_upsert_time,
            self.total,
        );
        print_time_and_size(
            "write storage (state_changes)",
            Some(self.state_storage_size),
            self.state_storage_time,
            self.total,
        );
        print_time_and_size(
            "    write storage iter time (state_changes)",
            None,
            self.state_storage_time - self.state_storage_upsert_time,
            self.total,
        );
        print_time_and_size(
            "    write storage upsert time (state_changes)",
            Some(self.state_storage_size),
            self.state_storage_upsert_time,
            self.total,
        );
    }
}

impl WriteToDbRecord {
    fn print_items(&self, ident: &str, total: u64) {
        print_time(
            &(ident.to_owned() + "write storage (revert state)"),
            self.revert_storage_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write storage iter time (revert state)"),
            self.revert_storage_time - self.revert_storage_append_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write storage append time (revert state)"),
            self.revert_storage_append_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "write account (revert state)"),
            self.revert_account_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write account iter time (revert state)"),
            self.revert_account_time - self.revert_account_append_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write account append time (revert state)"),
            self.revert_account_append_time,
            total,
        );
        print_time(&(ident.to_owned() + "write_receipts"), self.write_receipts_time, total);
        print_time(
            &(ident.to_owned() + "    write receipts iter time"),
            self.write_receipts_time - self.receipts_append_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write receipts append time"),
            self.receipts_append_time,
            total,
        );
        print_time(&(ident.to_owned() + "sort state changes"), self.sort_time, total);
        print_time(
            &(ident.to_owned() + "write account (state changes)"),
            self.state_account_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write account iter time (state changes)"),
            self.state_account_time - self.state_account_upsert_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write account upsert time (state changes)"),
            self.state_account_upsert_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "write bytecode (state changes)"),
            self.state_bytecode_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write bytecode iter time (state changes)"),
            self.state_bytecode_time - self.state_bytecode_upsert_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write bytecode upsert time (state changes)"),
            self.state_bytecode_upsert_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "write storage (state_changes)"),
            self.state_storage_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write storage iter time (state_changes)"),
            self.state_storage_time - self.state_storage_upsert_time,
            total,
        );
        print_time(
            &(ident.to_owned() + "    write storage upsert time (state_changes)"),
            self.state_storage_upsert_time,
            total,
        );
    }
}

impl Print for ExecutionDurationRecord {
    fn print_title(&self) {
        println!(
            "===========================Breakdown of ExecutionStage=========================="
        );
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (s)", "Time (%)",
        );
    }

    fn print_content(&self) {
        let misc = self.total -
            self.block_td -
            self.block_with_senders -
            self.execution.total -
            self.write_to_db.total;
        print_time("total", self.total, self.total);
        print_time("misc", misc, self.total);
        print_time("block_td", self.block_td, self.total);
        print_time("block_with_senders", self.block_with_senders, self.total);
        print_time("execute_and_verify_receipt", self.execution.total, self.total);
        self.execution.print_items("    ", self.total);
        print_time("write_to_db", self.write_to_db.total, self.total);
        self.write_to_db.print_items("    ", self.total);
    }

    fn print(&self, _block_number: u64) {
        // Print all duration.
        println!();
        self.print_title();
        self.print_content();
        println!();

        // Print execute txs duration.
        self.execution.print(_block_number);

        // Print write_to_db duration.
        self.write_to_db.print(_block_number);
    }
}
