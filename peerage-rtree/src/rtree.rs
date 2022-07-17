use std::borrow::BorrowMut;
use std::marker::PhantomData;

use peerage_utils::traits::{Key, Ledger, Node};

use crate::node::RTreeNode;
use crate::node_type::{KeySetRes, KeyInsertRes};

#[derive(Clone, Copy)]
pub struct RTree<'a, K: Key, T: Node, L: Ledger> {
    ledger_root: RTreeNode<'a, K, T, L>,
    storage_root: Option<RTreeNode<'a, K, T, L>>,
    crypto_root: Option<RTreeNode<'a, K, T, L>>,
}

impl<'a, K: Key, T: Node, L: Ledger> RTree<'a, K, T, L> {
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

   

}
