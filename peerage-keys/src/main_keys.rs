use peerage_utils::traits::Key as KeyTrait;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Key {
    Init,
}



impl Default for Key {
    fn default() -> Self {
        Self::Init
    }
}


impl KeyTrait for Key {
    fn get_key_index(&self, size: usize) -> usize {
        todo!()
    }
}
