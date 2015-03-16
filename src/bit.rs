use std::ops::Not;
use std::ops::BitAnd;
use std::ops::BitOr;

#[derive(PartialEq, Debug)]
pub enum Bit { Zero, One }

impl Bit {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Bit::Zero => 0,
            Bit::One => 1
        }
    }

    pub fn to_u32(&self) -> u32 {
        match *self {
            Bit::Zero => 0u32,
            Bit::One => 1u32
        }
    }
}

#[cfg(test)]
mod to_i32_tests {
    use super::*;

    #[test]
    fn to_i32_zero_test() {
        assert_eq!(Bit::Zero.to_i32(), 0);
    }

    #[test]
    fn to_i32_one_test() {
        assert_eq!(Bit::One.to_i32(), 1);
    }
}

#[cfg(test)]
mod to_u32_tests {
    use super::*;

    #[test]
    fn to_u32_zero_test() {
        assert_eq!(Bit::Zero.to_u32(), 0);
    }

    #[test]
    fn to_u32_one_test() {
        assert_eq!(Bit::One.to_u32(), 1);
    }
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Bit {
        match self {
            Bit::Zero => Bit::One,
            Bit::One => Bit::Zero
        }
    }
}

#[cfg(test)]
mod not_tests {
    use super::*;

    #[test]
    fn not_zero_test() {
        assert_eq!(!Bit::Zero, Bit::One);
    }

    #[test]
    fn not_one_test() {
        assert_eq!(!Bit::One, Bit::Zero);
    }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Bit) -> Bit {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::Zero,
            (Bit::One, Bit::Zero) => Bit::Zero,
            (Bit::One, Bit::One) => Bit::One
        }
    }
}

#[cfg(test)]
mod bitwise_and_tests {
    use super::*;

    #[test]
    fn bitwise_and_zero_zero_test() {
        assert_eq!(Bit::Zero & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn bitwise_and_zero_one_test() {
        assert_eq!(Bit::Zero & Bit::One, Bit::Zero);
    }

    #[test]
    fn bitwise_and_one_zero_test() {
        assert_eq!(Bit::One & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn bitwise_and_one_one_test() {
        assert_eq!(Bit::One & Bit::One, Bit::One);
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Bit) -> Bit {
        match (self, rhs) {
            (Bit::Zero, Bit::Zero) => Bit::Zero,
            (Bit::Zero, Bit::One) => Bit::One,
            (Bit::One, Bit::Zero) => Bit::One,
            (Bit::One, Bit::One) => Bit::One
        }
    }
}

#[cfg(test)]
mod bitwise_or_tests {
    use super::*;

    #[test]
    fn bitwise_or_zero_zero_test() {
        assert_eq!(Bit::Zero | Bit::Zero, Bit::Zero);
    }

    #[test]
    fn bitwise_or_zero_one_test() {
        assert_eq!(Bit::Zero | Bit::One, Bit::One);
    }

    #[test]
    fn bitwise_or_one_zero_test() {
        assert_eq!(Bit::One | Bit::Zero, Bit::One);
    }

    #[test]
    fn bitwise_or_one_one_test() {
        assert_eq!(Bit::One | Bit::One, Bit::One);
    }
}
