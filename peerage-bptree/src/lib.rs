
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod node;
mod coll;
mod degee;
pub mod global_consts;


use paste::paste;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
