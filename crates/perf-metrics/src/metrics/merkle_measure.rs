#[cfg(feature = "enable_state_root_record")]

pub mod common {
    static INIT: std::sync::Once = std::sync::Once::new();

    pub fn set_block_number(block_number: u64) {
        INIT.call_once(|| {
            crate::metrics::metric::recorder().state_root_record.start_block_number = block_number;
        });
        crate::metrics::metric::recorder().state_root_record.end_block_number = block_number;
    }

    add_function!(
        add_update_keys |
            [state_root_record | update_keys] |
            [account_trie_count | storage_trie_count]
    );
    add_function!(add_total_txs_count | [state_root_record] | [total_txs_count]);
    add_function!(
        add_construct_prefix_sets_time | [state_root_record] | [construct_prefix_sets_time]
    );
    add_function!(add_state_write_to_db_time | [state_root_record] | [state_write_to_db_time]);
    add_function!(add_hashed_state_write_time | [state_root_record] | [hashed_state_write_time]);
    add_function!(add_hash_state_slow_time | [state_root_record] | [hash_state_slow_time]);
    add_function!(add_flush_time | [state_root_record] | [flush_time]);
}
pub mod try_next {
    add_function!(
        add_state_count_and_time |
            [state_root_record | state_calculate_record | try_next_stat] |
            [total_count | total_time]
    );
    add_function!(
        add_state_skip_branch_node_count |
            [state_root_record | state_calculate_record | try_next_stat] |
            [skip_branch_node_count]
    );
    add_function!(
        add_state_leaf_miss_count |
            [state_root_record | state_calculate_record | try_next_stat] |
            [leaf_miss_count]
    );
    add_function!(
        add_state_leaf_hit_count |
            [state_root_record | state_calculate_record | try_next_stat] |
            [leaf_hit_count]
    );

    add_function!(
        add_storage_count_and_time |
            [state_root_record | storage_calculate_record | try_next_stat] |
            [total_count | total_time]
    );
    add_function!(
        add_storage_skip_branch_node_count |
            [state_root_record | storage_calculate_record | try_next_stat] |
            [skip_branch_node_count]
    );

    add_function!(
        add_storage_leaf_miss_count |
            [state_root_record | storage_calculate_record | try_next_stat] |
            [leaf_miss_count]
    );
    add_function!(
        add_storage_leaf_hit_count |
            [state_root_record | storage_calculate_record | try_next_stat] |
            [leaf_hit_count]
    );
}

pub mod caculate {
    add_function!(
        add_state_calculate_time | [state_root_record | state_calculate_record] | [total_time]
    );
    add_function!(
        add_state_before_loop_time |
            [state_root_record | state_calculate_record] |
            [before_loop_time]
    );

    add_function!(
        add_state_add_branch |
            [state_root_record | state_calculate_record] |
            [add_branch_count | add_branch_time]
    );

    add_function!(
        add_state_cal_storage_root_and_add_leaf_time |
            [state_root_record | state_calculate_record] |
            [cal_storage_root_and_add_leaf_time]
    );

    add_function!(
        add_state_after_cal_storage_root_time |
            [state_root_record | state_calculate_record] |
            [after_cal_storage_root_time]
    );

    add_function!(
        add_state_add_leaf |
            [state_root_record | state_calculate_record] |
            [add_leaf_count | add_leaf_time]
    );

    add_function!(
        add_state_add_root |
            [state_root_record | state_calculate_record] |
            [add_root_count | add_root_time]
    );
    add_function!(
        add_state_after_add_root_time |
            [state_root_record | state_calculate_record] |
            [after_add_root_time]
    );

    add_function!(
        add_storage_calculate_time | [state_root_record | storage_calculate_record] | [total_time]
    );
    add_function!(
        add_storage_before_loop_time |
            [state_root_record | storage_calculate_record] |
            [before_loop_time]
    );

    add_function!(
        add_storage_add_branch |
            [state_root_record | storage_calculate_record] |
            [add_branch_count | add_branch_time]
    );

    add_function!(
        add_storage_add_leaf |
            [state_root_record | storage_calculate_record] |
            [add_leaf_count | add_leaf_time]
    );

    add_function!(
        add_storage_add_root |
            [state_root_record | storage_calculate_record] |
            [add_root_count | add_root_time]
    );
    add_function!(
        add_storage_after_add_root_time |
            [state_root_record | storage_calculate_record] |
            [after_add_root_time]
    );
}

pub mod hash {
    pub fn add_keccak256(record: alloy_trie::Keccak256Record) {
        crate::metrics::metric::recorder().state_root_record.keccak256_record.add_other(record);
    }
}

pub mod mpt {
    pub fn add_state_mpt_info(tree_node: alloy_trie::TreeNode) {
        crate::metrics::metric::recorder().state_root_record.state_trie_info.add(&tree_node);
    }

    pub fn add_storage_mpt_info(tree_node: alloy_trie::TreeNode) {
        crate::metrics::metric::recorder().state_root_record.storage_trie_info.add(&tree_node);
    }

    add_function!(add_delete_branch_count | [state_root_record] | [delete_branch_count]);
}

pub mod db {
    add_function!(add_seek | [state_root_record | db_read] | [seek_count | seek_time]);
    add_function!(add_next | [state_root_record | db_read] | [next_count | next_time]);
    add_function!(
        add_seek_exact | [state_root_record | db_read] | [seek_exact_count | seek_exact_time]
    );
    add_function!(
        add_seek_by_sub_key |
            [state_root_record | db_read] |
            [seek_by_sub_key_count | seek_by_sub_key_time]
    );
    add_function!(
        add_next_dup_val | [state_root_record | db_read] | [next_dup_val_count | next_dup_val_time]
    );
    add_function!(add_at_seek | [state_root_record | db_read] | [at_seek_count | at_seek_time]);
    add_function!(
        add_at_seek_exact |
            [state_root_record | db_read] |
            [at_seek_exact_count | at_seek_exact_time]
    );
    add_function!(
        add_at_current | [state_root_record | db_read] | [at_current_count | at_current_time]
    );
    add_function!(
        add_st_seek_by_subkey |
            [state_root_record | db_read] |
            [st_seek_by_subkey_count | st_seek_by_subkey_time]
    );
    add_function!(
        add_st_current | [state_root_record | db_read] | [st_current_count | st_current_time]
    );

    pub fn record_distribution(time_in_ns: f64) {
        crate::metrics::metric::recorder()
            .state_root_record
            .db_read_distribution
            .record(time_in_ns);
    }
}
