use std::time::{Duration, Instant};

#[derive(Debug, Default)]
struct MetricRecoder {
    read_size: usize,
    read_time: Duration,
    write_size: usize,
    write_time: Duration,
}

static mut METRIC_RECORD: Option<MetricRecoder> = None;

/// start db record
pub fn start_db_record() {
    unsafe {
        METRIC_RECORD = Some(MetricRecoder::default());
    }
}

/// add db read recorcd
pub fn add_db_read_record(size: usize, time: Duration) {
    unsafe {
        if METRIC_RECORD.is_none() {
            return;
        }

        let record = METRIC_RECORD.as_mut().unwrap();
        record.read_size = record.read_size.checked_add(size).expect("overflow");
        record.read_time = record.read_time.checked_add(time).expect("overflow");
       
    }
}

/// add db write record
pub fn add_db_write_record(size: usize, time: Duration) {
    unsafe {
        if METRIC_RECORD.is_none() {
            return;
        }

        let record = METRIC_RECORD.as_mut().unwrap();
        record.write_size = record.write_size.checked_add(size).expect("overflow");
        record.write_time = record.write_time.checked_add(time).expect("overflow");
       
    }
}

/// get db record
pub fn get_db_record() -> (usize, Duration, usize, Duration) {
    unsafe {
        if METRIC_RECORD.is_none() {
            return (0, Duration::default(), 0, Duration::default())
        }

        let record = METRIC_RECORD.as_ref().unwrap();
        (record.read_size, record.read_time, record.write_size, record.write_time)
    }
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
