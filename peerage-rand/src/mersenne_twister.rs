use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::QuadrupleWord;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MerseneTwisterRand {
    mt: PeerageCollection<QuadrupleWord>,
    index: usize,
    lower_mask: QuadrupleWord,
    upper_mask: QuadrupleWord,
    f: QuadrupleWord,
    w: usize,
    m: QuadrupleWord,
    r: QuadrupleWord,
    a: QuadrupleWord,

}

impl MerseneTwisterRand {
    pub fn new() -> Self {
        let seed_quadrupleword = Self::get_seed();
        let mt = PeerageCollection::<QuadrupleWord>::new_i0_from_item(seed_quadrupleword);
        let index = 1024usize + 1;
        let f = QuadrupleWord::from_usize(500);
        let w = 128usize;
        let m = QuadrupleWord::from_usize(64);
        let r = QuadrupleWord::from_usize(80);
        let a = QuadrupleWord::from_usize(10);
        let lower_mask = (r >> 1) - QuadrupleWord::from_usize(1);
        let upper_mask = seed_quadrupleword - lower_mask; 

        Self { 
            mt, 
            index, 
            lower_mask, 
            upper_mask, 
            f, 
            w, 
            m, 
            r, 
            a
        }

    }

    fn get_seed() -> QuadrupleWord {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    

        let duration_usize = (since_the_epoch.as_secs_f32() / 1000_000.0) as usize;

      
        let qdp = QuadrupleWord::from_usize(duration_usize);
    
        return qdp
    }

    fn seed_mt(&mut self) {
        let index = self.index;
        self.mt[0] = Self::get_seed();

        for i in 1..index {
            let i_qdw = QuadrupleWord::from_usize(i);
            self.mt[i] = (self.f * (self.mt[i - 1] ^ (self.mt[i - 1] >> (self.w - 2))) + i_qdw) 
        }
    }

    pub fn rng(&mut self) -> QuadrupleWord {
        if self.index >= 1024 {
            self.twist();
        }

        let mut y = self.mt[self.index];
        y = y ^ ((y >> 12) & self.a);
        y = y ^ ((y << 3) & self.f);
        y = y ^ ((y << 13) & self.r);
        y = y ^ (y >> 1);

        self.index += 1;

        y
    }

    fn twist(&mut self) {
        let two_qdw = QuadrupleWord::from_usize(2);
        let qdw_zero = QuadrupleWord::new_zeros();

        for i in 0usize..1024 - 1 {
            let x = self.mt[i] & self.upper_mask;
            let y = self.mt[(i + 1) % 1024] & self.lower_mask;
            
            let xy = x + y;

            let mut x_a = xy >> 1;

            match x % two_qdw {
                qdw_zero => (),
                _ => {
                    x_a = x_a ^ self.a;
                }
            }
            
            self.mt[i] = self.mt[((i as u128 + self.m.into_u128()) % 1024u128) as usize] ^ x_a;
        }

        self.index = 0;
    }

}