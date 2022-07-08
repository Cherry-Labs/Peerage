use peerage_utils::bin_utils::*;
use crate::convert_utils::{set_at_index_odd_even, add_all_bws_together};
use crate::scooch_over;

#[derive(Clone)]
pub struct Rounder {
    upper_quadruple: QuadrupleWord,
    lower_quadruple: QuadrupleWord,
    output: ByteWord,
    input: ByteWord,
    x_factor: ByteWord,
    subround_results: Vec<ByteWord>
    
}


impl Rounder {
    pub fn new(input: ByteWord) -> Self {
        let (
            a, 
            b,
            c,
            d,
            e,
            f,
            g,
            h
        ) = (
            ByteWord::new_zeros(),
            ByteWord::new_ones(),
            ByteWord::new_zeros(),
            ByteWord::new_ones(),
            ByteWord::new_zeros(),
            ByteWord::new_ones(),
            ByteWord::new_zeros(),
            ByteWord::new_ones(),
        );

        let subround_results: Vec<ByteWord> = vec![];
        let output = ByteWord::new_zeros();

        let x_factor = input.clone();

        Self { a, b, c, d, e, f, g, h, output, input, x_factor, subround_results }
    }

    fn set_variables(&mut self,
        a: ByteWord,
        b: ByteWord,
        c: ByteWord,
        d: ByteWord,
        e: ByteWord,
        f: ByteWord,
        g: ByteWord,
        h: ByteWord
    )
    {
        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;
        self.e = e;
        self.f = f;
        self.g = g;
        self.h = h;
    }


    fn one_round(&mut self, index: ByteWord) {
        let (
            mut a,
            mut b,
            mut c,
            mut d,
            mut e,
            mut f,
            mut g,
            mut h
        ) = (
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.g,
            self.h,
        );


        let alpha = index ^ self.input + a;
        let beta = alpha + (a >> 12) + self.input ^ b;
        let tetha = (beta >> 4) | (beta << (32 - 4)) + c + index;
        let zeta = index + e + f + g;
        let iota = self.input ^ a + b;
        let sigma = index + self.input + h;

        let a_set = (alpha | beta) + self.x_factor;
        let b_set = (tetha ^ zeta) + self.x_factor;
        let c_set = (sigma & iota) + self.x_factor;
        let d_set = (beta & sigma) + self.x_factor;
        let e_set = (zeta ^ tetha) + self.x_factor;
        let f_set = (iota & tetha) + self.x_factor;
        let g_set = (alpha & iota) + self.x_factor;
        let h_set = (iota ^ sigma) + self.x_factor;
      
        self.set_variables(
            a_set, b_set, c_set, d_set, e_set, f_set, g_set, h_set
        );

        scooch_over!(h, a, b, c, d, e, f, g);

        let addition = (a_set + 
            b_set |
            c_set +
            d_set |
            e_set +
            f_set |
            g_set +
            h_set |
            self.x_factor
        );


        self.x_factor = (self.x_factor | index) + addition;

        self.subround_results.push(addition);

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