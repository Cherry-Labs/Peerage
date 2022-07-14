use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::traits::{Node, Key};
use peerage_coll::collection::PeerageCollection;
use crate::node::RTreeNode;


#[derive(Clone, Copy)]
pub struct KeyValueItem<'a, K: Key, V: Copy + Clone> {
    key: K,
    value: Holder<'a, V>
}

impl<'a, K: Key, V: Copy + Clone> KeyValueItem<'a, K, V> {
    pub fn new(key: K, value: &'a V) -> Self {
        let value_holder = Holder::<'a, V>::new_refer(value);
        
        Self { key, value: value_holder }
    } 

    pub fn compare_key(&self, k: K) -> bool {
        self.key.is_equal_to(k)
    }

    pub fn unwrap_value(&self) -> &V {
        self.value.unwrap().unwrap()
    }

    pub fn mutate_value(&mut self, t: V){
        self.value.mutate(t);
    }

    pub fn get_key(&self) -> K {
        self.key.clone()
    }

    pub fn get_value(&self) -> V {
        self.value.unwrap_no_ref().clone().unwrap()
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

