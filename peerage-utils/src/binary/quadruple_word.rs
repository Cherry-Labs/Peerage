use crate::binary::bit::Bit;
use crate::binary::nibble::Nibble;
use crate::binary::byte::Byte;
use crate::binary::byte_word::ByteWord;
use crate::binary::lazy::HEX_MAP;

#[derive(Clone, Copy, Debug, Hash, Default, Eq, PartialEq)]
pub struct QuadrupleWord {
    upper_word: ByteWord,
    mid_upper_word: ByteWord,
    mid_lower_word: ByteWord,
    lower_word: ByteWord
}

impl QuadrupleWord {
    pub fn new(
        upper_word: ByteWord, 
        mid_upper_word: ByteWord, 
        mid_lower_word: ByteWord,
        lower_word: ByteWord,
    ) -> Self
    {
        Self { upper_word, mid_upper_word, mid_lower_word, lower_word }
    }

    pub fn new_ones() -> Self {
        let upper_word = ByteWord::new_ones();
        let mid_upper_word = ByteWord::new_ones();
        let mid_lower_word = ByteWord::new_ones();
        let lower_word = ByteWord::new_ones();

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn new_zeros() -> Self {
        let upper_word = ByteWord::new_zeros();
        let mid_upper_word = ByteWord::new_zeros();
        let mid_lower_word = ByteWord::new_zeros();
        let lower_word = ByteWord::new_zeros();

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn new_zeros_odd() -> Self {
        let upper_word = ByteWord::new_zeros();
        let mid_upper_word = ByteWord::new_ones();
        let mid_lower_word = ByteWord::new_zeros();
        let lower_word = ByteWord::new_ones();

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn new_zeros_even() -> Self {
        let upper_word = ByteWord::new_ones();
        let mid_upper_word = ByteWord::new_zeros();
        let mid_lower_word = ByteWord::new_ones();
        let lower_word = ByteWord::new_zeros();

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn from_128_u8s(v: Vec<u8>) -> Self {
        let bits_vec = Bit::vec_from_vec(v);

        Self::from_128_bits(bits_vec)
    }

    pub fn from_string(s: String) -> Self {
        let s_new = match s.len() > 16 {
            true => s[..16].to_string(),
            false => format!("{}{}", s, vec!["0"; 16 - s.len()].join("")).to_string(),
        };

        let chars = s_new.chars();

        let mut bit_vec: Vec<Bit> = vec![];

        chars
                    .into_iter()
                    .map(|x| format!("{:08b}", x as u8))
                    .for_each(|x| bit_vec.extend(Bit::vec_bit_from_char(x.chars().collect::<Vec<char>>())));

        Self::from_128_bits(bit_vec)
    }

    pub fn from_usize(u: usize) -> Self {
        let usize_128_str = format!("{u:0128b}");

        let bits = usize_128_str
                                .chars()
                                .map(|x| {
                                    let x_str = format!("{x}");

                                    let x_u8 = x_str.parse::<u8>().unwrap();

                                    let bit: Bit = x_u8.into();

                                    bit
                                }).collect::<Vec<Bit>>();
    
        Self::from_128_bits(bits)

    }

    pub fn from_u128(u: u128) -> Self {
        let u_bits = format!("{u:0128b}");
        
        let bits = u_bits
                                .chars()
                                .map(|x| {
                                    let x_str = format!("{x}");

                                    let x_u8 = x_str.parse::<u8>().unwrap();

                                    let bit: Bit = x_u8.into();

                                    bit
                                }).collect::<Vec<Bit>>();
    
        Self::from_128_bits(bits)

    }

    pub fn from_128_bits(v: Vec<Bit>) -> Self {

        let upper_bits_ref = &v[0..32].to_vec();      
        let mid_upper_bits_ref = &v[32..64].to_vec();
        let mid_lower_bits_ref = &v[64..96].to_vec();
        let lower_bits_ref = &v[96..128].to_vec();

        let (
            upper_bits,
            mid_upper_bits,
            mid_lower_bits,
            lower_bits
        ) = (
            upper_bits_ref.clone(),
            mid_upper_bits_ref.clone(),
            mid_lower_bits_ref.clone(),
            lower_bits_ref.clone()
        );

        let upper_word = ByteWord::from_32_bits(upper_bits);
        let mid_upper_word = ByteWord::from_32_bits(mid_upper_bits);
        let mid_lower_word = ByteWord::from_32_bits(mid_lower_bits);
        let lower_word = ByteWord::from_32_bits(lower_bits);

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn from_16_bytes(v: Vec<Byte>) -> Self {
        let upper_bytes_ref = &v[0..4].to_vec();
        let mid_upper_bytes_ref = &v[4..8].to_vec();
        let mid_lower_bytes_ref = &v[8..12].to_vec();
        let lower_bytes_ref = &v[12..16].to_vec();

        let (
            upper_bytes,
            mid_upper_bytes,
            mid_lower_bytes,
            lower_bytes
        ) = (
            upper_bytes_ref.clone(),
            mid_upper_bytes_ref.clone(),
            mid_lower_bytes_ref.clone(),
            lower_bytes_ref.clone()
        );

        let upper_word = ByteWord::from_byte_vec(upper_bytes);
        let mid_upper_word = ByteWord::from_byte_vec(mid_upper_bytes);
        let mid_lower_word = ByteWord::from_byte_vec(mid_lower_bytes);
        let lower_word = ByteWord::from_byte_vec(lower_bytes);

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)

    }

    pub fn from_octaplet_words_add_pairs(v: Vec<ByteWord>) -> Self {
        let upper_word = v[0] + v[7];
        let mid_upper_word = v[1] + v[6];
        let mid_lower_word = v[2] + v[5];
        let lower_word = v[3] + v[4];

        Self { upper_word, mid_upper_word, mid_lower_word, lower_word }
    }

    pub fn neg_self(&self) -> Self {
        let self_vec = self.into_bits();

        let mut v: Vec<Bit> = vec![Bit::default(); 128];

        for i in 0usize..128usize {
            v[i] = -self_vec[i]
        }

        Self::from_128_bits(v)
    }

    pub fn nand_together(&self, other: Self) -> Self {
        let self_unwrapped = self.into_bits();
        let other_unwrapped = other.into_bits();

        let mut v = vec![Bit::default(); 128];

        for i in 0usize..128usize {
            v[i] = self_unwrapped[i].nand(other_unwrapped[i]);
        }

        Self::from_128_bits(v)
    }

    pub fn get_bits(&self) -> Vec<Bit> {
        let s1 = self.upper_word.get_bits();
        let s2 = self.mid_upper_word.get_bits();
        let s3 = self.mid_lower_word.get_bits();
        let s4 = self.lower_word.get_bits();

        vec![s1, s2, s3, s4].into_iter().flatten().collect::<Vec<Bit>>()
    }

    pub fn into_bits(&self) -> Vec<Bit> {
        let v1 = self.upper_word.unravel_bit();
        let v2 = self.mid_upper_word.unravel_bit();
        let v3 = self.mid_lower_word.unravel_bit();
        let v4 = self.lower_word.unravel_bit();

        vec![v1, v2, v3, v4].into_iter().flatten().collect::<Vec<Bit>>()
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

    pub fn into_u128(&self) -> u128 {
        let self_unwrapped = self.into_bits();

        let mut bits_str = String::new();

        self_unwrapped
            .into_iter()
            .for_each(|x| {
                bits_str = format!("{}{}", bits_str, x.into_u8())
            });
        
        let num = u128::from_str_radix(&bits_str, 2).unwrap();

        num
    }

    pub fn pow(&self, num: usize) -> Self {
        let mut self_clone = self.clone();

        for _ in 0..num {
            self_clone = self_clone * self_clone;
        }

        self_clone
    }

    pub fn pow_self(&self, num: QuadrupleWord) -> Self {
        let mut self_clone = self.clone();

        let num_qw = num.into_u128();

        for _ in 0..num_qw {
            self_clone = self_clone * self_clone;
        }

        self_clone
    }

    pub fn into_nibbles(&self) -> [Nibble; 32] {
        let self_bits = self.into_bits();

        let mut vec_nibble = [Nibble::new_zeros(); 32];
        
        let mut j = 0usize;

        for i in (0usize..128usize).step_by(4) {
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

        Self::from_128_bits(bits)
    }


    pub fn shift_left(&self, num: usize) -> Self {
        let bits = self.into_bits();
        
        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; num];

        let mut trunc_clone = bits_truncated.clone();
        
        trunc_clone.extend(rem);

        Self::from_128_bits(trunc_clone)
    }

    pub fn shift_right(&self, num: usize) -> Self {
        let bits = self.into_bits();

        let prepend_bits = vec![Bit::Zero; 128 - num];

        let mut bits_clone = bits.clone();
        
        bits_clone.splice(0..0, prepend_bits.into_iter());

        let bits_splice = &bits_clone[0..128].to_vec();

        let bits_cloned = bits_splice.clone();

        Self::from_128_bits(bits_cloned)
    }

    pub fn xor_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..128 {
            zero[i] = self[i] ^ other[i];
        }

        zero
    }

    pub fn and_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..128 {
            zero[i] = self[i] & other[i];
        }

        zero
    }

    pub fn or_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..128 {
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

        res.splice(0..0, vec![Bit::Zero; 128 - res.len()]);

        Self::from_128_bits(res)

    }

    pub fn is_greater_than_or_equal(&self, other: Self) -> bool {
        let self_dec = self.into_u128();
        let other_dec = other.into_u128();

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

        let pad = 128 - res.len();

        let padding = vec![Bit::Zero; pad];

        res.splice(0..0, padding);

        Self::from_128_bits(res)
    }


    pub fn add_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.add_together(other)
    }

    pub fn subtract_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.subtract_together(other)
    }

    pub fn multiply_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.multiply_together(other)
    }

    pub fn divide_with_decimal(&self, dec: u128) -> (Self, Self) {
        let other = Self::from_u128(dec);

        self.divide_together(other)
    }

   
    pub fn and_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.and_together(other)
    }

    pub fn or_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.or_together(other)
    }

    pub fn xor_with_decimal(&self, dec: u128) -> Self {
        let other = Self::from_u128(dec);

        self.xor_together(other)
    }

    pub fn shift_left_with_self(&self, other: Self) -> Self {
        let num = other.into_u128() as usize;

        self.shift_left(num)
    }

    pub fn shift_right_with_self(&self, other: Self) -> Self {
        let num = other.into_u128() as usize;

        self.shift_right(num)
    }
    
}


impl std::ops::Add for QuadrupleWord {
    type Output = QuadrupleWord;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_together(rhs)
    }
}

impl std::ops::BitAnd for QuadrupleWord {
    type Output = QuadrupleWord;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and_together(rhs)
    }
}

impl std::ops::Mul for QuadrupleWord {
    type Output = QuadrupleWord;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_together(rhs)
    }
}

impl std::ops::BitOr for QuadrupleWord {
    type Output = QuadrupleWord;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or_together(rhs)
    }
}


impl std::ops::BitXor for QuadrupleWord {
    type Output = QuadrupleWord;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor_together(rhs)
    }
}

impl std::ops::Shl<usize> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl std::ops::Shr<usize> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl std::ops::Sub for QuadrupleWord {
    type Output = QuadrupleWord;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract_together(rhs)
    }
}

impl std::ops::Div for QuadrupleWord {
    type Output = QuadrupleWord;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.divide_together(rhs);

        q
    }
}

impl std::ops::Rem for QuadrupleWord {
    type Output = QuadrupleWord;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.divide_together(rhs);

        r
    }
}

impl std::ops::Index<&'static str> for QuadrupleWord {
    type Output = ByteWord;

    fn index(&self, index: &'static str) -> &Self::Output {
        match index {
            "upper" => &self.upper_word,
            "mid_upper" => &self.mid_upper_word,
            "mid_lower" => &self.mid_lower_word,
            "lower" => &self.lower_word,
            _ => panic!("Index can only be: upper, mid_upper, mid_lower, lower"),
        }
    }
}

impl std::ops::IndexMut<&'static str> for QuadrupleWord {
    fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
        match index {
            "upper" => &mut self.upper_word,
            "mid_upper" => &mut self.mid_upper_word,
            "mid_lower" => &mut self.mid_lower_word,
            "lower" => &mut self.lower_word,
            _ => panic!("Index can only be: upper, mid_upper, mid_lower, lower"),
        }
    }
}

impl std::ops::Index<usize> for QuadrupleWord {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 && index < 32 {
            &self.upper_word[index]
        } else if index > 32 && index < 64 {
            &self.mid_upper_word[index % 32]
        } else if index > 64 && index < 96 {
            &self.mid_lower_word[index % 32]
        } else if index > 96 && index < 128 {
            &self.lower_word[index % 32]
        } else {
            panic!("Index cannot be larger than 12")
        }
    }
}


impl std::ops::IndexMut<usize> for QuadrupleWord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 0 && index < 32 {
            &mut self.upper_word[index]
        } else if index > 32 && index < 64 {
            &mut self.mid_upper_word[index % 32]
        } else if index > 64 && index < 96 {
            &mut self.mid_lower_word[index % 32]
        } else if index > 96 && index < 128 {
            &mut self.lower_word[index % 32]
        } else {
            panic!("Index cannot be larger than 127")
        }
    }
}


impl std::ops::Neg for QuadrupleWord {
    type Output = QuadrupleWord;

    fn neg(self) -> Self::Output {
        self.neg_self()
    }
}


impl std::ops::Add<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn add(self, rhs: u128) -> Self::Output {
        self.add_with_decimal(rhs)
    }
}

impl std::ops::Sub<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn sub(self, rhs: u128) -> Self::Output {
        self.subtract_with_decimal(rhs)
    }
}

impl std::ops::Mul<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn mul(self, rhs: u128) -> Self::Output {
        self.multiply_with_decimal(rhs)
    }
}

impl std::ops::Div<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn div(self, rhs: u128) -> Self::Output {
        let (q, _) = self.divide_with_decimal(rhs);

        q
    }

}


impl std::ops::Rem<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn rem(self, rhs: u128) -> Self::Output {
        let (_, r) = self.divide_with_decimal(rhs);

        r
    }
}


impl std::ops::BitAnd<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn bitand(self, rhs: u128) -> Self::Output {
        self.and_with_decimal(rhs)
    }
}

impl std::ops::BitOr<u128> for QuadrupleWord {
    type Output = QuadrupleWord;

    fn bitor(self, rhs: u128) -> Self::Output {
        self.or_with_decimal(rhs)
    }
}

impl std::ops::Shl for QuadrupleWord {
    type Output = QuadrupleWord;

    fn shl(self, rhs: Self) -> Self::Output {
        self.shift_left_with_self(rhs)
    }
}

impl std::ops::Shr for QuadrupleWord {
    type Output = QuadrupleWord;

    fn shr(self, rhs: Self) -> Self::Output {
        self.shift_right_with_self(rhs)
    }
}