use derive_more::Deref;
use reth_primitives::{Account, Address, BlockNumber};
use std::collections::{btree_map::Entry, BTreeMap};

/// A mapping of `block -> address -> account` that represents what accounts were changed, and what
/// their state were prior to that change.
///
/// If the prior state was `None`, then the account is new.
#[derive(Default, Clone, Eq, PartialEq, Debug, Deref)]
pub struct AccountChanges {
    /// The inner mapping of block changes.
    #[deref]
    pub inner: BTreeMap<BlockNumber, BTreeMap<Address, Option<Account>>>,
    /// Hand tracked change size.
    pub size: usize,
}

impl AccountChanges {
    /// Insert account change at specified block number. The value is **not** updated if it already
    /// exists.
    pub fn insert(
        &mut self,
        block: BlockNumber,
        address: Address,
        old: Option<Account>,
        new: Option<Account>,
    ) {
        match self.inner.entry(block).or_default().entry(address) {
            Entry::Vacant(entry) => {
                self.size += 1;
                entry.insert(old);
            }
            Entry::Occupied(entry) => {
                // If the account state is the same before and after this block, collapse the state
                // changes.
                if entry.get() == &new {
                    entry.remove();
                    self.size -= 1;
                }
            }
        }
    }

    /// Insert account changes at specified block number. The values are **not** updated if they
    /// already exist.
    pub fn insert_for_block(
        &mut self,
        block: BlockNumber,
        changes: BTreeMap<Address, Option<Account>>,
    ) {
        let block_entry = self.inner.entry(block).or_default();
        for (address, account) in changes {
            if let Entry::Vacant(entry) = block_entry.entry(address) {
                entry.insert(account);
                self.size += 1;
            }
        }
    }

    /// Drain and return any entries above the target block number.
    pub fn drain_above(
        &mut self,
        target_block: BlockNumber,
    ) -> BTreeMap<BlockNumber, BTreeMap<Address, Option<Account>>> {
        let mut evicted = BTreeMap::new();
        self.inner.retain(|block_number, accounts| {
            if *block_number > target_block {
                self.size -= accounts.len();
                evicted.insert(*block_number, accounts.clone());
                false
            } else {
                true
            }
        });
        evicted
    }

    /// Retain entries only above specified block number.
    pub fn retain_above(&mut self, target_block: BlockNumber) {
        self.inner.retain(|block_number, accounts| {
            if *block_number > target_block {
                true
            } else {
                self.size -= accounts.len();
                false
            }
        });
    }

    /// Calculate size of the [AccountChanges].
    #[cfg(feature = "enable_db_speed_record")]
    pub fn size(&self) -> usize {
        self.inner
            .iter()
            .map(|(_, v)| {
                v.iter()
                    .map(|(_, v_in)| {
                        if v_in.is_some() {
                            std::mem::size_of::<Address>() + v_in.unwrap().size()
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
