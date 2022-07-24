use crate::constants::*;
use peerage_codecs::nibble_codec::NibbleCodec;
use peerage_utils::{bin_utils::{QuadrupleWord, Byte, Nibble, ByteWord, Bit}, traits::Key};
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

        let s_key = self.keyphrase.encode();
        
        let s_freq = self.encode_freqs();

        format!("__SEL__{s_self}__KEY__{s_key}__FRE__{s_freq}__")
    }

    fn encode_freqs(&self) -> String {
        let (nib_freq, 
            byt_freq,
            byw_freq,
            qyw_freq) = self.freq_counters;
        
        let mut s_1 = String::new();
        let mut s_2 = String::new();
        let mut s_3 = String::new();
        let mut s_4 = String::new();

        nib_freq
            .into_iter()
            .for_each(|x| { s_1 = format!("{s_1}.{x}") } );

        byt_freq
            .into_iter()
            .for_each(|x| { s_2 = format!("{s_2}.{x}") } );

        byw_freq
            .into_iter()
            .for_each(|x| { s_3 = format!("{s_3}.{x}") } );

        byw_freq
            .into_iter()
            .for_each(|x| { s_4 = format!("{s_4}.{x}") } );
    
        format!("//NIB//{s_1}//BYT//{s_2}//BYW//{s_3}//QYW//{s_4}//")
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

    fn from_encoded_inner(s_self: String, s_keyphrase: String, s_freq: String) -> Self {
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

    pub fn from_encoded(s: String) -> Self {
        let mut s_self = String::new();
        let mut s_keyphrase = String::new();
        let mut s_freq = String::new();

        let mut s_split = s.split("__");

        loop {
            match s_split.next() {
                Some(str) => {
                    match str {
                        "SEL" => {
                            let s_unwrap = s_split.next().unwrap();

                            s_self = s_unwrap.to_string();
                        },
                        "KEY" => {
                            let s_unwrap = s_split.next().unwrap();

                            s_keyphrase = s_unwrap.to_string();
                        },
                        "FRE" => {
                            let s_unwrap = s_split.next().unwrap();

                            s_freq = s_unwrap.to_string();
                        },
                        _ => continue,
                    }
                },
                None => break,
            }
        }

        Self::from_encoded_inner(s_self, s_keyphrase, s_freq)
    }

    fn freq_round_one_of_four_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 4 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (nib_freq, _, _, _) = self.freq_counters;
        
        let mut freq_iter = nib_freq.into_iter();

        let mut opped_vec: Vec<Nibble>= vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();
            
            for _ in 0..freq_iter.clone().count() {
                let curr_freq= freq_iter.next().unwrap() as u8;

                one = one + curr_freq;
                
            };

            let two = one / (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);


            let four = one + two + three;

            opped_vec.extend(vec!(one, two, three, four));

        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    
    }
    
}