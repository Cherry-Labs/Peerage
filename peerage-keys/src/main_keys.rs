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


impl KeyTrait for Key {}
