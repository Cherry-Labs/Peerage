use peerage_utils::traits::{NodeGlobal, Key, Ledger};

use crate::node::RTreeNode;
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
        crypto_root: None 
    }
   } 

   pub fn set_ledger_root(&mut self, ledger_root: RTreeNode<'a, K, T, L>) {
        self.ledger_root = ledger_root; 
   }
    
}