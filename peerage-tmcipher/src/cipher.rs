use crate::constants::*;
use peerage_utils::{bin_utils::{QuadrupleWord, Byte, Nibble, ByteWord}, traits::Key};
use crate::key::KeyPhrase;


#[derive(Clone, Copy)]
pub struct Cipher {
    keyphrase: KeyPhrase,
    bytes: [Byte; 32],
    nibbles: [Nibble; 64],
    byte_words: [ByteWord; 16],
}

