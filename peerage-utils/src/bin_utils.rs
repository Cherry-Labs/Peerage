// NODE: all these are unsigned


#[derive(Clone, Copy)]
pub enum Bit {
    One,
    Zero,
}

impl Bit {
    pub fn from_u8(u: u8) -> Self {
        match u {
            1 => Self::One,
            0 => Self::Zero,
            _ => panic!("Wrong binary digit")
        }
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


#[derive(Clone, Copy)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Clone, Copy)]
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

    pub fn from_byte_vec(v: Vec<Byte>) -> Self {
        Self { upper_byte: v[0], up_mid_byte: v[1], low_mid_byte: v[2], lower_byte: v[3] }
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

    pub fn shift_left(&self, num: usize) -> ByteWord {
        let bits = self.unravel_bit();
        
        let bits_truncated = &bits[num..].to_vec();

        let rem = vec![Bit::Zero; 32 - num];

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