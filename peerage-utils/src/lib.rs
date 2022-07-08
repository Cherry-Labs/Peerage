#![allow(unused)]
#[macro_use]
extern crate lazy_static;

pub mod bin_utils;
mod rng;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
