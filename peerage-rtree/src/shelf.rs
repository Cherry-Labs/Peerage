use peerage_hashmap::hashmap::PeerageMap;
use peerage_hash::hasher::PeerageHash;


#[derive(Clone, Copy)]
pub struct ShelfKey(PeerageHash);

impl ShelfKey {
    pub fn new() -> Self {
        let hash = PeerageHash::new()
    }
}

