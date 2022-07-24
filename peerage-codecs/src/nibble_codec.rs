use peerage_utils::bin_utils::{QuadrupleWord, Nibble, ByteWord, Byte};
use crate::constant_values::{BASE_128_STRS, BASE_128_NIBBLES};
use peerage_coll::collection::PeerageCollection;
use peerage_macros::coll;
use peerage_rand::rand::{RandomByte, RandomByteWord, RandomNibble, RandomQuadrupleWord};

#[derive(Clone, Copy)]
pub struct NibbleCodec {
    quadruple_words: PeerageCollection<QuadrupleWord>,
    nibbles: PeerageCollection<Nibble>,
    byte_words: PeerageCollection<ByteWord>,
    bytes: PeerageCollection<Byte>
}


impl NibbleCodec {
    pub fn new(
        quadruple_words: PeerageCollection<QuadrupleWord>,
        byte_words: PeerageCollection<ByteWord>,
        bytes: PeerageCollection<Byte>,
        nibbles: PeerageCollection<Nibble>,

    ) -> Self {
        Self { 
            quadruple_words,
            nibbles, 
            byte_words, 
            bytes
         }
    }

    pub fn from_vec(
        quadruple_words: Vec<QuadrupleWord>,
        byte_words: Vec<ByteWord>,
        bytes: Vec<Byte>,
        nibbles: Vec<Nibble>,
    ) -> Self
    {
        Self::new(
            quadruple_words.into(),
            byte_words.into(),
            bytes.into(),
            nibbles.into()
        )
    }

    pub fn new_random() -> Self {
        let quadruple_words: PeerageCollection<QuadrupleWord> = coll![RandomQuadrupleWord::rng_inner();32];
        let byte_words: PeerageCollection<ByteWord> = coll![RandomByteWord::rng_inner();16];
        let bytes: PeerageCollection<Byte> = coll![RandomByte::rng_inner();8];
        let nibbles: PeerageCollection<Nibble> = coll![RandomNibble::rng_inner();4];

        Self { 
            quadruple_words,
            nibbles, 
            byte_words, 
            bytes
         }

    
    }

    pub fn get_qw(&self) -> PeerageCollection<QuadrupleWord> {
        self.quadruple_words.clone()
    }

    pub fn get_bw(&self) -> PeerageCollection<ByteWord> {
        self.byte_words.clone()
    }

    pub fn get_by(&self) -> PeerageCollection<Byte> {
        self.bytes.clone()
    }

    pub fn get_ni(&self) -> PeerageCollection<Nibble> {
        self.nibbles.clone()
    }

    pub fn encode(&self) -> String {
        let mut qw_nibbles: Vec<Nibble> = vec![];
        let mut bw_nibbles: Vec<Nibble> = vec![];
        let mut by_nibbles: Vec<Nibble> = vec![];
        let mut ni_nibbles: Vec<Nibble> = vec![];


        
        let qw_unwrap = self.quadruple_words.clone();

        for qw in qw_unwrap {
            let qw_nibbles_g = qw.into_nibbles();

            qw_nibbles.extend(qw_nibbles_g.to_vec())
        }
        

        let bw_unwrap = self.byte_words.clone();

        for bw in bw_unwrap {
            let bw_nibbles_g = bw.into_nibble();

            bw_nibbles.extend(bw_nibbles_g.to_vec());
        }

        let by_cloned = self.bytes.clone();

        for by in by_cloned {
            let by_nibble_g = by.into_nibble();

            by_nibbles.extend(by_nibble_g.to_vec());
        }


        let nibbles_cloned = self.nibbles.clone();

        for nib in nibbles_cloned {
            ni_nibbles.push(nib)
        }

        let qw_str = {
            let mut s = String::new();

            for nib in qw_nibbles {
                let str = &BASE_128_STRS[&nib];
                s = format!("{}_{}_", s, str);
            }

            s
        };

        let bw_str = {
            let mut s = String::new();

            for nib in bw_nibbles {
                let str = &BASE_128_STRS[&nib];
                s = format!("{}_{}_", s, str);
            }

            s
        };

        let by_str = {
            let mut s = String::new();

            for nib in by_nibbles {
                let str = &BASE_128_STRS[&nib];
                s = format!("{}_{}_", s, str);
            }

            s
        };


        let ni_str = {
            let mut s = String::new();

            for nib in ni_nibbles {
                let str = &BASE_128_STRS[&nib];
                s = format!("{}_{}_", s, str);
            }

            s
        };

        format!("CDS___QWS__{qw_str}__BWS__{bw_str}__BYS__{by_str}__NIS__{ni_str}__CDE")
    }

    
    pub fn decodec(s: String) -> Self {

     
        struct Insert {
            quadruple_words: Vec<QuadrupleWord>,
            byte_words: Vec<ByteWord>,
            bytes: Vec<Byte>,
            nibbles: Vec<Nibble>,
            qw_nibbles: Vec<Nibble>,
            bw_nibbles: Vec<Nibble>,
            by_nibbles: Vec<Nibble>
        }

        impl Insert {            
            pub fn new() -> Self {
                let quadruple_words: Vec<QuadrupleWord> = vec![];
                let byte_words: Vec<ByteWord> = vec![];
                let bytes: Vec<Byte> = vec![];
                let nibbles: Vec<Nibble> = vec![];
                let qw_nibbles: Vec<Nibble> = vec![];
                let bw_nibbles: Vec<Nibble> = vec![];
                let by_nibbles: Vec<Nibble> = vec![];

                Self { 
                    quadruple_words, 
                    byte_words, 
                    bytes, 
                    nibbles,
                    qw_nibbles,
                    bw_nibbles,
                    by_nibbles,
                }

            }

            pub fn insert_into_quadruple_word(&mut self, new: QuadrupleWord) {
                self.quadruple_words.push(new)
            }

            pub fn insert_into_byte_word(&mut self, new: ByteWord) {
                self.byte_words.push(new)
            }

            pub fn insert_into_byte(&mut self, new: Byte) {
                self.bytes.push(new)
            }

            pub fn insert_into_nibble(&mut self, new: Nibble) {
                self.nibbles.push(new)
            }

            pub fn insert_into_qw_nibble(&mut self, new: Nibble) {
                self.qw_nibbles.push(new)
            }

            pub fn insert_into_bw_nibble(&mut self, new: Nibble) {
                self.bw_nibbles.push(new)
            }

            pub fn insert_into_by_nibble(&mut self, new: Nibble) {
                self.by_nibbles.push(new)
            }

            pub fn check_qw_nibble_insert(&mut self) {
                if self.qw_nibbles.len() == 32 {
                    let mut nib_arr = [Nibble::new_zeros(); 32];

                    for i in 0usize..32usize {
                        nib_arr[i] = self.qw_nibbles[i];
                    }

                    let new = QuadrupleWord::from_nibble(nib_arr);

                    self.insert_into_quadruple_word(new);

                    self.qw_nibbles = vec![];
                }
            }

            pub fn check_bw_nibble_insert(&mut self) {
                if self.bw_nibbles.len() == 8 {
                    let mut nib_arr = [Nibble::new_zeros(); 8];

                    for i in 0usize..8usize {
                        nib_arr[i] = self.bw_nibbles[i];
                    }

                    let new = ByteWord::from_nibble(nib_arr);

                    self.insert_into_byte_word(new);

                    self.bw_nibbles = vec![];
                }
            }

            pub fn check_by_nibble_insert(&mut self) {
                if self.by_nibbles.len() == 2 {
                    let mut nib_arr = [Nibble::new_zeros(); 2];

                    for i in 0usize..2usize {
                        nib_arr[i] = self.by_nibbles[i];
                    }

                    let new = Byte::from_nibble(nib_arr);

                    self.insert_into_byte(new);

                    self.by_nibbles = vec![];
                }
            }

            pub fn into_codec(&self) -> NibbleCodec {
                let quadruple_words = PeerageCollection::from_vector(self.quadruple_words.clone());
                let byte_words = PeerageCollection::from_vector(self.byte_words.clone());
                let bytes = PeerageCollection::from_vector(self.bytes.clone());
                let nibbles = PeerageCollection::from_vector(self.nibbles.clone());


                NibbleCodec::new(quadruple_words, byte_words, bytes, nibbles)
            }

        }
 
        let mut insert = Insert::new();
       
        let mut s_split = s.split("__");

        loop {
            match s_split.next() {
                Some(item) => {
                    match item {
                        "QWS" => {
                            let items = s_split.next().unwrap();

                            let mut items_split = items.split("_");

                            for _ in 0..items_split.clone().count() {
                                let item_unwrapped = items_split.next().unwrap().to_string();

                                let nibble = &BASE_128_NIBBLES[&item_unwrapped];

                                insert.insert_into_qw_nibble(*nibble);
                                insert.check_qw_nibble_insert();
                            }
                        },
                        "BWS" => {
                            {
                                let items = s_split.next().unwrap();
    
                                let mut items_split = items.split("_");
    
                                for _ in 0..items_split.clone().count() {
                                    let item_unwrapped = items_split.next().unwrap().to_string();
    
                                    let nibble = &BASE_128_NIBBLES[&item_unwrapped];
    
                                    insert.insert_into_bw_nibble(*nibble);
                                    insert.check_bw_nibble_insert();
                                }
                            }
                        },
                        "BYS" => {
                            {
                                let items = s_split.next().unwrap();
    
                                let mut items_split = items.split("_");
    
                                for _ in 0..items_split.clone().count() {
                                    let item_unwrapped = items_split.next().unwrap().to_string();
    
                                    let nibble = &BASE_128_NIBBLES[&item_unwrapped];
    
                                    insert.insert_into_by_nibble(*nibble);
                                    insert.check_by_nibble_insert();
                                }
                            }
                        },
                        "NIS" => {
                            {
                                let items = s_split.next().unwrap();
    
                                let mut items_split = items.split("_");
    
                                for _ in 0..items_split.clone().count() {
                                    let item_unwrapped = items_split.next().unwrap().to_string();
    
                                    let nibble = &BASE_128_NIBBLES[&item_unwrapped];
    
                                    insert.insert_into_nibble(*nibble)
                                }
                            }
                        },
                        _ => ()
                    }
                },
                None => break,
            }
        }

        insert.into_codec()

    }

}