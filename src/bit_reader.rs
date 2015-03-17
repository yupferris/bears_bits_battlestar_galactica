use super::bit::Bit;
use std::io::{Read, Bytes, Result};

pub struct BitReader<R: Read> {
    bytes: Bytes<R>
}

impl<R: Read> BitReader<R> {
    fn new(inner: R) -> BitReader<R> {
        BitReader { bytes: inner.bytes() }
    }
}

impl<R: Read> BitReader<R> {
    fn read_bit(&mut self) -> Result<Bit>
    {
        Ok(Bit::Zero)
    }
}
