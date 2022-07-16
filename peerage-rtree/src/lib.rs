
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

pub mod rtree;
pub mod node;
pub mod value_holder;
mod impler_dummy;
pub mod node_type;
mod tree_traverser;

use paste::paste;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
