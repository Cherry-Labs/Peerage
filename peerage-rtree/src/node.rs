
use std::{marker::PhantomData, borrow::Borrow};
use peerage_coll::collection::PeerageCollection;
use peerage_holder::holder::Holder;
use peerage_utils::traits::*;
use crate::{value_holder::{NodeColl, KeyValueItem}, node_type::KeyInsertRes};
use peerage_macros::{block_out_return_holder, index_forward};
use crate::node_type::{NodeType, KeySetRes, SetResult, InsertResult};
use peerage_keys::main_keys::Key as KeyStruct;
use peerage_ledger::ledger::Ledger as LedgerStruct;
use peerage_node::node_item::Node as NodeStruct;


#[derive(Clone, Default, Copy)]
pub struct RTreeNode<'a, K: Key, T: Node, L: Ledger> {
    node_type: NodeType,
    node_key: K,
    node_item: Option<T>,
    node_parent: Option<&'a Self>,
    ledger_data: Option<L>,
    kvs: Option<NodeColl<'a>>,
}


impl<'a, K: Key, T: Node, L: Ledger> RTreeNode<'a, K, T, L> {
    pub fn from_node_type(node_type: NodeType, node_key: K) -> Self {
        Self { 
            node_type, 
            node_item: None, 
            node_key,
            kvs: None, 
            ledger_data: None,
            node_parent: None,
        }
    }

    pub fn new_empty() -> Self {
        Self { 
            node_type: crate::node_type::NodeType::Empty,  
            node_item: None,
            node_key: K::default(),
            node_parent: None,
            ledger_data: None,
            kvs: None,
        }
    }

    pub fn is_storage(&self) -> bool {
        match self.node_type {
            NodeType::Empty => false,
            NodeType::LedgerNode(_) => false,
            NodeType::StorageNode(_) => true,
            NodeType::EncryptedNode(_) => false,
        }
    }

    pub fn is_ledger(&self) -> bool {
        match self.node_type {
            NodeType::Empty => false,
            NodeType::LedgerNode(_) => true,
            NodeType::StorageNode(_) => false,
            NodeType::EncryptedNode(_) => false,
        }
    }

    pub fn is_encrypted(&self) -> bool {
        match self.node_type {
            NodeType::Empty => false,
            NodeType::LedgerNode(_) => false,
            NodeType::StorageNode(_) => false,
            NodeType::EncryptedNode(_) => true,
        }
    }


    pub fn is_empty(&self) -> bool {
        match self.node_type {
            NodeType::Empty => true,
            NodeType::LedgerNode(_) => false,
            NodeType::StorageNode(_) => false,
            NodeType::EncryptedNode(_) => false,
        }
    }

    pub fn rep_kv_mem(&mut self, rep: Option<NodeColl<'a>>) {
        std::mem::replace(&mut self.kvs, rep);
    }

    pub fn get_self_key(&self) -> K {
        self.node_key.clone()
    }

    pub fn set_parent(&mut self, parent: &'a Self) {
        self.node_parent = Some(parent)
    }

    pub fn set_item(&mut self, item: T) {
        self.node_item = Some(item)
    }

    pub fn set_kvs(&mut self, kvs: NodeColl<'a>) {
        match self.node_type {
            NodeType::LedgerNode(u) => {
                self.kvs = Some(kvs);

            },
            NodeType::StorageNode(u) => {
               self.kvs = Some(kvs);

            },
            NodeType::EncryptedNode(u) => {
                self.kvs = Some(kvs);

            },
            NodeType::Empty => self.kvs = None,

        }
        
        self.kvs = Some(kvs)
    }

   
    
    pub fn get_kvs(&self) -> Option<NodeColl<'a>> {
        self.kvs.clone()
    }

    pub fn get_degree(&self) -> usize {
        match self.node_type {
            NodeType::Empty => 0,
            NodeType::LedgerNode(u) => u,
            NodeType::StorageNode(u) => u,
            NodeType::EncryptedNode(u) => u,
        }
    }

    pub fn get_node_item(&self) -> Option<T> {
        if self.node_item.is_none() {
            return None;
        }

        let item_unwrapped = self.node_item.unwrap();

        Some(item_unwrapped)
    }

    
}

impl<'a> Default for &'a RTreeNode<'a, KeyStruct, NodeStruct, LedgerStruct> {
    fn default() -> &'a RTreeNode<'a, KeyStruct, NodeStruct, LedgerStruct> {
        static new_empty: RTreeNode::<'static, 
                KeyStruct, 
                NodeStruct, 
                LedgerStruct> = RTreeNode {
                    node_type: NodeType::Empty,
                    node_key: KeyStruct::Init,
                    node_item: None,
                    node_parent: None,
                    ledger_data: None,
                    kvs: None,
                };

        &new_empty
    }
}