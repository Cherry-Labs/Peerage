use peerage_utils::bin_utils::QuadrupleWord;
use peerage_rand::quadrupleword::RandomQuadrupleWord;
use peerage_codecs::nibble_codec::NibbleCodec;


#[derive(Clone, Copy)]
pub struct KeyPhrase([QuadrupleWord; 8]);

impl KeyPhrase {
    pub fn from_str(s: String) -> Self {
        if s.len() != 128 {
            panic!("Key length must be equal to 128")
        }
        let rand_element = RandomQuadrupleWord::new().rng();

        let mut key = [QuadrupleWord::default(); 8];
                let rand_element = RandomQuadrupleWord::new().rng();

        let mut ind = 0usize;

        for i in (0..128).step_by(16) {
            let sub_str = &s[i..i + 16];

            key[ind] = QuadrupleWord::from_string(sub_str.to_string()) + rand_element;

            ind += 1;
        }

        let key_str = s.clone();


        KeyPhrase(key)
    }

    pub fn from_nibble_codec(s: String) -> Self {
        let vec_qw = NibbleCodec::decodec(s);

        let mut key = [QuadrupleWord::default(); 8];

        for i in 0..vec_qw.len() {
            key[i] = vec_qw[i];
        }

        KeyPhrase(key)
    }

    pub fn encde_nibble(&self) -> String {
        let KeyPhrase(self_keys) = self;
        let self_key_vec = self_keys.to_vec();

        NibbleCodec::encode(self_key_vec)
    }
}