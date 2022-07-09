use std::hash::Hash;

use peerage_utils::bin_utils::*;
use crate::{rounder::Rounder, convert_utils::add_quadruple_words};

#[derive(Clone, Copy, Debug)]
pub struct HashBin {
    a: QuadrupleWord,
    b: QuadrupleWord,
    c: QuadrupleWord,
    d: QuadrupleWord,
    e: QuadrupleWord,
    f: QuadrupleWord,
    g: QuadrupleWord,
    h: QuadrupleWord,
    x_factor: QuadrupleWord,
}

impl HashBin {
    pub fn from_1024bit_array(arr: [u8; 1024]) -> Self {
        let a_arr = &arr[0..128].to_vec();
        let b_arr = &arr[128..256].to_vec();
        let c_arr = &arr[256..384].to_vec();
        let d_arr = &arr[384..512].to_vec();
        let e_arr = &arr[512..640].to_vec();
        let f_arr = &arr[640..768].to_vec();
        let g_arr = &arr[768..896].to_vec();
        let h_arr = &arr[896..1024].to_vec();


        let a = QuadrupleWord::from_128_u8s(a_arr.clone());
        let b = QuadrupleWord::from_128_u8s(b_arr.clone());
        let c = QuadrupleWord::from_128_u8s(c_arr.clone());
        let d = QuadrupleWord::from_128_u8s(d_arr.clone());
        let e = QuadrupleWord::from_128_u8s(e_arr.clone());
        let f = QuadrupleWord::from_128_u8s(f_arr.clone());
        let g = QuadrupleWord::from_128_u8s(g_arr.clone());
        let h = QuadrupleWord::from_128_u8s(h_arr.clone());

        let x_factor = QuadrupleWord::new_ones();
        
        Self { a, b, c, d, e, f, g, h, x_factor }
    
    }

    pub fn do_one_round(&mut self, i: usize) -> QuadrupleWord {
        let mut a_rounder = Rounder::new(
                        self.a,
                              self.x_factor
        );
        let mut b_rounder = Rounder::new(
                        self.b,
                        self.x_factor
        );
        let mut c_rounder = Rounder::new(
                        self.c,
                        self.x_factor
        );
        let mut d_rounder = Rounder::new(
                        self.d,
                        self.x_factor
        );
        let mut e_rounder = Rounder::new(
                       self.e,
                       self.x_factor
        );
        let mut f_rounder = Rounder::new(
                      self.f,
                      self.x_factor
        );
        let mut g_rounder = Rounder::new(
                       self.g,
                       self.x_factor
        );
        let mut h_rounder = Rounder::new(
                     self.h,
                            self.x_factor
        );

        if i == 0 {
            let a_res = a_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor + a_res;
        }

        if i == 1 {
            let b_res = b_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor + b_res;
        }

        if i == 2 {
            let c_res = c_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor % c_res;
        }

        if i == 3 {
            let d_res = d_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor % d_res;
        }
        
        if i == 4 {
            let e_res = e_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor * e_res;
        }

        if i == 5 {
            let f_res = f_rounder.operate_rounds_and_return_output();
        
            self.x_factor = self.x_factor * f_res;
        }
        
        if i == 6 {
            let g_res = g_rounder.operate_rounds_and_return_output();
           
            self.x_factor = self.x_factor / g_res;      
        }

        if i == 7 {
            let h_res = h_rounder.operate_rounds_and_return_output();
           
            self.x_factor = self.x_factor / h_res;      
        }
                       

        self.a + 
            self.b / 
            self.c % 
            self.d - 
            self.e *
            self.f /
            self.g +
            self.h
    }

    pub fn op_with_x_factor(&mut self, oper: QuadrupleWord, i: usize) -> QuadrupleWord {
        let res = match i {
            0 => self.x_factor + oper,
            2 => self.x_factor / oper,
            4 => self.x_factor % oper,
            6 => self.x_factor * oper,
            _ => panic!("Wrong index")
        };

        self.x_factor = res;

        res
    }
}