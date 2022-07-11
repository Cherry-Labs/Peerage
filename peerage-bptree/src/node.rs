

use peerage_utils::pub_traits::Init;

use crate::coll::BpTreeCollection;

use crate::degee::Degree;


pub struct BPTreeNode<'a, T: Init  + Copy + Clone, const B: usize, const M: usize> {
    is_leaf: bool,
    degree: Degree,
    size: usize,
    item: T,
    children: BpTreeCollection<T, B, M>,
    parent: &'a Self,

}

