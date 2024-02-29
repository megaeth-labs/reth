//! This module is used to support the display of cached state related metrics.
use super::commons::*;
use revm_utils::{
    metrics::types::{CacheDbRecord, Function},
    time_utils::convert_cycles_to_ns_f64,
};

const COL_WIDTH_BIG: usize = 20;
const COL_WIDTH_MIDDLE: usize = 14;

#[derive(Default, Debug, Copy, Clone)]
struct CacheStat {
    hits: u64,
    misses: u64,
    miss_ratio: f64,
    penalty: f64,
    avg_penalty: f64,
}

const CACHE_STATS_LEN: usize = 5;
#[derive(Debug)]
struct CacheStats {
    functions: [CacheStat; CACHE_STATS_LEN],
}

impl CacheStats {
    fn print_item(&self, function: &str, index: usize) {
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$.3}{:>COL_WIDTH_BIG$.3}{:>COL_WIDTH_BIG$.3}",
            function, self.functions[index].hits, self.functions[index].misses, self.functions[index].miss_ratio * 100.0, self.functions[index].penalty, self.functions[index].avg_penalty
        );
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        CacheStats { functions: [CacheStat::default(); CACHE_STATS_LEN] }
    }
}

impl Print for CacheStats {
    fn print_title(&self) {
        println!("================================================ Metric of State ===========================================");
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$}{:>COL_WIDTH_BIG$}{:>COL_WIDTH_BIG$}",
            "State functions", "Hits", "Misses", "Miss ratio (%)","Penalty time(s)", "Avg penalty (us)"
        );
    }

    fn print_content(&self) {
        self.print_item("blockhash", Function::BlockHash as usize);
        self.print_item("code_by_hash", Function::CodeByHash as usize);
        self.print_item("load_account/basic", Function::LoadCacheAccount as usize);
        self.print_item("storage", Function::Storage as usize);
        self.print_item("total", CACHE_STATS_LEN - 1);
    }
}

trait StatsAndPrint {
    fn stats(&self) -> CacheStats;
    fn print_penalty_distribution(&self);
}

impl StatsAndPrint for CacheDbRecord {
    fn stats(&self) -> CacheStats {
        let mut cache_stats = CacheStats::default();

        let total_stats = self.access_count();
        let hit_stats = self.hit_stats();
        let miss_stats = self.miss_stats();
        let penalty_stats = self.penalty_stats();

        for index in 0..total_stats.function.len() {
            cache_stats.functions[index].hits = hit_stats.function[index];
            cache_stats.functions[index].misses = miss_stats.function[index];
            cache_stats.functions[index].miss_ratio =
                miss_stats.function[index] as f64 / total_stats.function[index] as f64;
            cache_stats.functions[index].penalty =
                cycles_as_secs(penalty_stats.time.function[index]);
            cache_stats.functions[index].avg_penalty =
                convert_cycles_to_ns_f64(penalty_stats.time.function[index]) /
                    (1000 * miss_stats.function[index]) as f64;
        }
        cache_stats.functions[CACHE_STATS_LEN - 1].hits = hit_stats.function.iter().sum();
        cache_stats.functions[CACHE_STATS_LEN - 1].misses = miss_stats.function.iter().sum();
        cache_stats.functions[CACHE_STATS_LEN - 1].miss_ratio =
            cache_stats.functions[CACHE_STATS_LEN - 1].misses as f64 /
                total_stats.function.iter().sum::<u64>() as f64;
        cache_stats.functions[CACHE_STATS_LEN - 1].penalty =
            cycles_as_secs(penalty_stats.time.function.iter().sum());
        cache_stats.functions[CACHE_STATS_LEN - 1].avg_penalty =
            convert_cycles_to_ns_f64(penalty_stats.time.function.iter().sum()) /
                (1000 * cache_stats.functions[CACHE_STATS_LEN - 1].misses) as f64;

        cache_stats
    }

    fn print_penalty_distribution(&self) {
        println!();
        println!("================Penalty percentile=============");
        self.penalty_stats().percentile.print_content();
        println!();
    }
}

impl Print for CacheDbRecord {
    fn print(&self, _block_number: u64) {
        self.stats().print(_block_number);
        self.print_penalty_distribution();
    }
}

pub(super) fn print_state_size(block_number: u64, size: usize) {
    println!();
    println! {"block_number: {:?}, State size: {:?}", block_number, size};
    println!();
}
