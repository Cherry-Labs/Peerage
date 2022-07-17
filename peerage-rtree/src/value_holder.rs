use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::traits::{Key, Ledger, Node};
use peerage_coll::collection::PeerageCollection;
use crate::{node::RTreeNode, node_type::NodeType};
use peerage_keys::main_keys::Key as KeyStruct;
use peerage_ledger::ledger::Ledger as LedgerStruct;
use peerage_node::node_item::Node as NodeStruct;


#[derive(Clone, Copy, Default)]
pub struct KeyValueItem<'a, K: Key, V: Copy + Clone> {
    pub key: K,
    pub value: V,
    phantom: PhantomData<&'a K>
}

impl<'a, K: Key, V: Copy + Clone> KeyValueItem<'a, K, &'a V> {
    pub fn new(key: K, value: &'a V) -> Self {                
        Self { key, value, phantom: PhantomData }
    } 


    pub fn compare_key(&self, k: K) -> bool {
        self.key == k
    }

   
    pub fn mutate_value(&mut self, value: &'a V){
        self.value = value
    }

    pub fn get_sub_key(&self) -> K {
        self.key.clone()
    }
}

pub type StructNode<'a> = RTreeNode<'a, KeyStruct, NodeStruct, LedgerStruct>;


type KeyNodeItem<'a> = KeyValueItem<'a, KeyStruct, &'a StructNode<'a>>;

pub type SubItems<'a> = PeerageCollection<KeyNodeItem<'a>>;
pub type Leaf<'a> = PeerageCollection<KeyNodeItem<'a>>;
pub type Branch<'a> = PeerageCollection<Leaf<'a>>;


#[derive(Clone, Copy, Default)]
pub struct NodeColl<'a> {
    sub_items: Leaf<'a>,
    previous_leaf: Option<&'a Leaf<'a>>,
    next_leaf: Option<&'a Leaf<'a>>,
    parent_branch: Option<Branch<'a>>,
    child_branch: Option<Branch<'a>>,
    len: usize,

}

impl<'a> NodeColl<'a> {
    pub fn new_empty() -> Self {
        let sub_items = PeerageCollection::<KeyNodeItem<'a>>::default();
        let len = 0usize;
        let previous_leaf =  None;
        let next_leaf = None;
        let parent_branch = None;
        let child_branch = None;

        Self { 
            sub_items, 
            previous_leaf, 
            next_leaf,
            parent_branch,
            child_branch, 
            len, 

        }

    }

  
     
}