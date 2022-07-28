use crate::constants::*;
use crate::key::KeyPhrase;
use crate::constants::CONST_PRIME;
use peerage_codecs::nibble_codec::NibbleCodec;
use peerage_coll::collection::PeerageCollection;
use peerage_rand::{rand::*, zero_eighty::rand_between_0_and_80};
use peerage_utils::bin_utils::*;
use peerage_utils::bit_feq_table::*;
use peerage_utils::traits::Key;



#[derive(Clone, Copy)]
pub struct Cipher {
    keyphrase: KeyPhrase,
    quadruple_words: [QuadrupleWord; 128],
    double_words: [DoubleWord; 64],
    byte_words: [ByteWord; 32],
    bytes: [Byte; 8],
    sessets: [Sesset; 6],
    nibbles: [Nibble; 4],
    rand_u64: [u64; 32],
    freq_counters: (
        NibbleFreqTable,
        SessetFreqTable,
        ByteFreqTable,
        ByteWordFreqTable,
        DoubleWordFreqTable,
        QuadrupleWordFreqTable,
    ),
}

impl Cipher {
    pub fn new(keyphrase: KeyPhrase) -> Self {
        let quadruple_words = [RandomQuadrupleWord::rng_inner(); 128];
        let double_words = [RandomDoubleWord::rng_inner(); 64];
        let byte_words = [RandomByteWord::rng_inner(); 32];
        let bytes = [RandomByte::rng_inner(); 8];
        let sessets = [RandomSesset::rng_inner(); 6];
        let nibbles = [RandomNibble::rng_inner(); 4];

        let freq_counters = (
            NibbleFreqTable::new_random(),
            SessetFreqTable::new_random(),
            ByteFreqTable::new_random(),
            ByteWordFreqTable::new_random(),
            DoubleWordFreqTable::new_random(),
            QuadrupleWordFreqTable::new_random(),
        );

        Self {
            keyphrase,
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles,
            freq_counters,
        }
    }

    fn get_rand_u64_array() -> [u64; 32] {
        let mut u64_nums = [0u64; 32];

        for i in 0usize..32usize {
            let rand_ind = rand_between_0_and_80();

            let rand_num_prime = CONST_PRIME[rand_ind];

            u64_nums[i] = rand_num_prime;
        }

        u64_nums
    } 


    pub fn encode(&self) -> String {
        let quadruple_words = PeerageCollection::from_vector(self.quadruple_words.to_vec());
        let double_words = PeerageCollection::from_vector(self.double_words.to_vec());
        let byte_words = PeerageCollection::from_vector(self.byte_words.to_vec());
        let bytes = PeerageCollection::from_vector(self.bytes.to_vec());
        let sessets = PeerageCollection::from_vector(self.sessets.to_vec());
        let nibbles = PeerageCollection::from_vector(self.nibbles.to_vec());

        let codec = NibbleCodec::new(
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles,
        );

        let s_self = codec.encode();

        let s_key = self.keyphrase.encode();

        let s_freq = self.encode_freqs();

        format!("__SEL__{s_self}__KEY__{s_key}__FRE__{s_freq}__")
    }

    fn encode_freqs(&self) -> String {
        let (nib_freq, ses_freq, byt_freq, byw_freq, dyw_freq, qyw_freq) = self.freq_counters;

        let mut s_1 = String::new();
        let mut s_2 = String::new();
        let mut s_3 = String::new();
        let mut s_4 = String::new();
        let mut s_5 = String::new();
        let mut s_6 = String::new();

        nib_freq
            .into_iter()
            .for_each(|x| s_1 = format!("{s_1}.{x}"));

        ses_freq
            .into_iter()
            .for_each(|x| s_2 = format!("{s_1}.{x}"));

        byt_freq
            .into_iter()
            .for_each(|x| s_3 = format!("{s_2}.{x}"));

        byw_freq
            .into_iter()
            .for_each(|x| s_4 = format!("{s_3}.{x}"));

        dyw_freq
            .into_iter()
            .for_each(|x| s_5 = format!("{s_3}.{x}"));

        qyw_freq
            .into_iter()
            .for_each(|x| s_6 = format!("{s_4}.{x}"));

        format!("//NIBF//{s_1}//SESF//{s_2}//BYTF//{s_3}//BYWF//{s_4}//DYWF//{s_5}//QYWF//{s_6}//")
    }

    fn decode_freqs(s: String) -> (String, String, String, String, String, String) {
        // -> //NIB//12.13.12.12//BYT//23.22.

        let mut s_split = s.split("//");

        let mut s_1 = String::new();
        let mut s_2 = String::new();
        let mut s_3 = String::new();
        let mut s_4 = String::new();
        let mut s_5 = String::new();
        let mut s_6 = String::new();

        loop {
            match s_split.next() {
                Some(str) => match str {
                    "NIBF" => s_1 = s_split.next().unwrap().to_string(),
                    "SESF" => s_2 = s_split.next().unwrap().to_string(),
                    "BYTF" => s_3 = s_split.next().unwrap().to_string(),
                    "BYWF" => s_4 = s_split.next().unwrap().to_string(),
                    "DYWF" => s_5 = s_split.next().unwrap().to_string(),
                    "QYWF" => s_6 = s_split.next().unwrap().to_string(),
                    _ => continue,
                },
                None => break,
            }
        }

        (s_1, s_2, s_3, s_4, s_5, s_6)
    }

    fn from_encoded_inner(s_self: String, s_keyphrase: String, s_freq: String) -> Self {
        let rand_codec = NibbleCodec::decodec(s_self);

        let qw_rand = rand_codec.get_qw().into_vec();
        let dw_rand = rand_codec.get_dw().into_vec();
        let bw_rand = rand_codec.get_bw().into_vec();
        let by_rand = rand_codec.get_by().into_vec();
        let se_rand = rand_codec.get_se().into_vec();
        let ni_rand = rand_codec.get_ni().into_vec();

        let mut quadruple_words = [QuadrupleWord::default(); 128];
        let mut double_words = [DoubleWord::default(); 64];
        let mut byte_words = [ByteWord::default(); 32];
        let mut bytes = [Byte::default(); 8];
        let mut sessets = [Sesset::default(); 6];
        let mut nibbles = [Nibble::default(); 4];

        for i in 0usize..128usize {
            quadruple_words[i] = qw_rand[i];
        }

        for i in 0usize..64usize {
            double_words[i] = dw_rand[i];
        }

        for i in 0usize..32usize {
            byte_words[i] = bw_rand[i];
        }

        for i in 0usize..8usize {
            bytes[i] = by_rand[i];
        }

        for i in 0usize..6usize {
            sessets[i] = se_rand[i];
        }

        for i in 0usize..4usize {
            nibbles[i] = ni_rand[i];
        }

        let keyphrase = KeyPhrase::from_encoded(s_keyphrase);

        let (nib_freq, ses_freq, byt_freq, byw_freq, dyw_freq, qyw_freq) =
            Self::decode_freqs(s_freq);

        let freq_counters = (
            NibbleFreqTable::from_enc_str(nib_freq),
            SessetFreqTable::from_enc_str(ses_freq),
            ByteFreqTable::from_enc_str(byt_freq),
            ByteWordFreqTable::from_enc_str(byw_freq),
            DoubleWordFreqTable::from_enc_str(dyw_freq),
            QuadrupleWordFreqTable::from_enc_str(qyw_freq),
        );

        Self {
            keyphrase,
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
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
                Some(str) => match str {
                    "SEL" => {
                        let s_unwrap = s_split.next().unwrap();

                        s_self = s_unwrap.to_string();
                    }
                    "KEY" => {
                        let s_unwrap = s_split.next().unwrap();

                        s_keyphrase = s_unwrap.to_string();
                    }
                    "FRE" => {
                        let s_unwrap = s_split.next().unwrap();

                        s_freq = s_unwrap.to_string();
                    }
                    _ => continue,
                },
                None => break,
            }
        }

        Self::from_encoded_inner(s_self, s_keyphrase, s_freq)
    }

    fn freq_round_one_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 4 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (nib_freq, _, _, _, _, _) = self.freq_counters;

        let mut freq_iter = nib_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one + curr_freq;
            }

            let two = one / (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);

            let four = one + two + three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_one_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 4 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (nib_freq, _, _, _, _, _) = self.freq_counters;

        let mut freq_iter = nib_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one - curr_freq;
            }

            let two = one * (freq_iter.get_quarters() as u8);
            let three = two / (freq_iter.get_three_fourths() as u8);

            let four = one - two - three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_two_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, ses_freq, _, _, _, _) = self.freq_counters;

        let mut freq_iter = ses_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one * curr_freq;
            }

            let two = one + (freq_iter.get_quarters() as u8);
            let three = two - (freq_iter.get_three_fourths() as u8);

            let four = one / two / three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_two_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, ses_freq, _, _, _, _) = self.freq_counters;

        let mut freq_iter = ses_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one / curr_freq;
            }

            let two = one - (freq_iter.get_quarters() as u8);
            let three = two + (freq_iter.get_three_fourths() as u8);

            let four = one * two * three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_three_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, byt_freq, _, _, _) = self.freq_counters;

        let mut freq_iter = byt_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one * curr_freq;
            }

            let two = one / (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);

            let four = one * two / three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_three_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, byt_freq, _, _, _) = self.freq_counters;

        let mut freq_iter = byt_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one / curr_freq;
            }

            let two = one * (freq_iter.get_quarters() as u8);
            let three = two / (freq_iter.get_three_fourths() as u8);

            let four = one / two * three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_four_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, byw_freq, _, _) = self.freq_counters;

        let mut freq_iter = byw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one + curr_freq;
            }

            let two = one * (freq_iter.get_quarters() as u8);
            let three = two / (freq_iter.get_three_fourths() as u8);

            let four = one * two / three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_four_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, byw_freq, _, _) = self.freq_counters;

        let mut freq_iter = byw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one - curr_freq;
            }

            let two = one / (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);

            let four = one / two * three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_five_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, _, dyw_freq, _) = self.freq_counters;

        let mut freq_iter = dyw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one * curr_freq;
            }

            let two = one / (freq_iter.get_quarters() as u8);
            let three = two / (freq_iter.get_three_fourths() as u8);

            let four = one * two * three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_five_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, _, dyw_freq, _) = self.freq_counters;

        let mut freq_iter = dyw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one / curr_freq;
            }

            let two = one * (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);

            let four = one / two / three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_six_of_six_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, _, _, qyw_freq) = self.freq_counters;

        let mut freq_iter = qyw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one / curr_freq;
            }

            let two = one - (freq_iter.get_quarters() as u8);
            let three = two * (freq_iter.get_three_fourths() as u8);

            let four = one + two - three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn freq_round_six_of_six_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let mut bits_clone = bits.clone();

        while bits_clone.len() % 8 != 0 {
            bits_clone.splice(0..0, vec![Bit::Zero]);
        }

        let (_, _, _, _, _, qyw_freq) = self.freq_counters;

        let mut freq_iter = qyw_freq.into_iter();

        let mut opped_vec: Vec<Nibble> = vec![];

        for i in (0usize..bits_clone.len()).step_by(4) {
            let v = bits_clone[i..i + 4].to_vec();

            let obj = Nibble::from_vec(v);

            let mut one = Nibble::new_zeros();

            for _ in 0..freq_iter.clone().count() {
                let curr_freq = freq_iter.next().unwrap() as u8;

                one = one * curr_freq;
            }

            let two = one + (freq_iter.get_quarters() as u8);
            let three = two / (freq_iter.get_three_fourths() as u8);

            let four = one - two + three;

            opped_vec.extend(vec![one, two, three, four]);
        }

        opped_vec
            .into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    fn key_qw_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let qw_self = self.quadruple_words.clone();
        let qw_key  = self.keyphrase.quadruple_words.clone();

        let mut qw_bits = QuadrupleWord::vec_self_from_vec_bit(bits.clone());

        for i in 0..qw_bits.len() {
            for j in 0..128 {
                qw_bits[i] = qw_bits[i] + qw_self[j] + qw_key[j];
                qw_bits[i] = qw_bits[i] * qw_self[j] * qw_key[j];
                qw_bits[i] = qw_bits[i] / qw_self[j] / qw_key[j];
                qw_bits[i] = qw_bits[i] - qw_self[j] - qw_key[j];
            }
        }

        QuadrupleWord::v_self_to_vec_bits(qw_bits)
    }

    fn key_qw_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let qw_self = self.quadruple_words.clone();
        let qw_key  = self.keyphrase.quadruple_words.clone();

        let mut qw_bits = QuadrupleWord::vec_self_from_vec_bit(bits.clone());

        for i in 0..qw_bits.len() {
            for j in 0..128 {
                qw_bits[i] = qw_bits[i] - qw_self[j] - qw_key[j];
                qw_bits[i] = qw_bits[i] / qw_self[j] / qw_key[j];
                qw_bits[i] = qw_bits[i] * qw_self[j] * qw_key[j];
                qw_bits[i] = qw_bits[i] + qw_self[j] + qw_key[j];
            }
        }

        QuadrupleWord::v_self_to_vec_bits(qw_bits)
    }

    fn key_dw_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let dw_self = self.double_words.clone();
        let dw_key  = self.keyphrase.double_words.clone();

        let mut dw_bits = DoubleWord::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..dw_bits.len() {
            for j in 0..64 {
                dw_bits[i] = dw_bits[i] + dw_self[j] - dw_key[j];
                dw_bits[i] = dw_bits[i] - dw_self[j] + dw_key[j];
                dw_bits[i] = dw_bits[i] * dw_self[j] / dw_key[j];
                dw_bits[i] = dw_bits[i] / dw_self[j] * dw_key[j];
            }
        }

        DoubleWord::v_self_to_vec_bits(dw_bits)
    }

    fn key_dw_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let dw_self = self.double_words.clone();
        let dw_key  = self.keyphrase.double_words.clone();

        let mut dw_bits = DoubleWord::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..dw_bits.len() {
            for j in 0..64 {
                dw_bits[i] = dw_bits[i] - dw_self[j] + dw_key[j];
                dw_bits[i] = dw_bits[i] + dw_self[j] - dw_key[j];
                dw_bits[i] = dw_bits[i] / dw_self[j] * dw_key[j];
                dw_bits[i] = dw_bits[i] * dw_self[j] / dw_key[j];
            }
        }

        DoubleWord::v_self_to_vec_bits(dw_bits)
    }

    fn key_bw_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let bw_self = self.byte_words.clone();
        let bw_key  = self.keyphrase.byte_words.clone();

        let mut bw_bits = ByteWord::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..bw_bits.len() {
            for j in 0..32 {
                bw_bits[i] = bw_bits[i] - bw_self[j] * bw_key[j];
                bw_bits[i] = bw_bits[i] + bw_self[j] - bw_key[j];
                bw_bits[i] = bw_bits[i] * bw_self[j] / bw_key[j];
                bw_bits[i] = bw_bits[i] / bw_self[j] * bw_key[j];
            }
        }

        ByteWord::v_self_to_vec_bits(bw_bits)
    }

    fn key_bw_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let bw_self = self.byte_words.clone();
        let bw_key  = self.keyphrase.byte_words.clone();

        let mut bw_bits = ByteWord::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..bw_bits.len() {
            for j in 0..32 {
                bw_bits[i] = bw_bits[i] + bw_self[j] / bw_key[j];
                bw_bits[i] = bw_bits[i] - bw_self[j] + bw_key[j];
                bw_bits[i] = bw_bits[i] / bw_self[j] * bw_key[j];
                bw_bits[i] = bw_bits[i] * bw_self[j] / bw_key[j];
            }
        }

        ByteWord::v_self_to_vec_bits(bw_bits)
    }

    fn key_by_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let by_self = self.bytes.clone();
        let by_key  = self.keyphrase.bytes.clone();

        let mut by_bits = Byte::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..by_bits.len() {
            for j in 0..8 {
                by_bits[i] = by_bits[i] - by_self[j] / by_key[j];
                by_bits[i] = by_bits[i] - by_self[j] / by_key[j];
                by_bits[i] = by_bits[i] * by_self[j] + by_key[j];
                by_bits[i] = by_bits[i] * by_self[j] + by_key[j];
            }
        }

        Byte::v_self_to_vec_bits(by_bits)
    }

    fn key_by_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let by_self = self.bytes.clone();
        let by_key  = self.keyphrase.bytes.clone();

        let mut by_bits = Byte::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..by_bits.len() {
            for j in 0..8 {
                by_bits[i] = by_bits[i] + by_self[j] * by_key[j];
                by_bits[i] = by_bits[i] + by_self[j] * by_key[j];
                by_bits[i] = by_bits[i] / by_self[j] - by_key[j];
                by_bits[i] = by_bits[i] / by_self[j] - by_key[j];
            }
        }

        Byte::v_self_to_vec_bits(by_bits)
    }

    fn key_se_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let se_self = self.sessets.clone();
        let se_key  = self.keyphrase.sessets.clone();

        let mut se_bits = Sesset::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..se_bits.len() {
            for j in 0..6 {
                se_bits[i] = se_bits[i] + se_self[j] / se_key[j];
                se_bits[i] = se_bits[i] - se_self[j] * se_key[j];
                se_bits[i] = se_bits[i] - se_self[j] * se_key[j];
                se_bits[i] = se_bits[i] + se_self[j] / se_key[j];
            }
        }

        Sesset::v_self_to_vec_bits(se_bits)
    }

    fn key_se_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let se_self = self.sessets.clone();
        let se_key  = self.keyphrase.sessets.clone();

        let mut se_bits = Sesset::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..se_bits.len() {
            for j in 0..6 {
                se_bits[i] = se_bits[i] - se_self[j] * se_key[j];
                se_bits[i] = se_bits[i] + se_self[j] / se_key[j];
                se_bits[i] = se_bits[i] + se_self[j] / se_key[j];
                se_bits[i] = se_bits[i] - se_self[j] * se_key[j];
            }
        }

        Sesset::v_self_to_vec_bits(se_bits)
    }

    fn key_ni_enc(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let ni_nilf = self.nibbles.clone();
        let ni_key  = self.keyphrase.nibbles.clone();

        let mut ni_bits = Nibble::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..ni_bits.len() {
            for j in 0..4 {
                ni_bits[i] = ni_bits[i] * ni_nilf[j] - ni_key[j];
                ni_bits[i] = ni_bits[i] * ni_nilf[j] - ni_key[j];
                ni_bits[i] = ni_bits[i] / ni_nilf[j] + ni_key[j];
                ni_bits[i] = ni_bits[i] / ni_nilf[j] + ni_key[j];
            }
        }

        Nibble::v_self_to_vec_bits(ni_bits)
    }

    fn key_ni_dec(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let ni_nilf = self.nibbles.clone();
        let ni_key  = self.keyphrase.nibbles.clone();

        let mut ni_bits = Nibble::vec_self_from_vec_bit(bits.clone());
        
        for i in 0..ni_bits.len() {
            for j in 0..4 {
                ni_bits[i] = ni_bits[i] / ni_nilf[j] + ni_key[j];
                ni_bits[i] = ni_bits[i] / ni_nilf[j] + ni_key[j];
                ni_bits[i] = ni_bits[i] * ni_nilf[j] - ni_key[j];
                ni_bits[i] = ni_bits[i] * ni_nilf[j] - ni_key[j];
            }
        }

        Nibble::v_self_to_vec_bits(ni_bits)
    }

    
    fn u64_round(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let rand_array = self.get_rand_u64_array();

        let mut bits_mut = bits.clone();

        while bits_mut.len() % 32 != 0 {

        }
    }

}
