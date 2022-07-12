use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::traits::{Node, Indexer};
use crate::node::RTreeNode;
use crate::degee::Degree;
use crate::coll::RtCollHolder;


#[derive(Clone, Copy)]
pub struct KeyValueItem<'a, K: Indexer + Clone + Copy, V: Clone + Copy> {
    key: K,
    value: Holder<'a, V>
}



pub type CollKvType<'a, K: Indexer + Clone + Copy, V: Clone + Copy> = RtCollHolder<KeyValueItem<'a, K, V>>;

#[derive(Clone, Copy)]
pub struct KeyValueHolder<'a, K: Indexer + Clone + Copy, V: Clone + Copy> {
    degree: Degree,
    coll: CollKvType<'a, K, V>,

}

pub type NodeVhType<'a, K: Indexer + Clone + Copy, T: Clone + Copy + Node> = KeyValueHolder<'a, K, &'a RTreeNode<'a, K, PhantomData<T>, T>>;



pub type NodeKhType<K: Indexer + Clone + Copy> = RtCollHolder<K>;