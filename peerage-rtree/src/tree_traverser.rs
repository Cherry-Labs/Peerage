use std::borrow::{BorrowMut, Borrow};
use std::ops::Index;

use peerage_coll::collection::PeerageCollection;
use peerage_utils::traits::{Node, Key, Ledger, NodeGlobal};
use crate::node_type::{KeySetRes, SetResult, InsertResult, KeyInsertRes};

use crate::node::RTreeNode;
use crate::value_holder::{NodeColl, KeyValueItem};

pub enum ReturnTraverse {
    Init,
    None, 
    Some,
}

impl ReturnTraverse {
    pub fn is_none(&self) -> bool {
        match self {
            ReturnTraverse::Init => false,
            ReturnTraverse::None => true,
            ReturnTraverse::Some => false,
        }
    }
}

fn traverse_updown_iter<'a, 
        K: Key, 
        T: NodeGlobal, 
        L: Ledger
        >(
        node: &RTreeNode<'a, K, T, L>, 
        v: &mut Vec<RTreeNode<'a, K, T, L>>,
        res: &mut ReturnTraverse
    )   
    {

    
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

        if res.is_none() {
            break;
        }
    }

    PeerageCollection::from_vector(v)


}

pub fn binary_search_key<'a, 
    K: Key, 
    T: NodeGlobal, 
    L: Ledger>(
        node: &'a RTreeNode<'a, K, T, L>, 
        key: K) -> Option<(usize, RTreeNode<'a, K, T, L>)> {
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

        let curr_node_key = curr_node.get_self_key();

        if curr_node_key.is_equal_to(key) {
            let key = curr_node.get_self_key();

            return Some((mid, curr_node))
        } else if key.is_greater_to(curr_node_key) {
            low = mid + 1;
        } else {
            high = mid + 1;
        }
    }


    None
}



pub fn replace_item_traversal<'a, 
        K: Key, 
        T: NodeGlobal, 
        L: Ledger>(
            node: &'a RTreeNode<'a, K, T, L>, 
            key: K, rep: &'a RTreeNode<'a, K, T, L>,
            item: T
        ) -> KeySetRes {
    let gotten_node = binary_search_key(node, key);

    if gotten_node.is_none() {
        return Err(SetResult::Failure);
    }

    let mut gotten_node_unwrap = gotten_node.unwrap().1;

    let gotten_key_mut = gotten_node_unwrap.borrow_mut();


    gotten_key_mut.set_item(item);

    Ok(SetResult::Success)
}   


pub fn insert_item_traversal<'a, 
        K: Key, 
        T: NodeGlobal, 
        L: Ledger>(
            node: &'a RTreeNode<'a, K, T, L>, 
            key: K,
            value: &'a RTreeNode<'a, K, T, L>,
        ) -> KeyInsertRes {
    let gotten_node = binary_search_key(node, key);

    if gotten_node.is_none() {
        return Err(InsertResult::Failure);
    }

    let mut gotten_node_unwrap = gotten_node.unwrap().1;

    gotten_node_unwrap.insert_at_kv(key, value);

    Ok(InsertResult::Success)
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


fn find_at_level_iter<'a,
                K: Key,
                T: NodeGlobal,
                L: Ledger>(
                    node: Option<RTreeNode<'a, K, T, L>>,
                    index: usize,
                ) -> Option<RTreeNode<'a, K, T, L>>
        {
            if node.is_none() {
                return None;
            }

            let curr_node = node.unwrap();

            let curr_values = curr_node.get_values();

            if curr_values.is_none() {
                return None;
            }

            let cv_unwrapped = curr_values.unwrap();

            let mut v_iter = cv_unwrapped.into_iter();
            
            let v = v_iter.nth_item_ref_and_consume(index);

            if v.is_nil() {
                return None;
            }

            let v_ref = v.unwrap_no_ref();

            v_ref
        }
        


pub fn find_at_level<'a,
                K: Key,
                T: NodeGlobal,
                L: Ledger>(
                    node: &'a RTreeNode<'a, K, T, L>,
                    indice_levels: Vec<usize>,
                ) -> Option<RTreeNode<'a, K, T, L>>
        {
            let node_values = node.get_values();

            if node_values.is_none() {
                return None;
            }

            let nv_unwrapped = node_values.unwrap();

            let nv_unw_iter = nv_unwrapped.into_iter();

            let mut counter = 0usize;
            let levels_size = indice_levels.len();

            let mut curr_node = Some(node.clone());

            for i in 0..levels_size {
                let curr_index = indice_levels[i];

                curr_node =  find_at_level_iter(curr_node, curr_index)
            }

            curr_node
        }



pub fn insert_at_level<'a,
        K: Key,
        T: NodeGlobal,
        L: Ledger>(
            node: &'a RTreeNode<'a, K, T, L>,
            key: K,
            indice_levels: Vec<usize>,
            value: &'a RTreeNode<'a, K, T, L>
        ) -> KeyInsertRes

    {
        let node_opt = find_at_level(node, indice_levels);

        if node_opt.is_none() {
            return Err(InsertResult::Failure);
        }

        let mut no_unwrapped = node_opt.unwrap();
    
        no_unwrapped.insert_at_kv(key, value);

        Ok(InsertResult::Success)
    }

    pub fn replace_at_level<
            'a, 
            K: Key, 
            T: NodeGlobal, 
            L: Ledger
        >
        (
            node: &'a mut RTreeNode<'a, K, T, L>,
            key: K,
            value: &'a RTreeNode<'a, K, T, L>,
            index: usize,
            indice_levels: Vec<usize>,
        ) -> KeySetRes
        {
            let curr_keys = node.get_values();
        
            if curr_keys.is_none() {
                return Err(SetResult::Failure);
            }

            let mut curr_node = node.clone();
            let mut curr_keys_unwrapped = curr_keys.unwrap();

            let len = indice_levels.len() - 2;

            let mut i = 0;

            loop {
                curr_node = curr_keys_unwrapped[i];
                
                let curr_node_values = curr_node.get_values();

                if curr_node_values.is_none() {
                    continue;
                }

                curr_keys_unwrapped = curr_node_values.unwrap();
            
                i += 1;

                if len == i {
                    break;
                }
                
            }

            
            let last_kv = curr_node.get_kvs();

            if last_kv.is_none() {
                return Err(SetResult::Failure);
            }

            
            let mut last_kv_unwrapped = last_kv.unwrap();

            let new_kv = KeyValueItem::new(key, value);

            if index > last_kv_unwrapped.len() {
                return Err(SetResult::Failure);
            }

            last_kv_unwrapped[index] = new_kv;

            let kv_deref = last_kv_unwrapped.clone();

            node.rep_kv_mem(Some(kv_deref));

            Ok(SetResult::Success)
        }