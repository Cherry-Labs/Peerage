// NODE: all these are unsigned

use std::{collections::HashMap, vec};

lazy_static! {
    static ref HEX_MAP: HashMap<Vec<Bit>, String> = {
        let mut hm = HashMap::<Vec<Bit>, String>::new();

        hm.insert(vec![
            Bit::Zero, 
            Bit::Zero, 
            Bit::Zero, 
            Bit::Zero], "0".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::Zero, 
            Bit::Zero, 
            Bit::One], "1".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::Zero, 
            Bit::One, 
            Bit::Zero], "2".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::Zero, 
            Bit::One, 
            Bit::One], "3".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::One, 
            Bit::Zero,
             Bit::Zero], "4".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::One, 
            Bit::Zero, 
            Bit::One], "5".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::One, 
            Bit::One, 
            Bit::Zero], "6".to_string());
        hm.insert(vec![
            Bit::Zero, 
            Bit::One, 
            Bit::One, 
            Bit::One], "7".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::Zero, 
            Bit::Zero, 
            Bit::Zero], "8".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::Zero, 
            Bit::Zero, 
            Bit::One], "9".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::Zero, 
            Bit::One, 
            Bit::Zero], "A".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::Zero, 
            Bit::One, 
            Bit::One], "B".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::One, 
            Bit::Zero, 
            Bit::Zero], "C".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::One, 
            Bit::Zero, 
            Bit::One], "D".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::One, 
            Bit::One, 
            Bit::Zero], "E".to_string());
        hm.insert(vec![
            Bit::One, 
            Bit::One, 
            Bit::One, 
            Bit::One], "F".to_string());

        hm
    };

}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Bit {
    One,
    Zero,
}

impl Default for Bit {
    fn default() -> Self {
        Self::Zero
    }
}

impl Bit {
    pub fn from_u8(u: u8) -> Self {
        match u {
            1 => Self::One,
            0 => Self::Zero,
            _ => panic!("Wrong binary digit")
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            Bit::One => 1u8,
            Bit::Zero => 0u8,
        }
    }

    pub fn vec_from_vec(v: Vec<u8>) -> Vec<Bit> {
        v.into_iter()
            .map(|x| Bit::from_u8(x))
            .collect::<Vec<Bit>>()
    }

    pub fn and(&self, other: Bit) -> Bit {
        match other {
            Bit::One => {
                match self {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
            Bit::Zero => {
                match self {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }

    pub fn or(&self, other: Bit) -> Bit {
        match other {
            Bit::One => {
                match self {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::One,
                }
            },
            Bit::Zero => {
                match self {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }

    pub fn xor(&self, other: Bit) -> Bit {
        match other {
            Bit::One => {
                match self {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::One,
                }
            },
            Bit::Zero => {
                match self {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}


impl std::ops::BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}


impl std::ops::BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}


impl std::ops::Add for Bit {
    type Output = u8;

    fn add(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();

        a + b
    }
}

impl std::ops::Sub for Bit {
    type Output = u8;

    fn sub(self, rhs: Self) -> Self::Output {
        let a: u8 = self.into();
        let b: u8 = rhs.into();

        a - b
    }
}


impl From<u8> for Bit {
    fn from(u: u8) -> Self {
        match u {
            1 => Self::One,
            0 => Self::Zero,
            _ => panic!("Error: wrong binry digit")
        }
    }
}

impl Into<u8> for Bit {
    fn into(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Zero => 0,
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Little
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Byte {
    msb: Bit,
    msb1: Bit,
    msb2: Bit,
    msb3: Bit,
    lsb3: Bit,
    lsb2: Bit,
    lsb1: Bit,
    lsb: Bit,
    endian: Endian
}

impl Byte {
    pub fn from_u8_vec_le(v: Vec<u8>) -> Self {
        Self { 
            msb: v[7].into(), 
            msb1: v[6].into(), 
            msb2: v[5].into(), 
            msb3: v[4].into(), 
            lsb3: v[3].into(), 
            lsb2: v[2].into(), 
            lsb1: v[1].into(), 
            lsb: v[0].into(),
            endian: Endian::Little
        }
    }

    pub fn from_bit_vec_le(v: Vec<Bit>) -> Self {
        Self { 
            msb: v[7], 
            msb1: v[6], 
            msb2: v[5], 
            msb3: v[4], 
            lsb3: v[3], 
            lsb2: v[2], 
            lsb1: v[1], 
            lsb: v[0],
            endian: Endian::Little
        }
    }

    pub fn from_u8_vec_be(v: Vec<u8>) -> Self {
        Self { 
            msb: v[0].into(), 
            msb1: v[1].into(), 
            msb2: v[2].into(), 
            msb3: v[3].into(), 
            lsb3: v[4].into(), 
            lsb2: v[5].into(), 
            lsb1: v[6].into(), 
            lsb: v[7].into(),
            endian: Endian::Big
        }
    }

    pub fn from_bit_vec_be(v: Vec<Bit>) -> Self {
        Self { 
            msb: v[0], 
            msb1: v[1], 
            msb2: v[2], 
            msb3: v[3], 
            lsb3: v[4], 
            lsb2: v[5], 
            lsb1: v[6], 
            lsb: v[7],
            endian: Endian::Big
        }
    }

    pub fn from_decimal(d: u8, endian: Endian) -> Self {
        let mut remainders: Vec<Bit> = vec![]; 

        let mut qoutient = d / 2;
        let rem_first = d % 2;

        let rem_first_bit: Bit = rem_first.into();

        remainders.push(rem_first_bit);

        loop {
            qoutient = qoutient / 2;
            let rem = qoutient % 2;

            let rem_bit: Bit = rem.into();

            remainders.push(rem_bit);

            if qoutient <= 0 {
                break; 
            }
        }

        let diff = vec![Bit::Zero; 8 - remainders.len()];

        remainders.splice(0..0, diff);

        match endian {
            Endian::Big => Self::from_bit_vec_be(remainders),
            Endian::Little => {
                remainders.reverse();

                Self::from_bit_vec_le(remainders)
            }
        }
    }

    pub fn unravel(&self) -> Vec<Bit> {
        match self.endian {
            Endian::Little => vec![self.lsb, self.lsb1, 
                        self.lsb2, self.lsb3, self.msb3, self.msb2, self.msb1, self.msb],
            Endian::Big => vec![self.msb, self.msb1, 
                    self.msb2, self.msb3, self.lsb3, self.lsb2, self.lsb1, self.lsb],
        }
    }

    pub fn and(&self, other: Self) -> Byte {
        let bits = self.unravel();
        let other_bits = self.unravel();

        let mut fin_bits: Vec<Bit> = vec![];

        for (b1, b2) in bits.into_iter().zip(other_bits.into_iter()) {
            let res = b1 & b2;

            fin_bits.push(res)
        }

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(fin_bits),
            Endian::Big => Self::from_bit_vec_be(fin_bits),
        }
    }

    pub fn or(&self, other: Self) -> Byte {
        let bits = self.unravel();
        let other_bits = self.unravel();

        let mut fin_bits: Vec<Bit> = vec![];

        for (b1, b2) in bits.into_iter().zip(other_bits.into_iter()) {
            let res = b1 | b2;

            fin_bits.push(res)
        }

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(fin_bits),
            Endian::Big => Self::from_bit_vec_be(fin_bits),
        }
    }

    pub fn xor(&self, other: Self) -> Byte {
        let bits = self.unravel();
        let other_bits = self.unravel();

        let mut fin_bits: Vec<Bit> = vec![];

        for (b1, b2) in bits.into_iter().zip(other_bits.into_iter()) {
            let res = b1 ^ b2;

            fin_bits.push(res)
        }

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(fin_bits),
            Endian::Big => Self::from_bit_vec_be(fin_bits),
        }
    }

    pub fn get_bits(&self) -> Vec<Bit> {
        let bits = self.unravel();

        bits
    }
    
}

impl std::ops::BitAnd for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for Byte {
    type Output = Byte;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}


impl std::ops::BitXor for Byte {
    type Output = Byte;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ByteWord {
    upper_byte: Byte,
    up_mid_byte: Byte,
    low_mid_byte: Byte,
    lower_byte: Byte,
}


impl ByteWord {
    pub fn from_32_bits(bits: Vec<Bit>) -> Self {
        let mut bytes: Vec<Byte> = vec![];

        for i in (0..32).step_by(8) {
            let chunk = &bits[i..i + 8].to_vec();

            let chunk_clone = chunk.clone();

            let byte = Byte::from_bit_vec_le(chunk_clone);

            bytes.push(byte);
        }

        Self::from_byte_vec(bytes)
    }

    pub fn from_4_integers(uv: Vec<u8>) -> Self {
        let mut bytes: Vec<Byte> = vec![];

        for i in uv {
            let b = Byte::from_decimal(i, Endian::Little);

            bytes.push(b);
        }

        Self::from_byte_vec(bytes)
    }

    pub fn from_byte_vec(v: Vec<Byte>) -> Self {
        Self { upper_byte: v[0], up_mid_byte: v[1], low_mid_byte: v[2], lower_byte: v[3] }
    }

    pub fn new_zeros() -> Self {
        let zero_bits = vec![Bit::Zero; 32];

        Self::from_32_bits(zero_bits)
    }

    pub fn new_ones() -> Self {
        let zero_bits = vec![Bit::One; 32];

        Self::from_32_bits(zero_bits)
    }

    pub fn unravel_byte(&self) -> Vec<Byte> {
        vec![self.upper_byte, self.up_mid_byte, self.low_mid_byte, self.lower_byte]
    }

    pub fn unravel_bit(&self) -> Vec<Bit> {
        let bytes_unraveled = self.unravel_byte();

        bytes_unraveled.into_iter().map(|x| x.unravel()).flatten().collect::<Vec<Bit>>()
    }

    pub fn and(&self, other: ByteWord) -> ByteWord {
        let bytes = self.unravel_byte();
        let other_bytes = other.unravel_byte();

        let mut v: Vec<Byte> = vec![];

        for (k, m) in bytes.into_iter().zip(other_bytes.into_iter()) {
            let res = k & m;

            v.push(res);
        }

        Self::from_byte_vec(v)
    }

    pub fn or(&self, other: ByteWord) -> ByteWord {
        let bytes = self.unravel_byte();
        let other_bytes = other.unravel_byte();

        let mut v: Vec<Byte> = vec![];

        for (k, m) in bytes.into_iter().zip(other_bytes.into_iter()) {
            let res = k | m;

            v.push(res);
        }

        Self::from_byte_vec(v)
    }

    pub fn xor(&self, other: ByteWord) -> ByteWord {
        let bytes = self.unravel_byte();
        let other_bytes = other.unravel_byte();

        let mut v: Vec<Byte> = vec![];

        for (k, m) in bytes.into_iter().zip(other_bytes.into_iter()) {
            let res = k ^ m;

            v.push(res);
        }

        Self::from_byte_vec(v)
    }

    pub fn assert_is_zero(&self) -> bool {
        let bits = self.unravel_bit();

        bits == vec![Bit::Zero; 32]
    }

    pub fn shift_left(&self, num: usize) -> ByteWord {
        let bits = self.unravel_bit();
        
        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; num];

        let mut trunc_clone = bits_truncated.clone();
        
        trunc_clone.extend(rem);

        Self::from_32_bits(trunc_clone)
    }

    pub fn shift_right(&self, num: usize) -> ByteWord {
        let bits = self.unravel_bit();

        let prepend_bits = vec![Bit::Zero; 32 - num];

        let mut bits_clone = bits.clone();
        
        bits_clone.splice(0..0, prepend_bits.into_iter());

        let bits_splice = &bits_clone[0..32].to_vec();

        let bits_cloned = bits_splice.clone();

        Self::from_32_bits(bits_cloned)
    }

    pub fn set_at_index(&mut self, index: usize, set: Bit) {
        if index > 31 {
            panic!("Index should be smaller than 31");
        }

        let mut bits = self.unravel_bit();

        bits[index] = set;

        let new_obj = Self::from_32_bits(bits);


        *self = new_obj;
    }

    pub fn assert_is_odd(&self) -> bool {
        let bits = self.unravel_bit();

        let last = bits.last().unwrap();

        match *last {
            Bit::One => true,
            Bit::Zero => false,
        }
    }

    pub fn assert_all_zero_with_one(&self) -> bool {
        let mut v = vec![Bit::Zero; 31];
        v.push(Bit::One);

        let bits = self.unravel_bit();


        bits == v
    }

    pub fn multiply_together(&self, other: Self) -> ByteWord {
       let b = self.unravel_bit();

       let size = 31;
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


    pub fn divide_together(&self, other: Self) -> (ByteWord, ByteWord) {
        let mut q = ByteWord::new_zeros();
        let mut r = ByteWord::new_zeros();

        let mut i = 31;

        let mut n = self.clone();
        let mut d = other.clone();

        let mut n_bits = self.unravel_bit();
        let mut d_bits = other.unravel_bit();

        loop {
            r = r << 1;

            r.set_at_index(31, n_bits[i]);

            if r.is_greater_than_or_equal(other) {
                r = r - d;

                q.set_at_index(31 - i, Bit::One);
            }

            i -= 1;

            if i == 0 {
                break;
            }

        }

        (q, r)

    }

    pub fn is_equal(&self, other: Self) -> bool {
        let all_zero = Self::new_zeros();

        let subtract = self.subtract_together(other);

        subtract == all_zero
    }

    pub fn is_greater_than(&self, other: Self) -> bool {
        let subtract = self.subtract_together(other);

        let sub_bits = subtract.unravel_bit();

        sub_bits[0] == Bit::Zero
        
    }

    pub fn is_greater_than_or_equal(&self, other: Self) -> bool {
        self.is_greater_than(other) && self.is_equal(other)
    }

    pub fn is_lesser_than(&self, other: Self) -> bool {
        !self.is_greater_than(other)
    }

    pub fn is_lesser_than_or_euqal(&self, other: Self) -> bool {
        !self.is_greater_than_or_equal(other)
    }

    pub fn subtract_together(&self, other: Self) -> ByteWord {
        let mut self_bits = self.unravel_bit();
        let mut other_bits = other.unravel_bit();

        let mut ai = 31;
        let mut bi = 31;

        let mut borrow_indices: Vec<usize> = vec![];

        let mut res: Vec<Bit> = vec![];

        loop {

            let pair = (self_bits[ai], other_bits[bi]);

            match pair {
                (Bit::One, Bit::One) => res.push(Bit::Zero),
                (Bit::One, Bit::Zero) => res.push(Bit::One),
                (Bit::Zero, Bit::One) => {
                    let mut found_index = 0usize;

                    for i in (0..31 - ai).rev() {
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

        res.splice(0..0, vec![Bit::Zero; 32 - res.len()]);

        Self::from_32_bits(res)

    }


    
    pub fn add_together(&self, other: Self) -> ByteWord {
        let self_bits = self.unravel_bit();
        let other_bits = other.unravel_bit();

        let mut ai = 31;
        let mut bi = 31;

        let mut carry = 0;

        let mut res: Vec<Bit> = vec![];
        loop {
            let mut val = self_bits[ai] + other_bits[bi] + carry;
            
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

        let pad = 32 - res.len();

        let padding = vec![Bit::Zero; pad];

        res.splice(0..0, padding);

        Self::from_32_bits(res)
    }

    pub fn get_bits(&self) -> Vec<Bit> {
        let bytes = self.unravel_byte();

        let mut ret = Vec::<Bit>::new();

        for b in bytes {
            let bits = b.get_bits();
            ret.extend(bits.into_iter());
        }

        ret
    }

}

impl std::ops::BitAnd for ByteWord {
    type Output = ByteWord;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl std::ops::BitOr for ByteWord {
    type Output = ByteWord;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}


impl std::ops::BitXor for ByteWord {
    type Output = ByteWord;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

impl std::ops::Shl<usize> for ByteWord {
    type Output = ByteWord;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl std::ops::Shr<usize> for ByteWord {
    type Output = ByteWord;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl std::ops::Add for ByteWord {
    type Output = ByteWord;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_together(rhs)
    }
}


impl std::ops::Sub for ByteWord {
    type Output = ByteWord;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract_together(rhs)
    }
}

impl std::ops::Mul for ByteWord {
    type Output = ByteWord;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_together(rhs)
    }    
}

impl std::ops::Div for ByteWord {
    type Output = ByteWord;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.divide_together(rhs);

        q
    }
}

impl std::ops::Rem for ByteWord {
    type Output = ByteWord;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.divide_together(rhs);

        r
    }
}



#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

    pub fn add_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word + other.upper_word;
        let mid_upper_word = self.mid_upper_word + other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word + other.mid_lower_word;
        let lower_word = self.lower_word + other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn multiply_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word * other.upper_word;
        let mid_upper_word = self.mid_upper_word * other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word * other.mid_lower_word;
        let lower_word = self.lower_word * other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn quotient_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word / other.upper_word;
        let mid_upper_word = self.mid_upper_word / other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word / other.mid_lower_word;
        let lower_word = self.lower_word / other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn rem_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word % other.upper_word;
        let mid_upper_word = self.mid_upper_word % other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word % other.mid_lower_word;
        let lower_word = self.lower_word % other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn subtract_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word - other.upper_word;
        let mid_upper_word = self.mid_upper_word - other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word - other.mid_lower_word;
        let lower_word = self.lower_word - other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn and_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word & other.upper_word;
        let mid_upper_word = self.mid_upper_word & other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word & other.mid_lower_word;
        let lower_word = self.lower_word & other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn or_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word | other.upper_word;
        let mid_upper_word = self.mid_upper_word | other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word | other.mid_lower_word;
        let lower_word = self.lower_word | other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn xor_together(&self, other: Self) -> QuadrupleWord {
        let upper_word = self.upper_word ^ other.upper_word;
        let mid_upper_word = self.mid_upper_word ^ other.mid_upper_word;
        let mid_lower_word = self.mid_lower_word ^ other.mid_lower_word;
        let lower_word = self.lower_word ^ other.lower_word;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn shift_left(&self, num: usize) -> QuadrupleWord {
        let upper_word = self.upper_word << num;
        let mid_upper_word = self.mid_upper_word << num;
        let mid_lower_word = self.mid_lower_word << num;
        let lower_word = self.lower_word << num;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
    }

    pub fn shift_right(&self, num: usize) -> QuadrupleWord {
        let upper_word = self.upper_word >> num;
        let mid_upper_word = self.mid_upper_word >> num;
        let mid_lower_word = self.mid_lower_word >> num;
        let lower_word = self.lower_word >> num;

        Self::new(upper_word, mid_upper_word, mid_lower_word, lower_word)
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
        self.quotient_together(rhs)
    }
}

impl std::ops::Rem for QuadrupleWord {
    type Output = QuadrupleWord;

    fn rem(self, rhs: Self) -> Self::Output {
        self.rem_together(rhs)
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
    type Output = ByteWord;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.upper_word,
            1 => &self.mid_upper_word,
            2 => &self.mid_lower_word,
            3 => &self.lower_word,
            _ => panic!("Index can only be between 0 and 3."),
        }
    }
}


impl std::ops::IndexMut<usize> for QuadrupleWord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.upper_word,
            1 => &mut self.mid_upper_word,
            2 => &mut self.mid_lower_word,
            3 => &mut self.lower_word,
            _ => panic!("Index can only be between 0 and 3."),
        }
    }
}


