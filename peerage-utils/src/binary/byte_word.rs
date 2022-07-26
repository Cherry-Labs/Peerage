
use crate::binary::bit::Bit;
use crate::binary::nibble::Nibble;
use crate::binary::byte::{Endian, Byte};
use crate::binary::lazy::HEX_MAP;

#[derive(Clone, Copy, Hash, Debug, Default, Eq, PartialEq)]
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

    pub fn from_u32(u: u32) -> Self {
        let v = format!("{u:032b}")
                .chars()
                .map(|x| Bit::from_u8(x as u8))
                .collect::<Vec<Bit>>();

        Self::from_32_bits(v)
    }

    pub fn into_u32(&self) -> u32 {
        let bits = self.unravel_bit();

        let mut str_bits = String::new();

        for b in bits {
            let b_u8 = b.into_u8();

            str_bits = format!("{}{}", str_bits, b_u8);

        }

        u32::from_str_radix(&str_bits, 2).unwrap()

    }

    pub fn from_nibble(n: [Nibble; 8]) -> Self {
        let mut bits: Vec<Bit>  = vec![];

        for i in 0..8 {
            let curr_nibble = n[i];

            let nib_bits = curr_nibble.unwrap_to_vec();

            for b in nib_bits {
                bits.push(b);
            }
        }

        Self::from_32_bits(bits)
    }

    pub fn into_nibble(&self) -> [Nibble; 8] {
        let mut nibbles = [Nibble::new_zeros(); 8];

        let self_bits = self.unravel_bit();

        let mut j = 0usize;

        for i in (0usize..32usize).step_by(4) {
            let bits = self_bits[i..i + 4].to_vec();

            let nibble = Nibble::from_vec(bits);

            nibbles[j] = nibble;

            j += 1;
        }

        nibbles
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

    pub fn neg_self(&self) -> Self {
        let self_vec = self.unravel_bit();

        let mut v: Vec<Bit> = vec![Bit::default(); 32];

        for i in 0usize..32usize {
            v[i] = -self_vec[i]
        }

        Self::from_32_bits(v)
    }

    pub fn nand_together(&self, other: Self) -> Self {
        let self_unwrapped = self.unravel_bit();
        let other_unwrapped = other.unravel_bit();

        let mut v = vec![Bit::default(); 32];

        for i in 0usize..32usize {
            v[i] = self_unwrapped[i].nand(other_unwrapped[i]);
        }

        Self::from_32_bits(v)
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

                q[31 - i] =  Bit::One;
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

    pub fn add_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.add_together(other)
    }

    pub fn subtract_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.subtract_together(other)
    }

    pub fn multiply_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.multiply_together(other)
    }

    pub fn divide_with_decimal(&self, dec: u32) -> (Self, Self) {
        let other = Self::from_u32(dec);

        self.divide_together(other)
    }

    pub fn and_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.and(other)
    }

    pub fn or_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.or(other)
    }

    pub fn xor_with_decimal(&self, dec: u32) -> Self {
        let other = Self::from_u32(dec);

        self.xor(other)
    }

    pub fn shift_left_with_self(&self, other: Self) -> Self {
        let num = other.into_u32() as usize;

        self.shift_left(num)
    }

    pub fn shift_right_with_self(&self, other: Self) -> Self {
        let num = other.into_u32() as usize;

        self.shift_right(num)
    }

}

impl std::ops::Neg for ByteWord {
    type Output = ByteWord;

    fn neg(self) -> Self::Output {
        self.neg_self()
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

impl std::ops::Index<usize> for ByteWord {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 0 && index < 8 {
            &self.upper_byte[index]
        } else if index > 8 && index < 16 {
            &self.up_mid_byte[index % 8]
        } else if index > 16 && index < 24 {
            &self.low_mid_byte[index % 8]
        } else if index > 24 && index < 32 {
            &self.lower_byte[index % 8]
        } else {
            panic!("Index cannot be larger than 32")
        }
    }
}

impl std::ops::IndexMut<usize> for ByteWord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 0 && index < 8 {
            &mut self.upper_byte[index]
        } else if index > 8 && index < 16 {
            &mut self.up_mid_byte[index % 8]
        } else if index > 16 && index < 24 {
            &mut self.low_mid_byte[index % 8]
        } else if index > 24 && index < 32 {
            &mut self.lower_byte[index % 8]
        } else {
            panic!("Index cannot be larger than 32")
        }
    }
}


impl std::ops::Add<u32> for ByteWord {
    type Output = ByteWord;

    fn add(self, rhs: u32) -> Self::Output {
        self.add_with_decimal(rhs)
    }
}

impl std::ops::Sub<u32> for ByteWord {
    type Output = ByteWord;

    fn sub(self, rhs: u32) -> Self::Output {
        self.subtract_with_decimal(rhs)
    }
}

impl std::ops::Mul<u32> for ByteWord {
    type Output = ByteWord;

    fn mul(self, rhs: u32) -> Self::Output {
        self.multiply_with_decimal(rhs)
    }
}

impl std::ops::Div<u32> for ByteWord {
    type Output = ByteWord;

    fn div(self, rhs: u32) -> Self::Output {
        let (q, _) = self.divide_with_decimal(rhs);

        q
    }

}


impl std::ops::Rem<u32> for ByteWord {
    type Output = ByteWord;

    fn rem(self, rhs: u32) -> Self::Output {
        let (_, r) = self.divide_with_decimal(rhs);

        r
    }
}

impl std::ops::BitAnd<u32> for ByteWord {
    type Output = ByteWord;

    fn bitand(self, rhs: u32) -> Self::Output {
        self.and_with_decimal(rhs)
    }
}

impl std::ops::BitOr<u32> for ByteWord {
    type Output = ByteWord;

    fn bitor(self, rhs: u32) -> Self::Output {
        self.or_with_decimal(rhs)
    }
}

impl std::ops::Shl for ByteWord {
    type Output = ByteWord;

    fn shl(self, rhs: Self) -> Self::Output {
        self.shift_left_with_self(rhs)
    }
}

impl std::ops::Shr for ByteWord {
    type Output = ByteWord;

    fn shr(self, rhs: Self) -> Self::Output {
        self.shift_right_with_self(rhs)
    }
}