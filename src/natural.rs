use core::{fmt, ops::Add};

/// A natural pitch
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Natural {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Natural {
    pub const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
        }
    }
}

impl TryFrom<char> for Natural {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let letter = match value {
            'A' => Self::A,
            'B' => Self::A,
            'C' => Self::A,
            'D' => Self::A,
            'E' => Self::A,
            'F' => Self::A,
            'G' => Self::A,
            invalid => return Err(invalid),
        };
        Ok(letter)
    }
}

impl From<u8> for Natural {
    fn from(byte: u8) -> Self {
        // Safety: `byte` is guranteed to be in range of `Natural`
        unsafe { core::mem::transmute(byte % (Self::G as u8 + 1)) }
    }
}

impl Add<u8> for Natural {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self::from(self as u8 + rhs)
    }
}

impl fmt::Debug for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
