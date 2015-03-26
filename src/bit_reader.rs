use super::bit::Bit;
use std::io::{Read, Bytes, Result};
use std::vec::Vec;

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

struct MemReader {
    buffer: Vec<u8>,
    pos: u32
}

impl MemReader {
    fn new(buffer: Vec<u8>) -> MemReader {
        MemReader { buffer: buffer, pos: 0 }
    }

    fn eof(&self) -> bool {
        true
    }
}

#[test]
fn mem_reader_tests_empty_eof() {
    assert!(MemReader::new(vec!()).eof());
}
