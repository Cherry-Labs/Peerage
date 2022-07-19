use std::borrow::BorrowMut;
use std::marker::PhantomData;

use peerage_utils::traits::{Key, Ledger, Node};
use crate::node::RTreeNode;

#[derive(Clone, Copy)]
pub struct RTree<'a, K: Key, T: Node, L: Ledger> {
    ledger_root: RTreeNode<'a, K, T, L>,
    storage_root: Option<RTreeNode<'a, K, T, L>>,
    crypto_root: Option<RTreeNode<'a, K, T, L>>,
     cv 
}

impl<'a, K: Key, T: Node, L: Ledger> RTree<'a, K, T, L> {
    pub fn new_empty() -> Self {
        Self {
            ledger_root: RTreeNode::new_empty(),
            storage_root: None,
            crypto_root: None,
        }
    }

    pub fn set_ledger_root(&mut self, ledger_root: StructNode<'a>) {
        self.ledger_root = ledger_root;
    }

    pub fn set_storage_root(&mut self, storage_root: StructNode<'a>) {
        self.storage_root = Some(storage_root);
    }

    pub fn set_crypto_root(&mut self, crypto_root: StructNode<'a>) {
        self.crypto_root = Some(crypto_root);
    }

    pub fn insert_at_

}
