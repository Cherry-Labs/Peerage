use crate::constants::*;
use peerage_utils::{bin_utils::{QuadrupleWord, Byte, Nibble, ByteWord}, traits::Key};
use crate::key::KeyPhrase;
use peerage_rand::rand::*

#[derive(Clone, Copy)]
pub struct Cipher {
    keyphrase: KeyPhrase,
    bytes: [Byte; 8],
    nibbles: [Nibble; 8],
    byte_words: [ByteWord; 8],
    quadruple_words: [QuadrupleWord; 8]
}

impl Cipher {
    pub fn new(keyphrase: KeyPhrase) -> Self {
        let bytes = [RandomByte::rng_inner(); 8];
        let nibbles = [RandomNibble::rng_inner(); 8];
        let byte_words = [RandomByteWord::rng_inner(); 8];
        let quadruple_words = [RandomQuadrupleWord::rng_inner(); 8];

        Self { 
            keyphrase,
            bytes,
            nibbles, 
            byte_words, 
            quadruple_words 
        }

    }
}