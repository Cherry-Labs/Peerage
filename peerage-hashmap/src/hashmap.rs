use peerage_coll::collection::PeerageCollection;
use crate::item::Item;

#[derive(Clone, Copy)]
pub struct PeerageMap<K: Key, V: Clone + Copy + Default> {
    default_len: usize,
    loader: f32, 
    table: PeerageCollection<Item<K, V>>,
}


impl<K: Key, V: Clone + Copy + Default> PeerageMap<K, V> {
    pub fn put(key: K, value: V) {
        let index = key
    }
}