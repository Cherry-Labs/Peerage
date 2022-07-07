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

pub struct Byte {
    msb: Bit,
    msb1: Bit,
    msb2: Bit,
    msb3: Bit,
    lsb3: Bit,
    lsb2: Bit,
    lsb1: Bit,
    lsb: Bit,
}

impl Byte 