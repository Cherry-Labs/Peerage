
use std::marker::PhantomData;
use peerage_utils::traits::{Node, Indexer};
use crate::coll::RtCollHolder;
use crate::degee::Degree;
use crate::stat::*;
use crate::value_holder::NodeVhType;
use crate::key_value_holder::KVCollPair;

use peerage_macros::block_out_return_holder;





#[derive(Clone, Copy)]
pub struct RTreeNode<'a, K: Indexer + Clone + Copy, V: Clone + Copy, T: Clone + Copy + Node> {
    degree: Degree,
    size: usize,
    node_item: T,
    kvs: Option<KVCollPair<'a, K, T>>,
    parent: Option<&'a Self>,

}

impl<'a, K: Indexer + Clone + Copy, V: Clone + Copy, T: Clone + Copy + Node> RTreeNode<'a, T, K, V> {
    pub fn new(degree: Degree) -> Self {
        let size: usize = degree.into();

        let node_item = T::new();

        let kvs = None;
        let parent = None;
        

        Self { degree, size, node_item, kvs, parent