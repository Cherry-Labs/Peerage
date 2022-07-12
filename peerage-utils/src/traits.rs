use std::marker::PhantomData;

pub trait Node {
    type InputType;

    fn new() -> Self;
    fn replace(&mut self, other: Self);
    fn into_self<C: Clone + Copy + Node, T: Clone + Copy + Node>(c: C, _: PhantomData<T>) -> T;
    fn is_equal_to(&self, other: Self::InputType) -> bool;
    fn is_greater_to(&self, other: Self::InputType) -> bool;
    fn is_lesser_to(&self, other: Self::InputType) -> bool;

}

pub trait IndexerCompaerTo {
    type InputType;

    fn is_equal_to(&self, other: Self::InputType) -> bool;
    fn is_greater_to(&self, other: Self::InputType) -> bool;
    fn is_lesser_to(&self, other: Self::InputType) -> bool;

}

pub trait IndexerInto {
    type Into;

    fn into(&self) -> Self::Into;
}


pub trait Indexer: IndexerCompaerTo + IndexerInto {}