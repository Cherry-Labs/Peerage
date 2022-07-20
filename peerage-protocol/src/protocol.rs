use std::net::SocketAddr;
use peerage_utils::bin_utils::QuadrupleWord;

#[derive(Clone)]
pub enum LoadType {
    Ledger,
    StorageData,
    EncryptedData
}

impl Into<u8> for LoadType {
    fn into(self) -> u8 {
        match self {
            LoadType::Ledger => 0,
            LoadType::StorageData => 1,
            LoadType::EncryptedData => 2,
        }
    }
}

#[derive(Clone)]
pub struct PeerageProtocol {
    load_type: LoadType,
    data_len: usize,
    client: SocketAddr,
    servers: Vec<SocketAddr>,
    public_key: (QuadrupleWord, 
                        QuadrupleWord,
                        QuadrupleWord,
                        QuadrupleWord),
    
}