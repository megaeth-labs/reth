use super::commons::*;
use crate::metrics::StateRootRecord;
use revm_utils::time_utils::convert_cycles_to_ns_f64;

const COL_WIDTH_MIDDLE: usize = 14;
const COL_WIDTH_BIG: usize = 20;
const COL_WIDTH_LITTLE_BIG: usize = 25;
const COL_WIDTH_LARGE: usize = 48;

fn print_line_u64(name: &str, value: u64) {
    println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}", name, value);
}

fn print_line_f64(name: &str, value: f64) {
    println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$.3}", name, value);
}

fn print_time_line(name: &str, time_cycles: u64, total_time_cycles: u64) {
    let time_s = cycles_as_secs(time_cycles);
    let ratio: f64 =
        if total_time_cycles == 0 { 0.0 } else { time_cycles as f64 / total_time_cycles as f64 } *
            100.0;

    println!(
        "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}",
        name, time_s, ratio
    );
}

fn print_count_line(name: &str, count: u64, total_count: u64) {
    let ratio: f64 = if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

    println!(
        "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}",
        name, count, ratio
    );
}

fn print_hashbuilder_line(
    name: &str,
    count: u64,
    total_count: u64,
    time_cycles: u64,
    total_time_cycles: u64,
) {
    let count_percent: f64 =
        if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

    let time_percent: f64 =
        if total_time_cycles == 0 { 0.0 } else { time_cycles as f64 / total_time_cycles as f64 } *
            100.0;

    let time_s = cycles_as_secs(time_cycles);
    let avg_time_ns =
        if count == 0 { 0.0 } else { convert_cycles_to_ns_f64(time_cycles) / count as f64 };

    println!(
        "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
        name, count, count_percent, time_s, time_percent, avg_time_ns
    );
}

fn print_update_keys_line(name: &str, count: u64, total_txs_count: u64) {
    let avg_count = if total_txs_count == 0 { 0.0 } else { count as f64 / total_txs_count as f64 };
    println!(
        "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_LITTLE_BIG$.3}",
        name, count, avg_count
    );
}

fn print_trie_metric_line(name: &str, number: u64, depth: u64) {
    let avg_depth = if number == 0 { 0.0 } else { depth as f64 / number as f64 };
    println!(
        "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$.3}",
        name, number, depth, avg_depth
    );
}

fn print_read_db_metric_line(
    name: &str,
    count: u64,
    total_count: u64,
    time_cycles: u64,
    total_time_cycles: u64,
) {
    let count_percent: f64 =
        if total_count == 0 { 0.0 } else { count as f64 / total_count as f64 } * 100.0;

    let time_percent: f64 =
        if total_time_cycles == 0 { 0.0 } else { time_cycles as f64 / total_time_cycles as f64 } *
            100.0;

    let time_s = cycles_as_secs(time_cycles);
    let avg_time_ns =
        if count == 0 { 0.0 } else { convert_cycles_to_ns_f64(time_cycles) / count as f64 };

    println!(
        "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
        name, count, count_percent,time_s, time_percent,avg_time_ns
    );
}

#[derive(Default, Debug)]
struct ChainDataStat {
    total_block_count: u64,
    total_txs_count: u64,
}

impl From<&StateRootRecord> for ChainDataStat {
    fn from(record: &StateRootRecord) -> Self {
        let total_block_count = record
            .end_block_number
            .checked_sub(record.start_block_number)
            .and_then(|x| x.checked_add(1))
            .expect("overflow");

        Self { total_block_count, total_txs_count: record.total_txs_count }
    }
}

impl Print for ChainDataStat {
    fn print_title(&self) {
        println!("=======Metric of chain data=======");
    }

    fn print_content(&self) {
        print_line_u64("total block      :", self.total_block_count);
        print_line_u64("total transaction:", self.total_txs_count);
    }
}

#[derive(Default, Debug)]
struct StorageCalculateStat {
    total_time: u64,
    misc_time: u64,
    before_loop_time: u64,
    try_next_time: u64,
    add_branch_time: u64,
    add_leaf_time: u64,
    root_time: u64,
    after_root_time: u64,
}

impl From<&StateRootRecord> for StorageCalculateStat {
    fn from(record: &StateRootRecord) -> Self {
        let rc = &record.storage_calculate_record;

        let misc_time = rc
            .total_time
            .checked_sub(rc.before_loop_time)
            .and_then(|x| x.checked_sub(rc.try_next_stat.total_time))
            .and_then(|x| x.checked_sub(rc.add_branch_time))
            .and_then(|x| x.checked_sub(rc.add_leaf_time))
            .and_then(|x| x.checked_sub(rc.add_root_time))
            .and_then(|x| x.checked_sub(rc.after_add_root_time))
            .expect("overflow");
        Self {
            total_time: rc.total_time,
            misc_time,
            before_loop_time: rc.before_loop_time,
            try_next_time: rc.try_next_stat.total_time,
            add_branch_time: rc.add_branch_time,
            add_leaf_time: rc.add_leaf_time,
            root_time: rc.add_root_time,
            after_root_time: rc.after_add_root_time,
        }
    }
}

impl Print for StorageCalculateStat {
    fn print_title(&self) {
        println!("\n=====================Breakdown of StorageRoot.calculate=====================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );
    }

    fn print_content(&self) {
        let total_time = self.total_time;

        print_time_line("total", self.total_time, total_time);
        print_time_line("misc", self.misc_time, total_time);
        print_time_line("before loop", self.before_loop_time, total_time);
        print_time_line("StorageNodeIter.try_next", self.try_next_time, total_time);
        print_time_line("HashBuilder.add_branch", self.add_branch_time, total_time);
        print_time_line("HashBuilder.add_leaf", self.add_leaf_time, total_time);
        print_time_line("HashBuilder.root", self.root_time, total_time);
        print_time_line("after HashBuilder.root", self.after_root_time, total_time);

        println!("");
    }
}

#[derive(Default, Debug)]
struct StateCalculateStat {
    total_time: u64,
    misc_time: u64,
    before_loop_time: u64,
    try_next_time: u64,
    add_branch_time: u64,
    cal_storage_root_and_add_leaf_time: u64,
    add_leaf_misc_time: u64,
    storage_cal_stat: StorageCalculateStat,
    add_leaf_time: u64,
    root_time: u64,
    after_root_time: u64,
}

impl From<&StateRootRecord> for StateCalculateStat {
    fn from(record: &StateRootRecord) -> Self {
        let storage_cal_stat: StorageCalculateStat = record.into();

        let sc = &record.state_calculate_record;
        let add_leaf_misc_time = sc
            .cal_storage_root_and_add_leaf_time
            .checked_sub(storage_cal_stat.total_time)
            .and_then(|x| x.checked_sub(sc.add_leaf_time))
            .expect("overflow");

        let misc_time = sc
            .total_time
            .checked_sub(sc.before_loop_time)
            .and_then(|x| x.checked_sub(sc.try_next_stat.total_time))
            .and_then(|x| x.checked_sub(sc.add_branch_time))
            .and_then(|x| x.checked_sub(sc.cal_storage_root_and_add_leaf_time))
            .and_then(|x| x.checked_sub(sc.add_root_time))
            .and_then(|x| x.checked_sub(sc.after_add_root_time))
            .expect("overflow");

        Self {
            total_time: sc.total_time,
            misc_time,
            before_loop_time: sc.before_loop_time,
            try_next_time: sc.try_next_stat.total_time,
            add_branch_time: sc.add_branch_time,
            cal_storage_root_and_add_leaf_time: sc.cal_storage_root_and_add_leaf_time,
            add_leaf_misc_time,
            storage_cal_stat,
            add_leaf_time: sc.add_leaf_time,
            root_time: sc.add_root_time,
            after_root_time: sc.after_add_root_time,
        }
    }
}

impl Print for StateCalculateStat {
    fn print_title(&self) {
        println!("\n======================Breakdown of StateRoot.calculate======================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );
    }

    fn print_content(&self) {
        let total_time = self.total_time;

        print_time_line("total", self.total_time, total_time);
        print_time_line("misc", self.misc_time, total_time);
        print_time_line("before loop", self.before_loop_time, total_time);
        print_time_line("AccountNodeIter.try_next", self.try_next_time, total_time);
        print_time_line("HashBuilder.add_branch", self.add_branch_time, total_time);
        print_time_line("add_leaf", self.cal_storage_root_and_add_leaf_time, total_time);
        print_time_line("    misc", self.add_leaf_misc_time, total_time);
        print_time_line("    StorageRoot.calculate", self.storage_cal_stat.total_time, total_time);
        print_time_line("        misc", self.storage_cal_stat.misc_time, total_time);
        print_time_line("        before loop", self.storage_cal_stat.before_loop_time, total_time);
        print_time_line(
            "        StorageNodeIter.try_next",
            self.storage_cal_stat.try_next_time,
            total_time,
        );
        print_time_line(
            "        HashBuilder.add_branch",
            self.storage_cal_stat.add_branch_time,
            total_time,
        );
        print_time_line(
            "        HashBuilder.add_leaf",
            self.storage_cal_stat.add_leaf_time,
            total_time,
        );
        print_time_line("        HashBuilder.root", self.storage_cal_stat.root_time, total_time);
        print_time_line(
            "        after HashBuilder.root",
            self.storage_cal_stat.after_root_time,
            total_time,
        );

        print_time_line("    HashBuilder.add_leaf", self.add_leaf_time, total_time);
        print_time_line("HashBuilder.root", self.root_time, total_time);
        print_time_line("after HashBuilder.root", self.after_root_time, total_time);

        println!("");
    }
}

#[derive(Default, Debug)]
struct BreakdownStat {
    total_time: u64,
    misc_time: u64,
    block_td_time: u64,
    block_with_senders_time: u64,
    execute_and_verify_receipt_time: u64,
    hash_state_slow_time: u64,
    construct_prefix_sets_time: u64,
    state_trie_cal_stats: StateCalculateStat,
    state_write_to_db_time: u64,
    hashed_state_write_time: u64,
    flush_time: u64,
}

impl BreakdownStat {
    pub fn total_time(record: &StateRootRecord) -> u64 {
        if record.total_time != 0 {
            record.total_time
        } else {
            record
                .hash_state_slow_time
                .checked_add(record.construct_prefix_sets_time)
                .and_then(|x| x.checked_add(record.state_calculate_record.total_time))
                .and_then(|x| x.checked_add(record.state_write_to_db_time))
                .and_then(|x| x.checked_add(record.hashed_state_write_time))
                .and_then(|x| x.checked_add(record.flush_time))
                .expect("overflow")
        }
    }
}

impl From<&StateRootRecord> for BreakdownStat {
    fn from(record: &StateRootRecord) -> Self {
        let state_trie_cal_stats: StateCalculateStat = record.into();

        let total_time = Self::total_time(record);

        let misc_time = total_time
            .checked_sub(record.block_td_time)
            .and_then(|x| x.checked_sub(record.block_with_senders_time))
            .and_then(|x| x.checked_sub(record.execute_and_verify_receipt_time))
            .and_then(|x| x.checked_sub(record.hash_state_slow_time))
            .and_then(|x| x.checked_sub(record.construct_prefix_sets_time))
            .and_then(|x| x.checked_sub(state_trie_cal_stats.total_time))
            .and_then(|x| x.checked_sub(record.state_write_to_db_time))
            .and_then(|x| x.checked_sub(record.hashed_state_write_time))
            .and_then(|x| x.checked_sub(record.flush_time))
            .expect("overflow");

        Self {
            total_time,
            misc_time,
            block_td_time: record.block_td_time,
            block_with_senders_time: record.block_with_senders_time,
            execute_and_verify_receipt_time: record.execute_and_verify_receipt_time,
            hash_state_slow_time: record.hash_state_slow_time,
            construct_prefix_sets_time: record.construct_prefix_sets_time,
            state_trie_cal_stats,
            state_write_to_db_time: record.state_write_to_db_time,
            hashed_state_write_time: record.hashed_state_write_time,
            flush_time: record.flush_time,
        }
    }
}

impl Print for BreakdownStat {
    fn print_title(&self) {
        println!("\n============================Breakdown of Function===========================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );
    }

    fn print_content(&self) {
        let total_time = self.total_time;

        print_time_line("total", total_time, total_time);
        print_time_line("misc", self.misc_time, total_time);
        print_time_line("block_td", self.block_td_time, total_time);
        print_time_line("block_with_senders", self.block_with_senders_time, total_time);
        print_time_line(
            "execute_and_verify_receipt",
            self.execute_and_verify_receipt_time,
            total_time,
        );
        print_time_line("hash_state_slow", self.hash_state_slow_time, total_time);
        print_time_line("construct_prefix_sets", self.construct_prefix_sets_time, total_time);
        print_time_line("StateRoot.calculate", self.state_trie_cal_stats.total_time, total_time);
        print_time_line("    misc", self.state_trie_cal_stats.misc_time, total_time);
        print_time_line("    before loop", self.state_trie_cal_stats.before_loop_time, total_time);
        print_time_line(
            "    AccountNodeIter.try_next",
            self.state_trie_cal_stats.try_next_time,
            total_time,
        );
        print_time_line(
            "    HashBuilder.add_branch",
            self.state_trie_cal_stats.add_branch_time,
            total_time,
        );
        print_time_line(
            "    add_leaf",
            self.state_trie_cal_stats.cal_storage_root_and_add_leaf_time,
            total_time,
        );
        print_time_line("        misc", self.state_trie_cal_stats.misc_time, total_time);
        print_time_line(
            "        StorageRoot.calculate",
            self.state_trie_cal_stats.storage_cal_stat.total_time,
            total_time,
        );
        print_time_line(
            "            misc",
            self.state_trie_cal_stats.storage_cal_stat.misc_time,
            total_time,
        );
        print_time_line(
            "            before loop",
            self.state_trie_cal_stats.storage_cal_stat.before_loop_time,
            total_time,
        );
        print_time_line(
            "            StorageNodeIter.try_next",
            self.state_trie_cal_stats.storage_cal_stat.try_next_time,
            total_time,
        );
        print_time_line(
            "            HashBuilder.add_branch",
            self.state_trie_cal_stats.storage_cal_stat.add_branch_time,
            total_time,
        );
        print_time_line(
            "            HashBuilder.add_leaf",
            self.state_trie_cal_stats.storage_cal_stat.add_leaf_time,
            total_time,
        );
        print_time_line(
            "            HashBuilder.root",
            self.state_trie_cal_stats.storage_cal_stat.root_time,
            total_time,
        );
        print_time_line(
            "            after HashBuilder.root",
            self.state_trie_cal_stats.storage_cal_stat.after_root_time,
            total_time,
        );

        print_time_line(
            "        HashBuilder.add_leaf",
            self.state_trie_cal_stats.add_leaf_time,
            total_time,
        );
        print_time_line("    HashBuilder.root", self.state_trie_cal_stats.root_time, total_time);
        print_time_line(
            "    after HashBuilder.root",
            self.state_trie_cal_stats.after_root_time,
            total_time,
        );
        print_time_line("state.write_to_db", self.state_write_to_db_time, total_time);
        print_time_line("hashed_state_write", self.hashed_state_write_time, total_time);
        print_time_line("flush", self.flush_time, total_time);

        println!("");
    }
}

#[derive(Default, Debug)]
struct OneDimensionalData {
    total: u64,
    account_trie: u64,
    storage_trie: u64,
}

#[derive(Default, Debug)]
struct TwoDimensionalData {
    total_count: u64,
    account_trie_count: u64,
    storage_trie_count: u64,

    total_time: u64,
    account_trie_time: u64,
    storage_trie_time: u64,
}

#[derive(Default, Debug)]
struct TryNextStat {
    data: TwoDimensionalData,
}
impl From<&StateRootRecord> for TryNextStat {
    fn from(record: &StateRootRecord) -> Self {
        let state_try_next = &record.state_calculate_record.try_next_stat;
        let storage_try_next = &record.storage_calculate_record.try_next_stat;

        let total_count =
            state_try_next.total_count.checked_add(storage_try_next.total_count).expect("overflow");
        let total_time =
            state_try_next.total_time.checked_add(storage_try_next.total_time).expect("overflow");

        let data = TwoDimensionalData {
            total_count,
            account_trie_count: state_try_next.total_count,
            storage_trie_count: storage_try_next.total_count,
            total_time,
            account_trie_time: state_try_next.total_time,
            storage_trie_time: storage_try_next.total_time,
        };

        Self { data }
    }
}

impl Print for TryNextStat {
    fn print_title(&self) {
        println!("\n====================== metric of try_next ====================");
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Time (s)", "Avg time (ns)"
        );
    }

    fn print_content(&self) {
        let avg_time = convert_cycles_to_ns_f64(self.data.total_time / self.data.total_count);

        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            "total", self.data.total_count, cycles_as_secs(self.data.total_time), avg_time
        );

        let state_avg_time =
            convert_cycles_to_ns_f64(self.data.account_trie_time / self.data.account_trie_count);
        println!(
                "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
                "AccountTrie", self.data.account_trie_count, cycles_as_secs(self.data.account_trie_time), state_avg_time
            );

        let storage_avg_time =
            convert_cycles_to_ns_f64(self.data.storage_trie_time / self.data.storage_trie_count);

        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_BIG$.3}",
            "StorageTrie", self.data.storage_trie_count, cycles_as_secs(self.data.storage_trie_time), storage_avg_time
        );

        println!("");
    }
}

#[derive(Default, Debug)]
struct BreakdownCategoryStat {
    total_time: u64,
    other_time: u64,
    try_next_time: OneDimensionalData,
    add_branch_time: OneDimensionalData,
    add_leaf_time: OneDimensionalData,
    root_time: OneDimensionalData,
}

impl From<&StateRootRecord> for BreakdownCategoryStat {
    fn from(record: &StateRootRecord) -> Self {
        let try_next_stat: TryNextStat = record.into();

        let state_record = &record.state_calculate_record;
        let storage_record = &record.storage_calculate_record;

        let total_add_branch_time = state_record
            .add_branch_time
            .checked_add(storage_record.add_branch_time)
            .expect("overflow");
        let total_add_leaf_time =
            state_record.add_leaf_time.checked_add(storage_record.add_leaf_time).expect("overflow");
        let total_root_time =
            state_record.add_root_time.checked_add(storage_record.add_root_time).expect("overflow");
        let other_time = state_record
            .total_time
            .checked_sub(try_next_stat.data.total_time)
            .and_then(|x| x.checked_sub(total_add_branch_time))
            .and_then(|x| x.checked_sub(total_add_leaf_time))
            .and_then(|x| x.checked_sub(total_root_time))
            .expect("overflow");

        let try_next_time = OneDimensionalData {
            total: try_next_stat.data.total_time,
            account_trie: try_next_stat.data.account_trie_time,
            storage_trie: try_next_stat.data.storage_trie_time,
        };

        let add_branch_time = OneDimensionalData {
            total: total_add_branch_time,
            account_trie: state_record.add_branch_time,
            storage_trie: storage_record.add_branch_time,
        };

        let add_leaf_time = OneDimensionalData {
            total: total_add_leaf_time,
            account_trie: state_record.add_leaf_time,
            storage_trie: storage_record.add_leaf_time,
        };

        let root_time = OneDimensionalData {
            total: total_root_time,
            account_trie: state_record.add_root_time,
            storage_trie: storage_record.add_root_time,
        };

        Self {
            total_time: state_record.total_time,
            other_time,
            try_next_time,
            add_branch_time,
            add_leaf_time,
            root_time,
        }
    }
}

impl Print for BreakdownCategoryStat {
    fn print_title(&self) {
        println!("\n============================================================================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Time (s)", "Time (%)"
        );
    }

    fn print_content(&self) {
        let total_time = self.total_time;

        print_time_line("total", self.total_time, total_time);
        print_time_line("other", self.other_time, total_time);
        print_time_line("try_next", self.try_next_time.total, total_time);
        print_time_line(
            "    AccountNodeIter.try_next(StateRoot)",
            self.try_next_time.account_trie,
            total_time,
        );
        print_time_line(
            "    StorageNodeIter.try_next(StorageRoot)",
            self.try_next_time.storage_trie,
            total_time,
        );

        print_time_line("add_branch", self.add_branch_time.total, total_time);
        print_time_line(
            "    HashBuilder.add_branch(StateRoot)",
            self.add_branch_time.account_trie,
            total_time,
        );
        print_time_line(
            "    HashBuilder.add_branch(StorageRoot)",
            self.add_branch_time.storage_trie,
            total_time,
        );

        print_time_line("add_leaf", self.add_leaf_time.total, total_time);
        print_time_line(
            "    HashBuilder.add_leaf(StateRoot)",
            self.add_leaf_time.account_trie,
            total_time,
        );
        print_time_line(
            "    HashBuilder.add_leaf(StorageRoot)",
            self.add_leaf_time.storage_trie,
            total_time,
        );

        print_time_line("root", self.root_time.total, total_time);
        print_time_line("    HashBuilder.root(StateRoot)", self.root_time.account_trie, total_time);
        print_time_line(
            "    HashBuilder.root(StorageRoot)",
            self.root_time.storage_trie,
            total_time,
        );

        println!("");
    }
}

#[derive(Default, Debug)]
struct ReadNodeStat {
    total_count: u64,
    total_branch_node_count: u64,
    ro_branch_node: OneDimensionalData,
    update_branch_node_count: u64,
    total_leaf_count: u64,
    boundary_reading_leaf: OneDimensionalData,
    total_work_leaf_count: u64,
    ro_leaf: OneDimensionalData,
    updated_leaf: OneDimensionalData,
}

impl From<&StateRootRecord> for ReadNodeStat {
    fn from(record: &StateRootRecord) -> Self {
        let state_try_next = &record.state_calculate_record.try_next_stat;
        let storage_try_next = &record.storage_calculate_record.try_next_stat;

        let ro_branch_node_count = state_try_next
            .skip_branch_node_count
            .checked_add(storage_try_next.skip_branch_node_count)
            .expect("overflow");

        let branch_node_count =
            ro_branch_node_count.checked_add(record.delete_branch_count).expect("overflow");

        let boundary_reading_leaf_count = state_try_next
            .leaf_miss_count
            .checked_add(storage_try_next.leaf_miss_count)
            .expect("overflow");

        let work_leaf_count = state_try_next
            .leaf_hit_count
            .checked_add(storage_try_next.leaf_hit_count)
            .expect("overflow");

        let leaf_node_count =
            boundary_reading_leaf_count.checked_add(work_leaf_count).expect("overflow");
        let total_node_count = branch_node_count.checked_add(leaf_node_count).expect("overflow");

        let ro_branch_node = OneDimensionalData {
            total: ro_branch_node_count,
            account_trie: state_try_next.skip_branch_node_count,
            storage_trie: storage_try_next.skip_branch_node_count,
        };

        let boundary_reading_leaf = OneDimensionalData {
            total: boundary_reading_leaf_count,
            account_trie: state_try_next.leaf_miss_count,
            storage_trie: storage_try_next.leaf_miss_count,
        };

        let account_update_leaf_count = record.update_keys.account_trie_count;
        let storage_update_leaf_count = record.update_keys.storage_trie_count;
        let account_ro_leaf_count =
            state_try_next.leaf_hit_count.checked_sub(account_update_leaf_count).expect("overflow");
        let storage_ro_leaf_count = storage_try_next
            .leaf_hit_count
            .checked_sub(storage_update_leaf_count)
            .expect("overflow");
        let ro_leaf_count =
            account_ro_leaf_count.checked_add(storage_ro_leaf_count).expect("overflow");
        let update_leaf_count =
            account_update_leaf_count.checked_add(storage_update_leaf_count).expect("overflow");

        let ro_leaf = OneDimensionalData {
            total: ro_leaf_count,
            account_trie: account_ro_leaf_count,
            storage_trie: storage_ro_leaf_count,
        };

        let updated_leaf = OneDimensionalData {
            total: update_leaf_count,
            account_trie: account_update_leaf_count,
            storage_trie: storage_update_leaf_count,
        };

        Self {
            total_count: total_node_count,
            total_branch_node_count: branch_node_count,
            ro_branch_node,
            update_branch_node_count: record.delete_branch_count,
            total_leaf_count: leaf_node_count,
            boundary_reading_leaf,
            total_work_leaf_count: work_leaf_count,
            ro_leaf,
            updated_leaf,
        }
    }
}

impl Print for ReadNodeStat {
    fn print_title(&self) {
        println!("\n============================ metric of read node ===========================");
        println!(
            "{:COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Category", "Count", "Count (%)"
        );
    }

    fn print_content(&self) {
        let total_node_count = self.total_count;

        print_count_line("total", total_node_count, total_node_count);
        print_count_line("branch_nodes", self.total_branch_node_count, total_node_count);
        print_count_line("    ro_branch_nodes", self.ro_branch_node.total, total_node_count);
        print_count_line("        accountTrie", self.ro_branch_node.account_trie, total_node_count);

        print_count_line("        storageTrie", self.ro_branch_node.storage_trie, total_node_count);

        print_count_line(
            "    updated_branch_nodes",
            self.update_branch_node_count,
            total_node_count,
        );

        print_count_line("leaf_nodes", self.total_leaf_count, total_node_count);
        print_count_line(
            "    boundary_reading_leaf",
            self.boundary_reading_leaf.total,
            total_node_count,
        );
        print_count_line(
            "        accountTrie",
            self.boundary_reading_leaf.account_trie,
            total_node_count,
        );
        print_count_line(
            "        storageTrie",
            self.boundary_reading_leaf.storage_trie,
            total_node_count,
        );
        print_count_line("    work_leaf", self.total_work_leaf_count, total_node_count);
        print_count_line("        ro_leaves", self.ro_leaf.total, total_node_count);
        print_count_line("            accountTrie", self.ro_leaf.account_trie, total_node_count);
        print_count_line("            storageTrie", self.ro_leaf.storage_trie, total_node_count);
        print_count_line("        updated_leaves", self.updated_leaf.total, total_node_count);
        print_count_line(
            "            accountTrie",
            self.updated_leaf.account_trie,
            total_node_count,
        );
        print_count_line(
            "            storageTrie",
            self.updated_leaf.storage_trie,
            total_node_count,
        );

        println!("");
    }
}

#[derive(Default, Debug)]
struct HashBuilderStat {
    total_count: u64,
    total_time: u64,
    add_branch: TwoDimensionalData,
    add_leaf: TwoDimensionalData,
    root: TwoDimensionalData,
}

impl From<&StateRootRecord> for HashBuilderStat {
    fn from(record: &StateRootRecord) -> Self {
        let state_calculate = &record.state_calculate_record;
        let storage_calculate = &record.storage_calculate_record;

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
        let add_branch_time = state_calculate
            .add_branch_time
            .checked_add(storage_calculate.add_branch_time)
            .expect("overflow");
        let add_leaf_time = state_calculate
            .add_leaf_time
            .checked_add(storage_calculate.add_leaf_time)
            .expect("overflow");
        let root_time = state_calculate
            .add_root_time
            .checked_add(storage_calculate.add_root_time)
            .expect("overflow");
        let total_time = add_branch_time
            .checked_add(add_leaf_time)
            .and_then(|x| x.checked_add(root_time))
            .expect("overflow");

        let add_branch = TwoDimensionalData {
            total_count: add_branch_count,
            total_time: add_branch_time,
            account_trie_count: state_calculate.add_branch_count,
            storage_trie_count: storage_calculate.add_branch_count,
            account_trie_time: state_calculate.add_branch_time,
            storage_trie_time: storage_calculate.add_branch_time,
        };

        let add_leaf = TwoDimensionalData {
            total_count: add_leaf_count,
            total_time: add_leaf_time,
            account_trie_count: state_calculate.add_leaf_count,
            storage_trie_count: storage_calculate.add_leaf_count,
            account_trie_time: state_calculate.add_leaf_time,
            storage_trie_time: storage_calculate.add_leaf_time,
        };

        let root = TwoDimensionalData {
            total_count: root_count,
            total_time: root_time,
            account_trie_count: state_calculate.add_root_count,
            storage_trie_count: storage_calculate.add_root_count,
            account_trie_time: state_calculate.add_root_time,
            storage_trie_time: storage_calculate.add_root_time,
        };

        Self { total_count, total_time, add_branch, add_leaf, root }
    }
}

impl Print for HashBuilderStat {
    fn print_title(&self) {
        println!("\n====================================== metric of HashBuilder ========================================");
        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Count (%)","Time(s)", "Time(%)","Avg time(ns)"
        );
    }

    fn print_content(&self) {
        let total_count = self.total_count;
        let total_time = self.total_time;

        print_hashbuilder_line("total", total_count, total_count, total_time, total_time);
        print_hashbuilder_line(
            "HashBuilder.add_branch",
            self.add_branch.total_count,
            total_count,
            self.add_branch.total_time,
            total_time,
        );
        print_hashbuilder_line(
            "    AccountTrie",
            self.add_branch.account_trie_count,
            total_count,
            self.add_branch.account_trie_time,
            total_time,
        );
        print_hashbuilder_line(
            "    StorageTrie",
            self.add_branch.storage_trie_count,
            total_count,
            self.add_branch.storage_trie_time,
            total_time,
        );

        print_hashbuilder_line(
            "HashBuilder.add_leaf",
            self.add_leaf.total_count,
            total_count,
            self.add_leaf.total_time,
            total_time,
        );
        print_hashbuilder_line(
            "    AccountTrie",
            self.add_leaf.account_trie_count,
            total_count,
            self.add_leaf.account_trie_time,
            total_time,
        );
        print_hashbuilder_line(
            "    StorageTrie",
            self.add_leaf.storage_trie_count,
            total_count,
            self.add_leaf.storage_trie_time,
            total_time,
        );

        print_hashbuilder_line(
            "HashBuilder.root",
            self.root.total_count,
            total_count,
            self.root.total_time,
            total_time,
        );
        print_hashbuilder_line(
            "    AccountTrie",
            self.root.account_trie_count,
            total_count,
            self.root.account_trie_time,
            total_time,
        );
        print_hashbuilder_line(
            "    StorageTrie",
            self.root.storage_trie_count,
            total_count,
            self.root.storage_trie_time,
            total_time,
        );

        println!("");
    }
}

#[derive(Default, Debug)]
struct Keccak256Stat {
    execute_count: u64,
    execute_time: u64,
}

impl From<&StateRootRecord> for Keccak256Stat {
    fn from(record: &StateRootRecord) -> Self {
        Self {
            execute_count: record.keccak256_record.count,
            execute_time: record.keccak256_record.time_cycles,
        }
    }
}

impl Print for Keccak256Stat {
    fn print_title(&self) {
        println!("\n==========Metric of keccak256==========");
    }

    fn print_content(&self) {
        print_line_u64("execute count           :", self.execute_count);
        print_line_f64("execute time(s)         :", cycles_as_secs(self.execute_time));

        let avg_time =
            if self.execute_count == 0 { 0 } else { self.execute_time / self.execute_count };

        print_line_f64("average execute time(ns):", convert_cycles_to_ns_f64(avg_time));
        println!("");
    }
}

#[derive(Default, Debug)]
struct UpdateKeysStat {
    total_txs_count: u64,
    update_key: OneDimensionalData,
}

impl From<&StateRootRecord> for UpdateKeysStat {
    fn from(record: &StateRootRecord) -> Self {
        let total_update_keys_count = record
            .update_keys
            .account_trie_count
            .checked_add(record.update_keys.storage_trie_count)
            .expect("overflow");

        let data = OneDimensionalData {
            total: total_update_keys_count,
            account_trie: record.update_keys.account_trie_count,
            storage_trie: record.update_keys.storage_trie_count,
        };

        Self { total_txs_count: record.total_txs_count, update_key: data }
    }
}

impl Print for UpdateKeysStat {
    fn print_title(&self) {
        println!("\n===================update keys ======================");
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_LITTLE_BIG$}",
            "Category", "Count", "Keys per transaction"
        );
    }

    fn print_content(&self) {
        print_update_keys_line("AccountTrie", self.update_key.account_trie, self.total_txs_count);
        print_update_keys_line("StorageTrie", self.update_key.storage_trie, self.total_txs_count);
        print_update_keys_line("StateTrie", self.update_key.total, self.total_txs_count);
        println!("");
    }
}

#[derive(Default, Debug)]
struct TrieStat {
    account_trie_leaf_count: u64,
    storage_trie_leaf_count: u64,

    account_trie_leaf_depth: u64,
    storage_trie_leaf_depth: u64,

    state_trie_leaf_count: u64,
    state_trie_leaf_depth: u64,
}

impl From<&StateRootRecord> for TrieStat {
    fn from(record: &StateRootRecord) -> Self {
        Self {
            account_trie_leaf_count: record.state_trie_info.trie_node.leaf_number,
            storage_trie_leaf_count: record.storage_trie_info.trie_node.leaf_number,
            account_trie_leaf_depth: record.state_trie_info.trie_node.leaf_depth,
            storage_trie_leaf_depth: record.storage_trie_info.trie_node.leaf_depth,
            state_trie_leaf_count: record.state_trie_info.state_node.leaf_number,
            state_trie_leaf_depth: record.state_trie_info.state_node.leaf_depth,
        }
    }
}

impl Print for TrieStat {
    fn print_title(&self) {
        println!("\n====================== metric of trie ========================");
        println!(
            "{:COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Trie Type", "Leaf number", "Leaf depth", "Avg.leaf depth"
        );
    }

    fn print_content(&self) {
        print_trie_metric_line(
            "AccountTrie",
            self.account_trie_leaf_count,
            self.account_trie_leaf_depth,
        );
        print_trie_metric_line(
            "StorageTrie",
            self.storage_trie_leaf_count,
            self.storage_trie_leaf_depth,
        );
        print_trie_metric_line("StateTrie", self.state_trie_leaf_count, self.state_trie_leaf_depth);
    }
}

#[derive(Default, Debug)]
struct CountAndTimeDate {
    count: u64,
    time: u64,
}

#[derive(Default, Debug)]
struct ReadDBStat {
    total: CountAndTimeDate,
    leaf_table: CountAndTimeDate,
    ha: CountAndTimeDate,
    ha_seek: CountAndTimeDate,
    ha_next: CountAndTimeDate,
    hs: CountAndTimeDate,
    hs_seek_exact: CountAndTimeDate,
    hs_seek_by_sub_key: CountAndTimeDate,
    hs_next_dup_val: CountAndTimeDate,
    branch_table: CountAndTimeDate,
    at: CountAndTimeDate,
    at_current: CountAndTimeDate,
    at_seek: CountAndTimeDate,
    at_seek_exact: CountAndTimeDate,
    st: CountAndTimeDate,
    st_seek_by_subkey: CountAndTimeDate,
    st_current: CountAndTimeDate,
}

impl From<&StateRootRecord> for ReadDBStat {
    fn from(record: &StateRootRecord) -> Self {
        let db_read = &record.db_read;

        let ha_count = db_read.seek_count.checked_add(db_read.next_count).expect("overflow");
        let ha_time = db_read.seek_time.checked_add(db_read.next_time).expect("overflow");

        let hs_count = db_read
            .seek_exact_count
            .checked_add(db_read.seek_by_sub_key_count)
            .and_then(|x| x.checked_add(db_read.next_dup_val_count))
            .expect("overflow");

        let hs_time = db_read
            .seek_exact_time
            .checked_add(db_read.seek_by_sub_key_time)
            .and_then(|x| x.checked_add(db_read.next_dup_val_time))
            .expect("overflow");

        let leaf_table_count = ha_count.checked_add(hs_count).expect("overflow");
        let leaf_table_time = ha_time.checked_add(hs_time).expect("overflow");

        let at_count = db_read
            .at_seek_count
            .checked_add(db_read.at_seek_exact_count)
            .and_then(|x| x.checked_add(db_read.at_current_count))
            .expect("overflow");
        let at_time = db_read
            .at_seek_time
            .checked_add(db_read.at_seek_exact_time)
            .and_then(|x| x.checked_add(db_read.at_current_time))
            .expect("overflow");
        let st_count = db_read
            .st_seek_by_subkey_count
            .checked_add(db_read.st_current_count)
            .expect("overflow");
        let st_time =
            db_read.st_seek_by_subkey_time.checked_add(db_read.st_current_time).expect("overflow");

        let branch_table_count = at_count.checked_add(st_count).expect("overflow");
        let branch_table_time = at_time.checked_add(st_time).expect("overflow");

        let total_count = leaf_table_count.checked_add(branch_table_count).expect("overflow");
        let total_time_cycles = leaf_table_time.checked_add(branch_table_time).expect("overflow");

        let total = CountAndTimeDate { count: total_count, time: total_time_cycles };

        let leaf_table = CountAndTimeDate { count: leaf_table_count, time: leaf_table_time };
        let ha = CountAndTimeDate { count: ha_count, time: ha_time };
        let ha_seek = CountAndTimeDate { count: db_read.seek_count, time: db_read.seek_time };
        let ha_next = CountAndTimeDate { count: db_read.next_count, time: db_read.next_time };
        let hs = CountAndTimeDate { count: hs_count, time: hs_time };
        let hs_seek_exact =
            CountAndTimeDate { count: db_read.seek_exact_count, time: db_read.seek_exact_time };
        let hs_seek_by_sub_key = CountAndTimeDate {
            count: db_read.seek_by_sub_key_count,
            time: db_read.seek_by_sub_key_time,
        };
        let hs_next_dup_val =
            CountAndTimeDate { count: db_read.next_dup_val_count, time: db_read.next_dup_val_time };

        let branch_table = CountAndTimeDate { count: branch_table_count, time: branch_table_time };

        let at = CountAndTimeDate { count: at_count, time: at_time };
        let at_current =
            CountAndTimeDate { count: db_read.at_current_count, time: db_read.at_current_time };
        let at_seek = CountAndTimeDate { count: db_read.at_seek_count, time: db_read.at_seek_time };
        let at_seek_exact = CountAndTimeDate {
            count: db_read.at_seek_exact_count,
            time: db_read.at_seek_exact_time,
        };
        let st = CountAndTimeDate { count: st_count, time: st_time };
        let st_seek_by_subkey = CountAndTimeDate {
            count: db_read.st_seek_by_subkey_count,
            time: db_read.st_seek_by_subkey_time,
        };
        let st_current =
            CountAndTimeDate { count: db_read.st_current_count, time: db_read.st_current_time };

        Self {
            total,
            leaf_table,
            ha,
            ha_seek,
            ha_next,
            hs,
            hs_seek_exact,
            hs_seek_by_sub_key,
            hs_next_dup_val,
            branch_table,
            at,
            at_current,
            at_seek,
            at_seek_exact,
            st,
            st_seek_by_subkey,
            st_current,
        }
    }
}

impl Print for ReadDBStat {
    fn print_title(&self) {
        println!("\n======================================== metric of read db ==========================================");
        println!(
            "{:COL_WIDTH_LITTLE_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category", "Count", "Count (%)","Time(s)", "Time(%)","Avg time(ns)"
        );
    }

    fn print_content(&self) {
        let total_count = self.total.count;
        let total_time_cycles = self.total.time;

        print_read_db_metric_line(
            "total",
            total_count,
            total_count,
            total_time_cycles,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "leaf_table",
            self.leaf_table.count,
            total_count,
            self.leaf_table.time,
            total_time_cycles,
        );

        print_read_db_metric_line(
            "    HA",
            self.ha.count,
            total_count,
            self.ha.time,
            total_time_cycles,
        );

        print_read_db_metric_line(
            "        seek",
            self.ha_seek.count,
            total_count,
            self.ha_seek.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        next",
            self.ha_next.count,
            total_count,
            self.ha_next.time,
            total_time_cycles,
        );

        print_read_db_metric_line(
            "    HS",
            self.hs.count,
            total_count,
            self.hs.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        seek_exact",
            self.hs_seek_exact.count,
            total_count,
            self.hs_seek_exact.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        seek_by_sub_key",
            self.hs_seek_by_sub_key.count,
            total_count,
            self.hs_seek_by_sub_key.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        next_dup_val",
            self.hs_next_dup_val.count,
            total_count,
            self.hs_next_dup_val.time,
            total_time_cycles,
        );

        print_read_db_metric_line(
            "branch_table",
            self.branch_table.count,
            total_count,
            self.branch_table.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "    AT",
            self.at.count,
            total_count,
            self.at.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        current",
            self.at_current.count,
            total_count,
            self.at_current.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        seek",
            self.at_seek.count,
            total_count,
            self.at_seek.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        seek_exact",
            self.at_seek_exact.count,
            total_count,
            self.at_seek_exact.time,
            total_time_cycles,
        );

        print_read_db_metric_line(
            "    ST",
            self.st.count,
            total_count,
            self.st.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        seek_by_subkey",
            self.st_seek_by_subkey.count,
            total_count,
            self.st_seek_by_subkey.time,
            total_time_cycles,
        );
        print_read_db_metric_line(
            "        current",
            self.st_current.count,
            total_count,
            self.st_current.time,
            total_time_cycles,
        );

        println!("");
    }
}

#[derive(Default, Debug)]
struct ReadDBCountAndTimeStat {
    total_update_keys_count: u64,
    total_read_db_count: u64,
    total_read_db_time: u64,
}

impl From<&StateRootRecord> for ReadDBCountAndTimeStat {
    fn from(record: &StateRootRecord) -> Self {
        let total_update_keys_count = record
            .update_keys
            .account_trie_count
            .checked_add(record.update_keys.storage_trie_count)
            .expect("overflow");

        let read_db_stat: ReadDBStat = record.into();

        Self {
            total_update_keys_count,
            total_read_db_count: read_db_stat.total.count,
            total_read_db_time: read_db_stat.total.time,
        }
    }
}

impl Print for ReadDBCountAndTimeStat {
    fn print_title(&self) {
        println!("\n============================================");
    }

    fn print_content(&self) {
        let avg_update_count = if self.total_update_keys_count == 0 {
            0.0
        } else {
            self.total_read_db_count as f64 / self.total_update_keys_count as f64
        };

        print_line_u64("read db count               :", self.total_read_db_count);
        print_line_f64("read db count / update keys :", avg_update_count);

        println!("\n");

        let avg_time_ns = if self.total_read_db_count == 0 {
            0.0
        } else {
            convert_cycles_to_ns_f64(self.total_read_db_time / self.total_read_db_count)
        };

        print_line_f64("read db time(s)         :", cycles_as_secs(self.total_read_db_time));
        print_line_f64("average read db time(ns):", avg_time_ns);

        println!("");
    }
}

impl Print for StateRootRecord {
    fn print_content(&self) {
        Into::<ChainDataStat>::into(self).print(self.end_block_number);
        Into::<BreakdownStat>::into(self).print(self.end_block_number);
        Into::<StateCalculateStat>::into(self).print(self.end_block_number);
        Into::<BreakdownCategoryStat>::into(self).print(self.end_block_number);
        Into::<StorageCalculateStat>::into(self).print(self.end_block_number);
        Into::<TryNextStat>::into(self).print(self.end_block_number);
        Into::<ReadNodeStat>::into(self).print(self.end_block_number);
        Into::<HashBuilderStat>::into(self).print(self.end_block_number);
        Into::<Keccak256Stat>::into(self).print(self.end_block_number);
        Into::<UpdateKeysStat>::into(self).print(self.end_block_number);
        Into::<TrieStat>::into(self).print(self.end_block_number);
        Into::<ReadDBStat>::into(self).print(self.end_block_number);
        Into::<ReadDBCountAndTimeStat>::into(self).print(self.end_block_number);

        println!(
            "\n=================================== db distribution ==========================================="
        );
        self.db_read_distribution.print_content();
        println!("");
    }
}
