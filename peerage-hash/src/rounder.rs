use peerage_utils::bin_utils::*;
use crate::convert_utils::{set_at_index_odd_even, add_all_bws_together, rem_quadruple_words, add_quadruple_words, div_quadruple_words, multiply_quadruple_words};
use crate::scooch_over;

#[derive(Clone)]
pub struct Rounder {
    upper_quadruple: QuadrupleWord,
    lower_quadruple: QuadrupleWord,
    middle_quadruple: QuadrupleWord,
    output: QuadrupleWord,
    input: QuadrupleWord,
    input_factor: QuadrupleWord,
    subround_results: Vec<QuadrupleWord>,
    x_factor: QuadrupleWord,
    input_shuffle: QuadrupleWord,
    
}


impl Rounder {
    pub fn new(input: QuadrupleWord, x_factor: QuadrupleWord) -> Self {
        let upper_quadruple = QuadrupleWord::new_zeros_even();
        let lower_quadruple = QuadrupleWord::new_zeros_odd();
        let middle_quadruple = QuadrupleWord::new_ones();
        let output = QuadrupleWord::new_zeros();
        let subround_results = vec![QuadrupleWord::new_zeros(); 32];
        
        
        let mut input_shuffle = input.clone();

        input_shuffle.shuffle_manually(vec![2usize, 1usize, 3usize, 0usize]);
        
        let input_factor = input % input_shuffle;


        Self { 
            upper_quadruple, 
            lower_quadruple, 
            middle_quadruple, 
            output,
            input, 
            input_factor,
            subround_results,
            x_factor,
            input_shuffle
        }
    }

    fn one_round(&mut self) {
        let (mut a, mut b, mut c) = (
            self.lower_quadruple,
            self.middle_quadruple,
            self.upper_quadruple,
        );

        let mut alpha = self.x_factor >> 12;
        let mut beta = (self.input_shuffle >> 12) | (self.input_shuffle << (32 - 12) );
        let mut tetha = self.input % self.input_shuffle << 12;
        

        for i in 0usize..4usize {
            self.input_factor.shuffle_manually(vec![0, 2, 1, 3]);

            alpha = (self.x_factor >> 12) + a;
            beta = (self.input_shuffle >> 12) | (self.input_shuffle << (32 - 12) ) * b;
            tetha =  c - (self.input % self.input_shuffle << 12);

            if i % 2 == 0 {
                a = alpha * beta / tetha;
                b = alpha % beta + tetha;
                c = alpha - beta * tetha;
            } else {
                a.shuffle_manually(vec![0, 2, 3, 1]);
                b.shuffle_manually(vec![3, 2, 1, 0]);
                c.shuffle_manually(vec![1, 2, 1, 3]);
            }
            
            scooch_over!(b, c, a);

            let subround_result = self.upper_quadruple + self.middle_quadruple + self.lower_quadruple;

            self.subround_results.push(subround_result);

        }

        self.upper_quadruple = a;
        self.middle_quadruple = b;
        self.lower_quadruple = c;

    }

    fn set_output(&mut self) {
        self.output = self.subround_results[0] +
                    self.subround_results[1]   +
                    self.subround_results[2]   *
                    self.subround_results[3];  


        
        self.subround_results = vec![];


    }


    pub fn operate_rounds_and_return_output(&mut self) -> QuadrupleWord {
        self.one_round();

        self.set_output();

        self.output
    }
}