use crate::convert_utils::*;
use peerage_utils::bin_utils::*;
use crate::rounder::Rounder;

#[derive(Clone)]
pub struct PeerageHash {
    byte_chunk: Vec<u8>,
    rounders: Vec<Rounder>,
    wb_octaplet: Vec<ByteWord>,
    rounder_outputs: Vec<ByteWord>,
    output_byte: ByteWord,
    output_hex: String
}

impl PeerageHash {
    pub fn new(byte_chunk: Vec<u8>) -> Self {
        let wb_octaplet = convert_256_byte_chunk_to_words(byte_chunk.clone());

        let rounders = wb_octaplet.clone()
                        .into_iter()
                        .map(|x| Rounder::new(x) )
                        .collect::<Vec<Rounder>>();

        let output_hex = String::new();
        
        let output_byte = ByteWord::new_zeros();

        let rounder_outputs = vec![ByteWord::new_zeros(); 8];

        Self { 
            byte_chunk, 
            rounders, 
            wb_octaplet, 
            rounder_outputs,            
            output_byte, 
            output_hex 
        }
    }

    fn do_all_the_rounds_and_output(&mut self) {
        self.rounders
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                let mut xx = x.clone();
                self.rounder_outputs[i] = xx.operate_rounds_and_return_output()
        });
    }

    fn xor_all_outputs_and_set_self_outputs(&mut self) {
        let all_xored = add_all_bws_together(self.rounder_outputs.clone());

        self.output_byte = all_xored;
        self.output_hex = all_xored.into_hex();
    }

    pub fn operate_rounds(&mut self) {
        self.do_all_the_rounds_and_output();
        self.xor_all_outputs_and_set_self_outputs();
    }

    pub fn get_byte_hash(&self) -> ByteWord {
        self.output_byte.clone()
    }

    pub fn get_hex_hash(&self) -> String {
        self.output_hex.clone()
    }

}