#![allow(unused, improper_ctypes_definitions)]
#[macro_use]
extern crate lazy_static;

pub mod bin_utils;
pub mod traits;
mod rng;
mod tty;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
