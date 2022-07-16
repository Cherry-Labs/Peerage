use peerage_utils::traits::{Key as KeyTrait, Node};
use crate::main_keys::Key;

impl KeyTrait for Key {
    fn init_empty() -> Self {
        Key
    }

}


impl Node for Key {
    type InputType = Self;

    fn new() -> Self {
        todo!()
    }

    fn replace(&mut self, _other: Self) {
        todo!()
    }

    fn is_equal_to(&self, _other: Self::InputType) -> bool {
        todo!()
    }

    fn is_greater_to(&self, _other: Self::InputType) -> bool {
        todo!()
    }

    fn is_lesser_to(&self, _other: Self::InputType) -> bool {
        todo!()
    }

    fn add_to(&self, _other: Self) -> Self {
        todo!()
    }

    fn mult_to(&self, _other: Self) -> Self {
        todo!()
    }

    fn div_to(&self, _other: Self) -> Self {
        todo!()
    }

    fn rem_to(&self, _other: Self) -> Self {
        todo!()
    }

    fn sub_to(&self, _other: Self) -> Self {
        todo!()
    }
}