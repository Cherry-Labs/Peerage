use crate::value_holder::KeyValueItem;
use peerage_utils::traits::*;


impl<'a, K: Key, V: Copy + Clone> Node for KeyValueItem<'a, K, V> {
    type InputType = i32;

    fn new() -> Self {
        todo!()
    }

    fn replace(&mut self, other: Self) {
        todo!()
    }

    fn is_equal_to(&self, other: Self::InputType) -> bool {
        todo!()
    }

    fn is_greater_to(&self, other: Self::InputType) -> bool {
        todo!()
    }

    fn is_lesser_to(&self, other: Self::InputType) -> bool {
        todo!()
    }

    fn add_to(&self, other: Self) -> Self {
        todo!()
    }

    fn mult_to(&self, other: Self) -> Self {
        todo!()
    }

    fn div_to(&self, other: Self) -> Self {
        todo!()
    }

    fn rem_to(&self, other: Self) -> Self {
        todo!()
    }

    fn sub_to(&self, other: Self) -> Self {
        todo!()
    }
}