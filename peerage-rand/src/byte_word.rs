use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::ByteWord;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Default)]
pub struct RandomByteWord {
    mt: PeerageCollection<ByteWord>,
    index: usize,
    lower_mask: ByteWord,
    upper_mask: ByteWord,
    f: ByteWord,
    w: usize,
    m: ByteWord,
    r: ByteWord,
    a: ByteWord,

}

impl RandomByteWord {
    pub fn new() -> Self {
        let seed_quadrupleword = Self::get_seed();
        let mt = PeerageCollection::<ByteWord>::new_i0_from_item(seed_quadrupleword);
        let index = 1024usize + 1;
        let f = ByteWord::from_u32(500);
        let w = 128usize;
        let m = ByteWord::from_u32(64);
        let r = ByteWord::from_u32(80);
        let a = ByteWord::from_u32(10);
        let lower_mask = (r >> 1) - ByteWord::from_u32(1);
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

    fn get_seed() -> ByteWord {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    

        let duration = since_the_epoch.as_nanos();
     
        let bwp = ByteWord::from_u32((duration % (u32::MAX as u128)) as u32);
    
        return bwp
    }

    fn seed_mt(&mut self) {
        let index = self.index;
        self.mt[0] = Self::get_seed();

        for i in 1..index {
            let i_qdw = ByteWord::from_u32(i as u32);
            self.mt[i] = (self.f * (self.mt[i - 1] ^ (self.mt[i - 1] >> (self.w - 2))) + i_qdw) 
        }
    }

    pub fn rng(&mut self) -> ByteWord {
        if self.index >= 256 {
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
        let two_qdw = ByteWord::from_u32(2);
        let qdw_zero = ByteWord::new_zeros();

        for i in 0usize..256 - 1 {
            let x = self.mt[i] & self.upper_mask;
            let y = self.mt[(i + 1) % 256] & self.lower_mask;
            
            let xy = x + y;

            let mut x_a = xy >> 1;

            match x % two_qdw {
                qdw_zero => (),
                _ => {
                    x_a = x_a ^ self.a;
                }
            }
            
            self.mt[i] = self.mt[((i as u32 + self.m.into_u32()) % u32::MAX) as usize] ^ x_a;
        }

        self.index = 0;
    }

    pub fn rng_inner() -> ByteWord {
        Self::new().rng()
    }

}