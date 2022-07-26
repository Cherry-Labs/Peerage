use crate::binary::bit::Bit;
use crate::binary::lazy::HEX_MAP;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct Nibble {
    bit_one: Bit,
    bit_two: Bit,
    bit_three: Bit,
    bit_four: Bit,
}

impl Nibble {
    pub fn from_vec(v: Vec<Bit>) -> Self {
        Self {
            bit_one: v[0],
            bit_two: v[1],
            bit_three: v[2],
            bit_four: v[3],
        }
    }

    pub fn new_zeros() -> Self {
        Self {
            bit_one: Bit::Zero,
            bit_two: Bit::Zero,
            bit_three: Bit::Zero,
            bit_four: Bit::Zero,
        }
    }

    pub fn from_quartet_bit(b: (Bit, Bit, Bit, Bit)) -> Self {
        Self {
            bit_one: b.0,
            bit_two: b.1,
            bit_three: b.2,
            bit_four: b.3,
        }
    }

    pub fn from_4_bit_number(num: u8) -> Self {
        let bits = format!("{num:08b}");

        let bits_slice = &bits[4..];

        let v = bits_slice
            .chars()
            .map(|x| Bit::from_u8(x as u8))
            .collect::<Vec<Bit>>();

        Self::from_vec(v)
    }

    pub fn vec_self_from_vec_bit(v: Vec<Bit>) -> Vec<Self> {
        let mut v_mut = v.clone();

        while v_mut.len() % 4 != 0 {
            v_mut.splice(0..0, vec![Bit::Zero]);
        }

        (0..v_mut.len())
                .step_by(4)
                .into_iter()
                .map(|i| Self::from_vec(v_mut[i..i + 4].to_vec()))
                .collect::<Vec<Nibble>>()
    }

    pub fn v_self_to_vec_bits(v: Vec<Nibble>) -> Vec<Bit> {
        v.into_iter()
            .map(|x| x.unwrap_to_vec())
            .flatten()
            .collect::<Vec<Bit>>()
    }

    pub fn unwrap_to_vec(&self) -> Vec<Bit> {
        vec![self.bit_one, self.bit_two, self.bit_three, self.bit_four]
    }

    pub fn to_decimal(&self) -> u8 {
        let mut unwrapped = self.unwrap_to_vec();

        unwrapped.reverse();

        let mut dec = 0u8;

        for (i, b) in unwrapped.into_iter().enumerate() {
            let b_u8 = b.into_u8();

            dec += b_u8 * 2u8.pow(i as u32);
        }

        dec
    }

    pub fn shift_left(&self, num: usize) -> Nibble {
        let bits = self.unwrap_to_vec();

        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; num];

        let mut trunc_clone = bits_truncated.clone();

        trunc_clone.extend(rem);

        Self::from_vec(trunc_clone)
    }

    pub fn shift_right(&self, num: usize) -> Nibble {
        let bits = self.unwrap_to_vec();

        let prepend_bits = vec![Bit::Zero; 4 - num];

        let mut bits_clone = bits.clone();

        bits_clone.splice(0..0, prepend_bits.into_iter());

        let bits_splice = &bits_clone[0..4].to_vec();

        let bits_cloned = bits_splice.clone();

        Self::from_vec(bits_cloned)
    }

    pub fn xor_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..4 {
            zero[i] = self[i] ^ other[i];
        }

        zero
    }

    pub fn neg_self(&self) -> Self {
        let self_vec = self.unwrap_to_vec();

        let mut v: Vec<Bit> = vec![Bit::default(); 4];

        for i in 0usize..4usize {
            v[i] = -self_vec[i]
        }

        Self::from_vec(v)
    }

    pub fn nand_together(&self, other: Self) -> Self {
        let self_unwrapped = self.unwrap_to_vec();
        let other_unwrapped = other.unwrap_to_vec();

        let mut v = vec![Bit::default(); 4];

        for i in 0usize..4usize {
            v[i] = self_unwrapped[i].nand(other_unwrapped[i]);
        }

        Self::from_vec(v)
    }

    pub fn and_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..4 {
            zero[i] = self[i] & other[i];
        }

        zero
    }

    pub fn or_together(&self, other: Self) -> Self {
        let mut zero = Self::new_zeros();

        for i in 0..4 {
            zero[i] = self[i] | other[i];
        }

        zero
    }

    pub fn subtract_together(&self, other: Self) -> Nibble {
        let mut self_bits = self.unwrap_to_vec();
        let mut other_bits = other.unwrap_to_vec();

        let mut ai = 3;
        let mut bi = 3;

        let mut borrow_indices: Vec<usize> = vec![];

        let mut res: Vec<Bit> = vec![];

        loop {
            let pair = (self_bits[ai], other_bits[bi]);

            match pair {
                (Bit::One, Bit::One) => res.push(Bit::Zero),
                (Bit::One, Bit::Zero) => res.push(Bit::One),
                (Bit::Zero, Bit::One) => {
                    let mut found_index = 0usize;

                    for i in (0..3 - ai).rev() {
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
                }
                (Bit::Zero, Bit::Zero) => res.push(Bit::Zero),
            }

            ai -= 1;
            bi -= 1;

            if ai == 0 || bi == 0 {
                break;
            }
        }

        res.reverse();

        res.splice(0..0, vec![Bit::Zero; 4 - res.len()]);

        Self::from_vec(res)
    }

    pub fn is_greater_than_or_equal(&self, other: Self) -> bool {
        let self_dec = self.to_decimal();
        let other_dec = other.to_decimal();

        self_dec >= other_dec
    }

    pub fn divide_together(&self, other: Self) -> (Nibble, Nibble) {
        let mut q = Nibble::new_zeros();
        let mut r = Nibble::new_zeros();

        let mut i = 3;

        let mut n = self.clone();
        let mut d = other.clone();

        let mut n_bits = self.unwrap_to_vec();
        let mut d_bits = other.unwrap_to_vec();

        loop {
            r = r << 1;

            r[3] = n_bits[i];

            if r.is_greater_than_or_equal(other) {
                r = r - d;

                q[3 - i] = Bit::One;
            }

            i -= 1;

            if i == 0 {
                break;
            }
        }

        (q, r)
    }

    pub fn multiply_together(&self, other: Self) -> Nibble {
        let b = self.unwrap_to_vec();

        let size = 3;
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

    pub fn add_together(&self, other: Self) -> Nibble {
        let self_bits = self.unwrap_to_vec();
        let other_bits = other.unwrap_to_vec();

        let mut ai = 3;
        let mut bi = 3;

        let mut carry = 0;

        let mut res: Vec<Bit> = vec![];
        loop {
            let mut val = self_bits[ai].into_u8() + other_bits[bi].into_u8() + carry;

            carry = match val > 1 {
                true => {
                    val %= 2;

                    1
                }
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

        Self::from_vec(res)
    }

    pub fn add_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.add_together(other)
    }

    pub fn subtract_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.subtract_together(other)
    }

    pub fn multiply_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.multiply_together(other)
    }

    pub fn divide_with_decimal(&self, dec: u8) -> (Self, Self) {
        let other = Self::from_4_bit_number(dec);

        self.divide_together(other)
    }

    pub fn and_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.and_together(other)
    }

    pub fn or_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.or_together(other)
    }

    pub fn xor_with_decimal(&self, dec: u8) -> Self {
        let other = Self::from_4_bit_number(dec);

        self.xor_together(other)
    }

    pub fn shift_left_with_self(&self, other: Self) -> Self {
        let num = other.to_decimal() as usize;

        self.shift_left(num)
    }

    pub fn shift_right_with_self(&self, other: Self) -> Self {
        let num = other.to_decimal() as usize;

        self.shift_right(num)
    }

    pub fn into_hex(&self) -> String {
        let self_unravel = self.unwrap_to_vec();

        HEX_MAP[&self_unravel].clone()
    }
}

impl std::ops::Neg for Nibble {
    type Output = Nibble;

    fn neg(self) -> Self::Output {
        self.neg_self()
    }
}

impl std::ops::Index<usize> for Nibble {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.bit_one,
            1 => &self.bit_two,
            2 => &self.bit_three,
            3 => &self.bit_four,
            _ => panic!("Index must not be larger than 3"),
        }
    }
}

impl std::ops::BitXor for Nibble {
    type Output = Nibble;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor_together(rhs)
    }
}

impl std::ops::BitAnd for Nibble {
    type Output = Nibble;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and_together(rhs)
    }
}

impl std::ops::BitOr for Nibble {
    type Output = Nibble;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or_together(rhs)
    }
}

impl std::ops::IndexMut<usize> for Nibble {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.bit_one,
            1 => &mut self.bit_two,
            2 => &mut self.bit_three,
            3 => &mut self.bit_four,
            _ => panic!("Index must not be larger than 3"),
        }
    }
}

impl std::ops::Add for Nibble {
    type Output = Nibble;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_together(rhs)
    }
}

impl std::ops::Sub for Nibble {
    type Output = Nibble;

    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract_together(rhs)
    }
}

impl std::ops::Mul for Nibble {
    type Output = Nibble;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply_together(rhs)
    }
}

impl std::ops::Div for Nibble {
    type Output = Nibble;

    fn div(self, rhs: Self) -> Self::Output {
        let (q, _) = self.divide_together(rhs);

        q
    }
}

impl std::ops::Rem for Nibble {
    type Output = Nibble;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, r) = self.divide_together(rhs);

        r
    }
}

impl std::ops::Shl<usize> for Nibble {
    type Output = Nibble;

    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl std::ops::Shr<usize> for Nibble {
    type Output = Nibble;

    fn shr(self, rhs: usize) -> Self::Output {
        self.shift_right(rhs)
    }
}

impl std::ops::Add<u8> for Nibble {
    type Output = Nibble;

    fn add(self, rhs: u8) -> Self::Output {
        self.add_with_decimal(rhs)
    }
}

impl std::ops::Sub<u8> for Nibble {
    type Output = Nibble;

    fn sub(self, rhs: u8) -> Self::Output {
        self.subtract_with_decimal(rhs)
    }
}

impl std::ops::Mul<u8> for Nibble {
    type Output = Nibble;

    fn mul(self, rhs: u8) -> Self::Output {
        self.multiply_with_decimal(rhs)
    }
}

impl std::ops::Div<u8> for Nibble {
    type Output = Nibble;

    fn div(self, rhs: u8) -> Self::Output {
        let (q, _) = self.divide_with_decimal(rhs);

        q
    }
}

impl std::ops::Rem<u8> for Nibble {
    type Output = Nibble;

    fn rem(self, rhs: u8) -> Self::Output {
        let (_, r) = self.divide_with_decimal(rhs);

        r
    }
}

impl std::ops::BitXor<u8> for Nibble {
    type Output = Nibble;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self.xor_with_decimal(rhs)
    }
}

impl std::ops::BitAnd<u8> for Nibble {
    type Output = Nibble;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.and_with_decimal(rhs)
    }
}

impl std::ops::BitOr<u8> for Nibble {
    type Output = Nibble;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.or_with_decimal(rhs)
    }
}

impl std::ops::Shl for Nibble {
    type Output = Nibble;

    fn shl(self, rhs: Self) -> Self::Output {
        self.shift_left_with_self(rhs)
    }
}

impl std::ops::Shr for Nibble {
    type Output = Nibble;

    fn shr(self, rhs: Self) -> Self::Output {
        self.shift_right_with_self(rhs)
    }
}
