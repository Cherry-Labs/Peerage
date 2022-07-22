use peerage_utils::bin_utils::QuadrupleWord;
use peerage_rand::mersenne_twister::MerseneTwisterRand;

pub struct KeyPhrase {
    key_str: String,
    key: [QuadrupleWord; 2],
    rand_element: QuadrupleWord,

}


impl KeyPhrase {
    pub fn from_str(s: String) -> Self {
        let key_str = match s.len() > 32 {
            true => s[..32].to_string(),
            false => {
                let pad = vec!["0"; 32 - s.len()].join("");

                let s_padded = format!("{}{}", s, pad);


                s_padded.to_string()
            
            },
        };


        let s1 = &key_str[..16];
        let s2 = &key_str[16..];


        let key_1 = QuadrupleWord::from_string(s1.to_string());
        let key_2 = QuadrupleWord::from_string(s2.to_string());


        let key = [key_1, key_2];

        let mut rand_gen = MerseneTwisterRand::new();

        let rand_element = rand_gen.rng();


        Self { key_str, key, rand_element }


    }
}