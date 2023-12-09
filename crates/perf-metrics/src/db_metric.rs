use std::time::{Duration, Instant};

#[derive(Debug, Default)]
struct MetricRecoder {
    read_size: usize,
    read_time: Duration,
    write_size: usize,
    write_time: Duration,
}

impl MetricRecoder {
    pub fn reset(&mut self) {
        *self = Default::default()
    }

    pub fn add_read_record(&mut self, size: usize, time: Duration) {
        self.read_size = self.read_size.checked_add(size).expect("overflow");
        self.read_time = self.read_time.checked_add(time).expect("overflow");
    }

    pub fn add_write_record(&mut self, size: usize, time: Duration) {
        self.write_size = self.write_size.checked_add(size).expect("overflow");
        self.write_time = self.write_time.checked_add(time).expect("overflow");
    }

    pub fn get_record(&self) -> (usize, Duration, usize, Duration) {
        (self.read_size, self.read_time, self.write_size, self.write_time)
    }
}

static mut DB_METRIC_RECODER: Option<MetricRecoder> = None;

#[ctor::ctor]
fn init() {
    unsafe { DB_METRIC_RECODER = Some(MetricRecoder::default()) }
}

/// start db record
pub fn start_db_record() {
    unsafe {
        DB_METRIC_RECODER.as_mut().expect("Metric recorder should not empty!").reset();
    }
}

/// add db read recorcd
pub fn add_db_read_record(size: usize, time: Duration) {
    unsafe {
        DB_METRIC_RECODER
            .as_mut()
            .expect("Metric recorder should not empty!")
            .add_read_record(size, time);
    }
}

/// add db write record
pub fn add_db_write_record(size: usize, time: Duration) {
    unsafe {
        DB_METRIC_RECODER
            .as_mut()
            .expect("Metric recorder should not empty!")
            .add_write_record(size, time);
    }
}

/// get db record:
/// TODO: Should split to two functions: get_read_record and get write_record.
pub fn get_db_record() -> (usize, Duration, usize, Duration) {
    unsafe { DB_METRIC_RECODER.as_mut().expect("Metric recorder should not empty!").get_record() }
}

pub struct WriteRecord {
    size: usize,
    time: Instant,
}

impl WriteRecord {
    pub fn new(size: usize) -> Self {
        WriteRecord { size, time: Instant::now() }
    }
}

impl Drop for WriteRecord {
    fn drop(&mut self) {
        add_db_write_record(self.size, self.time.elapsed());
    }
}

pub struct ReadRecord {
    key_prev_base: *mut ::libc::c_void,
    key_ptr: *const ffi::MDBX_val,
    value_ptr: *const ffi::MDBX_val,
    time: Instant,
}

impl ReadRecord {
    pub fn new(
        key_prev_base: *mut ::libc::c_void,
        key_ptr: *const ffi::MDBX_val,
        value_ptr: *const ffi::MDBX_val,
    ) -> Self {
        ReadRecord { key_prev_base, key_ptr, value_ptr, time: Instant::now() }
    }
}

impl Drop for ReadRecord {
    fn drop(&mut self) {
        let (key_size, value_size) = unsafe {
            let key_size = if self.key_prev_base != (*self.key_ptr).iov_base {
                (*self.key_ptr).iov_len
            } else {
                0
            };

            (key_size, (*self.value_ptr).iov_len)
        };

        let size = key_size.checked_add(value_size).expect("overflow");
        if size != 0 {
            add_db_read_record(size, self.time.elapsed());
        }
    }
}

pub struct ReadValueRecord {
    data_ptr: *const ffi::MDBX_val,
    time: Instant,
}

impl ReadValueRecord {
    pub fn new(data_ptr: *const ffi::MDBX_val) -> Self {
        ReadValueRecord { data_ptr, time: Instant::now() }
    }
}

impl Drop for ReadValueRecord {
    fn drop(&mut self) {
        let size = unsafe { (*self.data_ptr).iov_len };

        if size != 0 {
            add_db_read_record(size, self.time.elapsed());
        }
    }
}
