use super::commons::*;
use revm_utils::time_utils::convert_cycles_to_ns_f64;
use crate::DBRead;
use crate::metrics::StateRootUpdateRecord;
use crate::state_root::StateRootRecord;

const COL_WIDTH_MIDDLE: usize = 14;
const COL_WIDTH_BIG: usize = 20;
const COL_WIDTH_LITTLE_BIG: usize = 25;
const COL_WIDTH_LARGE: usize = 48;

#[derive(Debug, Default)]
pub(crate) struct StateRootUpdateDisplayer {
    recorder: StateRootRecord,
    record: StateRootUpdateRecord,
    last_print_block_number: u64,
}

impl StateRootUpdateDisplayer {
    const N: u64 = 1000;

    pub(crate) fn update_record(&mut self, recorder: StateRootRecord) {
        self.recorder = recorder;
    }

    pub(crate) fn record(&mut self, record: StateRootUpdateRecord) {
        self.record.add(record);
    }

    fn print_line_u64(&self, name: &str, value: u64) {
        println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}", name, value);
    }

    fn print_line_f64(&self, name: &str, value: f64) {
        println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$.3}", name, value);
    }

    fn print_caculate_stat(&self, record: &crate::metrics::CaculateStat) {
        println!("record:{:?}", record);
        self.print_line_f64("calculate(s)", cycles_as_secs(record.total_time));
        self.print_line_f64("before_loop(s)", cycles_as_secs(record.before_loop));
        self.print_line_f64("loop_begin(s)", cycles_as_secs(record.loop_begin));
        self.print_line_u64("try_next_count", record.try_next_stat.total_count);
        self.print_line_f64("try_next(s)", cycles_as_secs(record.try_next_stat.total_time));
        self.print_line_f64(
            "avg_try_next(ns)",
            convert_cycles_to_ns_f64(record.try_next_stat.total_time) /
                record.try_next_stat.total_count as f64,
        );
        self.print_line_u64("add_branch_count", record.add_branch_count);
        self.print_line_f64("add_branch(s)", cycles_as_secs(record.add_branch));
        self.print_line_f64(
            "avg_add_branch(ns)",
            convert_cycles_to_ns_f64(record.add_branch) / record.add_branch_count as f64,
        );
        self.print_line_f64(
            "cal_storage_root_and_add_leaf(s)",
            cycles_as_secs(record.cal_storage_root_and_add_leaf),
        );
        self.print_line_f64(
            "after_cal_storage_root(s)",
            cycles_as_secs(record.after_cal_storage_root),
        );
        self.print_line_u64("add_leaf_count", record.add_leaf_count);
        self.print_line_f64("add_leaf(s)", cycles_as_secs(record.add_leaf));
        self.print_line_f64(
            "avg_add_leaf(ns)",
            convert_cycles_to_ns_f64(record.add_leaf) / record.add_leaf_count as f64,
        );
        self.print_line_u64("add_root_count", record.add_root_count);
        self.print_line_f64("add_root(s)", cycles_as_secs(record.add_root));
        self.print_line_f64(
            "avg_add_root(ns)",
            convert_cycles_to_ns_f64(record.add_root) / record.add_root_count as f64,
        );
        let add_node_count =
            record.add_branch_count + record.add_leaf_count + record.add_root_count;
        let add_node_time = record.add_branch + record.add_leaf + record.add_root;
        self.print_line_u64("add_node_count", add_node_count);
        self.print_line_f64("add_node_time(s)", cycles_as_secs(add_node_time));
        self.print_line_f64(
            "avg_add_node_time(ns)",
            convert_cycles_to_ns_f64(add_node_time) / add_node_count as f64,
        );

        self.print_line_f64("after_loop(s)", cycles_as_secs(record.after_add_root));
        println!("");
        println!("");
    }

    fn print_trie_db_stat(&self, distribution: &crate::metrics::TimeDistributionStats) {
        let total_cnt: u64 = distribution.us_percentile.iter().map(|&v| v).sum();
        let mut cuml = 0.0;
        println!();
        println!();
        println!("total cnt: {:?}", total_cnt);
        println!("span_in_ns: {:?}", distribution.span_in_ns);
        println!("span_in_us: {:?}", distribution.span_in_us);
        println!("in_ns: {:?}", distribution.ns_percentile);
        println!("in_us: {:?}", distribution.us_percentile);
        println!();
        println!();
        println!("Time (ns)       Count (%)       Cuml. (%)");
        for index in 0..distribution.span_in_ns {
            let pct = distribution.ns_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!("{:9} {:15.3} {:15.3}", (index + 1) * 100, pct * 100.0, cuml * 100.0);
        }

        let ns_span_in_us = ((distribution.span_in_ns * 100) as f64 / 1000.0) as usize;
        for index in ns_span_in_us..distribution.span_in_us {
            let pct = distribution.us_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!("{:9} {:15.3} {:15.3}", (index + 1) * 1000, pct * 100.0, cuml * 100.0);
        }
    }

    fn print_db_read(&self, db_read: &DBRead) {
        println!("\n===============================Metric of db read ==============================================");
        println!("db_read:{:?}", db_read);
        let total_count = db_read.total_count();
        let total_time_cycles = db_read.total_time();
        self.print_line_u64("total_count", total_count);
        self.print_line_u64("total_time_cycles", total_time_cycles);
        self.print_line_f64("total_time(s)", cycles_as_secs(total_time_cycles));
        self.print_line_f64(
            "avg_time(ns)",
            convert_cycles_to_ns_f64(total_time_cycles) / total_count as f64,
        );

        self.print_line_u64("current_count", db_read.current_count);
        self.print_line_f64("current(s)", cycles_as_secs(db_read.current));
        self.print_line_f64(
            "avg_current(ns)",
            convert_cycles_to_ns_f64(db_read.current) / db_read.current_count as f64,
        );

        self.print_line_u64("seek_count", db_read.seek_count);
        self.print_line_f64("seek(s)", cycles_as_secs(db_read.seek));
        self.print_line_f64(
            "avg_seek(ns)",
            convert_cycles_to_ns_f64(db_read.seek) / db_read.seek_count as f64,
        );

        self.print_line_u64("next_count", db_read.next_count);
        self.print_line_f64("next(s)", cycles_as_secs(db_read.next));
        self.print_line_f64(
            "avg_next(ns)",
            convert_cycles_to_ns_f64(db_read.next) / db_read.next_count as f64,
        );

        self.print_line_u64("seek_exact_count", db_read.seek_exact_count);
        self.print_line_f64("seek_exact(s)", cycles_as_secs(db_read.seek_exact));
        self.print_line_f64(
            "avg_seek_exact(ns)",
            convert_cycles_to_ns_f64(db_read.seek_exact) / db_read.seek_exact_count as f64,
        );

        self.print_line_u64("seek_by_sub_key_count", db_read.seek_by_sub_key_count);
        self.print_line_f64("seek_by_sub_key(s)", cycles_as_secs(db_read.seek_by_sub_key));
        self.print_line_f64(
            "avg_seek_by_sub_key(ns)",
            convert_cycles_to_ns_f64(db_read.seek_by_sub_key) /
                db_read.seek_by_sub_key_count as f64,
        );

        self.print_line_u64("next_dup_val_count", db_read.next_dup_val_count);
        self.print_line_f64("next_dup_val(s)", cycles_as_secs(db_read.next_dup_val));
        self.print_line_f64(
            "avg_next_dup_val(ns)",
            convert_cycles_to_ns_f64(db_read.next_dup_val) / db_read.next_dup_val_count as f64,
        );

        self.print_line_u64("walker_seek_count", db_read.account_trie_seek_count);
        self.print_line_f64("walker_seek(s)", cycles_as_secs(db_read.account_trie_seek));
        self.print_line_f64(
            "avg_walker_seek(ns)",
            convert_cycles_to_ns_f64(db_read.account_trie_seek) /
                db_read.account_trie_seek_count as f64,
        );

        self.print_line_u64("walker_seek_exact_count", db_read.account_trie_seek_exact_count);
        self.print_line_f64(
            "walker_seek_exact(s)",
            cycles_as_secs(db_read.account_trie_seek_exact),
        );
        self.print_line_f64(
            "avg_walker_seek_exact(ns)",
            convert_cycles_to_ns_f64(db_read.account_trie_seek_exact) /
                db_read.account_trie_seek_exact_count as f64,
        );

        self.print_line_u64("walker_current_count", db_read.account_trie_current_count);
        self.print_line_f64("walker_current(s)", cycles_as_secs(db_read.account_trie_current));
        self.print_line_f64(
            "avg_walker_current(ns)",
            convert_cycles_to_ns_f64(db_read.account_trie_current) /
                db_read.account_trie_current_count as f64,
        );

        println!("");
    }

    pub(crate) fn print_time_line(&self, name: &str, time_cycles: u64, total_time_cycles: u64) {
        let time_s = cycles_as_secs(time_cycles);
        let percent: f64 = if total_time_cycles == 0 {
            0.0
        } else {
            time_cycles as f64 / total_time_cycles as f64
        } * 100.0;

        println!(
            "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}",
            name, time_s, percent
        );
    }

    pub(crate) fn print_breakdown_funtion(&self) {
        println!("");
        println!("");
        println!("============================Breakdown of Function===========================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );

        let state_calculate = self.record.state_calculate_record();
        let storage_calculate = self.record.storage_calculate_record();

        let total_time = self
            .record
            .hash_state_slow()
            .checked_add(self.record.construct_prefix_sets())
            .and_then(|x| x.checked_add(state_calculate.total_time))
            .and_then(|x| x.checked_add(self.record.state_write_to_db()))
            .and_then(|x| x.checked_add(self.record.hashed_state_write()))
            .and_then(|x| x.checked_add(self.record.flush()))
            .expect("overflow");

        let state_add_leaf_misc = state_calculate
            .cal_storage_root_and_add_leaf
            .checked_sub(storage_calculate.total_time)
            .and_then(|x| x.checked_sub(state_calculate.add_leaf))
            .expect("overflow");

        self.print_time_line("total", total_time, total_time);
        self.print_time_line("hash_state_slow", self.record.hash_state_slow(), total_time);
        self.print_time_line(
            "construct_prefix_sets",
            self.record.construct_prefix_sets(),
            total_time,
        );
        self.print_time_line("StateRoot.calculate", state_calculate.total_time, total_time);
        self.print_time_line("    misc", state_calculate.state_misc_time(), total_time);
        self.print_time_line("    before loop", state_calculate.before_loop, total_time);
        self.print_time_line(
            "    AccountNodeIter.try_next",
            state_calculate.try_next_stat.total_time,
            total_time,
        );
        self.print_time_line("    HashBuilder.add_branch", state_calculate.add_branch, total_time);
        self.print_time_line(
            "    add_leaf",
            state_calculate.cal_storage_root_and_add_leaf,
            total_time,
        );
        self.print_time_line("        misc", state_add_leaf_misc, total_time);
        self.print_time_line(
            "        StorageRoot.calculate",
            storage_calculate.total_time,
            total_time,
        );
        self.print_time_line("            misc", storage_calculate.storage_misc_time(), total_time);
        self.print_time_line("            before loop", storage_calculate.before_loop, total_time);
        self.print_time_line(
            "            StorageNodeIter.try_next",
            storage_calculate.try_next_stat.total_time,
            total_time,
        );
        self.print_time_line(
            "            HashBuilder.add_branch",
            storage_calculate.add_branch,
            total_time,
        );
        self.print_time_line(
            "            HashBuilder.add_leaf",
            storage_calculate.add_leaf,
            total_time,
        );
        self.print_time_line(
            "            HashBuilder.root",
            storage_calculate.add_root,
            total_time,
        );
        self.print_time_line(
            "            after HashBuilder.root",
            storage_calculate.after_add_root,
            total_time,
        );
        self.print_time_line("        HashBuilder.add_leaf", state_calculate.add_leaf, total_time);
        self.print_time_line("    HashBuilder.root", state_calculate.add_root, total_time);
        self.print_time_line(
            "    after HashBuilder.root",
            state_calculate.after_add_root,
            total_time,
        );
        self.print_time_line("state.write_to_db", self.record.state_write_to_db(), total_time);
        self.print_time_line("hashed_state_write", self.record.hashed_state_write(), total_time);
        self.print_time_line("flush", self.record.flush(), total_time);

        println!("");
        println!("");
        println!("======================Breakdown of StateRoot.calculate======================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );

        self.print_time_line("total", state_calculate.total_time, state_calculate.total_time);
        self.print_time_line("misc", state_calculate.state_misc_time(), state_calculate.total_time);
        self.print_time_line(
            "before loop",
            state_calculate.before_loop,
            state_calculate.total_time,
        );
        self.print_time_line(
            "AccountNodeIter.try_next",
            state_calculate.try_next_stat.total_time,
            state_calculate.total_time,
        );
        self.print_time_line(
            "HashBuilder.add_branch",
            state_calculate.add_branch,
            state_calculate.total_time,
        );
        self.print_time_line(
            "add_leaf",
            state_calculate.cal_storage_root_and_add_leaf,
            state_calculate.total_time,
        );
        self.print_time_line("    misc", state_add_leaf_misc, state_calculate.total_time);
        self.print_time_line(
            "    StorageRoot.calculate",
            storage_calculate.total_time,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        misc",
            storage_calculate.storage_misc_time(),
            state_calculate.total_time,
        );
        self.print_time_line(
            "        before loop",
            storage_calculate.before_loop,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        StorageNodeIter.try_next",
            storage_calculate.try_next_stat.total_time,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        HashBuilder.add_branch",
            storage_calculate.add_branch,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        HashBuilder.add_leaf",
            storage_calculate.add_leaf,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        HashBuilder.root",
            storage_calculate.add_root,
            state_calculate.total_time,
        );
        self.print_time_line(
            "        after HashBuilder.root",
            storage_calculate.after_add_root,
            state_calculate.total_time,
        );
        self.print_time_line(
            "    HashBuilder.add_leaf",
            state_calculate.add_leaf,
            state_calculate.total_time,
        );
        self.print_time_line(
            "HashBuilder.root",
            state_calculate.add_root,
            state_calculate.total_time,
        );
        self.print_time_line(
            "after HashBuilder.root",
            state_calculate.after_add_root,
            state_calculate.total_time,
        );

        println!("");
        println!("");

        // println!("================================= category =================================");
        println!("=================Breakdown of StateRoot.calculate category==================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );
        let total_try_next = state_calculate
            .try_next_stat
            .total_time
            .checked_add(storage_calculate.try_next_stat.total_time)
            .expect("overflow");

        let add_branch_time =
            state_calculate.add_branch.checked_add(storage_calculate.add_branch).expect("overflow");
        let add_leaf_time =
            state_calculate.add_leaf.checked_add(storage_calculate.add_leaf).expect("overflow");
        let root_time =
            state_calculate.add_root.checked_add(storage_calculate.add_root).expect("overflow");

        let other_time = state_calculate
            .total_time
            .checked_sub(total_try_next)
            .and_then(|x| x.checked_sub(add_branch_time))
            .and_then(|x| x.checked_sub(add_leaf_time))
            .and_then(|x| x.checked_sub(root_time))
            .expect("overflow");

        self.print_time_line("total", state_calculate.total_time, state_calculate.total_time);
        self.print_time_line("other", other_time, state_calculate.total_time);
        self.print_time_line("try_next", total_try_next, state_calculate.total_time);
        self.print_time_line(
            "    AccountNodeIter.try_next(StateRoot)",
            state_calculate.try_next_stat.total_time,
            state_calculate.total_time,
        );
        self.print_time_line(
            "    StorageNodeIter.try_next(StorageRoot)",
            storage_calculate.try_next_stat.total_time,
            state_calculate.total_time,
        );

        self.print_time_line("add_branch", add_branch_time, state_calculate.total_time);
        self.print_time_line(
            "    HashBuilder.add_branch(StateRoot)",
            state_calculate.add_branch,
            state_calculate.total_time,
        );
        self.print_time_line(
            "    HashBuilder.add_branch(StorageRoot)",
            storage_calculate.add_branch,
            state_calculate.total_time,
        );

        self.print_time_line("add_leaf", add_leaf_time, state_calculate.total_time);
        self.print_time_line(
            "    HashBuilder.add_leaf(StateRoot)",
            state_calculate.add_leaf,
            state_calculate.total_time,
        );
        self.print_time_line(
            "    HashBuilder.add_leaf(StorageRoot)",
            storage_calculate.add_leaf,
            state_calculate.total_time,
        );

        self.print_time_line("root", root_time, state_calculate.total_time);
        self.print_time_line(
            "    HashBuilder.root(StateRoot)",
            state_calculate.add_root,
            state_calculate.total_time,
        );
        self.print_time_line(
            "    HashBuilder.root(StorageRoot)",
            storage_calculate.add_root,
            state_calculate.total_time,
        );

        println!("");
        println!("");
        println!("=====================Breakdown of StorageRoot.calculate=====================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );

        self.print_time_line("total", storage_calculate.total_time, storage_calculate.total_time);
        self.print_time_line(
            "misc",
            storage_calculate.storage_misc_time(),
            storage_calculate.total_time,
        );
        self.print_time_line(
            "before loop",
            storage_calculate.before_loop,
            storage_calculate.total_time,
        );
        self.print_time_line(
            "StorageNodeIter.try_next",
            storage_calculate.try_next_stat.total_time,
            storage_calculate.total_time,
        );
        self.print_time_line(
            "HashBuilder.add_branch",
            storage_calculate.add_branch,
            storage_calculate.total_time,
        );
        self.print_time_line(
            "HashBuilder.add_leaf",
            storage_calculate.add_leaf,
            storage_calculate.total_time,
        );
        self.print_time_line(
            "HashBuilder.root",
            storage_calculate.add_root,
            storage_calculate.total_time,
        );
        self.print_time_line(
            "after HashBuilder.root",
            storage_calculate.after_add_root,
            storage_calculate.total_time,
        );

        println!("");
        println!("");
    }

    pub(crate) fn print_try_next(&self) {
        println!("");
        println!("");
        println!("====================== metric of try_next ====================");
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Time (s)", "Avg time (ns)"
        );

        let state_try_next = self.record.state_calculate_record().try_next_stat;
        let storage_try_next = self.record.storage_calculate_record().try_next_stat;

        let total_count =
            state_try_next.total_count.checked_add(storage_try_next.total_count).expect("overflow");
        let total_time_cycles =
            state_try_next.total_time.checked_add(storage_try_next.total_time).expect("overflow");
        let avg_time = convert_cycles_to_ns_f64(total_time_cycles / total_count);

        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            "total", total_count, cycles_as_secs(total_time_cycles), avg_time
        );

        let state_avg_time =
            convert_cycles_to_ns_f64(state_try_next.total_time) / state_try_next.total_count as f64;
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            "AccountTrie", state_try_next.total_count, cycles_as_secs(state_try_next.total_time ), state_avg_time
        );

        let storage_avg_time = convert_cycles_to_ns_f64(storage_try_next.total_time) /
            storage_try_next.total_count as f64;

        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            "StorageTrie", storage_try_next.total_count, cycles_as_secs(storage_try_next.total_time ), storage_avg_time
        );

        println!("");
        println!("");
    }

    pub(crate) fn print_count_line(&self, name: &str, count: u64, total_count: u64) {
        let percent: f64 =
            if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

        println!(
            "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}",
            name, count, percent
        );
    }

    pub(crate) fn print_try_next_read_node(&self) {
        println!("");
        println!("");
        println!("============================ metric of read node ===========================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Count", "Count (%)"
        );

        let state_calculate = self.record.state_calculate_record();
        let storage_calculate = self.record.storage_calculate_record();

        let not_updated_branch_node = state_calculate
            .try_next_stat
            .skip_branch_node_count
            .checked_add(storage_calculate.try_next_stat.skip_branch_node_count)
            .expect("overflow");

        let branch_node = not_updated_branch_node
            .checked_add(self.record.mpt_delete_branch_number())
            .expect("overflow");

        let boundary_reading_leaf = state_calculate
            .try_next_stat
            .leaf_miss_count
            .checked_add(storage_calculate.try_next_stat.leaf_miss_count)
            .expect("overflow");

        let work_leaf = state_calculate
            .try_next_stat
            .leaf_hit_count
            .checked_add(storage_calculate.try_next_stat.leaf_hit_count)
            .expect("overflow");

        let leaf_node = boundary_reading_leaf.checked_add(work_leaf).expect("overflow");

        let total_node = branch_node.checked_add(leaf_node).expect("overflow");

        self.print_count_line("total", total_node, total_node);
        self.print_count_line("branch_nodes", branch_node, total_node);
        self.print_count_line("    ro_branch_nodes", not_updated_branch_node, total_node);
        self.print_count_line(
            "        accountTrie",
            state_calculate.try_next_stat.skip_branch_node_count,
            total_node,
        );

        self.print_count_line(
            "        storageTrie",
            storage_calculate.try_next_stat.skip_branch_node_count,
            total_node,
        );

        self.print_count_line(
            "    updated_branch_nodes",
            self.record.mpt_delete_branch_number(),
            total_node,
        );

        self.print_count_line("leaf_nodes", leaf_node, total_node);
        self.print_count_line("    boundary_reading_leaf", boundary_reading_leaf, total_node);
        self.print_count_line(
            "        accountTrie",
            state_calculate.try_next_stat.leaf_miss_count,
            total_node,
        );
        self.print_count_line(
            "        storageTrie",
            storage_calculate.try_next_stat.leaf_miss_count,
            total_node,
        );

        self.print_count_line("    work_leaf", work_leaf, total_node);

        let account_update_leaves = self.record.account_changes();
        let storage_update_leaves = self.record.storage_changes();

        let account_ro_leaves = state_calculate
            .try_next_stat
            .leaf_hit_count
            .checked_sub(account_update_leaves)
            .expect("overflow");
        let storage_ro_leaves = storage_calculate
            .try_next_stat
            .leaf_hit_count
            .checked_sub(storage_update_leaves)
            .expect("overflow");

        let ro_leaves = account_ro_leaves.checked_add(storage_ro_leaves).expect("overflow");
        self.print_count_line("        ro_leaves", ro_leaves, total_node);
        self.print_count_line("            accountTrie", account_ro_leaves, total_node);
        self.print_count_line("            storageTrie", storage_ro_leaves, total_node);

        let update_leaves =
            account_update_leaves.checked_add(storage_update_leaves).expect("overflow");
        self.print_count_line("        updated_leaves", update_leaves, total_node);
        self.print_count_line("            accountTrie", account_update_leaves, total_node);
        self.print_count_line("            storageTrie", storage_update_leaves, total_node);

        println!("");
        println!("");
    }

    pub(crate) fn print_hashbuilder_line(
        &self,
        name: &str,
        count: u64,
        total_count: u64,
        time_cycles: u64,
        total_time_cycles: u64,
    ) {
        let count_percent: f64 =
            if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

        let time_percent: f64 = if total_time_cycles == 0 {
            0.0
        } else {
            time_cycles as f64 / total_time_cycles as f64
        } * 100.0;

        let time_s = cycles_as_secs(time_cycles);
        let avg_time_ns =
            if count == 0 { 0.0 } else { convert_cycles_to_ns_f64(time_cycles) / count as f64 };

        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            name, count, count_percent,time_s, time_percent,avg_time_ns
        );
    }

    pub(crate) fn print_hashbuilder(&self) {
        println!("");
        println!("");
        println!("====================================== metric of HashBuilder ========================================");
        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Count (%)","Time(s)", "Time(%)","Avg time(ns)"
        );

        let state_calculate = self.record.state_calculate_record();
        let storage_calculate = self.record.storage_calculate_record();

        let add_branch_count = state_calculate
            .add_branch_count
            .checked_add(storage_calculate.add_branch_count)
            .expect("overflow");
        let add_leaf_count = state_calculate
            .add_leaf_count
            .checked_add(storage_calculate.add_leaf_count)
            .expect("overflow");
        let root_count = state_calculate
            .add_root_count
            .checked_add(storage_calculate.add_root_count)
            .expect("overflow");
        let total_count = add_branch_count
            .checked_add(add_leaf_count)
            .and_then(|x| x.checked_add(root_count))
            .expect("overflow");

        let add_branch_time =
            state_calculate.add_branch.checked_add(storage_calculate.add_branch).expect("overflow");
        let add_leaf_time =
            state_calculate.add_leaf.checked_add(storage_calculate.add_leaf).expect("overflow");
        let root_time =
            state_calculate.add_root.checked_add(storage_calculate.add_root).expect("overflow");
        let total_time = add_branch_time
            .checked_add(add_leaf_time)
            .and_then(|x| x.checked_add(root_time))
            .expect("overflow");

        self.print_hashbuilder_line("total", total_count, total_count, total_time, total_time);
        self.print_hashbuilder_line(
            "HashBuilder.add_branch",
            add_branch_count,
            total_count,
            add_branch_time,
            total_time,
        );
        self.print_hashbuilder_line(
            "    AccountTrie",
            state_calculate.add_branch_count,
            total_count,
            state_calculate.add_branch,
            total_time,
        );
        self.print_hashbuilder_line(
            "    StorageTrie",
            storage_calculate.add_branch_count,
            total_count,
            storage_calculate.add_branch,
            total_time,
        );

        self.print_hashbuilder_line(
            "HashBuilder.add_leaf",
            add_leaf_count,
            total_count,
            add_leaf_time,
            total_time,
        );
        self.print_hashbuilder_line(
            "    AccountTrie",
            state_calculate.add_leaf_count,
            total_count,
            state_calculate.add_leaf,
            total_time,
        );
        self.print_hashbuilder_line(
            "    StorageTrie",
            storage_calculate.add_leaf_count,
            total_count,
            storage_calculate.add_leaf,
            total_time,
        );

        self.print_hashbuilder_line(
            "HashBuilder.root",
            root_count,
            total_count,
            root_time,
            total_time,
        );
        self.print_hashbuilder_line(
            "    AccountTrie",
            state_calculate.add_root_count,
            total_count,
            state_calculate.add_root,
            total_time,
        );
        self.print_hashbuilder_line(
            "    StorageTrie",
            storage_calculate.add_root_count,
            total_count,
            storage_calculate.add_root,
            total_time,
        );

        println!("");
        println!("");
    }

    pub(crate) fn print_metric_keccak256(&self) {
        println!("");
        println!("");
        println!("==========Metric of keccak256==========");
        self.print_line_u64("execute count           :", self.record.keccak256_execution_count());
        self.print_line_f64(
            "execute time(s)         :",
            cycles_as_secs(self.record.keccak256_execution_time()),
        );
        self.print_line_f64(
            "average execute time(ns):",
            convert_cycles_to_ns_f64(self.record.keccak256_avg_execution_time()),
        );
        println!("");
        println!("");
    }

    pub(crate) fn print_update_keys_line(&self, name: &str, count: u64, total_txs: u64) {
        let avg_count = if total_txs == 0 { 0.0 } else { count as f64 / total_txs as f64 };
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_LITTLE_BIG$.3}",
            name, count, avg_count
        );
    }

    pub(crate) fn print_update_keys(&self) {
        println!("");
        println!("");
        println!("===================update keys ======================");
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_LITTLE_BIG$}",
            "Category", "Count", "Keys per transaction"
        );
        let total_txs = self.record.total_txs();
        let total_update_keys = self
            .record
            .account_changes()
            .checked_add(self.record.storage_changes())
            .expect("overflow");

        self.print_update_keys_line("AccountTrie", self.record.account_changes(), total_txs);
        self.print_update_keys_line("StorageTrie", self.record.storage_changes(), total_txs);
        self.print_update_keys_line("StateTrie", total_update_keys, total_txs);
        println!("");
        println!("");
    }

    pub(crate) fn print_metric_trie_line(&self, name: &str, number: u64, depth: u64) {
        let avg_depth = if number == 0 { 0.0 } else { depth as f64 / number as f64 };
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$.3}",
            name, number, depth, avg_depth
        );
    }

    pub(crate) fn print_metric_trie(&self) {
        println!("");
        println!("");
        println!("====================== metric of trie ========================");

        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Trie Type", "Leaf number", "Leaf depth", "Avg.leaf depth"
        );

        self.record.account_mpt_leaf_number();
        self.record.account_mpt_leaf_depth();
        self.print_metric_trie_line(
            "AccountTrie",
            self.record.account_mpt_leaf_number(),
            self.record.account_mpt_leaf_depth(),
        );
        self.print_metric_trie_line(
            "StorageTrie",
            self.record.storage_mpt_leaf_number(),
            self.record.storage_mpt_leaf_depth(),
        );
        self.print_metric_trie_line(
            "StateTrie",
            self.record.mpt_leaf_number(),
            self.record.mpt_leaf_depth(),
        );

        println!("");
        println!("");
    }

    pub(crate) fn print_metric_db_cat_line(
        &self,
        name: &str,
        count: u64,
        total_count: u64,
        time_cycles: u64,
        total_time_cycles: u64,
    ) {
        let count_percent: f64 =
            if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

        let time_percent: f64 = if total_time_cycles == 0 {
            0.0
        } else {
            time_cycles as f64 / total_time_cycles as f64
        } * 100.0;

        let time_s = cycles_as_secs(time_cycles);
        let avg_time_ns =
            if count == 0 { 0.0 } else { convert_cycles_to_ns_f64(time_cycles) / count as f64 };

        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            name, count, count_percent,time_s, time_percent,avg_time_ns
        );
    }

    pub(crate) fn print_metric_db_cat(&self) {
        println!("");
        println!("");
        println!("======================================== metric of read db ==========================================");
        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Count (%)","Time(s)", "Time(%)","Avg time(ns)"
        );

        let db_read = self.record.db_read();

        let total_count = db_read.total_count();
        let total_time_cycles = db_read.total_time();

        self.print_metric_db_cat_line(
            "total",
            total_count,
            total_count,
            total_time_cycles,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "leaf_table",
            db_read.leaf_table_count(),
            total_count,
            db_read.leaf_table_time(),
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "    HA",
            db_read.hash_account_table_count(),
            total_count,
            db_read.hash_account_table_time(),
            total_time_cycles,
        );

        self.print_metric_db_cat_line(
            "        current",
            db_read.current_count,
            total_count,
            db_read.current,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek",
            db_read.seek_count,
            total_count,
            db_read.seek,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        next",
            db_read.next_count,
            total_count,
            db_read.next,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "    HS",
            db_read.hash_storage_table_count(),
            total_count,
            db_read.hash_storage_table_time(),
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek_exact",
            db_read.seek_exact_count,
            total_count,
            db_read.seek_exact,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek_by_sub_key",
            db_read.seek_by_sub_key_count,
            total_count,
            db_read.seek_by_sub_key,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        next_dup_val",
            db_read.next_dup_val_count,
            total_count,
            db_read.next_dup_val,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "branch_table",
            db_read.branch_table_count(),
            total_count,
            db_read.branch_table_time(),
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "    AT",
            db_read.account_trie_table_count(),
            total_count,
            db_read.account_trie_table_time(),
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        current",
            db_read.account_trie_current_count,
            total_count,
            db_read.account_trie_current,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek",
            db_read.account_trie_seek_count,
            total_count,
            db_read.account_trie_seek,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek_exact",
            db_read.account_trie_seek_exact_count,
            total_count,
            db_read.account_trie_seek_exact,
            total_time_cycles,
        );

        self.print_metric_db_cat_line(
            "    ST",
            db_read.storage_trie_table_count(),
            total_count,
            db_read.storage_trie_table_time(),
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        seek_by_subkey",
            db_read.storage_trie_seek_by_subkey_count,
            total_count,
            db_read.storage_trie_seek_by_subkey,
            total_time_cycles,
        );
        self.print_metric_db_cat_line(
            "        current",
            db_read.storage_trie_current_count,
            total_count,
            db_read.storage_trie_current,
            total_time_cycles,
        );

        println!("");
        println!("");
    }

    pub(crate) fn print_db_count_and_time(&self) {
        println!("");
        println!("");
        println!(
            "=================================================================================="
        );

        let db_read = self.record.db_read();
        let total_update_keys = self
            .record
            .account_changes()
            .checked_add(self.record.storage_changes())
            .expect("overflow");

        let avg_update_count = if total_update_keys == 0 {
            0.0
        } else {
            db_read.total_count() as f64 / total_update_keys as f64
        };

        self.print_line_u64("read db count               :", db_read.total_count());
        self.print_line_f64("read db count / update keys :", avg_update_count);

        println!("\n");

        let total_count = db_read.total_count();
        let avg_time_cycles = if total_count == 0 {
            0.0
        } else {
            convert_cycles_to_ns_f64(db_read.total_time()) / total_count as f64
        };

        self.print_line_f64("read db time(s)         :", cycles_as_secs(db_read.total_time()));
        self.print_line_f64("average read db time(ns):", avg_time_cycles);

        println!("");
        println!("");
    }

    pub(crate) fn print(&mut self) {
        if self.last_print_block_number == u64::default() {
            self.last_print_block_number = self.record.block_number();
        }

        let interval =
            self.record.block_number().checked_sub(self.last_print_block_number).expect("overflow");
        if interval < Self::N && 0 != self.record.block_number() % Self::N {
            return
        }
        self.last_print_block_number = self.record.block_number();

        println!();
        self.print_trie_db_stat(self.record.state_trie_db_read());
        self.print_breakdown_funtion();
        self.print_try_next();
        self.print_try_next_read_node();
        self.print_hashbuilder();
        self.print_metric_keccak256();
        self.print_update_keys();
        self.print_metric_trie();
        self.print_metric_db_cat();

        println!(
            "\nat-seek-exact:{:?},{:?}",
            self.record.db_read().account_trie_seek_exact,
            cycles_as_secs(self.record.db_read().account_trie_seek_exact)
        );
        println!(
            "\nmac-at-seek-exact:{:?},{:?}, {:?}\n\n",
            self.recorder.db_read.at_seek_exact_count,
            self.recorder.db_read.at_seek_exact_time,
            cycles_as_secs(self.recorder.db_read.at_seek_exact_time)
        );

        println!("\nmac-at-seek-exact:{:?}\n\n", self.recorder.db_read);

        self.print_db_count_and_time();

        println!();
        println!("===============================Metric of state root update ====================================================");
        println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}", "Key", "Value",);

        self.print_line_u64("block_number", self.record.block_number());
        self.print_line_f64("hash_state_slow(s)", cycles_as_secs(self.record.hash_state_slow()));

        println!("===============================Metric of state caculate ====================================================");
        self.print_caculate_stat(self.record.state_calculate_record());
        println!("===============================Metric of storage caculate ====================================================");
        self.print_caculate_stat(self.record.storage_calculate_record());
        println!("===============================Metric of state-trie caculate ====================================================");
        let add_branch_count = self.record.state_calculate_record().add_branch_count +
            self.record.storage_calculate_record().add_branch_count;
        let add_branch_time = self.record.state_calculate_record().add_branch +
            self.record.storage_calculate_record().add_branch;
        let add_leaf_count = self.record.state_calculate_record().add_leaf_count +
            self.record.storage_calculate_record().add_leaf_count;
        let add_leaf_time = self.record.state_calculate_record().add_leaf +
            self.record.storage_calculate_record().add_leaf;
        let add_root_count = self.record.state_calculate_record().add_root_count +
            self.record.storage_calculate_record().add_root_count;
        let add_root_time = self.record.state_calculate_record().add_root +
            self.record.storage_calculate_record().add_root;
        let add_total_count = add_branch_count + add_leaf_count + add_root_count;
        let add_total_time = add_branch_time + add_leaf_time + add_root_time;
        self.print_line_f64(
            "state_trie_avg_add_brach(ns)",
            convert_cycles_to_ns_f64(add_branch_time) / add_branch_count as f64,
        );
        self.print_line_f64(
            "state_trie_avg_add_leaf(ns)",
            convert_cycles_to_ns_f64(add_leaf_time) / add_leaf_count as f64,
        );
        self.print_line_f64(
            "state_trie_avg_add_root(ns)",
            convert_cycles_to_ns_f64(add_root_time) / add_root_count as f64,
        );
        self.print_line_f64(
            "state_trie_avg_add_node(ns)",
            convert_cycles_to_ns_f64(add_total_time) / add_total_count as f64,
        );

        println!("=============================== Metric ====================================================");
        self.print_line_u64("state_calculate(cycles)", self.record.state_calculate());
        self.print_line_f64("state_calculate(s)", cycles_as_secs(self.record.state_calculate()));
        self.print_line_u64("state_try_next(cycles)", self.record.state_try_next());
        self.print_line_f64("state_try_next(s)", cycles_as_secs(self.record.state_try_next()));
        self.print_line_u64("storage_calculate(cycles)", self.record.storage_calculate());
        self.print_line_f64(
            "storage_calculate(s)",
            cycles_as_secs(self.record.storage_calculate()),
        );
        self.print_line_u64("storage_try_next(cycles)", self.record.storage_try_next());
        self.print_line_f64("storage_try_next(s)", cycles_as_secs(self.record.storage_try_next()));
        self.print_line_f64("hashed_state(s)", cycles_as_secs(self.record.hashed_state_write()));
        self.print_line_f64(
            "state_root_calculator(s)",
            cycles_as_secs(self.record.construct_prefix_sets()),
        );
        self.print_line_f64("flush(s)", cycles_as_secs(self.record.flush()));
        self.print_line_f64("keccak256(s)", cycles_as_secs(self.record.keccak256()));
        self.print_line_u64("keccak256_execution_count", self.record.keccak256_execution_count());
        self.print_line_f64(
            "keccak256_execution_time(s)",
            cycles_as_secs(self.record.keccak256_execution_time()),
        );
        self.print_line_f64(
            "keccak256_avg_execution_time(ns)",
            convert_cycles_to_ns_f64(self.record.keccak256_avg_execution_time()),
        );
        self.print_line_f64("tx_avg_update_keys", self.record.tx_avg_update_keys());
        self.print_line_f64("keys_read_db_avg_count", self.record.keys_read_db_avg_count());
        self.print_line_f64(
            "key_read_db_avg_time(ns)",
            convert_cycles_to_ns_f64(self.record.keys_read_db_avg_time()),
        );

        self.print_line_f64("keys_update_leaf_avg_depth", self.record.keys_update_leaf_avg_depth());
        self.print_line_f64("keys_update_mpt_avg_dept", self.record.keys_update_mpt_avg_dept());

        self.print_db_read(self.record.db_read());
        self.print_line_f64(
            "total_read_db_time(s)",
            cycles_as_secs(self.record.db_read().total_time()),
        );
        self.print_line_u64("total_read_db_time_cycles1", self.record.db_read().total_time());

        self.print_line_u64("total_txs", self.record.total_txs());
        self.print_line_u64("mpt_updates_to_db", self.record.mpt_updates_to_db());
        self.print_line_u64("total_update_keys", self.record.total_update_keys());
        self.print_line_u64("account_changes", self.record.account_changes());
        self.print_line_u64("contract_account_changes", self.record.contract_account_changes());
        self.print_line_u64("storage_changes", self.record.storage_changes());
        self.print_line_u64("mpt_updated_branch_nodes", self.record.mpt_updated_branch_nodes());
        self.print_line_u64("mpt_delete_branch_nodes", self.record.mpt_delete_branch_nodes());
        self.print_line_u64(
            "account_mpt_updated_branch_nodes",
            self.record.account_mpt_updated_branch_nodes(),
        );
        self.print_line_u64(
            "account_mpt_delete_branch_nodes",
            self.record.account_mpt_delete_branch_nodes(),
        );
        self.print_line_u64(
            "storage_mpt_updated_branch_nodes",
            self.record.storage_mpt_updated_branch_nodes(),
        );
        self.print_line_u64(
            "storage_mpt_delete_branch_nodes",
            self.record.storage_mpt_delete_branch_nodes(),
        );
        println!("");
        self.print_line_u64("mpt_leaf_number", self.record.mpt_leaf_number());
        self.print_line_u64("mpt_leaf_depth", self.record.mpt_leaf_depth());
        self.print_line_f64("mpt_avg_leaf_depth", self.record.mpt_avg_leaf_depth());
        self.print_line_u64("mpt_nodes_number", self.record.mpt_nodes_number());
        self.print_line_u64("mpt_nodes_depth", self.record.mpt_nodes_depth());
        self.print_line_f64("mpt_nodes_avg_depth", self.record.mpt_nodes_avg_depth());
        println!("");
        self.print_line_u64("account_mpt_leaf_number", self.record.account_mpt_leaf_number());
        self.print_line_u64("account_mpt_leaf_depth", self.record.account_mpt_leaf_depth());
        self.print_line_f64("account_mpt_avg_leaf_depth", self.record.account_mpt_avg_leaf_depth());
        self.print_line_u64("account_mpt_node_number", self.record.account_mpt_node_number());
        self.print_line_u64("account_mpt_nodes_depth", self.record.account_mpt_nodes_depth());
        self.print_line_f64(
            "account_mpt_avg_nodes_depth",
            self.record.account_mpt_avg_nodes_depth(),
        );
        println!("");
        self.print_line_u64("storage_mpt_leaf_number", self.record.storage_mpt_leaf_number());
        self.print_line_u64("storage_mpt_leaf_depth", self.record.storage_mpt_leaf_depth());
        self.print_line_f64("storage_mpt_avg_leaf_depth", self.record.storage_mpt_avg_leaf_depth());
        self.print_line_u64("storage_mpt_nodes_number", self.record.storage_mpt_nodes_number());
        self.print_line_u64("storage_mpt_nodes_depth", self.record.storage_mpt_nodes_depth());
        self.print_line_f64(
            "storage_mpt_avg_nodes_depth",
            self.record.storage_mpt_avg_nodes_depth(),
        );

        self.print_line_u64("mpt_delete_branch_number", self.record.mpt_delete_branch_number());

        self.print_line_u64(
            "--for validate  mpt_add_node_number--",
            self.record.mpt_add_node_number(),
        );
        self.print_line_u64(
            "--for validate  storage_mpt_empty_hash_number--",
            self.record.storage_mpt_empty_hash_number(),
        );
        self.print_line_u64(
            "--for validate  storage_mpt_not_empty_hash_number--",
            self.record.storage_mpt_not_empty_hash_number(),
        );

        self.print_line_u64(
            "--for validate  mpt_node_number--",
            self.record.account_mpt_node_number() + self.record.storage_mpt_nodes_number() -
                self.record.storage_mpt_not_empty_hash_number(),
        );

        println!("\n\nself.recorder: {:?}\n", self.recorder);
    }
}