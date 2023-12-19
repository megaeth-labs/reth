const STEP_IN_US: usize = 1;
const STEP_IN_NS: usize = 100;

const US_SPAN_SIZE: usize = 200;
const NS_SPAN_SIZE: usize = 40;
const MAX_ARRAY_SIZE: usize = 200;
/// This is a structure for statistical time distribution, which records the
/// distribution of time from two levels: subtle and nanosecond.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct TimeDistributionStats {
    pub total_count: u64,
    /// The subtle range of statistical distribution, step is STEP_IN_US.
    pub span_in_us: usize,
    /// The nanosecond range of statistical distribution, step is STEP_IN_NS.
    pub span_in_ns: usize,
    /// Record the time distribution at a subtle level.
    pub us_percentile: [u64; MAX_ARRAY_SIZE],
    /// Record the time distribution at a nanosecond level.
    pub ns_percentile: [u64; MAX_ARRAY_SIZE],
}

impl Default for TimeDistributionStats {
    fn default() -> Self {
        Self::new(US_SPAN_SIZE, NS_SPAN_SIZE)
    }
}

impl TimeDistributionStats {
    pub(crate) fn new(span_in_us: usize, span_in_ns: usize) -> Self {
        TimeDistributionStats {
            total_count: 0,
            span_in_us,
            span_in_ns,
            us_percentile: [0; MAX_ARRAY_SIZE],
            ns_percentile: [0; MAX_ARRAY_SIZE],
        }
    }

    pub(crate) fn update(&mut self, other: &TimeDistributionStats) {
        self.total_count = self.total_count.checked_add(other.total_count).expect("overflow");

        for index in 0..self.span_in_us {
            self.us_percentile[index] = self.us_percentile[index]
                .checked_add(other.us_percentile[index])
                .expect("overflow");
        }
        for index in 0..self.span_in_ns {
            self.ns_percentile[index] = self.ns_percentile[index]
                .checked_add(other.ns_percentile[index])
                .expect("overflow");
        }
    }

    pub(crate) fn record(&mut self, time_in_ns: f64) {
        self.total_count = self.total_count.checked_add(1).expect("overflow");
        // Record the time distribution at a subtle level.
        let mut index = (time_in_ns / (1000.0 * STEP_IN_US as f64)) as usize;
        if index > self.span_in_us - 1 {
            index = self.span_in_us - 1;
        }
        self.us_percentile[index] = self.us_percentile[index].checked_add(1).expect("overflow");

        // When the time is less than 4 us, record the distribution of time at the nanosecond level.
        if time_in_ns < (self.span_in_ns * STEP_IN_NS) as f64 {
            let index = (time_in_ns / STEP_IN_NS as f64) as usize;
            self.ns_percentile[index] = self.ns_percentile[index].checked_add(1).expect("overflow");
        }
    }
}
