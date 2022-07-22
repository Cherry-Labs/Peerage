use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::{Byte, Endian};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Default)]
pub struct RandomByte {
    mt: PeerageCollection<Byte>,
    index: usize,
    lower_mask: Byte,
    upper_mask: Byte,
    f: Byte,
    w: usize,
    m: Byte,
    r: Byte,
    a: Byte,

}

impl RandomByte {
    pub fn new() -> Self {
        let seed_quadrupleword = Self::get_seed();
        let mt = PeerageCollection::<Byte>::new_i0_from_item(seed_quadrupleword);
        let index = 1024usize + 1;
        let f = Byte::from_decimal(50, Endian::Little);
        let w = 128usize;
        let m = Byte::from_decimal(64, Endian::Little);
        let r = Byte::from_decimal(80, Endian::Little);
        let a = Byte::from_decimal(10, Endian::Little);
        let lower_mask = (r >> 1) - Byte::from_usize(1);
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

    fn get_seed() -> Byte {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    

        let duration_usize = (since_the_epoch.as_secs_f32() / 1000_000.0) as usize;

      
        let qdp = Byte::from_decimal(duration_usize as u8, Endian::Little);
    
        return qdp
    }

    fn seed_mt(&mut self) {
        let index = self.index;
        self.mt[0] = Self::get_seed();

        for i in 1..index {
            let i_qdw = Byte::from_decimal(i as u8, Endian::Little);
            self.mt[i] = (self.f * (self.mt[i - 1] ^ (self.mt[i - 1] >> (self.w - 2))) + i_qdw) 
        }
    }

    pub fn rng(&mut self) -> Byte {
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
        let two_qdw = Byte::from_usize(2);
        let qdw_zero = Byte::new_zeros();

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