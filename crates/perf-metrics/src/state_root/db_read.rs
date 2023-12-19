#[derive(Clone, Copy, Default, Debug)]
pub struct DBReadRecord {
    pub(crate) current_count: u64,

    pub(crate) current_time: u64,

    pub(crate) seek_count: u64,

    pub(crate) seek_time: u64,

    pub(crate) next_count: u64,

    pub(crate) next_time: u64,

    pub(crate) seek_exact_count: u64,

    pub(crate) seek_exact_time: u64,

    pub(crate) seek_by_sub_key_count: u64,

    pub(crate) seek_by_sub_key_time: u64,

    pub(crate) next_dup_val_count: u64,

    pub(crate) next_dup_val_time: u64,

    pub(crate) at_seek_count: u64,

    pub(crate) at_seek_time: u64,

    pub(crate) at_seek_exact_count: u64,

    pub(crate) at_seek_exact_time: u64,

    pub(crate) at_current_count: u64,

    pub(crate) at_current_time: u64,

    pub(crate) st_seek_by_subkey_count: u64,

    pub(crate) st_seek_by_subkey_time: u64,

    pub(crate) st_current_count: u64,

    pub(crate) st_current_time: u64,
    // pub(crate) hash_account_cursor_seek_hit_count: u64,

    // pub(crate) hash_storage_cursor_seek_hit_count: u64,
}
