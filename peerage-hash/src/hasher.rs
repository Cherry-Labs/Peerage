use std::borrow::Borrow;

use crate::convert_utils::*;
use peerage_utils::bin_utils::*;
use crate::rounder::Rounder;
use crate::hash_bin::HashBin;
use peerage_coll::collection::PeerageCollection;
use peerage_codecs::hex_b64_pair::HexBase64Pair;

#[derive(Clone, Copy)]
pub struct PeerageHash {
    byte_chunk: [u8; 1024],
    hash_bin: HashBin,
    output_octuplet: [QuadrupleWord; 8],
    final_output: QuadrupleWord,
    output_bit_enum: PeerageCollection<Bit>,
    output_bit_digit: PeerageCollection<u8>,
    output_hex: HexBase64Pair,
}

impl PeerageHash {
    pub fn new(byte_chunk: [u8; 1024]) -> Self {
        let hash_bin = HashBin::from_1024bit_array(byte_chunk);
        let output_octuplet = [QuadrupleWord::new_zeros(); 8];
        let final_output = QuadrupleWord::new_zeros();

        let output_bit_enum = PeerageCollection::from_vector(vec![Bit::Zero; 128]);
        let output_bit_digit = PeerageCollection::from_vector(vec![0u8; 128]);
        let output_hex = HexBase64Pair::new_empty();


        Self { 
            byte_chunk, 
            hash_bin,
            output_octuplet,
            final_output,
            output_bit_enum,
            output_bit_digit,
            output_hex
        }
    }

    pub fn new_from_quadrupleworld(qw: QuadrupleWord) -> Self {
        let bits = qw.into_num_bits();

        let mut bits_array = [0u8; 1024];

        for i in 0..bits.len() {
            bits_array[i] = bits[i]
        }

        Self::new(bits_array)
    }

    fn do_all_the_rounds(&mut self) {
        for _ in 0..3 {
            for i in (0usize..8usize).step_by(2) {
                let res = self.hash_bin.do_one_round(i);
                let res_x_factor = self.hash_bin.op_with_x_factor(res, i);

                self.output_octuplet[i] = res;
                self.output_octuplet[i + 1] = res_x_factor;
            }
        }

        
    }

    fn add_octuplet(&mut self) {
        self.final_output = self.output_octuplet[0] +
                            self.output_octuplet[1] +
                            self.output_octuplet[2] +
                            self.output_octuplet[3] +
                            self.output_octuplet[4] +
                            self.output_octuplet[5] +
                            self.output_octuplet[6] +
                            self.output_octuplet[7]
    }

    pub fn get_final_output(&self) -> QuadrupleWord {
        self.final_output.clone()
    }

    fn set_outputs(&mut self) {
        self.output_bit_digit = PeerageCollection::from_vector(self.final_output.into_num_bits());
        self.output_bit_enum = PeerageCollection::from_vector(self.final_output.into_bits());
        let v = self.final_output.get_bits();
        
        self.output_hex = HexBase64Pair::from_bit_vec(v);
    }

    pub fn operate_rounds(&mut self) {
        self.do_all_the_rounds();
        self.add_octuplet();
        self.set_outputs();
    }

   pub fn get_bit_u8_output(&self) -> PeerageCollection<u8> {
        self.output_bit_digit.clone()
   }

   pub fn get_bit_enum_output(&self) -> PeerageCollection<Bit> {
        self.output_bit_enum.clone()
   }

    pub fn get_hex_hash_output(&self) -> HexBase64Pair {
        self.output_hex
    }

}


impl Default for PeerageHash {
    fn default() -> Self {
        Self::new([0u8; 1024])
    }
}

impl Eq for PeerageHash { }

impl PartialEq for PeerageHash {
    fn eq(&self, other: &Self) -> bool {
        self.final_output == other.final_output
    }
}