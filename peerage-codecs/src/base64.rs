use crate::constant_values::{CHAR_SESSET, SESSET_CHAR};
use peerage_utils::bin_utils::{Bit, Sesset};

pub struct Base64Encoder;

impl Base64Encoder {
    pub fn encode_bit(v: Vec<Bit>) -> String {
        let sessets = Sesset::vec_from_bits_vec(v);

        let mut ret = String::new();

        for sess in sessets {
            let ch = &SESSET_CHAR[&sess];
            ret = format!("{ret}{ch}")
        }

        while ret.len() % 64 != 0 {
            ret.push('=');
        }

        ret
    }

    pub fn encode_string(s: String) -> String {
        let v = Bit::vec_from_str(s);
        let sessets = Sesset::vec_from_bits_vec(v);

        let mut ret = String::new();

        for sess in sessets {
            let ch = &SESSET_CHAR[&sess];
            ret = format!("{ret}{ch}")
        }

        while ret.len() % 64 != 0 {
            ret.push('=');
        }

        ret
    }

    pub fn decode_sesset_to_bit(v: Vec<Sesset>) -> Vec<Bit> {
        let bits_vec = Sesset::vec_bits_from_vec_self(v);

        bits_vec
    }

    pub fn decode_str_to_bit(s: String) -> Vec<Bit> {
        let v = s.chars()
                                .map(|x| *&CHAR_SESSET[&x])
                                .collect::<Vec<Sesset>>();

        Self::decode_sesset_to_bit(v)
    }

    pub fn decode_sessets_to_str(v: Vec<Sesset>) -> String {
        let v = Self::decode_sesset_to_bit(v);

        Bit::vec_self_to_str(v)
    }

    pub fn decode_str_to_str(s: String) -> String {
        let v = Self::decode_str_to_bit(s);

        Bit::vec_self_to_str(v)
    }
}