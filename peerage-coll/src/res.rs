
pub enum SetResult {
    Success,
    Failure,
}

pub type SetResultType = std::result::Result<SetResult, SetResult>;
