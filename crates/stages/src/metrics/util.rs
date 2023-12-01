// #[cfg(feature = "enable_execution_duration_record")]
// use minstant::Instant;
#[cfg(feature = "enable_db_speed_record")]
use std::time::Duration;

#[cfg(feature = "enable_execution_duration_record")]
use revm_utils::time_utils::{instant::Instant, *};

#[cfg(feature = "enable_execution_duration_record")]
pub(crate) const COL_WIDTH_MIDDLE: usize = 14;
#[cfg(feature = "enable_execution_duration_record")]
pub(crate) const COL_WIDTH_BIG: usize = 18;

/// excution duration record
#[cfg(feature = "enable_execution_duration_record")]
#[derive(Debug, Clone, Copy)]
pub struct ExecutionDurationRecord {
    /// execute inner time recorder
    inner_start_time: Instant,
    /// time recorder
    start_time: Instant,

    /// total time of execute inner.
    pub execute_inner_time: u64,
    /// total time of get block td and block_with_senders.
    pub fetching_block_time: u64,
    /// total time of revm execute tx(execute_and_verify_receipt).
    pub execution_time: u64,
    /// total time of process state(state.extend)
    pub process_state_time: u64,
    /// total time of write to db
    pub write_to_db_time: u64,
}

#[cfg(feature = "enable_execution_duration_record")]
impl Default for ExecutionDurationRecord {
    fn default() -> Self {
        Self {
            inner_start_time: Instant::default(),
            start_time: Instant::default(),
            execute_inner_time: 0,
            fetching_block_time: 0,
            execution_time: 0,
            process_state_time: 0,
            write_to_db_time: 0,
        }
    }
}

#[cfg(feature = "enable_execution_duration_record")]
impl ExecutionDurationRecord {
    const NSSECONDS_ONE_HOUR: f64 = 60.0 * 60.0 * 1000_000_000.0;

    /// start inner time recorder
    pub(crate) fn start_inner_time_recorder(&mut self) {
        self.inner_start_time = Instant::now();
    }
    /// start time recorder
    pub(crate) fn start_time_recorder(&mut self) {
        self.start_time = Instant::now();
    }
    /// add time of execute_inner
    pub(crate) fn add_execute_inner_duration(&mut self) {
        let delta = Instant::now().checked_cycles_since(self.inner_start_time).expect("overflow");
        self.execute_inner_time = self.execute_inner_time.checked_add(delta).expect("overflow");
    }
    /// add time of get block td and block_with_senders
    pub(crate) fn add_read_block_duration(&mut self) {
        let delta = Instant::now().checked_cycles_since(self.start_time).expect("overflow");
        self.fetching_block_time = self.fetching_block_time.checked_add(delta).expect("overflow");
    }
    /// add time of revm execute tx
    pub(crate) fn add_execute_tx_duration(&mut self) {
        let delta = Instant::now().checked_cycles_since(self.start_time).expect("overflow");
        self.execution_time = self.execution_time.checked_add(delta).expect("overflow");
    }
    /// add time of process state
    pub(crate) fn add_process_state_duration(&mut self) {
        let delta = Instant::now().checked_cycles_since(self.start_time).expect("overflow");
        self.process_state_time = self.process_state_time.checked_add(delta).expect("overflow");
    }
    /// add time of write to db
    pub(crate) fn add_write_to_db_duration(&mut self) {
        let delta = Instant::now().checked_cycles_since(self.start_time).expect("overflow");
        self.write_to_db_time = self.write_to_db_time.checked_add(delta).expect("overflow");
    }
    /// add
    pub fn add(&mut self, other: ExecutionDurationRecord) {
        self.execute_inner_time =
            self.execute_inner_time.checked_add(other.execute_inner_time).expect("overflow");
        self.fetching_block_time =
            self.fetching_block_time.checked_add(other.fetching_block_time).expect("overflow");
        self.execution_time =
            self.execution_time.checked_add(other.execution_time).expect("overflow");
        self.process_state_time =
            self.process_state_time.checked_add(other.process_state_time).expect("overflow");
        self.write_to_db_time =
            self.write_to_db_time.checked_add(other.write_to_db_time).expect("overflow");
    }

    fn execute_inner_time(&self) -> f64 {
        convert_cycles_to_ns_f64(self.execute_inner_time) / Self::NSSECONDS_ONE_HOUR
    }

    fn fetching_block_time(&self) -> f64 {
        convert_cycles_to_ns_f64(self.fetching_block_time) / Self::NSSECONDS_ONE_HOUR
    }

    fn fetching_block_time_percent(&self) -> f64 {
        self.fetching_block_time() * 100.0 / self.execute_inner_time()
    }

    fn execution_time(&self) -> f64 {
        convert_cycles_to_ns_f64(self.execution_time) / Self::NSSECONDS_ONE_HOUR
    }

    fn execution_time_percent(&self) -> f64 {
        self.execution_time() * 100.0 / self.execute_inner_time()
    }

    fn process_state_time(&self) -> f64 {
        convert_cycles_to_ns_f64(self.process_state_time) / Self::NSSECONDS_ONE_HOUR
    }

    fn process_state_time_percent(&self) -> f64 {
        self.process_state_time() * 100.0 / self.execute_inner_time()
    }

    fn write_to_db_time(&self) -> f64 {
        convert_cycles_to_ns_f64(self.write_to_db_time) / Self::NSSECONDS_ONE_HOUR
    }

    fn write_to_db_time_percent(&self) -> f64 {
        self.write_to_db_time() * 100.0 / self.execute_inner_time()
    }

    fn misc_time(&self) -> f64 {
        self.execute_inner_time() -
            self.fetching_block_time() -
            self.execution_time() -
            self.process_state_time() -
            self.write_to_db_time()
    }

    fn misc_time_percent(&self) -> f64 {
        self.misc_time() * 100.0 / self.execute_inner_time()
    }

    fn print_line(&self, cat: &str, time: f64, time_percent: f64) {
        println!(
            "{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$.3}{: >COL_WIDTH_MIDDLE$.2}",
            cat, time, time_percent,
        );
    }

    /// print the information of the execution duration record.
    pub fn print(&self) {
        println!();
        println!("===============================Metric of execution duration==========================================================");
        println!(
            "{: <COL_WIDTH_BIG$}{: >COL_WIDTH_MIDDLE$}{: >COL_WIDTH_MIDDLE$}",
            "Cat.", "Time (h)", "Time (%)",
        );

        self.print_line("total", self.execute_inner_time(), 100.0);
        self.print_line("misc", self.misc_time(), self.misc_time_percent());
        self.print_line(
            "fetching_blocks",
            self.fetching_block_time(),
            self.fetching_block_time_percent(),
        );
        self.print_line("execution", self.execution_time(), self.execution_time_percent());
        self.print_line(
            "process_state",
            self.process_state_time(),
            self.process_state_time_percent(),
        );
        self.print_line("write_to_db", self.write_to_db_time(), self.write_to_db_time_percent());

        println!();
    }
}

/// db speed record
#[cfg(feature = "enable_db_speed_record")]
#[derive(Debug, Clone, Copy)]
pub struct DbSpeedRecord {
    /// time of read header td from db
    pub read_header_td_db_time: Duration,
    /// data size of read header td from db
    pub read_header_td_db_size: u64,
    /// time of read block with senders from db
    pub read_block_with_senders_db_time: Duration,
    /// data size of read block with senders from db
    pub read_block_with_senders_db_size: u64,
    /// time of write to db
    pub write_to_db_time: Duration,
    /// data size of write to db
    pub write_to_db_size: u64,
}

#[cfg(feature = "enable_db_speed_record")]
impl Default for DbSpeedRecord {
    fn default() -> Self {
        Self {
            read_header_td_db_time: Duration::default(),
            read_header_td_db_size: 0,
            read_block_with_senders_db_time: Duration::default(),
            read_block_with_senders_db_size: 0,
            write_to_db_time: Duration::default(),
            write_to_db_size: 0,
        }
    }
}

#[cfg(feature = "enable_db_speed_record")]
impl DbSpeedRecord {
    /// add record of read_header_td_db
    pub(crate) fn add_read_header_td_db_record(&mut self, size: u64, time: Duration) {
        self.read_header_td_db_size =
            self.read_header_td_db_size.checked_add(size).expect("overflow");
        self.read_header_td_db_time =
            self.read_header_td_db_time.checked_add(time).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_read_block_with_senders_db_record(&mut self, size: u64, time: Duration) {
        self.read_block_with_senders_db_size =
            self.read_block_with_senders_db_size.checked_add(size).expect("overflow");
        self.read_block_with_senders_db_time =
            self.read_block_with_senders_db_time.checked_add(time).expect("overflow");
    }

    /// add record of write to db
    pub(crate) fn add_write_to_db_record(&mut self, size: u64, time: Duration) {
        self.write_to_db_size = self.write_to_db_size.checked_add(size).expect("overflow");
        self.write_to_db_time = self.write_to_db_time.checked_add(time).expect("overflow");
    }

    /// add
    pub fn add(&mut self, other: Self) {
        self.read_header_td_db_time = self
            .read_header_td_db_time
            .checked_add(other.read_header_td_db_time)
            .expect("overflow");
        self.read_header_td_db_size = self
            .read_header_td_db_size
            .checked_add(other.read_header_td_db_size)
            .expect("overflow");

        self.read_block_with_senders_db_time = self
            .read_block_with_senders_db_time
            .checked_add(other.read_block_with_senders_db_time)
            .expect("overflow");
        self.read_block_with_senders_db_size = self
            .read_block_with_senders_db_size
            .checked_add(other.read_block_with_senders_db_size)
            .expect("overflow");

        self.write_to_db_time =
            self.write_to_db_time.checked_add(other.write_to_db_time).expect("overflow");
        self.write_to_db_size =
            self.write_to_db_size.checked_add(other.write_to_db_size).expect("overflow");
    }

    fn cover_size_bytes_to_m(&self, bytes_size: u64) -> f64 {
        bytes_size as f64 / 1024.0 / 1024.0
    }

    /// print the information of db speed record.
    pub fn print(&self, header: &str) {
        println!();
        println!("{}", header);

        let col_len = 15;

        let read_header_td_time = self.read_header_td_db_time.as_secs_f64();
        let read_header_td_size = self.cover_size_bytes_to_m(self.read_header_td_db_size);
        let read_header_td_rate = read_header_td_size / read_header_td_time;

        let read_block_with_senders_time = self.read_block_with_senders_db_time.as_secs_f64();
        let read_block_with_senders_size =
            self.cover_size_bytes_to_m(self.read_block_with_senders_db_size);
        let read_block_with_senders_rate =
            read_block_with_senders_size / read_block_with_senders_time;

        let write_to_db_time = self.write_to_db_time.as_secs_f64();
        let write_to_db_size = self.cover_size_bytes_to_m(self.write_to_db_size);
        let write_to_db_rate = write_to_db_size / write_to_db_time;

        println!("Cat.                           Size (MBytes)   Time (s)   Rate (MBytes/s)");
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "Read header td         ",
        read_header_td_size, read_header_td_time, read_header_td_rate};
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "Read header with sender",
        read_block_with_senders_size, read_block_with_senders_time, read_block_with_senders_rate};
        println! {"{:col_len$}{:>col_len$.3}{:>col_len$.3}{:>col_len$.3}", "Write to db            ",
        write_to_db_size, write_to_db_time, write_to_db_rate};

        println!();
    }
}
