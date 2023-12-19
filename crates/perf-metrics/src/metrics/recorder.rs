use revm_utils::time_utils::{convert_cycles_to_ns_f64, instant::Instant};

type AddCountAndTimeFuntion = fn(u64, u64);
pub struct DBRecorder {
    pub(crate) fun: AddCountAndTimeFuntion,
    start: Instant,
}

impl DBRecorder {
    pub fn new(fun: AddCountAndTimeFuntion) -> Self {
        Self { fun, start: Instant::now() }
    }
}

impl Drop for DBRecorder {
    fn drop(&mut self) {
        let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
        (self.fun)(1, time_cycles);

        let time_ns = convert_cycles_to_ns_f64(time_cycles);
        crate::metrics::merkle_measure::db::record_distribution(time_ns);
    }
}

pub struct CountAndTimeRecorder {
    pub(crate) fun: AddCountAndTimeFuntion,
    start: Instant,
}

impl CountAndTimeRecorder {
    pub fn new(fun: AddCountAndTimeFuntion) -> Self {
        Self { fun, start: Instant::now() }
    }
}

impl Drop for CountAndTimeRecorder {
    fn drop(&mut self) {
        let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
        (self.fun)(1, time_cycles);
    }
}

type TimeFunction = fn(u64);
pub struct TimeRecorder {
    pub(crate) time_fun: TimeFunction,
    start: Instant,
}

impl TimeRecorder {
    pub fn new(time_fun: TimeFunction) -> Self {
        Self { time_fun, start: Instant::now() }
    }
}

impl Drop for TimeRecorder {
    fn drop(&mut self) {
        let time_cycles = Instant::now().checked_cycles_since(self.start).unwrap();
        (self.time_fun)(time_cycles);
    }
}

pub struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self { start: Instant::now() }
    }
}

impl Timer {
    pub fn cycles_since(&self) -> u64 {
        Instant::now().checked_cycles_since(self.start).unwrap()
    }
}
