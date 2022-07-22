use std::collections::{BTreeMap,HashMap};
use peerage_utils::bin_utils::Nibble;

lazy_static! {
    pub static ref BASE64_TABLE: BTreeMap<u8, char> = {
        let mut btm = BTreeMap::<u8, char>::new();

        
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        for (u, c) in characters.char_indices() {
            btm.insert(u as u8, c);
        }

        btm
    }; 

    static ref NUM_STRS: Vec<String> = {
        let mut v: Vec<String> = vec![];

        v.push("AZDA".to_string());
        v.push("DBDA".to_string());
        v.push("QAZH".to_string());
        v.push("NMSA".to_string());
        v.push("BIDU".to_string());
        v.push("UOPL".to_string());
        v.push("BNHJ".to_string());
        v.push("QWRY".to_string());
        v.push("ZXCD".to_string());
        v.push("GJUI".to_string());
        v.push("TRWH".to_string());
        v.push("BVSJ".to_string());
        v.push("BTET".to_string());
        v.push("RYTG".to_string());
        v.push("BJHT".to_string());
        v.push("NBVX".to_string());



        v
    };

    pub static ref BASE_128_STRS: HashMap<Nibble, String> = {
        let mut hm = HashMap::<Nibble, String>::new();

        for i in 0u8..16 {
            let key = Nibble::from_4_bit_number(i);
            let value = &NUM_STRS[i as usize];

            hm.insert(key, value.clone());
        }

        hm
    };

    pub static ref BASE_128_NIBBLES: HashMap<String, Nibble> = {
        let mut hm = HashMap::<String, Nibble>::new();

        for i in 0u8..16 {
            let value = Nibble::from_4_bit_number(i);
            let key = &NUM_STRS[i as usize];

            hm.insert(key.clone(), value);
        }

        hm
    };
}