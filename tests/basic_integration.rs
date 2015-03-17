extern crate bears_bits_battlestar_galactica;

use bears_bits_battlestar_galactica::Bit;

#[test]
fn basic_integration_test()
{
    let x = Bit::Zero;
    let y = Bit::One;

    assert_eq!(x & y, Bit::Zero);
}
