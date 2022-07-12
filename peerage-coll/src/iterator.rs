
#[derive(Clone, Copy)]
pub struct PeerageCollectionIterator<T: Copy + Clone + Node, const M: usize> {
    coll: PeerageCollection<T, M>,
    curr_index: usize,
}

impl<T: Copy + Clone + Node, const M: usize> PeerageCollectionIterator<T, M> {
    pub fn new(coll: PeerageCollection<T, M>) -> Self {
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

impl<T: Copy + Clone + Node, const M: usize> std::iter::Iterator
    for PeerageCollectionIterator<T, M>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.get_at_index();
        self.increase_index();

        Some(item)
    }
}

impl<T: Copy + Clone + Node, const M: usize> std::iter::IntoIterator
    for PeerageCollection<T, M>
{
    type Item = T;

    type IntoIter = PeerageCollectionIterator<T, M>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
