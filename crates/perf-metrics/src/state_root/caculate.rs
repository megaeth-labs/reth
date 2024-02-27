use super::try_next::TryNextRecord;

#[derive(Clone, Copy, Default, Debug)]
pub struct CaculateRecord {
    pub(crate) total_time: u64,

    pub(crate) before_loop_time: u64,

    pub(crate) loop_begin_time: u64,

    pub(crate) try_next_stat: TryNextRecord,

    pub(crate) add_branch_count: u64,

    pub(crate) add_branch_time: u64,

    pub(crate) cal_storage_root_and_add_leaf_time: u64,

    pub(crate) after_cal_storage_root_time: u64,

    pub(crate) add_leaf_count: u64,

    pub(crate) add_leaf_time: u64,

    pub(crate) add_root_count: u64,

    pub(crate) add_root_time: u64,

    pub(crate) after_add_root_time: u64,
}
