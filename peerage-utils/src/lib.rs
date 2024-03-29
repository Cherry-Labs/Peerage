#![allow(unused, improper_ctypes_definitions)]
#[macro_use]
extern crate lazy_static;

mod binary;
pub mod bin_utils;
pub mod traits;
mod rng;
mod tty;
pub mod bit_feq_table;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
