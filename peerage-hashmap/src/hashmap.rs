use peerage_coll::collection::PeerageCollection;
use crate::item::Item;
use peerage_utils::traits::Key;

#[derive(Clone, Copy)]
pub struct PeerageMap<'a, K: Key, V: Clone + Copy + Default> {
    len: usize,
    loader: f32, 
    table: PeerageCollection<Option<Item<'a, K, V>>>,
}


impl<'a, K: Key, V: Clone + Copy + Default> PeerageMap<'a, K, V> {
    pub fn put(&mut self, key: K, value: V) {
        let index = key.get_key_index(self.len);

        let entry = self.table.get_at(index);

        match entry {
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}