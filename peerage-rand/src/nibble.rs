use peerage_coll::collection::PeerageCollection;
use peerage_utils::bin_utils::Nibble;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Default)]
pub struct RandomNibble {
    mt: PeerageCollection<Nibble>,
    index: usize,
    lower_mask: Nibble,
    upper_mask: Nibble,
    f: Nibble,
    w: usize,
    m: Nibble,
    r: Nibble,
    a: Nibble,

}

impl RandomNibble {
    pub fn new() -> Self {
        let seed_quadrupleword = Self::get_seed();
        let mt = PeerageCollection::<Nibble>::new_i0_from_item(seed_quadrupleword);
        let index = 1024usize + 1;
        let f = Nibble::from_4_bit_number(2);
        let w = 128usize;
        let m = Nibble::from_4_bit_number(8);
        let r = Nibble::from_4_bit_number(10);
        let a = Nibble::from_4_bit_number(15);
        let lower_mask = (r >> 1) - Nibble::from_4_bit_number(5);
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

    fn get_seed() -> Nibble {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
    

        let duration = since_the_epoch.as_nanos();
     
        let nip = Nibble::from_4_bit_number((duration % 16u128) as u8);
    
        return nip
    }

    fn seed_mt(&mut self) {
        let index = self.index;
        self.mt[0] = Self::get_seed();

        for i in 1..index {
            let i_qdw = Nibble::from_4_bit_number(i as u8);
            self.mt[i] = (self.f * (self.mt[i - 1] ^ (self.mt[i - 1] >> (self.w - 2))) + i_qdw) 
        }
    }

    pub fn rng(&mut self) -> Nibble {
        if self.index >= 24 {
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
        let two_qdw = Nibble::from_4_bit_number(2);
        let qdw_zero = Nibble::new_zeros();

        for i in 0usize..24 - 1 {
            let x = self.mt[i] & self.upper_mask;
            let y = self.mt[(i + 1) % 24] & self.lower_mask;
            
            let xy = x + y;

            let mut x_a = xy >> 1;

            match x % two_qdw {
                qdw_zero => (),
                _ => {
                    x_a = x_a ^ self.a;
                }
            }
            
            self.mt[i] = self.mt[((i as u8 + self.m.to_decimal()) % 15u8) as usize] ^ x_a;
        }

        self.index = 0;
    }

    pub fn rng_inner() -> Nibble {
        Self::new().rng()
    }

}