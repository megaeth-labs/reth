#[cfg(feature = "enable_opcode_metrics")]
use super::opcode::*;
#[cfg(feature = "enable_opcode_metrics")]
use revm::OpCode;
#[cfg(feature = "enable_opcode_metrics")]
use revm_utils::{metrics::types::OpcodeRecord, time_utils::convert_cycles_to_ns_f64};

#[cfg(feature = "enable_opcode_metrics")]
use std::collections::BTreeMap;

#[cfg(feature = "enable_cache_record")]
use revm_utils::metrics::types::{CacheDbRecord, Function};

#[cfg(feature = "enable_tps_gas_record")]
use minstant::Instant;
#[cfg(feature = "enable_tps_gas_record")]
use std::ops::{Div, Mul};

#[cfg(feature = "enable_execution_duration_record")]
use crate::metrics::ExecutionDurationRecord;

#[cfg(feature = "enable_db_speed_record")]
use crate::metrics::DatabaseOperationRecord;

#[cfg(feature = "enable_opcode_metrics")]
const MGAS_TO_GAS: u64 = 1_000_000u64;
#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure"
))]
const COL_WIDTH_MIDDLE: usize = 14;
#[cfg(feature = "enable_cache_record")]
const COL_WIDTH_BIG: usize = 20;
#[cfg(any(
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure"
))]
const COL_WIDTH_LARGE: usize = 40;

#[cfg(feature = "enable_opcode_metrics")]
struct OpcodeMergeRecord {
    count: u64,
    duration: u64,
    count_percent: f64,
    duration_percent: f64,
    ave_cost: f64,
}

#[cfg(feature = "enable_opcode_metrics")]
pub(crate) struct OpcodeStats {
    total_count: u64,
    total_duration: u64,
    total_duration_percent: f64,
    count_percent: [f64; OPCODE_NUMBER],
    duration_percent: [f64; OPCODE_NUMBER],
    ave_cost: [f64; OPCODE_NUMBER],
    opcode_gas: [(f64, f64); OPCODE_NUMBER],
    total_gas: f64,
    merge_records: BTreeMap<&'static str, OpcodeMergeRecord>,
    opcode_record: OpcodeRecord,
}

#[cfg(feature = "enable_opcode_metrics")]
impl OpcodeStats {
    pub(crate) fn print(&self) {
        self.print_header();
        self.print_opcode();
        self.print_category();
        self.print_sload_percentile();
        println!("\n");
    }

    fn base_gas(&self, opcode: u8) -> Option<u64> {
        Some(MERGE_MAP[opcode as usize]?.1.gas)
    }

    fn cat(&self, opcode: u8) -> Option<&'static str> {
        Some(MERGE_MAP[opcode as usize]?.1.category)
    }

    fn print_header(&self) {
        println!("\n================================================Metric of instruction================================================\n");
    }

    fn print_opcode_line(
        &self,
        opcode_jump: &str,
        count: u64,
        count_percent: f64,
        time: f64,
        time_percent: f64,
        cost: f64,
        total_gas: f64,
        gas_percent: f64,
        base_gas: u64,
        cat: &str,
    ) {
        println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.3} \
            {:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}",
            opcode_jump,
            count,
            count_percent,
            time,
            time_percent,
            cost,
            total_gas,
            gas_percent,
            base_gas,
            cat,
        );
    }

    fn print_opcode(&self) {
        println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$} \
            {:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}", 
            "Opcode", 
            "Count", 
            "Count (%)", 
            "Time (s)", 
            "Time (%)", 
            "Cost (ns)", 
            "Total Mgas",
            "Gas (%)",
            "Base gas",
            "Category"
            );

        let avg_cost = convert_cycles_to_ns_f64(self.total_duration) / self.total_count as f64;
        println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.3} \
            {:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}",
            "Overall",
            self.total_count,
            100f64,
            cycles_as_secs(self.total_duration),
            self.total_duration_percent * 100.0,
            avg_cost,
            self.total_gas,
            100f64,
            "NAN",
            "NAN",
        );

        for i in 0..OPCODE_NUMBER {
            let op = i as u8;
            let opcode_jump = OpCode::new(op);
            if opcode_jump.is_none() {
                continue
            }

            self.print_opcode_line(
                opcode_jump.unwrap().as_str(),
                self.opcode_record.opcode_record[i].0,
                self.count_percent[i] * 100.0,
                cycles_as_secs(self.opcode_record.opcode_record[i].1),
                self.duration_percent[i] * 100.0,
                self.ave_cost[i],
                self.opcode_gas[i].0,
                self.opcode_gas[i].1 * 100.0,
                self.base_gas(op).unwrap_or(0),
                self.cat(op).unwrap_or(""),
            );
        }
    }
    fn print_category(&self) {
        println!("\n");
        println!("==========================================================================================");
        println!("{:<COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}", 
                "Opcode Cat.", 
                "Count", 
                "Count (%)", 
                "Time (s)", 
                "Time (%)",
                "Cost (ns)", 
        );

        for (k, v) in self.merge_records.iter() {
            println!(
                "{:<COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.3}",
                *k,
                v.count,
                v.count_percent * 100.0,
                cycles_as_secs(v.duration),
                v.duration_percent * 100.0,
                v.ave_cost,
            );
        }
    }

    fn print_sload_percentile(&self) {
        let total_cnt: u64 =
            self.opcode_record.sload_percentile.us_percentile.iter().map(|&v| v).sum();
        let mut cuml = 0.0;
        println!();
        println!();
        println!("total cnt: {:?}", total_cnt);
        println!("span_in_ns: {:?}", self.opcode_record.sload_percentile.span_in_ns);
        println!("span_in_us: {:?}", self.opcode_record.sload_percentile.span_in_us);
        println!("in_ns: {:?}", self.opcode_record.sload_percentile.ns_percentile);
        println!("in_us: {:?}", self.opcode_record.sload_percentile.us_percentile);
        println!();
        println!();
        println!("================================sload time percentile=====================================");
        println!("Time (ns)    Count (%)    Cuml. (%)");
        for index in 0..self.opcode_record.sload_percentile.span_in_ns {
            let pct =
                self.opcode_record.sload_percentile.ns_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!("{:8} {:15.3} {:15.3}", (index + 1) * 100, pct * 100.0, cuml * 100.0);
        }

        let ns_span_in_us =
            ((self.opcode_record.sload_percentile.span_in_ns * 100) as f64 / 1000.0) as usize;
        for index in ns_span_in_us..self.opcode_record.sload_percentile.span_in_ns {
            let pct =
                self.opcode_record.sload_percentile.us_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!("{:8} {:15.3} {:15.3}", (index + 1) * 1000, pct * 100.0, cuml * 100.0);
        }
    }
}

#[cfg(feature = "enable_opcode_metrics")]
#[derive(Default, Debug)]
pub(crate) struct RevmMetricTimeDisplayer {
    /// opcode metric record
    opcode_record: OpcodeRecord,
}

#[cfg(feature = "enable_opcode_metrics")]
impl RevmMetricTimeDisplayer {
    pub(crate) fn update_opcode_record(&mut self, record: OpcodeRecord) {
        self.opcode_record = record;
    }

    fn category_name(&self, opcode: u8) -> Option<&'static str> {
        Some(MERGE_MAP[opcode as usize]?.1.category)
    }

    fn caculate_gas(&self, opcode: u8, record: &(u64, u64, i128)) -> f64 {
        let (base_gas, static_gas) = match MERGE_MAP[opcode as usize] {
            Some(opcode_info) => (opcode_info.1.gas, opcode_info.1.static_gas),
            None => return 0.0,
        };

        if static_gas {
            return base_gas.checked_mul(record.0).unwrap_or(0) as f64
        }

        record.2 as f64
    }

    pub(crate) fn stats(&self, metric_record: &OpcodeRecord) -> OpcodeStats {
        let mut merge_records: BTreeMap<&'static str, OpcodeMergeRecord> = BTreeMap::new();
        let mut total_count: u64 = 0;
        let total_duration = metric_record.total_time;
        let mut total_duration_percent: f64 = 0.0;

        for (i, v) in metric_record.opcode_record.iter().enumerate() {
            total_count = total_count.checked_add(v.0).expect("overflow");
            let cat = match self.category_name(i as u8) {
                Some(name) => name,
                None => continue,
            };

            merge_records
                .entry(cat)
                .and_modify(|r| {
                    r.count += v.0;
                    r.duration += v.1;
                })
                .or_insert(OpcodeMergeRecord {
                    count: v.0,
                    duration: v.1,
                    count_percent: 0.0,
                    duration_percent: 0.0,
                    ave_cost: 0.0,
                });
        }

        let mut opcode_gas: [(f64, f64); 256] = [(0.0, 0.0); 256];
        let mut total_gas: f64 = 0.0;
        for (i, v) in metric_record.opcode_record.iter().enumerate() {
            let op = i as u8;
            let op_gas = self.caculate_gas(op, v);
            opcode_gas[i].0 = op_gas / MGAS_TO_GAS as f64;
            if opcode_gas[i].0 > 0.0 {
                total_gas += opcode_gas[i].0;
            } else {
                total_gas -= opcode_gas[i].0;
            }
        }

        let mut count_percent: [f64; OPCODE_NUMBER] = [0.0; OPCODE_NUMBER];
        let mut duration_percent: [f64; OPCODE_NUMBER] = [0.0; OPCODE_NUMBER];
        let mut ave_cost: [f64; OPCODE_NUMBER] = [0.0; OPCODE_NUMBER];
        for (i, v) in self.opcode_record.opcode_record.iter().enumerate() {
            count_percent[i] = v.0 as f64 / total_count as f64;
            duration_percent[i] = v.1 as f64 / total_duration as f64;

            total_duration_percent += duration_percent[i];
            ave_cost[i] = convert_cycles_to_ns_f64(v.1) / v.0 as f64;
            opcode_gas[i].1 = opcode_gas[i].0 / total_gas;
        }

        for (_, value) in merge_records.iter_mut() {
            value.count_percent = value.count as f64 / total_count as f64;
            value.duration_percent = value.duration as f64 / total_duration as f64;
            value.ave_cost = convert_cycles_to_ns_f64(value.duration) / value.count as f64;
        }

        OpcodeStats {
            total_count,
            total_duration,
            total_duration_percent,
            count_percent,
            duration_percent,
            ave_cost,
            opcode_gas,
            total_gas,
            merge_records,
            opcode_record: metric_record.clone(),
        }
    }

    pub(crate) fn print(&self) {
        let stat = self.stats(&self.opcode_record);
        stat.print();
        println!("\n");
    }
}

#[cfg(feature = "enable_execution_duration_record")]
#[derive(Default, Debug)]
pub(crate) struct ExecutionDurationDisplayer {
    record: ExecutionDurationRecord,
}

#[cfg(feature = "enable_execution_duration_record")]
impl ExecutionDurationDisplayer {
    const SECONDS_ONE_HOUR: f64 = 60.0 * 60.0;
    pub(crate) fn update_excution_duration_record(&mut self, record: ExecutionDurationRecord) {
        self.record = record;
    }

    fn print_line(&self, cat: &str, cycles: u64) {
        let pct = cycles as f64 / self.record.total() as f64;
        let time = cycles_as_secs(cycles) / Self::SECONDS_ONE_HOUR;

        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}",
            cat,
            time,
            pct * 100.0,
        );
    }

    /// print the information of the execution duration record.
    pub(crate) fn print(&self) {
        println!();
        println!("===============================Metric of execution duration===============================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (h)", "Time (%)",
        );

        self.print_line("total", self.record.total());
        self.print_line("misc", self.record.misc());
        self.print_line("fetch_block", self.record.fetch_block());
        self.print_line("execute_and_verify_receipt", self.record.execution());
        self.print_line("take_output_state", self.record.take_output_state());
        self.print_line("write_to_db", self.record.write_to_db());

        println!();
    }
}

#[cfg(feature = "enable_db_speed_record")]
#[derive(Default, Debug)]
pub(crate) struct DBSpeedDisplayer {
    record: DatabaseOperationRecord,
}

#[cfg(feature = "enable_db_speed_record")]
impl DBSpeedDisplayer {
    pub(crate) fn update_db_speed_record(&mut self, record: DatabaseOperationRecord) {
        self.record = record;
    }

    fn convert_bytes_to_mega(&self, size: usize) -> f64 {
        size as f64 / 1024.0 / 1024.0
    }

    pub(crate) fn print(&self) {
        println!();
        println!("===============================Metric of db speed============================");
        println!("Cat.                           Size (MBytes)   Time (s)   Rate (MBytes/s)");

        let col_len = 20;

        let (size, time) = self.record.header_td_info();
        let header_td_size = self.convert_bytes_to_mega(size);
        let header_td_time = time.as_secs_f64();
        let header_td_rate = header_td_size / header_td_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "header_td         ",
        header_td_size, header_td_time, header_td_rate};

        let (size, time) = self.record.block_with_senders_info();
        let block_with_senders_time = time.as_secs_f64();
        let block_with_senders_size = self.convert_bytes_to_mega(size);
        let block_with_senders_rate = block_with_senders_size / block_with_senders_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "header_with_senders",
        block_with_senders_size, block_with_senders_time, block_with_senders_rate};

        let (size, time) = self.record.write_to_db_info();
        let write_to_db_time = time.as_secs_f64();
        let write_to_db_size = self.convert_bytes_to_mega(size);
        let write_to_db_rate = write_to_db_size / write_to_db_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "write_to_db            ",
        write_to_db_size, write_to_db_time, write_to_db_rate};

        println!();
    }
}

#[cfg(feature = "enable_cache_record")]
#[derive(Default, Debug)]
pub(crate) struct CacheDBRecordDisplayer {
    cache_db_record: CacheDbRecord,
    cachedb_size: usize,
    miss_pct: [f64; 5],
}

#[cfg(feature = "enable_cache_record")]
impl CacheDBRecordDisplayer {
    pub(crate) fn update_cachedb_record(&mut self, size: usize, record: CacheDbRecord) {
        self.cache_db_record = record;
        self.cachedb_size = size;
        self.calculate_miss_ratio();
    }

    fn calculate_miss_ratio(&mut self) {
        let total_stats = self.cache_db_record.access_count();
        let miss_stats = self.cache_db_record.miss_stats();

        for index in 0..total_stats.function.len() {
            self.miss_pct[index] =
                miss_stats.function[index] as f64 / total_stats.function[index] as f64;
        }

        self.miss_pct[4] = miss_stats.function.iter().sum::<u64>() as f64 /
            total_stats.function.iter().sum::<u64>() as f64;
    }

    fn print_line(&self, function: &str, hits: u64, misses: u64, misses_pct: f64) {
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$.3}",
            function, hits, misses, misses_pct
        );
    }

    fn print_pct(&self, name: &str, function: Function) {
        let index = function as usize;

        self.print_line(
            name,
            self.cache_db_record.hit_stats().function[index],
            self.cache_db_record.miss_stats().function[index],
            self.miss_pct[index] * 100.0,
        );
    }

    fn print_penalty(&self, name: &str, function: Function) {
        let index = function as usize;
        println! {"{: <COL_WIDTH_LARGE$}{:>COL_WIDTH_MIDDLE$.8}", name, cycles_as_secs(self.cache_db_record.penalty_stats().time.function[index]) / 60.0};
    }

    pub(crate) fn print(&self) {
        println!("==========================Metric of CacheDb=====================");
        println!("============================Hit in CacheDb======================");
        // print miss ratio
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$}",
            "CacheDb functions", "Hits", "Misses", "Miss ratio (%)",
        );
        self.print_pct("blockhash", Function::BlockHash);
        self.print_pct("code_by_hash", Function::CodeByHash);
        self.print_pct("load_account/basic", Function::LoadCacheAccount);
        self.print_pct("storage", Function::Storage);
        self.print_line(
            "total",
            self.cache_db_record.hit_stats().function.iter().sum(),
            self.cache_db_record.miss_stats().function.iter().sum(),
            self.miss_pct[4] * 100.0,
        );
        println!();

        // print total penalty
        let total_penalty = self.cache_db_record.penalty_stats().time.function.iter().sum();
        println!("=====================Misses penalty in CacheDb====================");
        println! {"{:<COL_WIDTH_LARGE$}{:>COL_WIDTH_MIDDLE$}", "CacheDb functions", "Penalty time(min)"};
        self.print_penalty("blockhash", Function::BlockHash);
        self.print_penalty("code_by_hash", Function::CodeByHash);
        self.print_penalty("load_account/basic", Function::LoadCacheAccount);
        self.print_penalty("storage", Function::Storage);
        println! {"{: <COL_WIDTH_LARGE$}{:>COL_WIDTH_MIDDLE$.3}", "total", cycles_as_secs(total_penalty) / 60.0 };
        println!();

        // print penalty distribution
        println!();
        let total_cnt: u64 =
            self.cache_db_record.penalty_stats().percentile.us_percentile.iter().map(|&v| v).sum();
        let mut cuml = 0.0;
        println!("========================Penalty percentile==========================");
        println! {"{:<COL_WIDTH_LARGE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}", "Time (us)", " Count (%)", " Cuml. (%)"};
        for index in 0..self.cache_db_record.penalty_stats().percentile.span_in_ns {
            let pct = self.cache_db_record.penalty_stats().percentile.ns_percentile[index] as f64 /
                total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH_LARGE$} {:>COL_WIDTH_MIDDLE$.3} {:>COL_WIDTH_MIDDLE$.3}",
                (index + 1) * 100,
                pct * 100.0,
                cuml * 100.0
            );
        }

        let ns_span_in_us = ((self.cache_db_record.penalty_stats().percentile.span_in_ns * 100)
            as f64 /
            1000.0) as usize;
        for index in ns_span_in_us..self.cache_db_record.penalty_stats().percentile.span_in_ns {
            let pct = self.cache_db_record.penalty_stats().percentile.us_percentile[index] as f64 /
                total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH_LARGE$} {:>COL_WIDTH_MIDDLE$.3} {:>COL_WIDTH_MIDDLE$.3}",
                (index + 1) * 1000,
                pct * 100.0,
                cuml * 100.0
            );
        }

        // print cache size
        println!();
        println! {"CacheDB size: {:?}", self.cachedb_size};
    }
}

#[cfg(feature = "enable_tps_gas_record")]
#[derive(Debug)]
pub(crate) struct TpsAndGasRecordDisplayer {
    pre_txs: u128,
    pre_gas: u128,
    last_txs: u128,
    last_gas: u128,
    pre_instant: minstant::Instant,
}

#[cfg(feature = "enable_tps_gas_record")]
impl TpsAndGasRecordDisplayer {
    const N: u64 = 1000;

    pub(crate) fn update_tps_and_gas(&mut self, block_number: u64, txs: u128, gas: u128) {
        if 0 == block_number % Self::N {
            self.print(txs, gas);
        }

        self.last_txs = txs;
        self.last_gas = gas;
    }

    pub(crate) fn start_record(&mut self) {
        self.pre_txs = self.last_txs;
        self.pre_gas = self.last_gas;
        self.pre_instant = Instant::now();
    }

    pub(crate) fn stop_record(&mut self) {
        self.print(self.last_txs, self.last_gas);
    }

    fn print(&mut self, txs: u128, gas: u128) {
        let elapsed_ns = self.pre_instant.elapsed().as_nanos();
        let delta_txs = txs - self.pre_txs;
        let delta_gas = gas - self.pre_gas;

        let tps = delta_txs.mul(1000_000_000).div(elapsed_ns);
        let mgas_ps = (delta_gas as f64).mul(1000_000_000 as f64).div(elapsed_ns as f64);

        self.pre_txs = txs;
        self.pre_gas = gas;
        self.pre_instant = Instant::now();

        println!("\n==================Metric of tps and gas========================");
        println!("elapsed(ns) : {:?}", elapsed_ns);
        println!("TPS : {:?}", tps);
        println!("MGas: {:.3}\n", mgas_ps);
    }
}

#[cfg(feature = "enable_tps_gas_record")]
impl Default for TpsAndGasRecordDisplayer {
    fn default() -> Self {
        Self { pre_txs: 0, pre_gas: 0, last_txs: 0, last_gas: 0, pre_instant: Instant::now() }
    }
}

#[cfg(feature = "enable_execute_measure")]
use crate::metrics::ExecuteTxsRecord;

#[cfg(feature = "enable_execute_measure")]
#[derive(Debug, Default)]
pub(crate) struct ExecuteTxsDisplayer {
    record: ExecuteTxsRecord,
}

#[cfg(feature = "enable_execute_measure")]
impl ExecuteTxsDisplayer {
    pub(crate) fn record(&mut self, record: ExecuteTxsRecord) {
        self.record = record;
    }

    pub(crate) fn print(&self) {
        let total = cycles_as_secs(self.record.total());
        let transact = cycles_as_secs(self.record.transact());
        let commit_changes = cycles_as_secs(self.record.commit_changes());
        let add_receipt = cycles_as_secs(self.record.add_receipt());
        let apply_post_block_changes =
            cycles_as_secs(self.record.apply_post_execution_state_change());
        let merge_transactions = cycles_as_secs(self.record.merge_transactions());
        let verify_receipt = cycles_as_secs(self.record.verify_receipt());
        let save_receipts = cycles_as_secs(self.record.save_receipts());
        let execute = transact +
            commit_changes +
            add_receipt +
            apply_post_block_changes +
            merge_transactions +
            verify_receipt +
            save_receipts;
        let misc = total - execute;

        let transact_pct = transact as f64 / total as f64;
        let commit_changes_pct = commit_changes as f64 / total as f64;
        let add_receipt_pct = add_receipt as f64 / total as f64;
        let apply_post_block_changes_pct = apply_post_block_changes as f64 / total as f64;
        let merge_transactions_pct = merge_transactions as f64 / total as f64;
        let verify_receipt_pct = verify_receipt as f64 / total as f64;
        let save_receipts_pct = save_receipts as f64 / total as f64;
        let misc_pct = misc as f64 / total as f64;

        println!();
        println!("===============================Metric of execute txs ====================================================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (s)", "Time (%)",
        );

        self.print_line("total", total, 1.0);
        self.print_line("misc", misc, misc_pct);
        self.print_line("transact", transact, transact_pct);
        self.print_line("commit", commit_changes, commit_changes_pct);
        self.print_line("add_receipt", add_receipt, add_receipt_pct);
        self.print_line(
            "apply_post_execution_state_change",
            apply_post_block_changes,
            apply_post_block_changes_pct,
        );
        self.print_line("merge_transactions", merge_transactions, merge_transactions_pct);
        self.print_line("verify_receipt", verify_receipt, verify_receipt_pct);
        self.print_line("save receipts", save_receipts, save_receipts_pct);

        println!();
    }

    fn print_line(&self, cat: &str, time: f64, percent: f64) {
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}",
            cat,
            time,
            percent * 100.0,
        );
    }
}

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure"
))]
fn cycles_as_secs(cycles: u64) -> f64 {
    revm_utils::time_utils::convert_cycles_to_duration(cycles).as_secs_f64()
}
