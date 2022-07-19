use std::borrow::Borrow;

use peerage_coll::collection::PeerageCollection;
use peerage_holder::holder::Holder;
use crate::item::Item;
use peerage_utils::traits::Key;

#[derive(Clone, Copy)]
pub struct PeerageMap<'a, K: Key, V: Clone + Copy + Default> {
    len: usize,
    table: PeerageCollection<Option<Item<'a, K, V>>>,
    curr_ret: [V; 1]
}


impl<'a, K: Key, V: Clone + Copy + Default> PeerageMap<'a, K, V> {
    pub fn new() -> Self {
        let len = 1024usize;
        let table = PeerageCollection::<Option<Item<'a, K, V>>>::new_i0_from_item(None);
        let curr_ret = [V::default(); 1];

        Self { len, table, curr_ret }
    }

    
    pub fn put(&mut self, key: K, value: V) {
        let index = key.get_key_index(self.len);

        let entry = self.table.get_at(index);

        if entry.is_none() {
            return ;
        }

        match entry.unwrap() {
            Some(mut next) => {
                let mut item = Item::<'a, K, V>::new(key, value);

                item.mutate_next(next);

                self.table[index] = Some(item);            
            },
            None => {
                let new_item = Item::new(key, value);

                self.table[index] = Some(new_item);

                self.len += 1;
            },
        }

        self.resize();

    }

    fn avoid_collision(&self, key: K, item: Item<'a, K, V>) -> Holder<V> {
        if key == *item.get_key() {{
            let value =  item.get_value_holder();

            return value;
        }} else {
            if item.get_next().is_some() {
                let item_next_unwrapped = item.get_next().unwrap().clone();
                return self.avoid_collision(key, item_next_unwrapped)
            }
        }

        item.get_value_holder()

    }

    

    fn resize(&mut self) {
        if !self.table.all_occupied(None) {
            return ;
        }

        let old_indices = self.table.into_iter().map(|x| {
            match x {
                Some(item) => item.get_key().get_key_index(self.len) as isize,
                None => -1,
            }
        }).collect::<Vec<isize>>();

        let new_indices = self.table.into_iter().map(|x| {
            match x {
                Some(item) => item.get_key().get_key_index(self.len + 1024) as isize,
                None => -1,
            }
        }).collect::<Vec<isize>>();

        self.table.take_size_to_next_level(None, old_indices, new_indices)
    }

    pub fn get(&mut self, key: K) -> Holder<V> {
        let index = key.get_key_index(self.len);

        if self.table[index].is_none() {
            return Holder::Nil;
        }

        let item = self.table[index].unwrap();

        let key_gotten = self.avoid_collision(key, item);

        let key_ref = key_gotten.get_refer_ref();

        self.curr_ret[0] = key_ref.unwrap().clone();

        key_gotten
    }

 

}



impl<'a, K: Key, V: Clone + Copy + Default> std::ops::Index<K> for PeerageMap<'a, K, V> {
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        &self.curr_ret[0]
    }
}

impl<'a, K: Key, V: Clone + Copy + Default> std::ops::IndexMut<K> for PeerageMap<'a, K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.curr_ret[0]
    }
}