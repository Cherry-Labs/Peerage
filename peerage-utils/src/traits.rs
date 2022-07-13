use std::marker::PhantomData;

pub trait Node: Copy + Clone {
    type InputType;

    fn new() -> Self;
    fn replace(&mut self, other: Self);
    fn is_equal_to(&self, other: Self::InputType) -> bool;
    fn is_greater_to(&self, other: Self::InputType) -> bool;
    fn is_lesser_to(&self, other: Self::InputType) -> bool;
    fn add_to(&self, other: Self) -> Self;
    fn mult_to(&self, other: Self) -> Self;
    fn div_to(&self, other: Self) -> Self;
    fn rem_to(&self, other: Self) -> Self;
    fn sub_to(&self, other: Self) -> Self;
}

pub trait Key: Copy + Clone + Node {}


pub trait Value {}