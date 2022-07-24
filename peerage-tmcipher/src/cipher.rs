use crate::constants::*;
use peerage_codecs::nibble_codec::NibbleCodec;
use peerage_utils::{bin_utils::{QuadrupleWord, Byte, Nibble, ByteWord}, traits::Key};
use crate::key::KeyPhrase;
use peerage_rand::rand::*;
use peerage_coll::collection::PeerageCollection;

#[derive(Clone, Copy)]
pub struct Cipher {
    keyphrase: KeyPhrase,
    quadruple_words: [QuadrupleWord; 32],
    byte_words: [ByteWord; 16],
    bytes: [Byte; 8],
    nibbles: [Nibble; 4],
}

impl Cipher {
    pub fn new(keyphrase: KeyPhrase) -> Self {
        let quadruple_words =  [RandomQuadrupleWord::rng_inner(); 32];
        let byte_words =  [RandomByteWord::rng_inner(); 16];
        let bytes =  [RandomByte::rng_inner(); 8];
        let nibbles =  [RandomNibble::rng_inner(); 4];

        Self { 
            keyphrase,
            bytes,
            nibbles, 
            byte_words, 
            quadruple_words 
        }

    }


    pub fn encode(&self) -> String {
        let quadruple_words = PeerageCollection::from_vector(self.quadruple_words.to_vec());
        let byte_words = PeerageCollection::from_vector(self.byte_words.to_vec());
        let bytes = PeerageCollection::from_vector(self.bytes.to_vec());
        let nibbles = PeerageCollection::from_vector(self.nibbles.to_vec());
    
        
        let codec = NibbleCodec::new(quadruple_words, byte_words, bytes, nibbles);
        
        codec.encode()
    }

    pub fn from_encoded(s_self: String, s_keyphrase: String) -> Self {
        let rand_codec = NibbleCodec::decodec(s_self);

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

        let keyphrase = KeyPhrase::from_encoded(s_keyphrase);

        Self { 
            keyphrase,
            quadruple_words, 
            byte_words, 
            bytes, 
            nibbles
         }
    }

    fn do_one_round_qw_encrypt(&self, qw: QuadrupleWord) -> QuadrupleWord {
        let mut res = qw.clone();

        for i in 0usize..32usize {
            let qw_i = QuadrupleWord::from_usize(i);
            res = self.quadruple_words[i] * qw_i;
            res = self.quadruple_words[i] + qw_i;
            res = self.quadruple_words[i] ^ qw_i;
        }

        res
    }

    fn do_one_round_qw_decrypt(&self, qw: QuadrupleWord) -> QuadrupleWord {
        let mut res = qw.clone();

        for i in 0usize..32usize {
            let qw_i = QuadrupleWord::from_usize(i);
            res = self.quadruple_words[i] ^ qw_i;
            res = self.quadruple_words[i] - qw_i;
            res = self.quadruple_words[i] / qw_i;

        }

        res
    }

    fn do_one_round_bw_encrypt(&self, bw: ByteWord) -> ByteWord {
        let mut res = bw.clone();

        for i in 0u32..32u32 {
            let bw_i = ByteWord::from_u32(i);
            res = self.byte_words[i as usize] / bw_i;
            res = self.byte_words[i as usize] - bw_i;
            res = self.byte_words[i as usize] & bw_i;

        }

        res
    }
    
}