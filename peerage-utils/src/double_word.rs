
use crate::bit::Bit;
use crate::nibble::Nibble;
use crate::byte::Byte;
use crate::byte_word::ByteWord;
use crate::lazy::HEX_MAP;

#[derive(Clone, Copy, Debug, Hash, Default, Eq, PartialEq)]
pub struct DoubleWord {
    upper_word: ByteWord,
    lower_word: ByteWord
}

impl DoubleWord {
    pub fn new(
        upper_word: ByteWord, 
        lower_word: ByteWord,
    ) -> Self
    {
        Self { upper_word,  lower_word }
    }

    pub fn new_ones() -> Self {
        let upper_word = ByteWord::new_ones();
        let lower_word = ByteWord::new_ones();

        Self::new(upper_word,  lower_word)
    }

    pub fn new_zeros() -> Self {
        let upper_word = ByteWord::new_zeros();
        let lower_word = ByteWord::new_zeros();

        Self::new(upper_word, lower_word)
    }

    pub fn new_zeros_odd() -> Self {
        let upper_word = ByteWord::new_zeros();
        let lower_word = ByteWord::new_ones();

        Self::new(upper_word, lower_word)
    }

    pub fn new_zeros_even() -> Self {
        let upper_word = ByteWord::new_ones();
        let lower_word = ByteWord::new_zeros();

        Self::new(upper_word, lower_word)
    }

    pub fn from_64_u8s(v: Vec<u8>) -> Self {
        let bits_vec = Bit::vec_from_vec(v);

        Self::from_64_bits(bits_vec)
    }

    pub fn from_string(s: String) -> Self {
        let s_new = match s.len() > 8 {
            true => s[..8].to_string(),
            false => format!("{}{}", s, vec!["0"; 8 - s.len()].join("")).to_string(),
        };

        let chars = s_new.chars();

        let mut bit_vec: Vec<Bit> = vec![];

        chars
                    .into_iter()
                    .map(|x| format!("{:08b}", x as u8))
                    .for_each(|x| bit_vec.extend(Bit::vec_bit_from_char(x.chars().collect::<Vec<char>>())));

        Self::from_64_bits(bit_vec)
    }

    pub fn from_usize(u: usize) -> Self {
        let usize_64_str = format!("{u:064b}");

        let bits = usize_64_str
                                .chars()
                                .map(|x| {
                                    let x_str = format!("{x}");

                                    let x_u8 = x_str.parse::<u8>().unwrap();

                                    let bit: Bit = x_u8.into();

                                    bit
                                }).collect::<Vec<Bit>>();
    
        Self::from_64_bits(bits)

    }

    pub fn from_u64(u: u64) -> Self {
        let u_bits = format!("{u:064b}");
        
        let bits = u_bits
                                .chars()
                                .map(|x| {
                                    let x_str = format!("{x}");

                                    let x_u8 = x_str.parse::<u8>().unwrap();

                                    let bit: Bit = x_u8.into();

                                    bit
                                }).collect::<Vec<Bit>>();
    
        Self::from_64_bits(bits)

    }

    pub fn from_64_bits(v: Vec<Bit>) -> Self {

        let upper_bits_ref = &v[0..32].to_vec();
        let lower_bits_ref = &v[32..64].to_vec();

        let (
            upper_bits,
            lower_bits
        ) = (
            upper_bits_ref.clone(),
            lower_bits_ref.clone()
        );

        let upper_word = ByteWord::from_32_bits(upper_bits);
                let lower_word = ByteWord::from_32_bits(lower_bits);

        Self::new(upper_word, lower_word)
    }

    pub fn from_8_bytes(v: Vec<Byte>) -> Self {
        let upper_bytes_ref = &v[0..4].to_vec();       
        let lower_bytes_ref = &v[4..8].to_vec();

        let (
            upper_bytes,
            lower_bytes
        ) = (
            upper_bytes_ref.clone(),
            lower_bytes_ref.clone()
        );

        let upper_word = ByteWord::from_byte_vec(upper_bytes);
              let lower_word = ByteWord::from_byte_vec(lower_bytes);

        Self::new(upper_word, lower_word)

    }

    pub fn neg_self(&self) -> Self {
        let self_vec = self.into_bits();

        let mut v: Vec<Bit> = vec![Bit::default(); 64];

        for i in 0usize..64usize {
            v[i] = -self_vec[i]
        }

        Self::from_64_bits(v)
    }

    pub fn nand_together(&self, other: Self) -> Self {
        let self_unwrapped = self.into_bits();
        let other_unwrapped = other.into_bits();

        let mut v = vec![Bit::default(); 64];

        for i in 0usize..64usize {
            v[i] = self_unwrapped[i].nand(other_unwrapped[i]);
        }

        Self::from_64_bits(v)
    }

    pub fn get_bits(&self) -> Vec<Bit> {
        let s1 = self.upper_word.get_bits();
        let s4 = self.lower_word.get_bits();

        vec![s1, s4].into_iter().flatten().collect::<Vec<Bit>>()
    }

    pub fn into_bits(&self) -> Vec<Bit> {
        let v1 = self.upper_word.unravel_bit();
        let v4 = self.lower_word.unravel_bit();

        vec![v1, v4].into_iter().flatten().collect::<Vec<Bit>>()
    }

    pub fn into_num_bits(&self) -> Vec<u8> {
        let vb = self.into_bits();

        vb.into_iter()
            .map(|x| x.into_u8())
            .collect::<Vec<u8>>() 
    }

    pub fn shuffle_fields(&mut self) {
        let new_shuffle = crate::rng::shufle_between_0_and_3();

        for (i, u) in new_shuffle.into_iter().enumerate() {
            self[i] = self[u]
        }


    }

    pub fn shuffle_manually(&mut self, new_shuffle: Vec<usize>) {
        for (i, u) in new_shuffle.into_iter().enumerate() {
            self[i] = self[u]
        }
    } 

    pub fn into_u64(&self) -> u64 {
        let self_unwrapped = self.into_bits();

        let mut bits_str = String::new();

        self_unwrapped
            .into_iter()
            .for_each(|x| {
                bits_str = format!("{}{}", bits_str, x.into_u8())
            });
        
        let num = u64::from_str_radix(&bits_str, 2).unwrap();

        num
    }

    pub fn pow(&self, num: usize) -> Self {
        let mut self_clone = self.clone();

        for _ in 0..num {
            self_clone = self_clone * self_clone;
        }

        self_clone
    }

    pub fn pow_self(&self, num: DoubleWord) -> Self {
        let mut self_clone = self.clone();

        let num_qw = num.into_u64();

        for _ in 0..num_qw {
            self_clone = self_clone * self_clone;
        }

        self_clone
    }

    pub fn into_nibbles(&self) -> [Nibble; 32] {
        let self_bits = self.into_bits();

        let mut vec_nibble = [Nibble::new_zeros(); 32];
        
        let mut j = 0usize;

        for i in (0usize..64usize).step_by(4) {
            let vec_slice = self_bits[i..i + 4].to_vec();

            let nibble = Nibble::from_vec(vec_slice);

            vec_nibble[j] = nibble;

            j += 1
        }

        vec_nibble
    }

    pub fn from_nibble(n: [Nibble; 32]) -> Self {
        let mut bits: Vec<Bit> = vec![];

        for i in 0..32 {
            let curr_nibble = n[i];

            let curr_nibble_unwrapped = curr_nibble.unwrap_to_vec();

            for n in curr_nibble_unwrapped {
                bits.push(n);
            }
        }

        Self::from_64_bits(bits)
    }


    pub fn shift_left(&self, num: usize) -> Self {
        let bits = self.into_bits();
        
        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; num];

        let mut trunc_clone = bits_truncated.clone();
        
        trunc_clone.extend(rem);

        Self::from_64_bits(trunc_clone)
    }

    pub fn shift_right(&self, num: usize) -> Self {
        let bits = self.into_bits();

        let prepend_bits = vec![Bit::Zero; 64 - num];

        let mut bits_clone = bits.clone();
        
        bits_clone.splice(0..0, prepend_bits.into_iter());

        let bits_splice = &bits_clone[0..64].to_vec();

        let bits_cloned = bits_splice.clone();

        Self::from_64_bits(bits_cloned)
    }

    pub fn xor_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..64 {
            zero[i] = self[i] ^ other[i];
        }

        zero
    }

    pub fn and_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..64 {
            zero[i] = self[i] & other[i];
        }

        zero
    }

    pub fn or_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..64 {
            zero[i] = self[i] | other[i];
        }

        zero
    }

    pub fn subtract_together(&self, other: Self) -> Self {
        let mut self_bits = self.into_bits();
        let mut other_bits = other.into_bits();

        let mut ai = 127;
        let mut bi = 127;

        let mut borrow_indices: Vec<usize> = vec![];

        let mut res: Vec<Bit> = vec![];

        loop {

            let pair = (self_bits[ai], other_bits[bi]);

            match pair {
                (Bit::One, Bit::One) => res.push(Bit::Zero),
                (Bit::One, Bit::Zero) => res.push(Bit::One),
                (Bit::Zero, Bit::One) => {
                    let mut found_index = 0usize;

                    for i in (0..127 - ai).rev() {
                        if self_bits[i] == Bit::One {
                            found_index = i;
                            break;
                        }
                    }

                    let mut num_ones = 2;
                    
                    for i in found_index..ai {
                        if self_bits[i] == Bit::One {
                            self_bits[i] = Bit::Zero;
                        } else if self_bits[i] == Bit::Zero {
                            if num_ones > 0 {
                                self_bits[i] = Bit::One;
                                num_ones -= 1;
                            }
                        }
                    }

                    if num_ones != 0 {
                        res.push(Bit::One);
                    }
                },
                (Bit::Zero, Bit::Zero) => res.push(Bit::Zero),
            }


            ai -= 1;
            bi -= 1;

            if ai == 0 || bi == 0 {
                break;;
            }

        }

        res.reverse();

        res.splice(0..0, vec![Bit::Zero; 64 - res.len()]);

        Self::from_64_bits(res)

    }

    pub fn is_greater_than_or_equal(&self, other: Self) -> bool {
        let self_dec = self.into_u64();
        let other_dec = other.into_u64();

        self_dec >= other_dec
    }

    pub fn divide_together(&self, other: Self) -> (Self, Self) {
        let mut q = Self::new_zeros();
        let mut r = Self::new_zeros();

        let mut i = 127;

        let mut n = self.clone();
        let mut d = other.clone();

        let mut n_bits = self.into_bits();
        let mut d_bits = other.into_bits();

        loop {
            r = r << 1;

            r[127] =  n_bits[i];

            if r.is_greater_than_or_equal(other) {
                r = r - d;

                q[127 - i] =  Bit::One;
            }

            i -= 1;

            if i == 0 {
                break;
            }

        }

        (q, r)

    }

    pub fn multiply_together(&self, other: Self) -> Self {
        let b = self.into_bits();
 
        let size = 127;
        let zeros = Self::new_zeros();
 
        let mut sums: Vec<Self> = vec![];
 
        for (i, d) in b.into_iter().enumerate() {
             if d == Bit::Zero {
                 sums.push(zeros.clone());
             } else {
                 let mut a_clone = self.clone();
                 a_clone = a_clone << i;
                 sums.push(a_clone);
             }
        }
 
 
        let mut res = Self::new_zeros();
 
        sums.into_iter().for_each(|x| res = res + x);
 
        res
 
     }


     pub fn add_together(&self, other: Self) -> Self {
        let self_bits = self.into_bits();
        let other_bits = other.into_bits();

        let mut ai = 127;
        let mut bi = 127;

        let mut carry = 0;

        let mut res: Vec<Bit> = vec![];
        loop {

            let mut val = self_bits[ai].into_u8() + other_bits[bi].into_u8() + carry;
            
            carry = match val > 1 {
                true => {
                    val %= 2;

                    1
                },
                false => 0,
            };

            let vb: Bit = val.into();

            res.push(vb);

            ai -= 1;
            bi -= 1;

            if ai == 0 || bi == 0 {
                break;
            }
            
        }

        let pad = 64 - res.len();

        let padding = vec![Bit::Zero; pad];

        res.splice(0..0, padding);

        Self::from_64_bits(res)
    }


    pub fn add_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.add_together(other)
    }

    pub fn subtract_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.subtract_together(other)
    }

    pub fn multiply_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.multiply_together(other)
    }

    pub fn divide_with_decimal(&self, dec: u64) -> (Self, Self) {
        let other = Self::from_u64(dec);

        self.divide_together(other)
    }

   
    pub fn and_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.and_together(other)
    }

    pub fn or_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.or_together(other)
    }

    pub fn xor_with_decimal(&self, dec: u64) -> Self {
        let other = Self::from_u64(dec);

        self.xor_together(other)
    }

    pub fn shift_left_with_self(&self, other: Self) -> Self {
        let num = other.into_u64() as usize;

        self.shift_left(num)
    }

    pub fn shift_right_with_self(&self, other: Self) -> Self {
        let num = other.into_u64() as usize;

        self.shift_right(num)
    }
    
}


impl std::ops::Add for DoubleWord {
    type Output = DoubleWord;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_together(rhs)
    }
}

impl std::ops::BitAnd for DoubleWord {
    type Output = DoubleWord;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and_together(rhs)
    }
}

impl std::ops::Mul for DoubleWord {
    type Output = DoubleWord;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_together(rhs)
    }
}

impl std::ops::BitOr for DoubleWord {
    type Output = DoubleWord;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or_together(rhs)
    }
}


impl std::ops::BitXor for DoubleWord {
    type Output = DoubleWord;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor_together(rhs)
    }
}

impl std::ops::Shl<usize> for DoubleWord {
    type Output = DoubleWord;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl std::ops::Shr<usize> for DoubleWord {
    type Output = DoubleWord;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl std::ops::Sub for DoubleWord {
    type Output = DoubleWord;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract_together(rhs)
    }
}

impl std::ops::Div for DoubleWord {
    type Output = DoubleWord;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.divide_together(rhs);

        q
    }
}

impl std::ops::Rem for DoubleWord {
    type Output = DoubleWord;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.divide_together(rhs);

        r
    }
}

impl std::ops::Index<&'static str> for DoubleWord {
    type Output = ByteWord;

    fn index(&self, index: &'static str) -> &Self::Output {
        match index {
            "upper" => &self.upper_word,
            "lower" => &self.lower_word,
            _ => panic!("Index can only be: upper, mid_upper, mid_lower, lower"),
        }
    }
}

impl std::ops::IndexMut<&'static str> for DoubleWord {
    fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
        match index {
            "upper" => &mut self.upper_word,
            "lower" => &mut self.lower_word,
            _ => panic!("Index can only be: upper, mid_upper, mid_lower, lower"),
        }
    }
}

impl std::ops::Index<usize> for DoubleWord {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        match index > 32 {
            true => &self.lower_word[index % 32],
            false => &self.upper_word[index]
        }
    }
}


impl std::ops::IndexMut<usize> for DoubleWord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index > 32 {
            true => &mut self.lower_word[index % 32],
            false => &mut self.upper_word[index]
        }
    }
}


impl std::ops::Neg for DoubleWord {
    type Output = DoubleWord;

    fn neg(self) -> Self::Output {
        self.neg_self()
    }
}


impl std::ops::Add<u64> for DoubleWord {
    type Output = DoubleWord;

    fn add(self, rhs: u64) -> Self::Output {
        self.add_with_decimal(rhs)
    }
}

impl std::ops::Sub<u64> for DoubleWord {
    type Output = DoubleWord;

    fn sub(self, rhs: u64) -> Self::Output {
        self.subtract_with_decimal(rhs)
    }
}

impl std::ops::Mul<u64> for DoubleWord {
    type Output = DoubleWord;

    fn mul(self, rhs: u64) -> Self::Output {
        self.multiply_with_decimal(rhs)
    }
}

impl std::ops::Div<u64> for DoubleWord {
    type Output = DoubleWord;

    fn div(self, rhs: u64) -> Self::Output {
        let (q, _) = self.divide_with_decimal(rhs);

        q
    }

}


impl std::ops::Rem<u64> for DoubleWord {
    type Output = DoubleWord;

    fn rem(self, rhs: u64) -> Self::Output {
        let (_, r) = self.divide_with_decimal(rhs);

        r
    }
}


impl std::ops::BitAnd<u64> for DoubleWord {
    type Output = DoubleWord;

    fn bitand(self, rhs: u64) -> Self::Output {
        self.and_with_decimal(rhs)
    }
}

impl std::ops::BitOr<u64> for DoubleWord {
    type Output = DoubleWord;

    fn bitor(self, rhs: u64) -> Self::Output {
        self.or_with_decimal(rhs)
    }
}

impl std::ops::Shl for DoubleWord {
    type Output = DoubleWord;

    fn shl(self, rhs: Self) -> Self::Output {
        self.shift_left_with_self(rhs)
    }
}

impl std::ops::Shr for DoubleWord {
    type Output = DoubleWord;

    fn shr(self, rhs: Self) -> Self::Output {
        self.shift_right_with_self(rhs)
    }
}