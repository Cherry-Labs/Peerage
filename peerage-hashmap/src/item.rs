use std::borrow::Borrow;

use peerage_utils::traits::Key;
use peerage_holder::holder::Holder;

#[derive(Clone, Copy, Default)]
pub struct Item<'a, K: Key, V: Clone + Copy + Default> {
    key: K,
    value: V,
    next: Holder<Item<'a, K, V>>,
}

impl<'a, K: Key, V: Clone + Copy + Default> Item<'a, K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value, next: Holder::Nil }
    }

    pub fn new_with_next(
        key: K, 
        value: V, 
        next: Item<'a, K, V>
    ) -> Self 
    {
        let next_holder = Holder::refer_from_item(next);

        Self { key, value, next: next_holder }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn get_value(&self) -> &V {
        &self.value
    }

    pub fn get_value_box(&self) -> Box<V> {
        Box::new(self.value)
    }

    pub fn mutate_next(&mut self, next: Item<'a, K, V>) {
        self.next = Holder::refer_from_item(next);
    }

    pub fn mutate_value(&mut self, value: V) {
        self.value = value;
    }
    
    pub fn get_value_in_array(&self) -> [V; 1] {
        [self.value; 1]
    }

    pub fn get_next(&self) -> Option<&Item<'a, K, V>> {
        self.next.get_refer_ref()
    }

    pub fn return_self_as_array(&self) -> [Self; 1] {
        [self.clone(); 1]
    }

    pub fn get_value_holder(&self) -> Holder<V> {
        let holder_value = Holder::refer_from_item(self.value);

        holder_value
    }

}

pub type OptItem<'a, K, V> = Option<Item<'a, K, V>>; 


