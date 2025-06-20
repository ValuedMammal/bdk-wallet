//! Module containing the locked outpoints change set.

use bdk_chain::Merge;
use bitcoin::OutPoint;
use serde::{Deserialize, Serialize};

use crate::collections::BTreeMap;

/// Represents changes to locked outpoints.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChangeSet {
    /// The lock status of an outpoint, `true == is_locked`.
    pub locked_outpoints: BTreeMap<OutPoint, bool>,
    /// The expiration height of the lock.
    pub expiration_heights: BTreeMap<OutPoint, Option<u32>>,
}

impl Merge for ChangeSet {
    fn merge(&mut self, other: Self) {
        self.locked_outpoints.extend(other.locked_outpoints);
        self.expiration_heights.extend(other.expiration_heights);
    }

    fn is_empty(&self) -> bool {
        self.locked_outpoints.is_empty() && self.expiration_heights.is_empty()
    }
}
