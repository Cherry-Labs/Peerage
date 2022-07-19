use crate::node::RTreeNode;
use peerage_keys::main_keys::Key;
use peerage_ledger::ledger::Ledger;
use peerage_node::node_item::Node;
use peerage_hashmap::hashmap::PeerageMap;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum NodeType {
    Empty,
    LedgerNode(usize),
    StorageNode(usize),
    EncryptedNode(usize),

}

impl Default for NodeType {
    fn default() -> Self {
        Self::Empty
    }
}

impl NodeType {
    pub fn new_ledger(u: usize) -> Self {
        Self::LedgerNode(u)
    }

    pub fn new_storage(u: usize) -> Self {
        Self::StorageNode(u)
    }

    pub fn new_encrypted(u: usize) -> Self {
        Self::EncryptedNode(u)
    }
}


pub enum SetResult {
    Success,
    Failure,
}

impl SetResult {
    pub fn is_success(&self) -> bool {
        match self {
            SetResult::Success => true,
            SetResult::Failure => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match self {
            SetResult::Success => false,
            SetResult::Failure => true,
        }
    }
}

impl SetResult {
    pub fn into_bool(&self) -> bool {
        match self {
            SetResult::Success => true,
            SetResult::Failure => false,
        }
    }
}

pub type KeySetRes = std::result::Result<SetResult, SetResult>;


pub enum InsertResult {
    Success,
    Failure,
}

impl InsertResult {
    pub fn is_success(&self) -> bool {
        match self {
            InsertResult::Success => true,
            InsertResult::Failure => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match self {
            InsertResult::Success => false,
            InsertResult::Failure => true,
        }
    }
}

impl InsertResult {
    pub fn into_bool(&self) -> bool {
        match self {
            InsertResult::Success => true,
            InsertResult::Failure => false,
        }
    }
}

pub type KeyInsertRes = std::result::Result<InsertResult, InsertResult>;



pub type Leaf<'a, K, V> = PeerageMap<'a, K, V>;