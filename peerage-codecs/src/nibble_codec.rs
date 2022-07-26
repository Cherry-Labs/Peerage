use peerage_utils::bin_utils::*;
use crate::constant_values::{BASE_128_STRS, BASE_128_NIBBLES};
use peerage_coll::collection::PeerageCollection;
use peerage_macros::coll;
use peerage_rand::rand::*;

#[derive(Clone, Copy)]
pub struct NibbleCodec {
    quadruple_words: PeerageCollection<QuadrupleWord>,
    double_words: PeerageCollection<DoubleWord>,
    byte_words: PeerageCollection<ByteWord>,
    bytes: PeerageCollection<Byte>,
    sessets: PeerageCollection<Sesset>,
    nibbles: PeerageCollection<Nibble>,

}


impl NibbleCodec {
    pub fn new(
        quadruple_words: PeerageCollection<QuadrupleWord>,
        double_words: PeerageCollection<DoubleWord>,
        byte_words: PeerageCollection<ByteWord>,
        bytes: PeerageCollection<Byte>,
        sessets: PeerageCollection<Sesset>,
        nibbles: PeerageCollection<Nibble>,
    ) -> Self {
        Self { 
            quadruple_words,
            double_words,
            byte_words,
            bytes,
            sessets,
            nibbles
         }
    }

    pub fn from_vec(
        quadruple_words: Vec<QuadrupleWord>,
        double_words: Vec<DoubleWord>,
        byte_words: Vec<ByteWord>,
        bytes: Vec<Byte>,
        sessets: Vec<Sesset>,
        nibbles: Vec<Nibble>,
    ) -> Self
    {
        Self::new(
            quadruple_words.into(),
            double_words.into(),
         byte_words.into(),
            bytes.into(),
            sessets.into(),
            nibbles.into()
        )
    }

    pub fn new_random() -> Self {
        let quadruple_words: PeerageCollection<QuadrupleWord> = coll![RandomQuadrupleWord::rng_inner();128];
        let double_words: PeerageCollection<DoubleWord> = coll![RandomDoubleWord::rng_inner();64];
        let byte_words: PeerageCollection<ByteWord> = coll![RandomByteWord::rng_inner();32];
        let bytes: PeerageCollection<Byte> = coll![RandomByte::rng_inner();8];
        let sessets: PeerageCollection<Sesset> = coll![RandomSesset::rng_inner();6];
        let nibbles: PeerageCollection<Nibble> = coll![RandomNibble::rng_inner();4];

        Self { 
            quadruple_words,
            double_words,
            nibbles, 
            byte_words, 
            sessets,
            bytes
         }

    
    }

    pub fn get_qw(&self) -> PeerageCollection<QuadrupleWord> {
        self.quadruple_words.clone()
    }

    pub fn get_dw(&self) -> PeerageCollection<DoubleWord> {
        self.double_words.clone()
    }

    pub fn get_bw(&self) -> PeerageCollection<ByteWord> {
        self.byte_words.clone()
    }

    pub fn get_by(&self) -> PeerageCollection<Byte> {
        self.bytes.clone()
    }

    pub fn get_se(&self) -> PeerageCollection<Sesset> {
        self.sessets.clone()
    }

    pub fn get_ni(&self) -> PeerageCollection<Nibble> {
        self.nibbles.clone()
    }

    pub fn encode(&self) -> String {
        let mut qw_nibbles: Vec<Nibble> = vec![];
        let mut dw_nibbles: Vec<Nibble> = vec![];
        let mut bw_nibbles: Vec<Nibble> = vec![];
        let mut by_nibbles: Vec<Nibble> = vec![];
        let mut se_nibbles: Vec<Nibble> = vec![];
        let mut ni_nibbles: Vec<Nibble> = vec![];


        
        let qw_unwrap = self.quadruple_words.clone();

        for qw in qw_unwrap {
            let qw_nibbles_g = qw.into_nibbles();

            qw_nibbles.extend(qw_nibbles_g.to_vec())
        }

        let dw_unwrap = self.double_words.clone();

        for qw in dw_unwrap {
            let dw_nibbles_g = qw.into_nibbles();

            dw_nibbles.extend(dw_nibbles_g.to_vec())
        }        

        let bw_unwrap = self.byte_words.clone();

        for bw in bw_unwrap {
            let bw_nibbles_g = bw.into_nibble();

            bw_nibbles.extend(bw_nibbles_g.to_vec());
        }

        let by_unwrap = self.bytes.clone();

        for by in by_unwrap {
            let by_nibble_g = by.into_nibble();

            by_nibbles.extend(by_nibble_g.to_vec());
        }

        let se_unwrap = self.sessets.clone();

        for se in se_unwrap {
            let se_nibble_g = se.into_nibble();

            se_nibbles.extend(se_nibble_g.to_vec());
        }



        let nibbles_unwrap = self.nibbles.clone();

        for nib in nibbles_unwrap {
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

        let dw_str = {
            let mut s = String::new();

            for nib in dw_nibbles {
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


        let se_str = {
            let mut s = String::new();

            for nib in se_nibbles {
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

        format!("CDS___QWS__{qw_str}__DWS__{dw_str}__BWS__{bw_str}__BYS__{by_str}__SES__{se_str}__NIS__{ni_str}__CDE")
    }

    
    pub fn decodec(s: String) -> Self {

     
        struct Insert {
            quadruple_words: Vec<QuadrupleWord>,
            double_words: Vec<DoubleWord>,
            byte_words: Vec<ByteWord>,
            sessets: Vec<Sesset>,
            bytes: Vec<Byte>,
            nibbles: Vec<Nibble>,
            qw_nibbles: Vec<Nibble>,
            dw_nibbles: Vec<Nibble>,
            bw_nibbles: Vec<Nibble>,
            by_nibbles: Vec<Nibble>,
            se_nibbles: Vec<Nibble>,
        }

        impl Insert {            
            pub fn new() -> Self {
                let quadruple_words: Vec<QuadrupleWord> = vec![];
                let double_words: Vec<DoubleWord> = vec![];
                let byte_words: Vec<ByteWord> = vec![];
                let bytes: Vec<Byte> = vec![];
                let sessets: Vec<Sesset> = vec![];
                let nibbles: Vec<Nibble> = vec![];
                let qw_nibbles: Vec<Nibble> = vec![];
                let dw_nibbles: Vec<Nibble> = vec![];
                let bw_nibbles: Vec<Nibble> = vec![];
                let by_nibbles: Vec<Nibble> = vec![];
                let se_nibbles: Vec<Nibble> = vec![];

                Self { 
                    quadruple_words, 
                    double_words,
                    byte_words, 
                    bytes, 
                    sessets,
                    nibbles,
                    qw_nibbles,
                    dw_nibbles,
                    bw_nibbles,
                    by_nibbles,
                    se_nibbles,
                }

            }

            pub fn insert_into_quadruple_word(&mut self, new: QuadrupleWord) {
                self.quadruple_words.push(new)
            }

            pub fn insert_into_double_word(&mut self, new: DoubleWord) {
                self.double_words.push(new)
            }

            pub fn insert_into_byte_word(&mut self, new: ByteWord) {
                self.byte_words.push(new)
            }

            pub fn insert_into_byte(&mut self, new: Byte) {
                self.bytes.push(new)
            }

            pub fn insert_into_sesset(&mut self, new: Sesset) {
                self.sessets.push(new)
            }

            pub fn insert_into_nibble(&mut self, new: Nibble) {
                self.nibbles.push(new)
            }

            pub fn insert_into_qw_nibble(&mut self, new: Nibble) {
                self.qw_nibbles.push(new)
            }

            pub fn insert_into_dw_nibble(&mut self, new: Nibble) {
                self.dw_nibbles.push(new)
            }

            pub fn insert_into_bw_nibble(&mut self, new: Nibble) {
                self.bw_nibbles.push(new)
            }

            pub fn insert_into_by_nibble(&mut self, new: Nibble) {
                self.by_nibbles.push(new)
            }

            pub fn insert_into_se_nibble(&mut self, new: Nibble) {
                self.se_nibbles.push(new)
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

            pub fn check_dw_nibble_insert(&mut self) {
                if self.dw_nibbles.len() == 16 {
                    let mut nib_arr = [Nibble::new_zeros(); 16];

                    for i in 0usize..16usize {
                        nib_arr[i] = self.dw_nibbles[i];
                    }

                    let new = DoubleWord::from_nibble(nib_arr);

                    self.insert_into_double_word(new);

                    self.dw_nibbles = vec![];
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

            pub fn check_se_nibble_insert(&mut self) {
                if self.by_nibbles.len() == 2 {
                    let mut nib_arr = [Nibble::new_zeros(); 2];

                    for i in 0usize..2usize {
                        nib_arr[i] = self.by_nibbles[i];
                    }

                    let new = Sesset::from_nibble(nib_arr);

                    self.insert_into_sesset(new);

                    self.se_nibbles = vec![];
                }
            }

            pub fn into_codec(&self) -> NibbleCodec {
                let quadruple_words = PeerageCollection::from_vector(self.quadruple_words.clone());
                let double_words = PeerageCollection::from_vector(self.double_words.clone());
                let byte_words = PeerageCollection::from_vector(self.byte_words.clone());
                let bytes = PeerageCollection::from_vector(self.bytes.clone());
                let sessets = PeerageCollection::from_vector(self.sessets.clone());
                let nibbles = PeerageCollection::from_vector(self.nibbles.clone());


                NibbleCodec::new(
                    quadruple_words, 
                    double_words,
                    byte_words,
                    bytes,
                    sessets,
                    nibbles
                )
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
                        "DWS" => {
                            let items = s_split.next().unwrap();

                            let mut items_split = items.split("_");

                            for _ in 0..items_split.clone().count() {
                                let item_unwrapped = items_split.next().unwrap().to_string();

                                let nibble = &BASE_128_NIBBLES[&item_unwrapped];

                                insert.insert_into_dw_nibble(*nibble);
                                insert.check_dw_nibble_insert();
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
                        "SES" => {
                            let items = s_split.next().unwrap();

                            let mut items_split = items.split("_");

                            for _ in 0..items_split.clone().count() {
                                let item_unwrapped = items_split.next().unwrap().to_string();

                                let nibble = &BASE_128_NIBBLES[&item_unwrapped];

                                insert.insert_into_se_nibble(*nibble);
                                insert.check_se_nibble_insert();
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