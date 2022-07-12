use peerage_utils::traits::{Node, Indexer};

use crate::value_holder::{NodeVhType, NodeKhType};


pub struct KVCollPair<'a, K: Indexer + Clone + Copy, T: Clone + Copy + Node> {
    keys: NodeKhType<K>,
    value: NodeVhType<'a, K, T>,
}