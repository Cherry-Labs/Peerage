use peerage_utils::traits::Ledger as LedgerTrait;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Ledger;


impl LedgerTrait for Ledger {}