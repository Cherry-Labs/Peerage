use crate::convert_utils::*;
use peerage_utils::bin_utils::*;
use crate::rounder::Rounder;
use crate::hash_bin::HashBin;

#[derive(Clone)]
pub struct PeerageHash {
    byte_chunk: [u8; 1024],
    hash_bin: HashBin,
    output_octuplet: [QuadrupleWord; 8],
    final_output: QuadrupleWord,
    output_bit_enum: Vec<Bit>,
    output_bit_digit: Vec<u8>,
    output_hex: String
}

impl PeerageHash {
    pub fn new(byte_chunk: [u8; 1024]) -> Self {
        let hash_bin = HashBin::from_1024bit_array(byte_chunk);
        let output_octuplet = [QuadrupleWord::new_zeros(); 8];
        let final_output = QuadrupleWord::new_zeros();

        let output_bit_enum = vec![Bit::Zero; 128];
        let output_bit_digit = vec![0u8; 128];
        let output_hex = String::new();


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

    fn do_all_the_rounds(&mut self) {
        for _ in 0..3 {
            for i in 0usize..8usize {
                self.output_octuplet[i] = self.hash_bin.do_one_round(i);
            
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

    fn set_outputs(&mut self) {
        self.output_bit_digit = self.final_output.into_num_bits();
        self.output_bit_enum = self.final_output.into_bits();
        self.output_hex = self.final_output.into_hex();
    }

    pub fn operate_rounds(&mut self) {
        self.do_all_the_rounds();
        self.add_octuplet();
        self.set_outputs();
    }

   pub fn get_bit_u8_output(&self) -> Vec<u8> {
        self.output_bit_digit.clone()
   }

   pub fn get_bit_enum_output(&self) -> Vec<Bit> {
        self.output_bit_enum.clone()
   }

    pub fn get_hex_hash_output(&self) -> String {
        self.output_hex.clone()
    }

}