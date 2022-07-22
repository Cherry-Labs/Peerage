use peerage_utils::bin_utils::QuadrupleWord;
use peerage_rand::mersenne_twister::MerseneTwisterRand;

pub struct KeyPhrase {
    key_str: String,
    key: [QuadrupleWord; 8],
    rand_element: QuadrupleWord,

}

pub type ResultKey = std::result::Result<KeyPhrase, ()>;

impl KeyPhrase {
    pub fn from_str(s: String) -> ResultKey {
        if s.len() != 128 {
            Err(())
        }

        let key = [QuadrupleWord::default(); 8];
        
        let mut ind = 0usize;

        for i in (0..128).step_by(16) {
            let sub_str = &s[i..i + 16];

            key[ind] = QuadrupleWord::from_string(sub_str.to_string());

            ind += 1;
        }

        let key_str = s.clone();

        let rand_element = MerseneTwisterRand::new().rng();

        Ok(Self { key_str, key, rand_element })


    }
}