use std::collections::{BTreeMap,HashMap};
use peerage_utils::bin_utils::QuadrupleWord;

lazy_static! {
    pub static ref BASE64_TABLE: BTreeMap<u8, char> = {
        let mut btm = BTreeMap::<u8, char>::new();

        
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        for (u, c) in characters.char_indices() {
            btm.insert(u as u8, c);
        }

        btm
    }; 

    pub static ref BASE_128_STRS: HashMap<QuadrupleWord, String> = {
        let mut hm = HashMap::<QuadrupleWord, String>::new();

        for i in 0u128..128 {
            let added_nums = match i < 124 {
                true => (2, 3),
                false => (0, 0)
            };
            
            let first_key = (i as u8) % 64;
            let second_key = ((i + added_nums.0) as u8) % 64;
            let third_key = ((i + added_nums.1) as u8) % 64;
        
            let char1 = BASE64_TABLE[&first_key];
            let char2 = BASE64_TABLE[&second_key];
            let char3 = BASE64_TABLE[&third_key];
            
            let value = format!("{}{}{}", char1, char2, char3);
        
            let key = QuadrupleWord::from_u128(i);

            hm.insert(key, value);
        
        }

        hm
    };
}