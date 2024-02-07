#[cfg(feature = "enable_opcode_metrics")]
use super::opcode::*;
#[cfg(feature = "enable_opcode_metrics")]
use revm::OpCode;
#[cfg(feature = "enable_opcode_metrics")]
use revm_utils::metrics::types::OpcodeRecord;
#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_state_root_record"
))]
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
#[cfg(feature = "enable_state_root_record")]
use crate::DBRead;

#[cfg(feature = "enable_db_speed_record")]
use crate::metrics::DatabaseOperationRecord;

#[cfg(feature = "enable_state_root_record")]
use crate::metrics::StateRootUpdateRecord;

#[cfg(feature = "enable_opcode_metrics")]
const MGAS_TO_GAS: u64 = 1_000_000u64;
#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure",
    feature = "enable_state_root_record"
))]
const COL_WIDTH_MIDDLE: usize = 14;
#[cfg(any(
    feature = "enable_cache_record",
    feature = "enable_write_to_db_measure",
    feature = "enable_state_root_record",
))]
const COL_WIDTH_BIG: usize = 20;
#[cfg(feature = "enable_state_root_record")]
const COL_WIDTH_LITTLE_BIG: usize = 25;
#[cfg(any(
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure",
    feature = "enable_state_root_record"
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
    last_print_block_number: u64,
}

#[cfg(feature = "enable_execution_duration_record")]
impl ExecutionDurationDisplayer {
    const N: u64 = 1000;

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
    pub(crate) fn print(&mut self, block_number: u64) {
        if self.last_print_block_number == u64::default() {
            self.last_print_block_number = block_number;
        }

        let interval = block_number.checked_sub(self.last_print_block_number).expect("overflow");
        if interval < Self::N && 0 != block_number % Self::N {
            return
        }
        self.last_print_block_number = block_number;

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

    timer_switch: bool,
    last_print_block_number: u64,
}

#[cfg(feature = "enable_tps_gas_record")]
impl TpsAndGasRecordDisplayer {
    const N: u64 = 1000;

    pub(crate) fn update_tps_and_gas(&mut self, block_number: u64, txs: u128, gas: u128) {
        // if 0 == block_number % Self::N {
        //     self.print(block_number, txs, gas);
        // }
        if self.last_print_block_number == u64::default() {
            self.last_print_block_number = block_number;
        }

        let interval = block_number.checked_sub(self.last_print_block_number).expect("overflow");
        if interval >= Self::N || 0 == block_number % Self::N {
            self.print(block_number, txs, gas);
            self.last_print_block_number = block_number;
        }

        self.last_txs = txs;
        self.last_gas = gas;
    }

    pub(crate) fn start_record(&mut self, _block_number: u64) {
        if self.timer_switch == false {
            self.pre_txs = self.last_txs;
            self.pre_gas = self.last_gas;
            self.pre_instant = Instant::now();

            self.timer_switch = true;
        }
    }

    pub(crate) fn stop_record(&mut self, _block_number: u64) {
        // self.print(block_number, self.last_txs, self.last_gas);
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
        Self {
            pre_txs: 0,
            pre_gas: 0,
            last_txs: 0,
            last_gas: 0,
            pre_instant: Instant::now(),
            timer_switch: false,
            last_print_block_number: u64::default(),
        }
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

#[cfg(feature = "enable_state_root_record")]
#[derive(Debug, Default)]
pub(crate) struct StateRootUpdateDisplayer {
    record: StateRootUpdateRecord,
    last_print_block_number: u64,
}

#[cfg(feature = "enable_state_root_record")]
impl StateRootUpdateDisplayer {
    const N: u64 = 1000;

    pub(crate) fn record(&mut self, record: StateRootUpdateRecord) {
        self.record.add(record);
    }

    fn print_line_u64(&self, name: &str, value: u64) {
        println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}", name, value);
    }

    fn print_line_f64(&self, name: &str, value: f64) {
        println!("{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$.3}", name, value);
    }

    fn print_caculate_stat(&self, record: &crate::metrics::CaculateRecord) {
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
            .checked_add(self.record.state_root_calculator())
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
            "state_root_calculator",
            self.record.state_root_calculator(),
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
            cycles_as_secs(self.record.state_root_calculator()),
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
    }
}

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
    feature = "enable_execute_measure",
    feature = "enable_write_to_db_measure",
    feature = "enable_state_root_record",
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
