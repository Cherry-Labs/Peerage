use peerage_utils::traits::Key;

#[derive(Clone, Copy, Default)]
pub struct Item<'a, K: Key, V: Clone + Copy + Default> {
    key: K,
    value: V,
    next: Option<&'a Item<'a, K, V>>
}

impl<'a, K: Key, V: Clone + Copy + Default> Item<'a, K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value, next: None }
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn get_value(&self) -> &V {
        &self.value
    }

    pub fn mutate_next(&mut self, next: &'a Item<K, V> ) {
        self.next =  Some(next)
    }

    pub fn mutate_value(&mut self, value: V) {
        self.value = value;
    }
    
}

pub type OptItem<'a, K, V> = Option<Item<'a, K, V>>; 


