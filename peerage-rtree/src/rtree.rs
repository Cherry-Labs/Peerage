use std::borrow::BorrowMut;
use std::marker::PhantomData;

use peerage_utils::traits::{Key, Ledger, Node};
use crate::value_holder::StructNode;
use crate::node::RTreeNode;
use crate::node_type::{KeySetRes, KeyInsertRes};

#[derive(Clone, Copy)]
pub struct RTree<'a> {
    ledger_root: StructNode<'a>,
    storage_root: Option<StructNode<'a>>,
    crypto_root: Option<StructNode<'a>>,
}

impl<'a> RTree<'a> {
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

   

}
