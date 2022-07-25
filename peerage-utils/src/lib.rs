#![allow(unused, improper_ctypes_definitions)]
#[macro_use]
extern crate lazy_static;

pub mod bin_utils;
pub mod traits;
mod rng;
mod tty;
pub mod bit_feq_table;
mod sesset;
mod bit;
mod nibble;
mod byte;
mod byte_word;
mod quadruple_word;
mod lazy;
mod double_word;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
