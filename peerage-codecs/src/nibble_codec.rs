use peerage_utils::bin_utils::{QuadrupleWord, Nibble};
use crate::constant_values::{BASE_128_STRS, BASE_128_NIBBLES};


pub struct NibbleCodec(Vec<QuadrupleWord>);


impl NibbleCodec {
    fn new(v: Vec<QuadrupleWord>) -> Self {
        NibbleCodec(v)
    }

    fn unwrap(&self) -> Vec<QuadrupleWord> {
        let NibbleCodec(item) = self;

        item.clone()
    }

    fn encode_inner(&self) -> String {
        let self_unwrap = self.unwrap();

        let mut ret = String::new();

        for v in self_unwrap {
            let v_nibbles = v.into_nibbles();

            for n in v_nibbles {
                ret = format!("{}{}", ret, &BASE_128_STRS[&n]);
            }
        }

        ret
    }
    
    pub fn decodec(s: String) -> Vec<QuadrupleWord> {
        let mut nibbles: Vec<Nibble> = vec![];
        
        for i in (0..s.len()).step_by(4) {
            let s_slice = &s[i..i + 4].to_string();

            let nibble = &BASE_128_NIBBLES[s_slice];

            nibbles.push(nibble.clone());
        }

        let mut qws: Vec<QuadrupleWord> = vec![];

        for i in (0usize..nibbles.len()).step_by(32) {
            let slice_nibble = nibbles[i..i + 32].to_vec();

            let qw = QuadrupleWord::from_nibble(slice_nibble);

            qws.push(qw);
        }

        qws
    }


}