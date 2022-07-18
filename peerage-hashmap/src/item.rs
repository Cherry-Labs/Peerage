use peerage_utils::traits::Key;

pub struct Item<K: Key, V> {
    key: K,
    value: V,
}

impl 