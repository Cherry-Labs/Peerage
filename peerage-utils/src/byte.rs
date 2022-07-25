use crate::bit::Bit;
use crate::nibble::Nibble;
use crate::lazy::HEX_MAP;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Endian {
    Little,
    Big,
}

impl Default for Endian {
    fn default() -> Self {
        Self::Little
    }
}


#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq)]
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

    pub fn new_zeros() -> Self {
        Self { 
            msb: Bit::Zero, 
            msb1: Bit::Zero, 
            msb2: Bit::Zero, 
            msb3: Bit::Zero, 
            lsb3: Bit::Zero, 
            lsb2: Bit::Zero, 
            lsb1: Bit::Zero, 
            lsb: Bit::Zero, 
            endian: Endian::Little
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

    pub fn into_nibble(&self) -> [Nibble; 2] {
        let mut nibbles = [Nibble::new_zeros(); 2];

        let self_bits = self.unravel();

        let mut j = 0usize;

        for i in (0usize..8usize).step_by(4) {
            let bits = self_bits[i..i + 4].to_vec();

            let nibble = Nibble::from_vec(bits);

            nibbles[j] = nibble;

            j += 1;
        }

        nibbles
    }

    pub fn from_nibble(n: [Nibble; 2]) -> Self {
        let first = n[0];
        let second = n[1];

        let first_unwrapped = first.unwrap_to_vec();
        let second_unwrapped = second.unwrap_to_vec();

        let flattened = vec![first_unwrapped, second_unwrapped]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Bit>>();

        Self::from_bit_vec_le(flattened)
    }

    pub fn neg_self(&self) -> Self {
        let self_vec = self.unravel();

        let mut v: Vec<Bit> = vec![Bit::default(); 8];

        for i in 0usize..8usize {
            v[i] = -self_vec[i]
        }

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(v),
            Endian::Big => Self::from_bit_vec_be(v),
        }
    }

    pub fn nand_together(&self, other: Self) -> Self {
        let self_unwrapped = self.unravel();
        let other_unwrapped = other.unravel();

        let mut v = vec![Bit::default(); 8];

        for i in 0usize..8usize {
            v[i] = self_unwrapped[i].nand(other_unwrapped[i]);
        }

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(v),
            Endian::Big => Self::from_bit_vec_be(v),
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
    
    pub fn into_u8(&self) -> u8 {
        let mut unwrapped = self.unravel();

        unwrapped.reverse();

        let mut ret = 0u8;

        for (i, u) in unwrapped.into_iter().enumerate() {
            let u_u8 = u.into_u8();

            ret += u_u8 * 2u8.pow(i as u32)
        }

        ret
    }

    pub fn get_bits(&self) -> Vec<Bit> {
        let bits = self.unravel();

        bits
    }

    pub fn shift_left(&self, num: usize) -> Byte {
        let bits = self.unravel();
        
        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; num];

        let mut trunc_clone = bits_truncated.clone();
        
        trunc_clone.extend(rem);

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(trunc_clone),
            Endian::Big => Self::from_bit_vec_be(trunc_clone),
        }
    }


    pub fn shift_right(&self, num: usize) -> Byte {
        let bits = self.unravel();

        let prepend_bits = vec![Bit::Zero; 8 - num];

        let mut bits_clone = bits.clone();
        
        bits_clone.splice(0..0, prepend_bits.into_iter());

        let bits_splice = &bits_clone[0..8].to_vec();

        let bits_cloned = bits_splice.clone();

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(bits_cloned),
            Endian::Big => Self::from_bit_vec_be(bits_cloned),
        }
    }

    pub fn is_greater_than_or_equal(&self, other: Self) -> bool {
        let dec = self.into_u8();
        let other_dec = other.into_u8();

        dec >= other_dec
    }

    pub fn subtract_together(&self, other: Self) -> Byte {
        let mut self_bits = self.unravel();
        let mut other_bits = other.unravel();

        let mut ai = 7;
        let mut bi = 7;

        let mut borrow_indices: Vec<usize> = vec![];

        let mut res: Vec<Bit> = vec![];

        loop {

            let pair = (self_bits[ai], other_bits[bi]);

            match pair {
                (Bit::One, Bit::One) => res.push(Bit::Zero),
                (Bit::One, Bit::Zero) => res.push(Bit::One),
                (Bit::Zero, Bit::One) => {
                    let mut found_index = 0usize;

                    for i in (0..7 - ai).rev() {
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

        res.splice(0..0, vec![Bit::Zero; 8 - res.len()]);

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(res),
            Endian::Big => Self::from_bit_vec_be(res),
        }

    }

    pub fn divide_together(&self, other: Self) -> (Byte, Byte) {
        let mut q = Byte::new_zeros();
        let mut r = Byte::new_zeros();

        let mut i = 7;

        let mut n = self.clone();
        let mut d = other.clone();

        let mut n_bits = self.unravel();
        let mut d_bits = other.unravel();

        loop {
            r = r << 1;

            r.lsb =  n_bits[i];

            if r.is_greater_than_or_equal(other) {
                r = r - d;

                q[7 - i] =  Bit::One;
            }

            i -= 1;

            if i == 0 {
                break;
            }

        }

        (q, r)

    }

    pub fn multiply_together(&self, other: Self) -> Byte {
        let b = self.unravel();
 
        let size = 7;
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


     pub fn add_together(&self, other: Self) -> Byte {
        let self_bits = self.unravel();
        let other_bits = other.unravel();

        let mut ai = 7;
        let mut bi = 7;

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

        let pad = 8 - res.len();

        let padding = vec![Bit::Zero; pad];

        res.splice(0..0, padding);

        match self.endian {
            Endian::Little => Self::from_bit_vec_le(res),
            Endian::Big => Self::from_bit_vec_be(res),
        }
    }

    pub fn add_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.add_together(other)
    }

    pub fn subtract_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.subtract_together(other)
    }

    pub fn multiply_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.multiply_together(other)
    }

    pub fn divide_with_decimal(&self, dec: u8) -> (Self, Self) {
        let other = Self::from_decimal(dec, self.endian);

        self.divide_together(other)
    }

    pub fn and_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.and(other)
    }

    pub fn or_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.or(other)
    }

    pub fn xor_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_decimal(dec, self.endian);

        self.xor(other)
    }

    pub fn shift_left_with_self(&self, other: Self) -> Self {
        let num = other.into_u8() as usize;

        self.shift_left(num)
    }

    pub fn shift_right_with_self(&self, other: Self) -> Self {
        let num = other.into_u8() as usize;

        self.shift_right(num)
    }
    
}


impl std::ops::Neg for Byte {
    type Output = Byte;

    fn neg(self) -> Self::Output {
        self.neg_self()
    }
}

impl std::ops::Index<usize> for Byte {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.msb,
            1 => &self.msb1,
            2 => &self.msb2, 
            3 => &self.msb3,
            4 => &self.lsb3,
            5 => &self.lsb2,
            6 => &self.lsb1,
            7 => &self.lsb,
            _ => panic!("Index should not be larger than 8")
        }
    }
}

impl std::ops::IndexMut<usize> for Byte {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.msb,
            1 => &mut self.msb1,
            2 => &mut self.msb2, 
            3 => &mut self.msb3,
            4 => &mut self.lsb3,
            5 => &mut self.lsb2,
            6 => &mut self.lsb1,
            7 => &mut self.lsb,
            _ => panic!("Index should not be larger than 8")
        }
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

impl std::ops::Shl<usize> for Byte {
    type Output = Byte;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl std::ops::Shr<usize> for Byte {
    type Output = Byte;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl std::ops::Add for Byte {
    type Output = Byte;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_together(rhs)
    }
}

impl std::ops::Sub for Byte {
    type Output = Byte;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract_together(rhs)
    }
    
}

impl std::ops::Mul  for Byte {
    type Output = Byte;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_together(rhs)
    }
}

impl std::ops::Div for Byte {
    type Output = Byte;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.divide_together(rhs);

        q
    }
}


impl std::ops::Rem for Byte {
    type Output = Byte;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.divide_together(rhs);

        r
    }
}


impl std::ops::Add<u8> for Byte {
    type Output = Byte;

    fn add(self, rhs: u8) -> Self::Output {
        self.add_with_decimal(rhs)
    }
}

impl std::ops::Sub<u8> for Byte {
    type Output = Byte;

    fn sub(self, rhs: u8) -> Self::Output {
        self.subtract_with_decimal(rhs)
    }
}

impl std::ops::Mul<u8> for Byte {
    type Output = Byte;

    fn mul(self, rhs: u8) -> Self::Output {
        self.multiply_with_decimal(rhs)
    }
}

impl std::ops::Div<u8> for Byte {
    type Output = Byte;

    fn div(self, rhs: u8) -> Self::Output {
        let (q, _) = self.divide_with_decimal(rhs);

        q
    }

}


impl std::ops::Rem<u8> for Byte {
    type Output = Byte;

    fn rem(self, rhs: u8) -> Self::Output {
        let (_, r) = self.divide_with_decimal(rhs);

        r
    }
}

impl std::ops::BitAnd<u8> for Byte {
    type Output = Byte;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.and_with_decimal(rhs)
    }
}

impl std::ops::BitOr<u8> for Byte {
    type Output = Byte;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.or_with_decimal(rhs)
    }
}

impl std::ops::Shl for Byte {
    type Output = Byte;

    fn shl(self, rhs: Self) -> Self::Output {
        self.shift_left_with_self(rhs)
    }
}

impl std::ops::Shr for Byte {
    type Output = Byte;

    fn shr(self, rhs: Self) -> Self::Output {
        self.shift_right_with_self(rhs)
    }
}
