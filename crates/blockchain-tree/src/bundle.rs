//! [`ExecutionDataProvider`] implementations used by the tree.

use reth_primitives::{BlockHash, BlockNumber, ForkBlock};
use reth_provider::{BlockExecutionForkProvider, ExecutionDataProvider, ExecutionOutcome};
use std::collections::BTreeMap;

/// Structure that combines references to required data to be an [`ExecutionDataProvider`].
#[derive(Clone, Debug)]
pub struct BundleStateDataRef<'a> {
    /// The execution outcome after executing one or more transactions and/or blocks.
    pub execution_outcome: &'a ExecutionOutcome,
    /// Block hashes in the sidechain.
    pub sidechain_block_hashes: &'a BTreeMap<BlockNumber, BlockHash>,
    /// Block hashes in the canonical chain.
    pub canonical_block_hashes: &'a BTreeMap<BlockNumber, BlockHash>,
    /// The fork point in the canonical chain.
    pub canonical_fork: ForkBlock,
}

impl<'a> ExecutionDataProvider for BundleStateDataRef<'a> {
    fn execution_outcome(&self) -> &ExecutionOutcome {
        self.execution_outcome
    }

    fn block_hash(&self, block_number: BlockNumber) -> Option<BlockHash> {
        self.sidechain_block_hashes
            .get(&block_number)
            .copied()
            .or_else(|| self.canonical_block_hashes.get(&block_number).copied())
    }
}

impl<'a> BlockExecutionForkProvider for BundleStateDataRef<'a> {
    fn canonical_fork(&self) -> ForkBlock {
        self.canonical_fork
    }
}

/// Structure that owns the relevant data needed to be an [`ExecutionDataProvider`].
#[derive(Clone, Debug)]
pub struct ExecutionData {
    /// The execution outcome after executing transactions and/or blocks.
    pub execution_outcome: ExecutionOutcome,
    /// Parent block hashes needed for the EVM `BLOCKHASH` opcode.
    /// NOTE: Not all hashes may be present, but finalized ones are included.
    /// Other hashes can be obtained from the provider.
    pub parent_block_hashes: BTreeMap<BlockNumber, BlockHash>,
    /// The fork point in the canonical chain.
    pub canonical_fork: ForkBlock,
}

impl ExecutionDataProvider for ExecutionData {
    fn execution_outcome(&self) -> &ExecutionOutcome {
        &self.execution_outcome
    }

    fn block_hash(&self, block_number: BlockNumber) -> Option<BlockHash> {
        self.parent_block_hashes.get(&block_number).copied()
    }
}

impl BlockExecutionForkProvider for ExecutionData {
    fn canonical_fork(&self) -> ForkBlock {
        self.canonical_fork
    }
}
