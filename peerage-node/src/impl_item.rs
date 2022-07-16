use peerage_utils::traits::Node as NodeTrait;
use crate::node_item::Node;

impl NodeTrait for Node {
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