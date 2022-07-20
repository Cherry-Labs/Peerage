#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod key_pair;
mod prime_number_db;
mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
