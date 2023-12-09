//! This module is used to record the amount and time of database read and
//! write data in execution stage.
use std::time::Duration;

/// This structure is used to record the amount and time of database read and
/// write data in function execute_inner. This structure records the information
/// of reading and writing to the database for three sub functions, namely header_td,
/// block_with_senders, and write_to_db. Function header_td and block_with_senders
/// are read database operations, while write_to_db is write database operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct DatabaseOperationRecord {
    /// Time of read header td from db.
    header_td_time: Duration,
    /// Data size of read header td from db.
    header_td_size: usize,
    /// Time of read block with senders from db.
    block_with_senders_time: Duration,
    /// Data size of read block with senders from db.
    block_with_senders_size: usize,
    /// Time of write to db.
    write_to_db_time: Duration,
    /// Data size of write to db.
    write_to_db_size: usize,
}

impl DatabaseOperationRecord {
    /// add record of read_header_td_db
    pub(crate) fn header_td_record(&mut self, size: usize, time: Duration) {
        self.header_td_size = self.header_td_size.checked_add(size).expect("overflow");
        self.header_td_time = self.header_td_time.checked_add(time).expect("overflow");
    }

    /// add time of write to db
    pub(crate) fn block_with_senders_record(&mut self, size: usize, time: Duration) {
        self.block_with_senders_size =
            self.block_with_senders_size.checked_add(size).expect("overflow");
        self.block_with_senders_time =
            self.block_with_senders_time.checked_add(time).expect("overflow");
    }

    /// add record of write to db
    pub(crate) fn write_to_db_record(&mut self, size: usize, time: Duration) {
        self.write_to_db_size = self.write_to_db_size.checked_add(size).expect("overflow");
        self.write_to_db_time = self.write_to_db_time.checked_add(time).expect("overflow");
    }
}

impl DatabaseOperationRecord {
    pub fn header_td_info(&self) -> (usize, Duration) {
        (self.header_td_size, self.header_td_time)
    }

    pub fn block_with_senders_info(&self) -> (usize, Duration) {
        (self.block_with_senders_size, self.block_with_senders_time)
    }

    pub fn write_to_db_info(&self) -> (usize, Duration) {
        (self.write_to_db_size, self.write_to_db_time)
    }
}
