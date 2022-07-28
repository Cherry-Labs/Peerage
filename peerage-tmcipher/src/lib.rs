#![allow(unused)]

mod constants;
mod cipher;
pub mod key;
pub mod trig_identities;

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
