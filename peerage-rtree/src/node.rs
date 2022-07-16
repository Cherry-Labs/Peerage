
use std::marker::PhantomData;
use peerage_coll::collection::PeerageCollection;
use peerage_utils::traits::*;
use crate::{value_holder::{NodeColl, KeyValueItem}, node_type::KeyInsertRes};
use peerage_macros::{block_out_return_holder, index_forward};
use crate::node_type::{NodeType, KeySetRes, SetResult, InsertResult};



#[derive(Clone, Copy)]
pub struct RTreeNode<'a, K: Key, T: NodeGlobal, L: Ledger> {
    node_type: NodeType,
    node_key: K,
    node_item: Option<T>,
    node_parent: Option<&'a Self>,
    ledger_data: Option<L>,
    kvs: Option<NodeColl<'a, K, T, L>>,
}


impl<'a, K: Key, T: NodeGlobal, L: Ledger> RTreeNode<'a, K, T, L> {
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
            node_key: K::init_empty(),
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

    pub fn rep_kv_mem(&mut self, rep: Option<NodeColl<'a, K, T, L>>) {
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

    pub fn set_kvs(&mut self, kvs: NodeColl<'a, K, T, L>) {
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
            NodeType::Empty => self.kvs = None,

        }
        
        self.kvs = Some(kvs)
    }

    pub fn get_sub_key(&self, key: K) -> Option<&Self> {
        if self.kvs.is_none() { return None; }
        
        let kvs = self.kvs.unwrap();
        
        let mut iter_kvs = kvs.into_iter();

        let size = iter_kvs.clone().count();

        for i in 0..size {
            let curr_kv = iter_kvs.next().unwrap();

            if curr_kv.compare_key(key) {
                return Some(curr_kv.value);
            }
        }
        None
    }

    pub fn replace(&mut self, key: K, rep: &'a Self) -> KeySetRes {
        if self.kvs.is_none() { return Err(SetResult::Failure) }
        
        let kvs = self.kvs.unwrap();
        
        let mut iter_kvs = kvs.into_iter();

        let size = iter_kvs.clone().count();

        for i in 0..size {
            let mut curr_kv = iter_kvs.next().unwrap();

            if curr_kv.compare_key(key) {
                curr_kv.mutate_value(rep);

                return Ok(SetResult::Success);
            }
        }        

        Err(SetResult::Failure)
    }

    pub fn get_sub_keys(&self) -> Option<PeerageCollection<K>> {
        if self.kvs.is_none() { return None; }


        let kvs = self.kvs.unwrap();
        
        let mut iter_kvs = kvs.into_iter();

        let mut keys: Vec<K> = vec![];

        for _ in 0..iter_kvs.clone().count() {
            let kv = iter_kvs.next().unwrap();

            let key = kv.get_sub_key();

            keys.push(key)
        }

        let peerage_keys = PeerageCollection::from_vector(keys);


        Some(peerage_keys)
    
    }

    pub fn insert_at_kv(&mut self, 
        key: K,
        value: &'a RTreeNode<'a, K, T, L>) -> KeyInsertRes {
        let self_kvs = self.kvs;

        let mut kvs_unwrapped= if self_kvs.is_none() { 
            let kvs_unwrapped = PeerageCollection::<KeyValueItem<'a, K, &'a Self>>::new();
        
            kvs_unwrapped
        } else {
            let mut kvs_unwrapped = self_kvs.unwrap();
        
            kvs_unwrapped
        }; 

        let new_kv = KeyValueItem::new(key, value);

        kvs_unwrapped.push(new_kv);
        
        self.kvs = Some(kvs_unwrapped);

        

        Ok(InsertResult::Success)
    }

   
    pub fn set_at_kv_index(&mut self, key: K, value: &'a Self, index: usize) -> KeySetRes {
        let self_kvs = self.kvs;

        if self_kvs.is_none() 
            || self.kv_len() == 0
            || self.kv_len() > index {
            return Err(SetResult::Failure);
        }

        let mut  self_kvs_unwrapped = self_kvs.unwrap();

        let new_kv = KeyValueItem::new(key, value);
        
        self_kvs_unwrapped[index] = new_kv;

        self.kvs = Some(self_kvs_unwrapped);

        Ok(SetResult::Success)
    }
    
    pub fn get_kvs(&self) -> Option<NodeColl<'a, K, T, L>> {
        self.kvs.clone()
    }

    pub fn get_values(&self) -> Option<PeerageCollection<Self>> {
        if self.kvs.is_none() { return None; }
        
        let kvs = self.kvs.unwrap();
        
        let mut iter_kvs = kvs.into_iter();

        let mut values: Vec<Self> = vec![];

        for _ in 0..iter_kvs.clone().count() {
            let kv = iter_kvs.next().unwrap();

            let value = kv.value;

            values.push(value.clone())
        }

        let peerage_values = PeerageCollection::from_vector(values);


        Some(peerage_values)
    
    }

    pub fn kv_len(&self) -> usize {
        if self.kvs.is_none() {
            return 0usize;
        }

        let kv_unwrapped = self.kvs.unwrap();

        kv_unwrapped.len()
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

impl<'a, K: Key, T: NodeGlobal, L: Ledger> std::ops::Index<K> for RTreeNode<'a, K, T, L> {
    type Output = Self;

    fn index(&self, index: K) -> &Self::Output {
       self.get_sub_key(index).unwrap()
    }
}





