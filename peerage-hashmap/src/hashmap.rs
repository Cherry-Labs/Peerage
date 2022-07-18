use peerage_coll::collection::PeerageCollection;
use crate::item::Item;
use peerage_utils::traits::Key;

#[derive(Clone, Copy)]
pub struct PeerageMap<K: Key, V: Clone + Copy + Default> {
    len: usize,
    loader: f32, 
    table: PeerageCollection<Item<K, V>>,
}


impl<K: Key, V: Clone + Copy + Default> PeerageMap<K, V> {
    pub fn put(key: K, value: V) {
        let index = key.get_key_index(self.len);

        let entry = self.tabl
    }
}