use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::traits::{Node, Key, Value};
use peerage_coll::collection::PeerageCollection;
use crate::node::RTreeNode;
use crate::degee::Degree;


#[derive(Clone, Copy)]
pub struct KeyValueItem<'a, K: Key, V: Copy + Clone> {
    key: K,
    value: Holder<'a, V>
}


type KeyNodeItem<'a, 
            K, 
            T> = KeyValueItem<
                'a,
                K,
                &'a RTreeNode<'a, K, T>
            >;



pub type NodeColl<'a, 
                    K, 
                    T,
                > = PeerageCollection<
                    KeyNodeItem<'a, K, T>                
            >;

