//! Stable node identifiers for document content.
//!
//! Each `Document` owns a monotone counter and hands out ids via
//! `alloc_id`. Ids are unique per document, not global, and the model
//! keeps no global or static state.

use std::fmt;

/// Stable identifier for any node in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u64);

impl NodeId {
    /// Wrap a raw counter value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Raw numeric value of the id.
    pub fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn equality_and_copy() {
        let a = NodeId::new(7);
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a.value(), 7);
        assert_ne!(a, NodeId::new(8));
    }

    #[test]
    fn display_roundtrips_number() {
        let id = NodeId::new(42);
        assert_eq!(id.to_string(), "42");
        let parsed: u64 = id.to_string().parse().expect("digits only");
        assert_eq!(NodeId::new(parsed), id);
    }

    #[test]
    fn usable_as_map_key() {
        let mut set = HashSet::new();
        set.insert(NodeId::new(1));
        set.insert(NodeId::new(1));
        set.insert(NodeId::new(2));
        assert_eq!(set.len(), 2);
    }
}
