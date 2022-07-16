
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