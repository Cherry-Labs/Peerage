
#[derive(Clone, Hash, Copy, PartialEq, Eq, Debug)]
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

    pub fn vec_bit_from_char(v: Vec<char>) -> Vec<Bit> {
        v.into_iter()
            .map(|x| Self::from_u8(x as u8) )
            .collect::<Vec<Bit>>()
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

    pub fn nand(&self, other: Self) -> Self {
        match self {
            Bit::One => {
                match other {
                    Bit::One => Self::Zero,
                    Bit::Zero => Self::One,
                }
            },
            Bit::Zero => {
                match other {
                    Bit::One => Self::One,
                    Bit::Zero => Self::One,
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

    pub fn not_self(&self) -> Self {
        match self {
            Bit::One => Self::Zero,
            Bit::Zero => Self::One,
        }
    }
}


impl std::ops::Not for Bit {
    type Output = Bit;

    fn not(self) -> Self::Output {
        self.not_self()
    }
}

impl std::ops::Neg for Bit  {
    type Output = Bit;

    fn neg(self) -> Self::Output {
        self.not_self()
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
    type Output = Bit;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::One => {
                match rhs {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::One,
                }
            },
            Self::Zero => {
                match rhs {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}

impl std::ops::Sub for Bit {
    type Output = Bit;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::One => {
                match rhs {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::One,
                }
            },
            Self::Zero => {
                match rhs {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}

impl std::ops::Mul for Bit {
    type Output = Bit;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::One => {
                match rhs {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
            Self::Zero => {
                match rhs {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}


impl std::ops::Div for Bit {
    type Output = Bit;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::One => {
                match rhs {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
            Self::Zero => {
                match rhs {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}


impl std::ops::Rem for Bit {
    type Output = Bit;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Self::One => {
                match rhs {
                    Bit::One => Bit::One,
                    Bit::Zero => Bit::Zero,
                }
            },
            Self::Zero => {
                match rhs {
                    Bit::One => Bit::Zero,
                    Bit::Zero => Bit::Zero,
                }
            },
        }
    }
}

impl From<u8> for Bit {
    fn from(u: u8) -> Self {
        match u {
            1 => Self::One,
            0 => Self::Zero,
            _ => Bit::Zero
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
