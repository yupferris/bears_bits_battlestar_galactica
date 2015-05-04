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

#[test]
fn partial_eq_tests_eq_zero_zero() {
    assert!(Bit::Zero == Bit::Zero);
}

#[test]
#[should_panic]
fn partial_eq_tests_eq_zero_one() {
    assert!(Bit::Zero == Bit::One);
}

#[test]
#[should_panic]
fn partial_eq_tests_eq_one_zero() {
    assert!(Bit::One == Bit::Zero);
}

#[test]
fn partial_eq_tests_eq_one_one() {
    assert!(Bit::One == Bit::One);
}

#[test]
#[should_panic]
fn partial_eq_tests_ne_zero_zero() {
    assert!(Bit::Zero != Bit::Zero);
}

#[test]
fn partial_eq_tests_ne_zero_one() {
    assert!(Bit::Zero != Bit::One);
}

#[test]
fn partial_eq_tests_ne_one_zero() {
    assert!(Bit::One != Bit::Zero);
}

#[test]
#[should_panic]
fn partial_eq_tests_ne_one_one() {
    assert!(Bit::One != Bit::One);
}

impl Bit {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Bit::Zero => 0,
            Bit::One => 1
        }
    }
}

#[test]
fn to_i32_tests_zero() {
    assert_eq!(Bit::Zero.to_i32(), 0);
}

#[test]
fn to_i32_tests_one() {
    assert_eq!(Bit::One.to_i32(), 1);
}

impl Bit {
    pub fn to_u32(&self) -> u32 {
        match *self {
            Bit::Zero => 0u32,
            Bit::One => 1u32
        }
    }
}

#[test]
fn to_u32_tests_zero() {
    assert_eq!(Bit::Zero.to_u32(), 0);
}

#[test]
fn to_u32_tests_one() {
    assert_eq!(Bit::One.to_u32(), 1);
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

#[test]
fn not_tests_zero() {
    assert_eq!(!Bit::Zero, Bit::One);
}

#[test]
fn not_tests_one() {
    assert_eq!(!Bit::One, Bit::Zero);
}

impl<'a> Not for &'a Bit {
    type Output = <Bit as Not>::Output;

    fn not(self) -> <Bit as Not>::Output {
        not_impl(self)
    }
}

#[test]
fn not_ref_tests_zero() {
    assert_eq!(!&Bit::Zero, Bit::One);
}

#[test]
fn not_ref_tests_one() {
    assert_eq!(!&Bit::One, Bit::Zero);
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

#[test]
fn bitwise_and_tests_zero_zero() {
    assert_eq!(Bit::Zero & Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_tests_zero_one() {
    assert_eq!(Bit::Zero & Bit::One, Bit::Zero);
}

#[test]
fn bitwise_and_tests_one_zero() {
    assert_eq!(Bit::One & Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_tests_one_one() {
    assert_eq!(Bit::One & Bit::One, Bit::One);
}

impl<'a> BitAnd<Bit> for &'a Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(self, &rhs)
    }
}

#[test]
fn bitwise_and_ref_lhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero & Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_tests_zero_one() {
    assert_eq!(&Bit::Zero & Bit::One, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_tests_one_zero() {
    assert_eq!(&Bit::One & Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_tests_one_one() {
    assert_eq!(&Bit::One & Bit::One, Bit::One);
}

impl<'a> BitAnd<&'a Bit> for Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: &'a Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(&self, rhs)
    }
}

#[test]
fn bitwise_and_ref_rhs_tests_zero_zero() {
    assert_eq!(Bit::Zero & &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_rhs_tests_zero_one() {
    assert_eq!(Bit::Zero & &Bit::One, Bit::Zero);
}

#[test]
fn bitwise_and_ref_rhs_tests_one_zero() {
    assert_eq!(Bit::One & &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_rhs_tests_one_one() {
    assert_eq!(Bit::One & &Bit::One, Bit::One);
}

impl<'a, 'b> BitAnd<&'a Bit> for &'b Bit {
    type Output = <Bit as BitAnd<Bit>>::Output;

    fn bitand(self, rhs: &'a Bit) -> <Bit as BitAnd<Bit>>::Output {
        bitand_impl(self, rhs)
    }
}

#[test]
fn bitwise_and_ref_lhs_and_rhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero & &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_and_rhs_tests_zero_one() {
    assert_eq!(&Bit::Zero & &Bit::One, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_and_rhs_tests_one_zero() {
    assert_eq!(&Bit::One & &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_and_ref_lhs_and_rhs_tests_one_one() {
    assert_eq!(&Bit::One & &Bit::One, Bit::One);
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

#[test]
fn bitwise_or_tests_zero_zero() {
    assert_eq!(Bit::Zero | Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_or_tests_zero_one() {
    assert_eq!(Bit::Zero | Bit::One, Bit::One);
}

#[test]
fn bitwise_or_tests_one_zero() {
    assert_eq!(Bit::One | Bit::Zero, Bit::One);
}

#[test]
fn bitwise_or_tests_one_one() {
    assert_eq!(Bit::One | Bit::One, Bit::One);
}

impl<'a> BitOr<Bit> for &'a Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(self, &rhs)
    }
}

#[test]
fn bitwise_or_ref_lhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero | Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_or_ref_lhs_tests_zero_one() {
    assert_eq!(&Bit::Zero | Bit::One, Bit::One);
}

#[test]
fn bitwise_or_ref_lhs_tests_one_zero() {
    assert_eq!(&Bit::One | Bit::Zero, Bit::One);
}

#[test]
fn bitwise_or_ref_lhs_tests_one_one() {
    assert_eq!(&Bit::One | Bit::One, Bit::One);
}

impl<'a> BitOr<&'a Bit> for Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: &'a Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(&self, rhs)
    }
}

#[test]
fn bitwise_or_ref_rhs_tests_zero_zero() {
    assert_eq!(Bit::Zero | &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_or_ref_rhs_tests_zero_one() {
    assert_eq!(Bit::Zero | &Bit::One, Bit::One);
}

#[test]
fn bitwise_or_ref_rhs_tests_one_zero() {
    assert_eq!(Bit::One | &Bit::Zero, Bit::One);
}

#[test]
fn bitwise_or_ref_rhs_tests_one_one() {
    assert_eq!(Bit::One | &Bit::One, Bit::One);
}

impl<'a, 'b> BitOr<&'a Bit> for &'b Bit {
    type Output = <Bit as BitOr<Bit>>::Output;

    fn bitor(self, rhs: &'a Bit) -> <Bit as BitOr<Bit>>::Output {
        bitor_impl(self, rhs)
    }
}

#[test]
fn bitwise_or_ref_lhs_and_rhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero | &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_or_ref_lhs_and_rhs_tests_zero_one() {
    assert_eq!(&Bit::Zero | &Bit::One, Bit::One);
}

#[test]
fn bitwise_or_ref_lhs_and_rhs_tests_one_zero() {
    assert_eq!(&Bit::One | &Bit::Zero, Bit::One);
}

#[test]
fn bitwise_or_ref_lhs_and_rhs_tests_one_one() {
    assert_eq!(&Bit::One | &Bit::One, Bit::One);
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

#[test]
fn bitwise_xor_tests_zero_zero() {
    assert_eq!(Bit::Zero ^ Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_xor_tests_zero_one() {
    assert_eq!(Bit::Zero ^ Bit::One, Bit::One);
}

#[test]
fn bitwise_xor_tests_one_zero() {
    assert_eq!(Bit::One ^ Bit::Zero, Bit::One);
}

#[test]
fn bitwise_xor_tests_one_one() {
    assert_eq!(Bit::One ^ Bit::One, Bit::Zero);
}

impl<'a> BitXor<Bit> for &'a Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(self, &rhs)
    }
}

#[test]
fn bitwise_xor_ref_lhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero ^ Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_xor_ref_lhs_tests_zero_one() {
    assert_eq!(&Bit::Zero ^ Bit::One, Bit::One);
}

#[test]
fn bitwise_xor_ref_lhs_tests_one_zero() {
    assert_eq!(&Bit::One ^ Bit::Zero, Bit::One);
}

#[test]
fn bitwise_xor_ref_lhs_tests_one_one() {
    assert_eq!(&Bit::One ^ Bit::One, Bit::Zero);
}

impl<'a> BitXor<&'a Bit> for Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: &'a Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(&self, rhs)
    }
}

#[test]
fn bitwise_xor_ref_rhs_tests_zero_zero() {
    assert_eq!(Bit::Zero ^ &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_xor_ref_rhs_tests_zero_one() {
    assert_eq!(Bit::Zero ^ &Bit::One, Bit::One);
}

#[test]
fn bitwise_xor_ref_rhs_tests_one_zero() {
    assert_eq!(Bit::One ^ &Bit::Zero, Bit::One);
}

#[test]
fn bitwise_xor_ref_rhs_tests_one_one() {
    assert_eq!(Bit::One ^ &Bit::One, Bit::Zero);
}

impl<'a, 'b> BitXor<&'a Bit> for &'b Bit {
    type Output = <Bit as BitXor<Bit>>::Output;

    fn bitxor(self, rhs: &'a Bit) -> <Bit as BitXor<Bit>>::Output {
        bitxor_impl(self, rhs)
    }
}

#[test]
fn bitwise_xor_ref_lhs_and_rhs_tests_zero_zero() {
    assert_eq!(&Bit::Zero ^ &Bit::Zero, Bit::Zero);
}

#[test]
fn bitwise_xor_ref_lhs_and_rhs_tests_zero_one() {
    assert_eq!(&Bit::Zero ^ &Bit::One, Bit::One);
}

#[test]
fn bitwise_xor_ref_lhs_and_rhs_tests_one_zero() {
    assert_eq!(&Bit::One ^ &Bit::Zero, Bit::One);
}

#[test]
fn bitwise_xor_ref_lhs_and_rhs_tests_one_one() {
    assert_eq!(&Bit::One ^ &Bit::One, Bit::Zero);
}

impl Default for Bit {
    fn default() -> Bit {
        Bit::Zero
    }
}

#[test]
fn default_tests_test()
{
    let def : Bit = Default::default();
    assert_eq!(def, Bit::Zero);
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

#[test]
fn from_str_tests_zero()
{
    assert_eq!(FromStr::from_str("0"), Ok(Bit::Zero));
}

#[test]
fn from_str_tests_one()
{
    assert_eq!(FromStr::from_str("1"), Ok(Bit::One));
}

#[test]
fn from_str_tests_invalid()
{
    assert!(<Bit as FromStr>::from_str("something else").is_err());
}

#[test]
fn from_str_tests_error_display()
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
fn from_str_tests_error_message()
{
    match <Bit as FromStr>::from_str("something else") {
        Ok (..) => unreachable!(),
        Err (ref e) =>
            assert_eq!(
                Error::description(e),
                "failed to parse Bit")
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

#[test]
fn display_tests_zero()
{
    assert_eq!(format!("{}", Bit::Zero), "0");
}

#[test]
fn display_tests_one()
{
    assert_eq!(format!("{}", Bit::One), "1");
}

#[derive(PartialEq, Debug)]
pub struct ParseBitError;

impl Display for ParseBitError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        "provided string was not `0` or `1`".fmt(f)
    }
}

#[test]
fn parse_bit_error_display_tests_display() {
    assert_eq!(
        format!("{}", ParseBitError),
        "provided string was not `0` or `1`");
}

impl Error for ParseBitError {
    fn description(&self) -> &str { "failed to parse Bit" }
}

#[test]
fn parse_bit_error_description_tests_description() {
    assert_eq!(
        Error::description(&ParseBitError),
        "failed to parse Bit");
}
