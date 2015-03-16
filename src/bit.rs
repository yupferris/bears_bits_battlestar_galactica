use std::ops::Not;

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
