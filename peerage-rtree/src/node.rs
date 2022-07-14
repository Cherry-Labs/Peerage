
use std::marker::PhantomData;
use peerage_utils::traits::*;
use crate::value_holder::{NodeColl, KeyValueItem};
use peerage_macros::block_out_return_holder;
use crate::node_type::NodeType;




#[derive(Clone, Copy)]
pub struct RTreeNode<'a, K: Key, T: Node> {
    node_type: NodeType,
    node_item: Option<T>,
    node_parent: Option<&'a Self>,
    kvs: Option<NodeColl<'a, K, T>>,

}


impl<'a, K: Key, T: Node> RTreeNode<'a, K, T> {
    pub fn new(node_type: NodeType) -> Self {
        Self { 
            node_type, 
            node_item: None, 
            kvs: None, 
            node_parent: None 
        }
    }

    pub fn set_parent(&mut self, parent: &'a Self) {
        self.node_parent = Some(parent)
    }

    pub fn set_item(&mut self, item: T) {
        self.node_item = Some(item)
    }

    pub fn set_kvs(&mut self, kvs: NodeColl<'a, K, T>) {
        match self.node_type {
            NodeType::LedgerNode(u) => {
                if kvs.len() != u {
                    panic!("Uneuqual kvs")
                }

                self.kvs = Some(kvs);

            },
            NodeType::StorageNode(u) => {
                if kvs.len() != u {
                    panic!("Uneuqual kvs")
                }

                self.kvs = Some(kvs);

            },
            NodeType::EncryptedNode(u) => {
                if kvs.len() != u {
                    panic!("Uneuqual kvs")
                }

                self.kvs = Some(kvs);

            },

        }
        
        self.kvs = Some(kvs)
    }

    pub fn get_key(&self, key: K) -> Option<&Self> {
        if self.kvs.is_none() { panic!("no key-values!" )}
        
        let kvs = self.kvs.unwrap();
        
        let mut iter_kvs = kvs.into_iter();

        let size = iter_kvs.clone().count();

        for i in 0..size {
            let curr_kv = iter_kvs.next().unwrap();

            if curr_kv.compare_key(key) {
                return Some(curr_kv.unwrap_value());
            }
        }
        None
    }

    

}