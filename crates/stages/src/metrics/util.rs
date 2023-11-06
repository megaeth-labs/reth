#[cfg(feature = "enable_execution_duration_record")]
use minstant::Instant;
#[cfg(any(feature = "enable_execution_duration_record", feature = "enable_db_speed_record"))]
use std::time::Duration;

/// excution duration record
#[cfg(feature = "enable_execution_duration_record")]
#[derive(Debug, Clone, Copy)]
pub struct ExecutionDurationRecord {
    /// execute inner time recorder
    inner_recorder: Instant,
    /// time recorder
    time_recorder: Instant,

    /// tuple means:(counter of execute inner, total time of execute inner).
    pub execute_inner: (u64, Duration),
    /// tuple means:(counter, total time) of get block td and block_with_senders.
    pub read_block: (u64, Duration),
    /// tuple means:(counter, total time) of revm execute tx(execute_and_verify_receipt).
    pub execute_tx: (u64, Duration),
    /// tuple means:(counter, total time) of process state(state.extend)
    pub process_state: (u64, Duration),
    /// tuple means:(counter, total time) of write to db
    pub write_to_db: (u64, Duration),
}

#[cfg(feature = "enable_execution_duration_record")]
impl Default for ExecutionDurationRecord {
    fn default() -> Self {
        Self {
            inner_recorder: Instant::now(),
            time_recorder: Instant::now(),
            execute_inner: (0, Duration::default()),
            read_block: (0, Duration::default()),
            execute_tx: (0, Duration::default()),
            process_state: (0, Duration::default()),
            write_to_db: (0, Duration::default()),
        }
    }
}

#[cfg(feature = "enable_execution_duration_record")]
impl ExecutionDurationRecord {
    /// start inner time recorder
    pub(crate) fn start_inner_time_recorder(&mut self) {
        self.inner_recorder = Instant::now();
    }
    /// start time recorder
    pub(crate) fn start_time_recorder(&mut self) {
        self.time_recorder = Instant::now();
    }
    /// add time of execute_inner
    pub(crate) fn add_execute_inner(&mut self) {
        self.execute_inner.0 += 1;
        self.execute_inner.1 =
            self.execute_inner.1.checked_add(self.inner_recorder.elapsed()).expect("overflow");
    }
    /// add time of get block td and block_with_senders
    pub(crate) fn add_read_block(&mut self) {
        self.read_block.0 += 1;
        self.read_block.1 =
            self.read_block.1.checked_add(self.time_recorder.elapsed()).expect("overflow");
    }
    /// add time of revm execute tx
    pub(crate) fn add_execute_tx(&mut self) {
        self.execute_tx.0 += 1;
        self.execute_tx.1 =
            self.execute_tx.1.checked_add(self.time_recorder.elapsed()).expect("overflow");
    }
    /// add time of process state
    pub(crate) fn add_process_state(&mut self) {
        self.process_state.0 += 1;
        self.process_state.1 =
            self.process_state.1.checked_add(self.time_recorder.elapsed()).expect("overflow");
    }
    /// add time of write to db
    pub(crate) fn add_write_to_db(&mut self) {
        self.write_to_db.0 += 1;
        self.write_to_db.1 =
            self.write_to_db.1.checked_add(self.time_recorder.elapsed()).expect("overflow");
    }

    /// add
    pub fn add(&mut self, other: ExecutionDurationRecord) {
        self.execute_inner = (
            self.execute_inner.0 + other.execute_inner.0,
            self.execute_inner.1.checked_add(other.execute_inner.1).expect("overflow"),
        );
        self.read_block = (
            self.read_block.0 + other.read_block.0,
            self.read_block.1.checked_add(other.read_block.1).expect("overflow"),
        );
        self.execute_tx = (
            self.execute_tx.0 + other.execute_inner.0,
            self.execute_tx.1.checked_add(other.execute_tx.1).expect("overflow"),
        );
        self.process_state = (
            self.process_state.0 + other.process_state.0,
            self.process_state.1.checked_add(other.process_state.1).expect("overflow"),
        );
        self.write_to_db = (
            self.write_to_db.0 + other.write_to_db.0,
            self.write_to_db.1.checked_add(other.write_to_db.1).expect("overflow"),
        );
    }

    /// get pure execution duation record
    pub fn pure_record(&self) -> Self {
        const RDTSC_OVERHEAD: u64 = 7;

        let mut ret = ExecutionDurationRecord::default();

        let rdtsc_overhead: u64 = self.execute_inner.0 * RDTSC_OVERHEAD;
        if self.execute_inner.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.execute_inner =
            (self.execute_inner.0, self.execute_inner.1 - Duration::from_nanos(rdtsc_overhead));

        let rdtsc_overhead: u64 = self.read_block.0 * RDTSC_OVERHEAD;
        if self.read_block.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.read_block =
            (self.read_block.0, self.read_block.1 - Duration::from_nanos(rdtsc_overhead));

        let rdtsc_overhead: u64 = self.execute_tx.0 * RDTSC_OVERHEAD;
        if self.execute_tx.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.execute_tx =
            (self.execute_tx.0, self.execute_tx.1 - Duration::from_nanos(rdtsc_overhead));

        let rdtsc_overhead: u64 = self.process_state.0 * RDTSC_OVERHEAD;
        if self.process_state.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.process_state =
            (self.process_state.0, self.process_state.1 - Duration::from_nanos(rdtsc_overhead));

        let rdtsc_overhead: u64 = self.write_to_db.0 * RDTSC_OVERHEAD;
        if self.write_to_db.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.write_to_db =
            (self.write_to_db.0, self.write_to_db.1 - Duration::from_nanos(rdtsc_overhead));

        ret
    }

    /// print the information of the execution duration record.
    pub fn print(&self, header: &str) {
        println!();
        println!("{}", header);
        println!("Time Of Execute Inner(ns)    : {}", self.execute_inner.1.as_nanos());
        println!("Time Of Read Block(ns)       : {}", self.read_block.1.as_nanos());
        println!("Time Of Extend PostState(ns) : {}", self.process_state.1.as_nanos());
        println!(
            "Time Of Executes The Block And Checks Receipts (ns) : {}",
            self.execute_tx.1.as_nanos()
        );
        println!(
            "Time Of Write The Post State To The Database(ns)    : {}",
            self.write_to_db.1.as_nanos()
        );
        println!();
    }
}

/// db speed record
#[cfg(feature = "enable_db_speed_record")]
#[derive(Debug, Clone, Copy)]
pub struct DbSpeedRecord {
    /// time of read header td from db
    pub read_header_td_db_time: (u64, Duration),
    /// data size of read header td from db
    pub read_header_td_db_size: u64,
    /// time of read block with senders from db
    pub read_block_with_senders_db_time: (u64, Duration),
    /// data size of read block with senders from db
    pub read_block_with_senders_db_size: u64,
    /// time of write to db
    pub write_to_db_time: (u64, Duration),
    /// data size of write to db
    pub write_to_db_size: u64,
}

#[cfg(feature = "enable_db_speed_record")]
impl Default for DbSpeedRecord {
    fn default() -> Self {
        Self {
            read_header_td_db_time: (0, Duration::default()),
            read_header_td_db_size: 0,
            read_block_with_senders_db_time: (0, Duration::default()),
            read_block_with_senders_db_size: 0,
            write_to_db_time: (0, Duration::default()),
            write_to_db_size: 0,
        }
    }
}

#[cfg(feature = "enable_db_speed_record")]
impl DbSpeedRecord {
    /// add time of write to db
    pub(crate) fn add_read_header_td_db_time(&mut self, add_time: Duration, get_time_count: u64) {
        self.read_header_td_db_time.0 =
            self.read_header_td_db_time.0.checked_add(get_time_count).expect("overflow");
        self.read_header_td_db_time.1 =
            self.read_header_td_db_time.1.checked_add(add_time).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_read_header_td_db_size(&mut self, add_size: u64) {
        self.read_header_td_db_size =
            self.read_header_td_db_size.checked_add(add_size).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_read_block_with_senders_db_time(
        &mut self,
        add_time: Duration,
        get_time_count: u64,
    ) {
        self.read_block_with_senders_db_time.0 =
            self.read_block_with_senders_db_time.0.checked_add(get_time_count).expect("overflow");
        self.read_block_with_senders_db_time.1 =
            self.read_block_with_senders_db_time.1.checked_add(add_time).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_read_block_with_senders_db_size(&mut self, add_size: u64) {
        self.read_block_with_senders_db_size =
            self.read_block_with_senders_db_size.checked_add(add_size).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_write_to_db_time(&mut self, add_time: Duration, get_time_count: u64) {
        self.write_to_db_time.0 +=
            self.write_to_db_time.0.checked_add(get_time_count).expect("overflow");
        self.write_to_db_time.1 = self.write_to_db_time.1.checked_add(add_time).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn add_write_to_db_size(&mut self, add_size: u64) {
        self.write_to_db_size = self.write_to_db_size.checked_add(add_size).expect("overflow");
    }

    /// add
    pub fn add(&mut self, other: Self) {
        self.read_header_td_db_time = (
            self.read_header_td_db_time
                .0
                .checked_add(other.read_header_td_db_time.0)
                .expect("overflow"),
            self.read_header_td_db_time
                .1
                .checked_add(other.read_header_td_db_time.1)
                .expect("overflow"),
        );
        self.read_header_td_db_size = self
            .read_header_td_db_size
            .checked_add(other.read_header_td_db_size)
            .expect("overflow");

        self.read_block_with_senders_db_time = (
            self.read_block_with_senders_db_time
                .0
                .checked_add(other.read_block_with_senders_db_time.0)
                .expect("overflow"),
            self.read_block_with_senders_db_time
                .1
                .checked_add(other.read_block_with_senders_db_time.1)
                .expect("overflow"),
        );
        self.read_block_with_senders_db_size = self
            .read_block_with_senders_db_size
            .checked_add(other.read_block_with_senders_db_size)
            .expect("overflow");

        self.write_to_db_time = (
            self.write_to_db_time.0.checked_add(other.write_to_db_time.0).expect("overflow"),
            self.write_to_db_time.1.checked_add(other.write_to_db_time.1).expect("overflow"),
        );
        self.write_to_db_size =
            self.write_to_db_size.checked_add(other.write_to_db_size).expect("overflow");
    }

    /// get pure execution duation record
    pub fn pure_record(&self) -> Self {
        const RDTSC_OVERHEAD: u64 = 7;

        let mut ret = self.clone();

        let rdtsc_overhead: u64 = self.read_header_td_db_time.0 * RDTSC_OVERHEAD;
        if self.read_header_td_db_time.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.read_header_td_db_time.1 = self
            .read_header_td_db_time
            .1
            .checked_sub(Duration::from_nanos(rdtsc_overhead))
            .expect("overflow");

        let rdtsc_overhead: u64 = self.read_block_with_senders_db_time.0 * RDTSC_OVERHEAD;
        if self.read_block_with_senders_db_time.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.read_block_with_senders_db_time.1 = self
            .read_block_with_senders_db_time
            .1
            .checked_sub(Duration::from_nanos(rdtsc_overhead))
            .expect("overflow");

        let rdtsc_overhead: u64 = self.write_to_db_time.0 * RDTSC_OVERHEAD;
        if self.write_to_db_time.1.as_nanos() < rdtsc_overhead as u128 {
            panic!("rdtsc overhead too larget");
        }
        ret.write_to_db_time.1 = self
            .write_to_db_time
            .1
            .checked_sub(Duration::from_nanos(rdtsc_overhead))
            .expect("overflow");

        ret
    }

    fn cover_size_bytes_to_m(&self, bytes_size: u64) -> f64 {
        bytes_size as f64 / 1024.0 / 1024.0
    }

    /// print the information of db speed record.
    pub fn print(&self, header: &str) {
        println!();
        println!("{}", header);

        let precise = 3;

        let read_header_td_time = self.read_header_td_db_time.1.as_secs_f64();
        let read_header_td_size = self.cover_size_bytes_to_m(self.read_header_td_db_size);
        let read_header_td_rate = read_header_td_size / read_header_td_time;
        println!("Time Of Read Header TD(second)     : {:.precise$}", read_header_td_time);
        println!("DB Size Of Read Header TD(Mbytes)  : {:.precise$}", read_header_td_size);
        println!("Rate Of Read Header TD(M/s)        : {:.precise$}", read_header_td_rate);

        let read_block_with_senders_time = self.read_block_with_senders_db_time.1.as_secs_f64();
        let read_block_with_senders_size =
            self.cover_size_bytes_to_m(self.read_block_with_senders_db_size);
        let read_block_with_senders_rate =
            read_block_with_senders_size / read_block_with_senders_time;
        println!(
            "Time Of Read Block With Sender(second)   : {:.precise$}",
            read_block_with_senders_time
        );
        println!(
            "DB Size Of Read Block With Sender(Mbytes): {:.precise$}",
            read_block_with_senders_size
        );
        println!("Rate Of Read Block With Sender(M/s)      : {:.3}", read_block_with_senders_rate);

        let write_to_db_time = self.write_to_db_time.1.as_secs_f64();
        let write_to_db_size = self.cover_size_bytes_to_m(self.write_to_db_size);
        let write_to_db_rate = write_to_db_size / write_to_db_time;

        println!("Time Of Write To DB(second)         : {:.precise$}", write_to_db_time);
        println!("DB Size Of Write To DB(Mbytes)      : {:.precise$}", self.write_to_db_size);
        println!("Rate Of Write To DB(M/s)            : {:.precise$}", write_to_db_rate);

        println!();
    }
}
