#[cfg(feature = "enable_opcode_metrics")]
use super::opcode::*;
#[cfg(feature = "enable_opcode_metrics")]
use revm::OpCode;
#[cfg(feature = "enable_opcode_metrics")]
use revm_utils::metrics::types::OpcodeRecord;
#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record"))]
use revm_utils::time_utils::convert_cycles_to_ns_f64;

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
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure"
))]
const COL_WIDTH_MIDDLE: usize = 14;
#[cfg(any(feature = "enable_cache_record", feature = "enable_write_to_db_measure"))]
const COL_WIDTH_BIG: usize = 20;
#[cfg(any(
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure"
))]
const COL_WIDTH_LARGE: usize = 48;

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
    dyn_gas: [f64; OPCODE_NUMBER],
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

    fn static_gas(&self, opcode: u8) -> Option<u64> {
        Some(MERGE_MAP[opcode as usize]?.1.gas)
    }

    fn cat(&self, opcode: u8) -> Option<&'static str> {
        Some(MERGE_MAP[opcode as usize]?.1.category)
    }

    fn print_header(&self) {
        println!("\n=================================================================Metric of instruction==========================================================\n");
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
        static_gas: u64,
        dyn_gas: f64,
        cat: &str,
    ) {
        // TODO: This needs to be modified.
        if dyn_gas < 50.0 {
            println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.3} \
            {:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}",
            opcode_jump,
            count,
            count_percent,
            time,
            time_percent,
            cost,
            total_gas,
            gas_percent,
            static_gas,
            "",
            cat,
        );
        } else {
            println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.3} \
            {:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}",
            opcode_jump,
            count,
            count_percent,
            time,
            time_percent,
            cost,
            total_gas,
            gas_percent,
            static_gas,
            dyn_gas,
            cat,
        );
        }
    }

    fn print_opcode(&self) {
        println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$} \
            {:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}", 
            "Opcode", 
            "Count", 
            "Count (%)", 
            "Time (s)", 
            "Time (%)", 
            "Cost (ns)", 
            "Total Mgas",
            "Gas (%)",
            "Static gas",
            "Dyn. gas",
            "Category"
            );

        let avg_cost = convert_cycles_to_ns_f64(self.total_duration) / self.total_count as f64;
        println!(
            "{: <COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$.3}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.3} \
            {:>COL_WIDTH_MIDDLE$.1}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$.2}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}",
            "Overall",
            self.total_count,
            100f64,
            cycles_as_secs(self.total_duration),
            self.total_duration_percent * 100.0,
            avg_cost,
            self.total_gas,
            100f64,
            "NAN",
            "",
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
                self.static_gas(op).unwrap_or(0),
                self.dyn_gas[i],
                self.cat(op).unwrap_or(""),
            );
        }

        println!();
        println!();
        println!("call additional rdtsc count: {}", self.opcode_record.additional_count[0]);
        println!("call_code additional rdtsc count: {}", self.opcode_record.additional_count[1]);
        println!(
            "delegate_call additional rdtsc count: {}",
            self.opcode_record.additional_count[2]
        );
        println!("static_call additional rdtsc count: {}", self.opcode_record.additional_count[3]);
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
        println!("==========sload time percentile=========");
        println!("Time (ns)      Count (%)       Cuml. (%)");
        for index in 0..self.opcode_record.sload_percentile.span_in_ns {
            let pct =
                self.opcode_record.sload_percentile.ns_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!("{:8} {:15.3} {:15.3}", (index + 1) * 100, pct * 100.0, cuml * 100.0);
        }

        let ns_span_in_us =
            ((self.opcode_record.sload_percentile.span_in_ns * 100) as f64 / 1000.0) as usize;
        for index in ns_span_in_us..self.opcode_record.sload_percentile.span_in_us {
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

    fn caculate_gas(&self, opcode: u8, count: u64, total_gas: i128) -> (f64, f64) {
        let (base_gas, is_static) = match MERGE_MAP[opcode as usize] {
            Some(opcode_info) => (opcode_info.1.gas, opcode_info.1.static_gas),
            None => return (0.0, 0.0),
        };

        let static_gas = base_gas.checked_mul(count).unwrap_or(0);
        if is_static {
            return (static_gas as f64, 0.0)
        }

        let dyn_gas = if total_gas > static_gas as i128 {
            (total_gas - static_gas as i128) as f64 / count as f64
        } else {
            0.0
        };

        (total_gas as f64, dyn_gas)
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

        let mut opcode_gas: [(f64, f64); OPCODE_NUMBER] = [(0.0, 0.0); OPCODE_NUMBER];
        let mut dyn_gas: [f64; OPCODE_NUMBER] = [0.0; OPCODE_NUMBER];
        let mut total_gas: f64 = 0.0;
        for (i, v) in metric_record.opcode_record.iter().enumerate() {
            let op = i as u8;
            let (op_gas, dync) = self.caculate_gas(op, v.0, v.2);
            opcode_gas[i].0 = op_gas / MGAS_TO_GAS as f64;
            if opcode_gas[i].0 > 0.0 {
                total_gas += opcode_gas[i].0;
            } else {
                total_gas -= opcode_gas[i].0;
            }
            dyn_gas[i] = dync;
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
            dyn_gas,
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
    pub(crate) fn update_excution_duration_record(&mut self, record: ExecutionDurationRecord) {
        self.record = record;
    }

    fn print_line(&self, cat: &str, cycles: u64) {
        let pct = cycles as f64 / self.record.total() as f64;
        let time = cycles_as_secs(cycles);

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
        println!("=========================Breakdown of ExecutionStage========================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (s)", "Time (%)",
        );

        self.print_line("total", self.record.total());
        self.print_line("misc", self.record.misc());
        self.print_line("block_td", self.record.block_td());
        self.print_line("block_with_senders", self.record.block_with_senders());
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

    pub(crate) fn print(&self) {
        let col_len = 20;
        println!();
        println!(
            "==================================Metric of db speed============================"
        );
        println!(
            "{:col_len$}{:>col_len$}{:>col_len$}{:>col_len$}",
            "Cat.", "Size (MB)", "Time (s)", "Rate (MB/s)"
        );

        let (size, time) = self.record.header_td_info();
        let header_td_size = convert_bytes_to_mega(size);
        let header_td_time = time.as_secs_f64();
        let header_td_rate = header_td_size / header_td_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "header_td",
        header_td_size, header_td_time, header_td_rate};

        let (size, time) = self.record.block_with_senders_info();
        let block_with_senders_time = time.as_secs_f64();
        let block_with_senders_size = convert_bytes_to_mega(size);
        let block_with_senders_rate = block_with_senders_size / block_with_senders_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "header_with_senders",
        block_with_senders_size, block_with_senders_time, block_with_senders_rate};

        let (size, time) = self.record.write_to_db_info();
        let write_to_db_time = time.as_secs_f64();
        let write_to_db_size = convert_bytes_to_mega(size);
        let write_to_db_rate = write_to_db_size / write_to_db_time;
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "write_to_db",
        write_to_db_size, write_to_db_time, write_to_db_rate};

        println!();
    }
}

#[cfg(feature = "enable_cache_record")]
#[derive(Default, Debug)]
pub(crate) struct CacheDBRecordDisplayer {
    cache_db_record: CacheDbRecord,
    cachedb_size: usize,
    block_number: u64,
    miss_pct: [f64; 5],
}

#[cfg(feature = "enable_cache_record")]
impl CacheDBRecordDisplayer {
    pub(crate) fn update_cachedb_record(
        &mut self,
        block_number: u64,
        size: usize,
        record: CacheDbRecord,
    ) {
        self.cache_db_record = record;
        self.cachedb_size = size;
        self.block_number = block_number;
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

    fn print_line(
        &self,
        function: &str,
        hits: u64,
        misses: u64,
        misses_pct: f64,
        penalty: f64,
        avg_penalty: f64,
    ) {
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$.3}{:>COL_WIDTH_BIG$.3}{:>COL_WIDTH_BIG$.3}",
            function, hits, misses, misses_pct, penalty, avg_penalty
        );
    }

    fn print_cat(&self, name: &str, function: Function) {
        let index = function as usize;
        let miss_count = self.cache_db_record.miss_stats().function[index];
        let penalty = cycles_as_secs(self.cache_db_record.penalty_stats().time.function[index]);
        let avg_penalty =
            convert_cycles_to_ns_f64(self.cache_db_record.penalty_stats().time.function[index]) /
                (1000 * miss_count) as f64;

        self.print_line(
            name,
            self.cache_db_record.hit_stats().function[index],
            miss_count,
            self.miss_pct[index] * 100.0,
            penalty,
            avg_penalty,
        );
    }

    pub(crate) fn print(&self) {
        println!("================================================ Metric of State ===========================================");
        println!(
            "{: <COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_BIG$}{:>COL_WIDTH_BIG$}{:>COL_WIDTH_BIG$}",
            "State functions", "Hits", "Misses", "Miss ratio (%)","Penalty time(s)", "Avg penalty (us)"
        );
        self.print_cat("blockhash", Function::BlockHash);
        self.print_cat("code_by_hash", Function::CodeByHash);
        self.print_cat("load_account/basic", Function::LoadCacheAccount);
        self.print_cat("storage", Function::Storage);

        let total_penalty = self.cache_db_record.penalty_stats().time.function.iter().sum();
        let total_miss_count: u64 = self.cache_db_record.miss_stats().function.iter().sum();
        self.print_line(
            "total",
            self.cache_db_record.hit_stats().function.iter().sum(),
            self.cache_db_record.miss_stats().function.iter().sum(),
            self.miss_pct[4] * 100.0,
            cycles_as_secs(total_penalty),
            convert_cycles_to_ns_f64(total_penalty) / (total_miss_count * 1000) as f64,
        );
        println!();

        // print penalty distribution
        println!();
        let total_cnt: u64 =
            self.cache_db_record.penalty_stats().percentile.us_percentile.iter().map(|&v| v).sum();
        let mut cuml = 0.0;
        println!("===================Penalty percentile=============");
        println!("Time (ns)                 Count (%)      Cuml. (%)");
        // println! {"{:<COL_WIDTH_BIG$}{:>COL_WIDTH_MIDDLE$}{:>COL_WIDTH_MIDDLE$}", "Time (ns)",
        // "Count (%)", "Cuml. (%)"};
        for index in 0..self.cache_db_record.penalty_stats().percentile.span_in_ns {
            let pct = self.cache_db_record.penalty_stats().percentile.ns_percentile[index] as f64 /
                total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH_BIG$} {:>COL_WIDTH_MIDDLE$.3} {:>COL_WIDTH_MIDDLE$.3}",
                (index + 1) * 100,
                pct * 100.0,
                cuml * 100.0
            );
        }

        let ns_span_in_us = ((self.cache_db_record.penalty_stats().percentile.span_in_ns * 100)
            as f64 /
            1000.0) as usize;
        for index in ns_span_in_us..self.cache_db_record.penalty_stats().percentile.span_in_us {
            let pct = self.cache_db_record.penalty_stats().percentile.us_percentile[index] as f64 /
                total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH_BIG$} {:>COL_WIDTH_MIDDLE$.3} {:>COL_WIDTH_MIDDLE$.3}",
                (index + 1) * 1000,
                pct * 100.0,
                cuml * 100.0
            );
        }

        // print State size
        println!();
        println! {"block_number: {:?}, State size: {:?}", self.block_number, self.cachedb_size};
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
            self.print(block_number, txs, gas);
        }

        self.last_txs = txs;
        self.last_gas = gas;
    }

    pub(crate) fn start_record(&mut self) {
        self.pre_txs = self.last_txs;
        self.pre_gas = self.last_gas;
        self.pre_instant = Instant::now();
    }

    pub(crate) fn stop_record(&mut self, block_number: u64) {
        self.print(block_number, self.last_txs, self.last_gas);
    }

    fn print(&mut self, block_number: u64, txs: u128, gas: u128) {
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
        println!("block_number: {:?}, TPS : {:?}", block_number, tps);
        println!("block_number: {:?}, MGas: {:.3}\n", block_number, mgas_ps);
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
        println!();
        println!("=============================Breakdown of execute txs ======================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (s)", "Time (%)",
        );

        self.print_line("total", self.record.total());
        self.print_line("misc", self.record.misc());
        self.print_line("transact", self.record.transact());
        self.print_line("    revm_transact", self.record.revm_transact().total);
        self.print_line(
            "    preverify_transaction_inner",
            self.record.revm_transact().preverify_transaction_inner,
        );
        self.print_line(
            "    before execute(transact_preverified_inner)",
            self.record.revm_transact().transact_preverified_inner.before_execute,
        );
        self.print_line(
            "    execute(transact_preverified_inner)",
            self.record.revm_transact().transact_preverified_inner.execute,
        );
        self.print_line(
            "    after_execute(transact_preverified_inner)",
            self.record.revm_transact().transact_preverified_inner.after_execute,
        );
        self.print_line("    handler_end", self.record.revm_transact().handle_end);
        self.print_line("commit", self.record.commit_changes());
        self.print_line("add_receipt", self.record.add_receipt());
        self.print_line(
            "apply_post_execution_state_change",
            self.record.apply_post_execution_state_change(),
        );
        self.print_line("merge_transactions", self.record.merge_transactions());
        self.print_line("verify_receipt", self.record.verify_receipt());
        self.print_line("save receipts", self.record.save_receipts());

        println!();
    }

    fn print_line(&self, cat: &str, cycles: u64) {
        let time = cycles_as_secs(cycles);
        let pct = time / cycles_as_secs(self.record.total());

        println!(
            "{:<COL_WIDTH_LARGE$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}",
            cat,
            time,
            pct * 100.0,
        );
    }
}

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure"
))]
fn cycles_as_secs(cycles: u64) -> f64 {
    revm_utils::time_utils::convert_cycles_to_duration(cycles).as_secs_f64()
}

#[cfg(feature = "enable_write_to_db_measure")]
use crate::metrics::WriteToDbRecord;

#[cfg(feature = "enable_write_to_db_measure")]
#[derive(Debug, Default)]
pub(crate) struct WriteToDbDisplayer {
    record: WriteToDbRecord,
}

#[cfg(feature = "enable_write_to_db_measure")]
impl WriteToDbDisplayer {
    pub(crate) fn record(&mut self, record: WriteToDbRecord) {
        self.record = record;
    }

    pub(crate) fn print(&self) {
        // size
        let revert_storage_size = self.record.revert_storage_size();
        let revert_account_size = self.record.revert_account_size();
        let write_receipts_size = self.record.write_receipts_size();
        let state_account_size = self.record.state_account_size();
        let state_bytecode_size = self.record.state_bytecode_size();
        let state_storage_size = self.record.state_storage_size();
        let total_size = revert_storage_size +
            revert_account_size +
            write_receipts_size +
            state_account_size +
            state_bytecode_size +
            state_storage_size;

        // time
        let total_time = self.record.total_time();
        let revert_storage_time = self.record.revert_storage_time();
        let revert_account_time = self.record.revert_account_time();
        let write_receipts_time = self.record.write_receipts_time();
        let sort_time = self.record.sort_time();
        let state_account_time = self.record.state_account_time();
        let state_bytecode_time = self.record.state_bytecode_time();
        let state_storage_time = self.record.state_storage_time();

        let revert_storage_append_time = self.record.revert_storage_append_time();
        let revert_account_append_time = self.record.revert_account_append_time();
        let receipts_append_time = self.record.receipts_append_time();
        let state_account_upsert_time = self.record.state_account_upsert_time();
        let state_bytecode_upsert_time = self.record.state_bytecode_upsert_time();
        let state_storage_upsert_time = self.record.state_storage_upsert_time();

        // print
        println!();
        println!("=================================================Breakdown of write_to_db ==========================================");
        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_BIG$}",
            "Category",  
            "Size (MB)",   
            "Time (s)",    
            "Time (%)",   
            "Rate (MB/s)"
        );

        self.print_line("total", Some(total_size), total_time);
        self.print_line(
            "write storage (revert state)",
            Some(revert_storage_size),
            revert_storage_time,
        );
        self.print_line(
            "    write storage iter time (revert state)",
            None,
            revert_storage_time - revert_storage_append_time,
        );
        self.print_line(
            "    write storage append time (revert state)",
            Some(revert_storage_size),
            revert_storage_append_time,
        );
        self.print_line(
            "write account (revert state)",
            Some(revert_account_size),
            revert_account_time,
        );
        self.print_line(
            "    write account iter time (revert state)",
            None,
            revert_account_time - revert_account_append_time,
        );
        self.print_line(
            "    write account append time (revert state)",
            Some(revert_account_size),
            revert_account_append_time,
        );
        self.print_line("write_receipts", Some(write_receipts_size), write_receipts_time);
        self.print_line(
            "    write receipts iter time",
            None,
            write_receipts_time - receipts_append_time,
        );
        self.print_line(
            "    write receipts append time",
            Some(write_receipts_size),
            receipts_append_time,
        );
        self.print_line("sort state changes", None, sort_time);
        self.print_line(
            "write account (state changes)",
            Some(state_account_size),
            state_account_time,
        );
        self.print_line(
            "    write account iter time (state changes)",
            None,
            state_account_time - state_account_upsert_time,
        );
        self.print_line(
            "    write account upsert time (state changes)",
            Some(state_account_size),
            state_account_upsert_time,
        );
        self.print_line(
            "write bytecode (state changes)",
            Some(state_bytecode_size),
            state_bytecode_time,
        );
        self.print_line(
            "    write bytecode iter time (state changes)",
            None,
            state_bytecode_time - state_bytecode_upsert_time,
        );
        self.print_line(
            "    write bytecode upsert time (state changes)",
            Some(state_bytecode_size),
            state_bytecode_upsert_time,
        );
        self.print_line(
            "write storage (state_changes)",
            Some(state_storage_size),
            state_storage_time,
        );
        self.print_line(
            "    write storage iter time (state_changes)",
            None,
            state_storage_time - state_storage_upsert_time,
        );
        self.print_line(
            "    write storage upsert time (state_changes)",
            Some(state_storage_size),
            state_storage_upsert_time,
        );
    }
    fn print_line(&self, cat: &str, size: Option<usize>, cycles: u64) {
        let total_time = cycles_as_secs(self.record.total_time());
        let time = cycles_as_secs(cycles);
        let pct = time / total_time;

        if size.is_none() {
            println!(
                "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_BIG$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}{: >COL_WIDTH_BIG$.3}",
                cat,
                "NAN",   
                time,
                pct * 100.0,
                "NAN"
            );

            return
        }

        let size = convert_bytes_to_mega(size.expect("Size is empty"));
        let rate = size / time;

        println!(
            "{: <COL_WIDTH_LARGE$}{: >COL_WIDTH_BIG$.3}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}{: >COL_WIDTH_BIG$.3}",
            cat,
            size,
            time,
            pct * 100.0,
            rate,
        );
    }
}

#[cfg(any(
    feature = "enable_write_to_db_measure",
    feature = "enable_write_to_db_measure",
    feature = "enable_db_speed_record"
))]
fn convert_bytes_to_mega(size: usize) -> f64 {
    size as f64 / 1024.0 / 1024.0
}
