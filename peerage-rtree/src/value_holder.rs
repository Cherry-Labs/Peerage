use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::traits::{Node, Key};
use peerage_coll::collection::PeerageCollection;
use crate::node::RTreeNode;


#[derive(Clone, Copy)]
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
        self.key.is_equal_to(k)
    }

   
    pub fn mutate_value(&mut self, value: &'a V){
        self.value = value
    }

    pub fn get_sub_key(&self) -> K {
        self.key.clone()
    }

}

type KeyNodeItem<'a, 
            K, 
            T,
            L,
            > = KeyValueItem<
                'a,
                K,
                &'a RTreeNode<'a, K, T, L>
            >;



pub type NodeColl<'a, 
                    K, 
                    T,
                    L,
                > = PeerageCollection<
                    KeyNodeItem<'a, K, T, L>                
            >;

