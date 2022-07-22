#![allow(unused)]

pub mod hex_b64;
pub mod passphrase_codec;
mod constant_values;
pub mod nibble_codec;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
