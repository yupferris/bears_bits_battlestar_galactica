extern crate bitstreams;

use bitstreams::Bit;

#[test]
fn basic_integration_test()
{
    let x = Bit::Zero;
    let y = Bit::One;

    assert_eq!(x & y, Bit::Zero);
}
