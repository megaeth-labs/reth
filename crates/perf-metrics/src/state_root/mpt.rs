use alloy_trie::utils::TreeNode;

#[derive(Debug, Clone, Copy, Default)]
pub struct MPTRecord {
    root_node: TreeNode,
    update_branch_number: u64,
    delete_branch_number: u64,
}

impl MPTRecord {
    pub fn add(&mut self, other: MPTRecord) {
        self.root_node.add(&other.root_node);
        self.add_updates(other.delete_branch_number, other.update_branch_number);
    }

    pub fn add_node(&mut self, other: TreeNode) {
        self.root_node.add(&other);
    }

    pub fn add_updates(&mut self, delete_branch_nodes: u64, update_branch_nodes: u64) {
        self.delete_branch_number =
            self.delete_branch_number.checked_add(delete_branch_nodes).expect("overflow");
        self.update_branch_number =
            self.update_branch_number.checked_add(update_branch_nodes).expect("overflow");
    }
}
