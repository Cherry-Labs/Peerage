#![allow(unused)]

pub mod hex_b64_pair;
pub mod passphrase_codec;
mod constant_values;
pub mod nibble_codec;
pub mod base64;

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
