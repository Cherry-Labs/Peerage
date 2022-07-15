use std::borrow::Borrow;

use peerage_utils::traits::Node;

use crate::collection::PeerageCollection;
use peerage_holder::holder::Holder;

#[derive(Clone, Copy)]
pub struct PeerageCollectionIterator<T: Copy + Clone + Node> {
    coll: PeerageCollection<T>,
    curr_index: usize,
}

impl<T: Copy + Clone + Node> PeerageCollectionIterator<T> {
    pub fn new(coll: PeerageCollection<T>) -> Self {
        let curr_index = 0usize;

        Self { coll, curr_index }
    }

    pub fn increase_index(&mut self) {
        match self.curr_index > self.coll.len() {
            true => self.curr_index = 0,
            false => self.curr_index += 1,
        }
    }

    pub fn decrease_index(&mut self) {
        match self.curr_index == 0 {
            true => self.curr_index = self.coll.len(),
            false => self.curr_index -= 1,
        }
    }

 
    pub fn get_at_index(&self) -> Option<T> {
        self.coll.get_at(self.curr_index).clone()
    }

    pub fn step_back(&mut self) -> Option<T> {
        self.decrease_index();
        self.coll.get_at(self.curr_index).clone()
    }

    pub fn nth_item(&self, n: usize) -> Option<T> {
        self.coll.get_at(n)
    }

    pub fn nth_item_ref_and_consume<'a>(self, n: usize) -> Holder<'a, T> {
        let t = self.coll.get_at(n);
        if t.is_none() {
            return Holder::Nil;
        }

        let t_unwrapped = t.unwrap();

        Holder::<'a, T>::new_selfer(t_unwrapped)
    }

}

impl<T: Copy + Clone + Node> std::iter::Iterator
    for PeerageCollectionIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_at_index();
        self.increase_index();

        item
    }
}

impl<T: Copy + Clone + Node> std::iter::IntoIterator
    for PeerageCollection<T>
{
    type Item = T;

    type IntoIter = PeerageCollectionIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PeerageCollectionIterator::new(self)
    }
}
