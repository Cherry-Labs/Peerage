

pub trait Init {
    fn init() -> Self;
    fn mutate(&mut self, other: Self);
}