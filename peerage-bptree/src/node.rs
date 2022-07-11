
use std::marker::PhantomData;

use peerage_holder::holder::Holder;
use peerage_utils::pub_traits::Tree;
use crate::coll::BpCollHolder;
use crate::degee::Degree;


use peerage_macros::block_out_return_holder;


#[derive(Clone, Copy)]
pub struct CollHolder<'a, T: Tree + Copy + Clone>(&'a Holder<'a, BpCollHolder<BPTreeNode<'a, T>>>);






#[derive(Clone, Copy)]
pub struct BPTreeNode<'a, T: Tree + Copy + Clone> {
    is_leaf: bool,
    degree: Degree,
    size: usize,
    item: BpCollHolder<T>,
    children: CollHolder<'a, T>,
    parent: Option<&'a Self>,

}


impl<'a, T: Tree  + Copy + Clone> BPTreeNode<'a, T> {
    pub fn new(degree: Degree) -> Self {
        let is_leaf = false;
        let size: usize = degree.into();

        let item = BpCollHolder::<T>::new(degree);

        let children = CollHolder(&Holder::Nil);         


        let parent = None;
        

        Self { is_leaf, degree, size, item, children, parent }

    }

}



impl<'a, T: Tree  + Copy + Clone> From<usize> for BPTreeNode<'a, T> {
    fn from(u: usize) -> Self {
        let degree: Degree = u.into();

        Self::new(degree)
    }
}


pub struct BpTree<'a, T: Tree  + Copy + Clone> {
    root: BPTreeNode<'a, T>,
    degree: Degree,
}

impl<'a, T: Tree  + Copy + Clone> BpTree<'a, T> {
    pub fn new(degree: Degree) -> Self {
        let root = BPTreeNode::<T>::new(degree);

        Self { root, degree }
    }

    pub fn get_root(&self) -> BPTreeNode<'a, T> {
        self.root
    }

    pub fn search_tree(&self, key: T, node: Option<BPTreeNode<'a, T>>) -> Option<BPTreeNode<'a, T>> {
        if node.is_none() {
            return None
        }

        let mut cursor = node.unwrap().clone();

        let mut childrens = cursor.children.unwrap_no_ref();

        if childrens.is_none() {
            return cursor;
        }

        let children_wrapper = childrens.unwrap();
/* 
        let children_coll = match self.degree {
            Degree::Two => {
                let item = block_out_return_holder!(%children_wrapper *ref $2);

                item as BpTreeCollection<T, 2, 1>
            },
            Degree::Four => todo!(),
            Degree::Sixteen => todo!(),
            Degree::TwoFiftySix => todo!(),
            Degree::FiveTwelve => todo!(),
            Degree::TenTwnentyFour => todo!(),
            Degree::TwoFourtyEight => todo!(),
            Degree::FortyNightySix => todo!(),
        };

*/     
    }

}
        



impl<'a, T: Tree + Copy + Clone> Tree for BPTreeNode<'a, T> { 
    type LedgerType = peerage_ledger::Ledger;
    type InputType = Degree;

    fn init() -> Self {
        Self::new(Degree::Two)
    }
    fn new(input: Degree) -> Self {
        Self(PhantomData)

    }
    fn mutate(&mut self, other: Self) {

    }
    fn get_ledger_id(&self) -> Self::LedgerType {
        peerage_ledger::Ledger
    }
    fn is_equal_to(&self, other: Self) -> bool {
        true
    }
    fn is_greater_than(&self, other: Self) -> bool {
        true
    }
    fn is_lesser_than(&self, other: Self) -> bool {
        true
    }

}