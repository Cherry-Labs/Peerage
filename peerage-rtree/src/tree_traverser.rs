use std::borrow::BorrowMut;

use peerage_coll::collection::PeerageCollection;
use peerage_utils::traits::{Node, Key, Ledger, NodeGlobal};
use crate::node_type::{KeySetRes, SetResult};

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
        T: NodeGlobal, 
        L: Ledger
        >(
        node: &RTreeNode<'a, K, T, L>, 
        v: &mut Vec<RTreeNode<'a, K, T, L>>,
        res: &mut ReturnTraverse
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

pub fn traverse_in_order<'a, K: Key, T: NodeGlobal, L: Ledger>(node: &'a RTreeNode<'a, K, T, L>) -> PeerageCollection<RTreeNode<'a, K, T, L>> {
    let mut v: Vec<RTreeNode<'a, K, T, L>> = vec![node.clone()];

    let mut res = ReturnTraverse::Init;

    loop {
        traverse_updown_iter(node, &mut v, &mut res);

        if res.is_storage_reached() {
            break;
        }
    }

    PeerageCollection::from_vector(v)


}

pub fn get_sub_key_traversal<'a, K: Key, T: NodeGlobal, L: Ledger>(node: &'a RTreeNode<'a, K, T, L>, key: K) -> Option<RTreeNode<'a, K, T, L>> {
    if node.get_self_key().is_equal_to(key) {
        let node_owned = node.to_owned();

        return Some(node_owned)
    }
    
    let nodes_traversed = traverse_in_order(node);

    let mut node_traversed_iter = nodes_traversed.into_iter();

    for _ in 0..node_traversed_iter.clone().count() {
        let curr_node = node_traversed_iter.next();
        
        if curr_node.is_none() {
            continue;
        }

        let curr_node_unwrapped = curr_node.unwrap();

        if curr_node_unwrapped.get_self_key().is_equal_to(key) {
            return Some(curr_node_unwrapped);
        }    
    }

    None
}


pub fn replace_traversal<'a, K: Key, T: NodeGlobal, L: Ledger>(node: &'a RTreeNode<'a, K, T, L>, key: K, rep: &'a RTreeNode<'a, K, T, L>) -> KeySetRes {
    let gotten_key = get_sub_key_traversal(node, key);

    if gotten_key.is_none() {
        return Err(SetResult::Failure);
    }

    let mut gotten_key_unwrap = gotten_key.unwrap();

    let gotten_key_mut = gotten_key_unwrap.borrow_mut();


    let res = gotten_key_mut.replace(key, rep);

    res
}   




pub fn binary_search_traversal<'a, K: Key, T: NodeGlobal, L: Ledger>(
    node: &'a RTreeNode<'a, K, T, L>, 
    other: T) -> Option<(usize, K)> {
    let node_traversed = traverse_in_order(node);
    
    if node_traversed.len() == 0 {
        return None;
    }

    let mut low = 0usize;
    let mut high = node_traversed.len();

    loop {
        if low == high {
            break;
        }

        let mid = (low + high) / 2;

        let curr_node = node_traversed[mid];

        let curr_node_item = curr_node.get_node_item();

        if curr_node_item.is_none() {
            continue;
        }

        let curr_node_item_unwrapped = curr_node_item.unwrap();

        if curr_node_item_unwrapped.is_equal_to(other) {
            let key = curr_node.get_self_key();

            return Some((mid, key))
        } else if other.is_greater_to(curr_node_item_unwrapped) {
            low = mid + 1;
        } else {
            high = mid + 1;
        }
    }


    None
}