use std::ops::Add;

use peerage_hashmap::hashmap::PeerageMap;
use peerage_holder::holder::Holder;
use peerage_rand::quadrupleword::RandomQuadrupleWord;
use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::QuadrupleWord;
use peerage_utils::traits::Key;
use crate::node_type::RNodeType;

use crate::node::{RTreeNode, self};

#[derive(Clone, Copy, Default)]
pub struct ShelfKey(QuadrupleWord);

impl ShelfKey {
    pub fn new(quadruple_word: QuadrupleWord) -> Self {
        ShelfKey(quadruple_word)
    }

    pub fn unwrap(&self) -> QuadrupleWord {
        let ShelfKey(item) = *self;

        item
    }

}

impl Key for ShelfKey {
    fn get_key_index(&self, size: usize) -> usize {
        let self_qw = self.unwrap();
        let self_u128 = self_qw.into_u128();

        (self_u128 % (size as u128)) as usize
    }
}

impl PartialEq for ShelfKey {
    fn eq(&self, other: &Self) -> bool {
        let self_unwrap = self.unwrap();
        let other_unwrap = other.unwrap();

        self_unwrap == other_unwrap
        
    }
}

impl Eq for ShelfKey {}



#[derive(Clone, Copy, Default)]
pub struct ShelfKeyGen {
    generated_keys: PeerageCollection<ShelfKey>,
    rand_gen: RandomQuadrupleWord,
    last_key: ShelfKey,
    rand_nums: PeerageCollection<QuadrupleWord>,
}

impl ShelfKeyGen {
    pub fn new() -> Self {
        let generated_keys = PeerageCollection::<ShelfKey>::default();
        let rand_gen = RandomQuadrupleWord::new();
        let last_key = ShelfKey::default();
        let rand_nums = PeerageCollection::<QuadrupleWord>::default();

        Self { generated_keys, rand_gen, last_key, rand_nums }
    }

    pub fn new_key(&mut self) -> ShelfKey {
        let mut rand_qw = self.rand_gen.rng();

        loop {
            if self.rand_nums.contains(rand_qw) {
                rand_qw = self.rand_gen.rng();
            } else {
                self.rand_nums.push(rand_qw);
                break;
            }
        }

        let key = ShelfKey::new(rand_qw);

        self.last_key = key;

        self.generated_keys.push(key);
        
        key
    }
}

#[derive(Clone, Copy, Default)]
pub struct Address(PeerageCollection<ShelfKey>);

impl Address {
    pub fn from_vec(v: Vec<ShelfKey>) -> Self {
        let coll = PeerageCollection::from_vector(v);

        Address(coll)
    }

    fn unwrap(&self) -> PeerageCollection<ShelfKey> {
        let Address(item) = *self;
   
        item
    }


    pub fn navigate_to<'a>(&self, parent_node: RNodeType<'a>) -> Option<RNodeType<'a>> {
        let node_kv = parent_node.get_kvs();

        if node_kv.is_none() {
            return None;
        }

        let items = self.unwrap();

        let mut items_iter = items.into_iter();

        let mut curr_node = RNodeType::new_empty();
        let mut curr_kv = node_kv.unwrap();

        for _ in 0..items_iter.clone().count() {
            let key = items_iter.next().unwrap();

            curr_node = *curr_kv[key].get_refer_ref().unwrap();
            
            let node_kv_list = curr_node.get_kvs();

            if node_kv_list.is_none() {
                break;
            }

            curr_kv = node_kv_list.unwrap();

        }

        Some(curr_node)

    }
}