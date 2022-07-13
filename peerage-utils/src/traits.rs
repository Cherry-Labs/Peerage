use std::marker::PhantomData;

pub trait Node {
    type InputType;

    fn new() -> Self;
    fn replace(&mut self, other: Self);
    fn into_self<C: Clone + Copy + Node, T: Clone + Copy + Node>(c: C, _: PhantomData<T>) -> T;
    fn is_equal_to(&self, other: Self::InputType) -> bool;
    fn is_greater_to(&self, other: Self::InputType) -> bool;
    fn is_lesser_to(&self, other: Self::InputType) -> bool;
    fn add_to(&self, other: Self) -> Self;
    fn mult_to(&self, other: Self) -> Self;
    fn div_to(&self, other: Self) -> Self;
    fn rem_to(&self, other: Self) -> Self;
    fn sub_to(&self, other: Self) -> Self;
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



pub trait CArray: Sync + Send {
    type ContainedType;

    fn get_at_index(&self, index: usize) -> Self::ContainedType;
    fn replace_at_index(&self, index: usize, value: Self::ContainedType);

}