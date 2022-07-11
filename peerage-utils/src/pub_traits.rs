

pub trait Tree {
    type LedgerType;
    type InputType;
    

    fn init() -> Self;
    fn new(input: Self::InputType) -> Self;
    fn mutate(&mut self, other: Self);
    fn get_ledger_id(&self) -> Self::LedgerType;
    fn is_equal_to(&self, other: Self) -> bool;
    fn is_greater_than(&self, other: Self) -> bool;
    fn is_lesser_than(&self, other: Self) -> bool;

}