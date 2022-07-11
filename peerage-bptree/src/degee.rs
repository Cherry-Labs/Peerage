
#[derive(Clone, Copy)]
pub enum Degree {
    Two,
    Four,
    Sixteen,
    TwoFiftySix,
    FiveTwelve,
    TenTwnentyFour,
    TwoFourtyEight,
    FortyNightySix,
}


impl Degree {
    pub fn from_usize(u: usize) -> Self {
        match u {
            2 => Self::Two,
            4 => Self::Four,
            16 => Self::Sixteen,
            256 => Self::TwoFiftySix,
            512 => Self::FiveTwelve,
            1024 => Self::TenTwnentyFour,
            2048 => Self::TwoFourtyEight,
            4096 => Self::FortyNightySix,
            _ => panic!("Degree should be 2, 4, 16, 256, 512, 1024, 2048, 4096")
        }
    }

    pub fn into_usize(&self) -> usize {
        match self {
            Degree::Two => 2,
            Degree::Four => 4,
            Degree::Sixteen => 16,
            Degree::TwoFiftySix => 256,
            Degree::FiveTwelve => 512,
            Degree::TenTwnentyFour => 1024,
            Degree::TwoFourtyEight => 2048,
            Degree::FortyNightySix => 4096,
        }
    }
}

impl From<usize> for Degree {
    fn from(u: usize) -> Self {
        Self::from_usize(u)
    }
}

impl Into<usize> for Degree {
    fn into(self) -> usize {
        self.into_usize()
    }
}

