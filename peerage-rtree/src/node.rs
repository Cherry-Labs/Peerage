
use std::marker::PhantomData;
use peerage_utils::traits::*;
use crate::degee::Degree;
use crate::value_holder::{NodeColl, KeyValueItem};
use peerage_macros::block_out_return_holder;





#[derive(Clone, Copy)]
pub struct RTreeNode<'a, K: Key, T: Node> {
    degree: usize,
    size: usize,
    node_item: T,
    kvs: Option<NodeColl<'a, K, T>>,
    parent: Option<&'a Self>,

}


impl<'a, K: Key, T: Node> RTreeNode<'a, K, T> {
    pub fn new()


}