#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record",))]
use revm_utils::metrics::types::TimeDistributionStats;

/// This trait is used to support the display of metric records.
pub trait Print {
    fn print_title(&self) {}
    fn print_content(&self) {}
    fn print(&self, _block_number: u64) {
        println!();
        self.print_title();
        self.print_content();
        println!();
    }
}

#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record",))]
const COL_WIDTH: usize = 15;

#[cfg(any(feature = "enable_opcode_metrics", feature = "enable_cache_record",))]
impl Print for TimeDistributionStats {
    fn print_content(&self) {
        let total_cnt: u64 = self.us_percentile.iter().map(|&v| v).sum();
        let mut cuml = 0.0;

        println!(
            "{:<COL_WIDTH$} {:>COL_WIDTH$} {:>COL_WIDTH$}",
            "Time (ns)", "Count (%)", "Cuml. (%)"
        );
        for index in 0..self.span_in_ns {
            let pct = self.ns_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH$} {:>COL_WIDTH$.3} {:>COL_WIDTH$.3}",
                (index + 1) * 100,
                pct * 100.0,
                cuml * 100.0
            );
        }

        let ns_span_in_us = ((self.span_in_ns * 100) as f64 / 1000.0) as usize;
        for index in ns_span_in_us..self.span_in_us {
            let pct = self.us_percentile[index] as f64 / total_cnt as f64;
            cuml += pct;
            println!(
                "{:<COL_WIDTH$} {:>COL_WIDTH$.3} {:>COL_WIDTH$.3}",
                (index + 1) * 1000,
                pct * 100.0,
                cuml * 100.0
            );
        }

        println!();
        println!();
        println!("========>for debug:");
        println!("total cnt: {:?}", total_cnt);
        println!("span_in_ns: {:?}", self.span_in_ns);
        println!("span_in_us: {:?}", self.span_in_us);
        println!("in_ns: {:?}", self.ns_percentile);
        println!("in_us: {:?}", self.us_percentile);
        println!();
        println!();
    }
}

#[cfg(any(
    feature = "enable_opcode_metrics",
    feature = "enable_cache_record",
    feature = "enable_execution_duration_record",
))]
pub(super) fn cycles_as_secs(cycles: u64) -> f64 {
    revm_utils::time_utils::convert_cycles_to_duration(cycles).as_secs_f64()
}

#[cfg(any(feature = "enable_execution_duration_record"))]
pub(super) fn convert_bytes_to_mega(size: usize) -> f64 {
    size as f64 / 1024.0 / 1024.0
}
