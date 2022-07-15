
#![allow(unused)]

#[macro_use]
extern crate lazy_static;

pub mod rtree;
mod node;
mod value_holder;
mod impler_dummy;
mod node_type;
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
