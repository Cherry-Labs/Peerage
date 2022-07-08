use peerage_utils::bin_utils::*;
use crate::convert_utils::{set_at_index_odd_even, add_all_bws_together};
use crate::scooch_over;

#[derive(Clone)]
pub struct Rounder {
    upper_quadruple: QuadrupleWord,
    lower_quadruple: QuadrupleWord,
    middle_quadruple: QuadrupleWord,
    output: QuadrupleWord,
    input: ByteWord,
    x_factor: ByteWord,
    input_factor: QuadrupleWord,
    subround_results: Vec<QuadrupleWord>
    
}


impl Rounder {
    pub fn new(input: ByteWord) -> Self {
        let upper_quadruple = QuadrupleWord::new_zeros_even();
        let lower_quadruple = QuadrupleWord::new_zeros_odd();
        let middle_quadruple = QuadrupleWord::new_ones();
        let output = QuadrupleWord::new_zeros();
        let subround_results: Vec<QuadrupleWord> = vec![];
        let x_factor = input.clone();


        Self { 
            upper_quadruple, 
            lower_quadruple, 
            middle_quadruple, 
            output,
            input, 
            x_factor,
            subround_results
        }
    }

    fn one_round(&mut self) {
        for _ in 0..24 {




            scooch_over!(self.lower_quadruple, self.upper_quadruple, self.middle_quadruple)
        }

    }

    pub fn operate_rounds_and_return_output(&mut self) -> ByteWord {
        for i in 0usize..32usize {
            let index = ByteWord::from_4_integers(vec![0, 0, 0, i as u8]);
            self.one_round(index);
        }

        self.output = add_all_bws_together(self.subround_results.clone());

        self.output
    }
}