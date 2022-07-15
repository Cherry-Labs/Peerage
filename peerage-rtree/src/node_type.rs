use peerage_utils::traits::Key;


#[derive(Clone, Copy)]
pub enum NodeType {
    Empty,
    LedgerNode(usize),
    StorageNode(usize),
    EncryptedNode(usize),

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
