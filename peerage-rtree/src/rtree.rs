use std::borrow::BorrowMut;
use std::marker::PhantomData;

use peerage_utils::traits::{Key, Ledger, NodeGlobal};

use crate::node::RTreeNode;
use crate::node_type::{KeySetRes, KeyInsertRes};
use crate::tree_traverser::*;

#[derive(Clone, Copy)]
pub struct RTree<'a, K: Key, T: NodeGlobal, L: Ledger> {
    ledger_root: RTreeNode<'a, K, T, L>,
    storage_root: Option<RTreeNode<'a, K, T, L>>,
    crypto_root: Option<RTreeNode<'a, K, T, L>>,
}

impl<'a, K: Key, T: NodeGlobal, L: Ledger> RTree<'a, K, T, L> {
    pub fn new_empty() -> Self {
        Self {
            ledger_root: RTreeNode::new_empty(),
            storage_root: None,
            crypto_root: None,
        }
    }

    pub fn set_ledger_root(&mut self, ledger_root: RTreeNode<'a, K, T, L>) {
        self.ledger_root = ledger_root;
    }

    pub fn set_storage_root(&mut self, storage_root: RTreeNode<'a, K, T, L>) {
        self.storage_root = Some(storage_root);
    }

    pub fn set_crypto_root(&mut self, crypto_root: RTreeNode<'a, K, T, L>) {
        self.crypto_root = Some(crypto_root);
    }

    pub fn insert_node_at_ledger_root(&self, key: K, value: &'a RTreeNode<'a, K, T, L>) -> KeyInsertRes {
        insert_item_traversal(&self.ledger_root, key, value)
    }

    pub fn replace_item_at_ledger_root(&'a mut self, key: K, rep_with: RTreeNode<'a, K, T, L>) -> KeySetRes {
        replace_item_traversal(&mut self.ledger_root, key, rep_with)
    }

}
