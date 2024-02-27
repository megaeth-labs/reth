#[derive(Clone, Copy, Default, Debug)]
pub struct TryNextRecord {
    /// total execution count of try_next function
    pub(crate) total_count: u64,

    /// total execution time of try_next function
    pub(crate) total_time: u64,

    /// count of th branch node walker advance to but can skip.
    pub(crate) skip_branch_node_count: u64,

    /// count of boundary reading leaf
    pub(crate) leaf_miss_count: u64,

    /// count of work leaf
    pub(crate) leaf_hit_count: u64,
    // pub(crate) walk_next_unprocessed_key_count: u64,

    // pub(crate) walk_advance_count: u64,

    // pub(crate) loop_count: u64,
}
