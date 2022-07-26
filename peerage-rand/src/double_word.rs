use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::DoubleWord;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Default)]
pub struct RandomDoubleWord {
    mt: PeerageCollection<DoubleWord>,
    index: usize,
    lower_mask: DoubleWord,
    upper_mask: DoubleWord,
    f: DoubleWord,
    w: usize,
    m: DoubleWord,
    r: DoubleWord,
    a: DoubleWord,

}

impl RandomDoubleWord {
    pub fn new() -> Self {
        let seed_quadrupleword = Self::get_seed();
        let mt = PeerageCollection::<DoubleWord>::new_i0_from_item(seed_quadrupleword);
        let index = 1024usize + 1;
        let f = DoubleWord::from_u64(500);
        let w = 128usize;
        let m = DoubleWord::from_u64(64);
        let r = DoubleWord::from_u64(80);
        let a = DoubleWord::from_u64(10);
        let lower_mask = (r >> 1) - DoubleWord::from_u64(1);
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

    fn get_seed() -> DoubleWord {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    

        let duration_usize = (since_the_epoch.as_secs_f64() / 1000_000.0) as usize;

      
        let qdp = DoubleWord::from_u64(duration_usize as u64);
    
        return qdp
    }

    fn seed_mt(&mut self) {
        let index = self.index;
        self.mt[0] = Self::get_seed();

        for i in 1..index {
            let i_qdw = DoubleWord::from_u64(i as u64);
            self.mt[i] = (self.f * (self.mt[i - 1] ^ (self.mt[i - 1] >> (self.w - 2))) + i_qdw) 
        }
    }

    pub fn rng(&mut self) -> DoubleWord {
        if self.index >= 512 {
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
        let two_qdw = DoubleWord::from_u64(2);
        let qdw_zero = DoubleWord::new_zeros();

        for i in 0usize..512 - 1 {
            let x = self.mt[i] & self.upper_mask;
            let y = self.mt[(i + 1) % 512] & self.lower_mask;
            
            let xy = x + y;

            let mut x_a = xy >> 1;

            match x % two_qdw {
                qdw_zero => (),
                _ => {
                    x_a = x_a ^ self.a;
                }
            }
            
            self.mt[i] = self.mt[((i as u64 + self.m.into_u64()) % u64::MAX) as usize] ^ x_a;
        }

        self.index = 0;
    }

    pub fn rng_inner() -> DoubleWord {
        Self::new().rng()
    }

}