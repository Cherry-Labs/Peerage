use peerage_coll::collection::PeerageCollection;
use peerage_utils::traits::{Node, Key, Ledger};

use crate::node::RTreeNode;

#[derive(Clone, Copy)]
pub enum TreeTraverser<'a, K: Key, T: Node, L: Ledger> {
    UpDown(RTreeNode<'a, K, T, L>),
    DownUp(RTreeNode<'a, K, T, L>),
}

pub enum NoneOrSome {
    None, 
    Some,
}

type NoneOrSomeResult = std::result::Result<NoneOrSome, NoneOrSome>;

impl<'a, K: Key, T: Node, L: Ledger> TreeTraverser<'a, K, T, L> {
    pub fn traverse_updown_iter(&self, v: &mut Vec<RTreeNode<'a, K, T, L>>) -> NoneOrSomeResult  {
        let TreeTraverser::UpDown(node) = self;

        let node_values = node.get_values();

        if node_values.is_none() {
            return Err(NoneOrSome::None);
        }

        let node_vales_unwrapped = node_values.unwrap();

        let mut iter_node = node_vales_unwrapped.into_iter();

        for _ in 0..iter_node.clone().count() {
            let curr_node = iter_node.next();

            if curr_node.is_none() { continue; }

            let curr_node_unwrapped = curr_node.unwrap();

            v.push(curr_node_unwrapped);
        }

        Ok(NoneOrSome::Some)
    }

    pub fn traverse_downup_iter(&self, v: &mut Vec<RTreeNode<'a, K, T, L>>) -> NoneOrSomeResult  {
        let TreeTraverser::UpDown(node) = self;

        let node_values = node.get_values();

        if node_values.is_none() {
            return Err(NoneOrSome::None);
        }

        let node_vales_unwrapped = node_values.unwrap();

        let mut iter_node = node_vales_unwrapped.into_iter();

        for _ in 0..iter_node.clone().count() {
            let curr_node = iter_node.step_back();

            let curr_node_unwrapped = curr_node.unwrap();

            v.push(curr_node_unwrapped);
        }

        Ok(NoneOrSome::Some)
    }
}