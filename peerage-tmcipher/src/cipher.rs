use crate::constants::*;
use peerage_codecs::nibble_codec::NibbleCodec;
use peerage_utils::{bin_utils::{QuadrupleWord, Byte, Nibble, ByteWord}, traits::Key};
use crate::key::KeyPhrase;
use peerage_rand::rand::*;
use peerage_coll::collection::PeerageCollection;
use peerage_utils::bit_feq_table::*;

#[derive(Clone, Copy)]
pub struct Cipher {
    keyphrase: KeyPhrase,
    quadruple_words: [QuadrupleWord; 32],
    byte_words: [ByteWord; 16],
    bytes: [Byte; 8],
    nibbles: [Nibble; 4],
    freq_counters: (
        NibbleFreqTable,
        ByteFreqTable,
        ByteWordFreqTable,
        QuadrupleWordFreqTable
    )
}

impl Cipher {
    pub fn new(keyphrase: KeyPhrase) -> Self {
        let quadruple_words =  [RandomQuadrupleWord::rng_inner(); 32];
        let byte_words =  [RandomByteWord::rng_inner(); 16];
        let bytes =  [RandomByte::rng_inner(); 8];
        let nibbles =  [RandomNibble::rng_inner(); 4];
        let freq_counters = (
            NibbleFreqTable::new_random(),
            ByteFreqTable::new_random(),
            ByteWordFreqTable::new_random(),
            QuadrupleWordFreqTable::new_random(),
        );

        Self { 
            keyphrase,
            bytes,
            nibbles, 
            byte_words, 
            quadruple_words,
            freq_counters,
        }

    }


    pub fn encode(&self) -> String {
        let quadruple_words = PeerageCollection::from_vector(self.quadruple_words.to_vec());
        let byte_words = PeerageCollection::from_vector(self.byte_words.to_vec());
        let bytes = PeerageCollection::from_vector(self.bytes.to_vec());
        let nibbles = PeerageCollection::from_vector(self.nibbles.to_vec());
    
        
        let codec = NibbleCodec::new(quadruple_words, byte_words, bytes, nibbles);
        
        let s_self = codec.encode();

        let s_keyphrase = self.keyphrase
    }

    fn decode_freqs(s: String) -> (String, String, String, String) {
        // -> //NIB//12.13.12.12//BYT//23.22.
        
        let mut s_split = s.split("//");

        let mut s_1 = String::new();
        let mut s_2 = String::new();
        let mut s_3 = String::new();
        let mut s_4 = String::new();

        loop {
            match s_split.next() {
                Some(str) => {
                    match str {
                        "NIB" => s_1 = s_split.next().unwrap().to_string(),
                        "BYT" => s_2 = s_split.next().unwrap().to_string(),
                        "BYW" => s_3 = s_split.next().unwrap().to_string(),
                        "QYW" => s_4 = s_split.next().unwrap().to_string(),
                        _ => continue,
                    }
                },
                None => break,
            }
        }

        (s_1, s_2, s_3, s_4)
    }

    pub fn from_encoded(s_self: String, s_keyphrase: String, s_freq: String) -> Self {
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

        let (nib_freq, 
            byt_freq,
            byw_freq,
            qyw_freq) = Self::decode_freqs(s_freq);
        
        let freq_counters = (
            NibbleFreqTable::from_enc_str(nib_freq),
            ByteFreqTable::from_enc_str(byt_freq),
            ByteWordFreqTable::from_enc_str(byw_freq),
            QuadrupleWordFreqTable::from_enc_str(qyw_freq),
        );

        Self { 
            keyphrase,
            quadruple_words, 
            byte_words, 
            bytes, 
            nibbles,
            freq_counters,
         }
    }

    fn do_one_round_qw_encrypt(&mut self, qw: QuadrupleWord) -> QuadrupleWord {
        let mut res = qw.clone();

        for i in 0usize..32usize {
            self.freq_counters.3[i] = ((self.quadruple_words[i] % res) + i as u128).into_u128() as usize;
            res = self.quadruple_words[i] * qw_i;
            res = self.quadruple_words[i] + qw_i;
            res = self.quadruple_words[i] ^ qw_i;

        }


        for (j, k) in (0usize..128usize)
                    .into_iter()
                    .zip((0usize..128usize)
                    .rev().into_iter()) 
        {
            res = res - (self.freq_counters.3[j] * self.freq_counters.3[k]) as u128;
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

        for (j, k) in (0usize..128usize)
                    .into_iter()
                    .zip((0usize..128usize)
                    .rev().into_iter()) 
        {
            res = res + (self.freq_counters.3[j] / self.freq_counters.3[k]) as u128;
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