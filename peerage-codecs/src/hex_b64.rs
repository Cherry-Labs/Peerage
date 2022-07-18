

#[derive(Clone, Copy, Default)]
pub struct HexBase64Pair;
use peerage_utils::bin_utils::Bit;

impl HexBase64Pair {
    pub fn from_string(_: String) -> Self {
        todo!()
    }

    pub fn from_bit_vec(_: Vec<Bit>) -> Self {
        todo!()
    }

    pub fn new_empty() -> Self {
        Self
    }

}