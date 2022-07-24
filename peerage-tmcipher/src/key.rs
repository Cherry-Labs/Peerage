use peerage_utils::bin_utils::{QuadrupleWord, ByteWord, Byte, Nibble};
use peerage_codecs::nibble_codec::NibbleCodec;


#[derive(Clone, Copy)]
pub struct KeyPhrase {
    quadruple_words: [QuadrupleWord; 32],
    byte_words: [ByteWord; 16],
    bytes: [Byte; 8],
    nibbles: [Nibble; 4],
}

impl KeyPhrase {
   pub fn new() -> Self {
        let rand_codec = NibbleCodec::new_random();

        let qw_rand = rand_codec.get_qw().into_vec();
        let bw_rand = rand_codec.get_bw().into_vec();
        let by_rand = rand_codec.get_by().into_vec();
        let ni_rand = rand_codec.get_ni().into_vec();


        let mut quadruple_words = [QuadrupleWord::default(); 32];
        let mut byte_words = [ByteWord::default(); 16];
        let mut bytes = [Byte::default(); 8];
        let mut nibbles = [Nibble::default(); 4];

        for i in 0usize..32usize {
            quadruple_words[i] = qw_rand[i];
        }

        for i in 0usize..16usize {
            byte_words[i] = bw_rand[i];
        }

        for i in 0usize..8usize {
            bytes[i] = by_rand[i];
        }

        for i in 0usize..4usize {
            nibbles[i] = ni_rand[i];
        }

        Self { 
            quadruple_words, 
            byte_words, 
            bytes, 
            nibbles
         }
    
   
   
    }    

    pub fn from_encoded(s: String) -> Self {
        let rand_codec = NibbleCodec::decodec(s);

        let qw_rand = rand_codec.get_qw().into_vec();
        let bw_rand = rand_codec.get_bw().into_vec();
        let by_rand = rand_codec.get_by().into_vec();
        let ni_rand = rand_codec.get_ni().into_vec();


        let mut quadruple_words = [QuadrupleWord::default(); 32];
        let mut byte_words = [ByteWord::default(); 16];
        let mut bytes = [Byte::default(); 8];
        let mut nibbles = [Nibble::default(); 4];

        for i in 0usize..32usize {
            quadruple_words[i] = qw_rand[i];
        }

        for i in 0usize..16usize {
            byte_words[i] = bw_rand[i];
        }

        for i in 0usize..8usize {
            bytes[i] = by_rand[i];
        }

        for i in 0usize..4usize {
            nibbles[i] = ni_rand[i];
        }

        Self { 
            quadruple_words, 
            byte_words, 
            bytes, 
            nibbles
         }
    
   
    }
}