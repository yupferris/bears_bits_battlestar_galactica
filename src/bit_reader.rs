use super::bit::Bit;
use std::io::Result;

pub trait BitReader {
    fn read_bit(&mut self) -> Result<Bit>;
}
