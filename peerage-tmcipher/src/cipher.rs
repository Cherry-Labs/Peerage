use crate::constants::*;
 use peerage_utils::bin_utils::QuadrupleWord;

pub struct Key {
    upper_bits: QuadrupleWord,
    mid_upper_bits: QuadrupleWord,
    md_lower_bits: QuadrupleWord,
    lower_bits: QuadrupleWord,
}

impl Key {
    pub fn new(
        upper_bits: QuadrupleWord,
        mid_upper_bits: QuadrupleWord,
        md_lower_bits: QuadrupleWord,
        lower_bits: QuadrupleWord,
    ) -> Self
    {
        Self { 
            upper_bits, 
            mid_upper_bits, 
            md_lower_bits, 
            lower_bits
         }
    }

    pub fn do_one_round(qw: QuadrupleWord) -> QuadrupleWord {
        qw 
    }
}


#[derive(Clone, Copy)]
pub struct Cipher {
    subkeys: [QuadrupleWord; 80],
    keys: [QuadrupleWord; 40]
}

