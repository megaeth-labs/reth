use reth_primitives::{BlockHash, BlockNumHash, BlockNumber};
use std::collections::BTreeMap;

/// Tracks non-finalized blocks of the canonical chain.
///
/// This is a wrapper around an ordered map of block numbers and hashes that belong to the
/// canonical chain but are not yet finalized.
#[derive(Debug, Clone, Default)]
pub(crate) struct CanonicalChain {
    /// Ordered mapping of canonical chain blocks by their block number.
    chain: BTreeMap<BlockNumber, BlockHash>,
}

impl CanonicalChain {
    /// Creates a new [`CanonicalChain`] with the given chain data.
    pub(crate) const fn new(chain: BTreeMap<BlockNumber, BlockHash>) -> Self {
        Self { chain }
    }

    /// Replaces the current chain with the given one.
    #[inline]
    pub(crate) fn replace(&mut self, chain: BTreeMap<BlockNumber, BlockHash>) {
        self.chain = chain;
    }

    /// Returns the block hash of the (non-finalized) canonical block with the given number.
    #[inline]
    pub(crate) fn canonical_hash(&self, number: &BlockNumber) -> Option<BlockHash> {
        self.chain.get(number).copied()
    }

    /// Returns the block number of the (non-finalized) canonical block with the given hash.
    #[inline]
    pub(crate) fn canonical_number(&self, block_hash: &BlockHash) -> Option<BlockNumber> {
        self.chain.iter().find_map(|(&number, &hash)| (hash == *block_hash).then_some(number))
    }

    /// Extends the chain with the given iterator of block number and hash pairs.
    #[inline]
    pub(crate) fn extend(&mut self, blocks: impl IntoIterator<Item = (BlockNumber, BlockHash)>) {
        self.chain.extend(blocks);
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub(crate) fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&BlockNumber, &mut BlockHash) -> bool,
    {
        self.chain.retain(f);
    }

    /// Returns a reference to the inner chain data.
    #[inline]
    pub(crate) const fn inner(&self) -> &BTreeMap<BlockNumber, BlockHash> {
        &self.chain
    }

    /// Returns the tip (latest block) of the canonical chain.
    #[inline]
    pub(crate) fn tip(&self) -> BlockNumHash {
        self.chain
            .last_key_value()
            .map(|(&number, &hash)| BlockNumHash { number, hash })
            .unwrap_or_default()
    }

    /// Returns an iterator over the chain.
    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (BlockNumber, BlockHash)> + '_ {
        self.chain.iter().map(|(&number, &hash)| (number, hash))
    }

    /// Consumes the structure and returns an iterator over the chain.
    #[inline]
    pub(crate) fn into_iter(self) -> impl Iterator<Item = (BlockNumber, BlockHash)> {
        self.chain.into_iter()
    }
}
