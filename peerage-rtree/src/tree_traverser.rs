use peerage_coll::collection::PeerageCollection;
use peerage_utils::traits::{Node, Key, Ledger};

use crate::node::RTreeNode;

pub enum ReturnTraverse {
    Init,
    None, 
    Some,
    StorageReached
}

impl ReturnTraverse {
    pub fn is_storage_reached(&self) -> bool {
        match self {
            ReturnTraverse::Init => false,
            ReturnTraverse::None => false,
            ReturnTraverse::Some => false,
            ReturnTraverse::StorageReached => true,
        }
    }
}

pub fn traverse_updown_iter<'a, 
        K: Key, 
        T: Node, 
        L: Ledger
        >(
        node: &'a RTreeNode<'a, K, T, L>, 
        v: &'a mut Vec<RTreeNode<'a, K, T, L>>,
        res: &'a mut ReturnTraverse
    )   
    {

    if node.is_storage() {
        *res =  ReturnTraverse::StorageReached;
    }

    if node.kv_len() == 0 {
        *res =  ReturnTraverse::None;
    }

    let node_values = node.get_values();

    if node_values.is_none() {
        *res = ReturnTraverse::None;
    }

    let node_vales_unwrapped = node_values.unwrap();

    let mut iter_node = node_vales_unwrapped.into_iter();

    for _ in 0..iter_node.clone().count() {
        let curr_node = iter_node.next();

        if curr_node.is_none() { continue; }

        let curr_node_unwrapped = curr_node.unwrap();

        v.push(curr_node_unwrapped);

        traverse_updown_iter(&curr_node_unwrapped, v, res);

    }

    *res = ReturnTraverse::Some
}

pub fn traverse_in_order<'a, K: Key, T: Node, L: Ledger>(node: &'a RTreeNode<'a, K, T, L>) -> PeerageCollection<RTreeNode<'a, K, T, L>> {
    let mut v: Vec<RTreeNode<'a, K, T, L>> = vec![];

    let mut res = ReturnTraverse::Init;

    loop {
        traverse_updown_iter(node, &mut v, &mut res);

        if res.is_storage_reached() {
            break;
        }
    }

    PeerageCollection::from_vector(v)


}