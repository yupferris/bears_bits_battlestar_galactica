use std::ops::Not;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::default::Default;
use std::str::FromStr;
use std::fmt::{self, Display, Formatter};
use std::result::Result::{self, Ok, Err};
use std::error::Error;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Hash)]
pub enum Bit { Zero, One }

#[cfg(test)]
mod partial_eq_tests {
    use super::*;

    #[test]
    fn eq_zero_zero() {
        assert!(Bit::Zero == Bit::Zero);
    }

    #[test]
    #[should_panic]
    fn eq_zero_one() {
        assert!(Bit::Zero == Bit::One);
    }

    #[test]
    #[should_panic]
    fn eq_one_zero() {
        assert!(Bit::One == Bit::Zero);
    }

    #[test]
    fn eq_one_one() {
        assert!(Bit::One == Bit::One);
    }

    #[test]
    #[should_panic]
    fn ne_zero_zero() {
        assert!(Bit::Zero != Bit::Zero);
    }

    #[test]
    fn ne_zero_one() {
        assert!(Bit::Zero != Bit::One);
    }

    #[test]
    fn ne_one_zero() {
        assert!(Bit::One != Bit::Zero);
    }

    #[test]
    #[should_panic]
    fn ne_one_one() {
        assert!(Bit::One != Bit::One);
    }
}

impl Bit {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Bit::Zero => 0,
            Bit::One => 1
        }
    }
}

#[cfg(test)]
mod to_i32_tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Bit::Zero.to_i32(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(Bit::One.to_i32(), 1);
    }
}

impl Bit {
    pub fn to_u32(&self) -> u32 {
        match *self {
            Bit::Zero => 0u32,
            Bit::One => 1u32
        }
    }
}

#[cfg(test)]
mod to_u32_tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Bit::Zero.to_u32(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(Bit::One.to_u32(), 1);
    }
}

impl Not for Bit {
    type Output = Bit;

    fn not(self) -> Bit {
        not_impl(&self)
    }
}

fn not_impl(x: &Bit) -> Bit
{
    match *x {
        Bit::Zero => Bit::One,
        Bit::One => Bit::Zero
    }
}

#[cfg(test)]
mod not_tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(!Bit::Zero, Bit::One);
    }

    #[test]
    fn one() {
        assert_eq!(!Bit::One, Bit::Zero);
    }
}

impl<'a> Not for &'a Bit {
    type Output = <Bit as Not>::Output;

    fn not(self) -> <Bit as Not>::Output {
        not_impl(self)
    }
}

#[cfg(test)]
mod not_ref_tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(!&Bit::Zero, Bit::One);
    }

    #[test]
    fn one() {
        assert_eq!(!&Bit::One, Bit::Zero);
    }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Bit) -> Bit {
        bitand_impl(&self, &rhs)
    }
}

fn bitand_impl(x: &Bit, y: &Bit) -> Bit {
    match (x, y) {
        (&Bit::Zero, &Bit::Zero) => Bit::Zero,
        (&Bit::Zero, &Bit::One) => Bit::Zero,
        (&Bit::One, &Bit::Zero) => Bit::Zero,
        (&Bit::One, &Bit::One) => Bit::One
    }
}

#[cfg(test)]
mod bitwise_and_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero & Bit::One, Bit::Zero);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One & Bit::One, Bit::One);
    }
}

impl<'a> BitAnd<Bit> for &'a Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(self, &rhs)
    }
}

#[cfg(test)]
mod bitwise_and_ref_lhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero & Bit::One, Bit::Zero);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One & Bit::Zero, Bit::Zero);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One & Bit::One, Bit::One);
    }
}

impl<'a> BitAnd<&'a Bit> for Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: &'a Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(&self, rhs)
    }
}

#[cfg(test)]
mod bitwise_and_ref_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero & &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero & &Bit::One, Bit::Zero);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One & &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One & &Bit::One, Bit::One);
    }
}

impl<'a, 'b> BitAnd<&'a Bit> for &'b Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: &'a Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(self, rhs)
    }
}

#[cfg(test)]
mod bitwise_and_ref_lhs_and_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero & &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero & &Bit::One, Bit::Zero);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One & &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One & &Bit::One, Bit::One);
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Bit) -> Bit {
        bitor_impl(&self, &rhs)
    }
}

fn bitor_impl(x: &Bit, y: &Bit) -> Bit
{
    match (x, y) {
        (&Bit::Zero, &Bit::Zero) => Bit::Zero,
        (&Bit::Zero, &Bit::One) => Bit::One,
        (&Bit::One, &Bit::Zero) => Bit::One,
        (&Bit::One, &Bit::One) => Bit::One
    }
}

#[cfg(test)]
mod bitwise_or_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero | Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero | Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One | Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One | Bit::One, Bit::One);
    }
}

impl<'a> BitOr<Bit> for &'a Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(self, &rhs)
    }
}

#[cfg(test)]
mod bitwise_or_ref_lhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero | Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero | Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One | Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One | Bit::One, Bit::One);
    }
}

impl<'a> BitOr<&'a Bit> for Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: &'a Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(&self, rhs)
    }
}

#[cfg(test)]
mod bitwise_or_ref_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero | &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero | &Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One | &Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One | &Bit::One, Bit::One);
    }
}

impl<'a, 'b> BitOr<&'a Bit> for &'b Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: &'a Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(self, rhs)
    }
}

#[cfg(test)]
mod bitwise_or_ref_lhs_and_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero | &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero | &Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One | &Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One | &Bit::One, Bit::One);
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Bit) -> Bit {
        bitxor_impl(&self, &rhs)
    }
}

fn bitxor_impl(x: &Bit, y: &Bit) -> Bit {
    match (x, y) {
        (&Bit::Zero, &Bit::Zero) => Bit::Zero,
        (&Bit::Zero, &Bit::One) => Bit::One,
        (&Bit::One, &Bit::Zero) => Bit::One,
        (&Bit::One, &Bit::One) => Bit::Zero
    }
}

#[cfg(test)]
mod bitwise_xor_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero ^ Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero ^ Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One ^ Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One ^ Bit::One, Bit::Zero);
    }
}

impl<'a> BitXor<Bit> for &'a Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(self, &rhs)
    }
}

#[cfg(test)]
mod bitwise_xor_ref_lhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero ^ Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero ^ Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One ^ Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One ^ Bit::One, Bit::Zero);
    }
}

impl<'a> BitXor<&'a Bit> for Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: &'a Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(&self, rhs)
    }
}

#[cfg(test)]
mod bitwise_xor_ref_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(Bit::Zero ^ &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(Bit::Zero ^ &Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(Bit::One ^ &Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(Bit::One ^ &Bit::One, Bit::Zero);
    }
}

impl<'a, 'b> BitXor<&'a Bit> for &'b Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: &'a Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(self, rhs)
    }
}

#[cfg(test)]
mod bitwise_xor_ref_lhs_and_rhs_tests {
    use super::*;

    #[test]
    fn zero_zero() {
        assert_eq!(&Bit::Zero ^ &Bit::Zero, Bit::Zero);
    }

    #[test]
    fn zero_one() {
        assert_eq!(&Bit::Zero ^ &Bit::One, Bit::One);
    }

    #[test]
    fn one_zero() {
        assert_eq!(&Bit::One ^ &Bit::Zero, Bit::One);
    }

    #[test]
    fn one_one() {
        assert_eq!(&Bit::One ^ &Bit::One, Bit::Zero);
    }
}

impl Default for Bit {
    fn default() -> Bit {
        Bit::Zero
    }
}

#[cfg(test)]
mod default_tests {
    use std::default::Default;
    use super::*;

    #[test]
    fn test()
    {
        assert_eq!(Default::default(), Bit::Zero);
    }
}

impl FromStr for Bit {
    type Err = ParseBitError;

    fn from_str(s : &str) -> Result<Bit, ParseBitError> {
        match s {
            "0" => Ok(Bit::Zero),
            "1" => Ok(Bit::One),
            _ => Err(ParseBitError)
        }
    }
}

#[cfg(test)]
mod from_str_tests {
    use std::str::FromStr;
    use std::error::Error;
    use super::*;

    #[test]
    fn zero()
    {
        assert_eq!(FromStr::from_str("0"), Ok(Bit::Zero));
    }

    #[test]
    fn one()
    {
        assert_eq!(FromStr::from_str("1"), Ok(Bit::One));
    }

    #[test]
    fn invalid()
    {
        assert!(<Bit as FromStr>::from_str("something else").is_err());
    }

    #[test]
    fn error_display()
    {
        match <Bit as FromStr>::from_str("something else") {
            Ok (..) => unreachable!(),
            Err (e) =>
                assert_eq!(
                    format!("{}", e),
                    "provided string was not `0` or `1`")
        }
    }

    #[test]
    fn error_message()
    {
        match <Bit as FromStr>::from_str("something else") {
            Ok (..) => unreachable!(),
            Err (ref e) =>
                assert_eq!(
                    Error::description(e),
                    "failed to parse Bit")
        }
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Bit::Zero => "0",
            Bit::One => "1"
        }
        .fmt(f)
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn zero()
    {
        assert_eq!(format!("{}", Bit::Zero), "0");
    }

    #[test]
    fn one()
    {
        assert_eq!(format!("{}", Bit::One), "1");
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseBitError;

impl Display for ParseBitError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        "provided string was not `0` or `1`".fmt(f)
    }
}

mod parse_bit_error_display_tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", ParseBitError),
            "provided string was not `0` or `1`");
    }
}

impl Error for ParseBitError {
    fn description(&self) -> &str { "failed to parse Bit" }
}

mod parse_bit_error_description_tests {
    use std::error::Error;
    use super::*;

    #[test]
    fn description() {
        assert_eq!(
            Error::description(&ParseBitError),
            "failed to parse Bit");
    }
}
