
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod node;
mod coll;
mod degee;
mod stat;
mod value_holder;
mod key_value_holder;

use paste::paste;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
