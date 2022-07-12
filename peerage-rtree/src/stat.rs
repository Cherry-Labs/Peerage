use peerage_holder::holder::Holder;
use peerage_utils::pub_traits::Tree;

use crate::degee::Degree;
use crate::coll::PeerageCollection;
use std::collections::HashMap;
use crate::node::RTreeNode;

lazy_static! {
    pub static ref SIZE_TABLE: HashMap<usize, usize> = {
        let mut hm = HashMap::<usize, usize>::new();

        hm.insert(2, 1);
        hm.insert(4, 2);
        hm.insert(16, 8);
        hm.insert(256, 64);
        hm.insert(512, 128);
        hm.insert(1024, 256);
        hm.insert(2046, 512);
        hm.insert(4096, 1024);

        hm
    };
}






pub fn binary_search<'a, T: Clone + Copy>(root_cursor: RTreeNode<'a, T>, key: &'a T) -> (usize, Holder<'a, BPTreeNode<'a, T>>) {
    let mut current_node = root_cursor.clone();
    
    loop {
        if !current_node.is_leaf() {
            return (0usize, Holder::Nil);
        }

        let holder_items = current_node.get_items();

        for t in holder_items.into_itl


    }
}