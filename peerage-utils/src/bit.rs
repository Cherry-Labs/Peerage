use crate::byte::Byte;


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

    pub fn from_char(c: char) -> Self {
        match c {
            '1' => Self::One,
            '0' => Self::Zero,
            _ => panic!("Wrong binary digit!")
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

    pub fn vec_from_str(s: String) -> Vec<Self> {     
        s.chars()
            .map(|x| { 
                let ss = format!("{:08b}", x as u8);
                let bits_iter = ss.chars()
                            .map(|c| Self::from_char(c))
                            .collect::<Vec<Bit>>();

                bits_iter       
        })
        .flatten()
        .collect::<Vec<Bit>>()

    }

    pub fn vec_self_to_str(v: Vec<Bit>) -> String {
        let mut v_mut = v.clone();

        while v_mut.len() % 8 != 0 {
            v_mut.splice(0..0, vec![Bit::Zero]);
        }

        let mut s = String::new();

        for i in (0..v_mut.len()).step_by(8) {
            let sub = v_mut[i..i + 8].to_vec();
            
            let by = Byte::from_bit_vec_le(sub);

            let by_num = by.into_u8();

            let by_char = by_num as char;

            s = format!("{s}{by_char}")
        }

        s
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
