use peerage_utils::traits::Node;

use crate::collection::PeerageCollection;


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
        match self.curr_index > 1023 {
            true => self.curr_index = 0,
            false => self.curr_index += 1,
        }
    }

    pub fn get_at_index(&self) -> T {
        self.coll.get_at(self.curr_index).clone()
    }
}

impl<T: Copy + Clone + Node> std::iter::Iterator
    for PeerageCollectionIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_at_index();
        self.increase_index();

        Some(item)
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
