use std::io::{self, Write};

pub struct BitWriter<W: Write> {
    writer: W,
    acc: u8,   //accumulator
    nbits: u8, //number of valid bit accumulator has
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: writer,
            acc: 0,
            nbits: 0,
        }
    }
    pub fn write(&mut self, bits: u64, len: u8) -> io::Result<()> {
        for i in (0..len).rev() {
            let bit = ((bits >> i) & 1) as u8;
            self.acc = (self.acc << 1) | bit;
            self.nbits += 1;
            if self.nbits == 8 {
                self.writer.write_all(&[self.acc])?;
                self.acc = 0;
                self.nbits = 0;
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        if self.nbits > 0 {
            self.acc <<= 8 - self.nbits;
            self.writer.write_all(&[self.acc]);
        }
        self.writer.flush()?;
        Ok(())
    }
}

pub struct BitReader<'a> {
    bytes: &'a [u8],
    bit_index: usize,
    byte_index: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            bit_index: 0,
            byte_index: 0,
        }
    }
}

impl<'a> Iterator for BitReader<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let &byte = self.bytes.get(self.byte_index)?;
        let bit = ((byte << self.bit_index) & 0x80) == 0x80;
        self.bit_index += 1;
        if self.bit_index == 8 {
            self.byte_index += 1;
            self.bit_index = 0;
        }
        Some(bit)
    }
}

#[cfg(test)]
mod bit_writer_tester {

    use super::*;

    #[test]
    fn direct_byte_write_test() {
        let byte = 0b000101011 as u64;
        let mut writer = BitWriter::new();
        writer.write(byte, 9);
        let out = writer.finish();
        assert_eq!(&out[..], [0b00010101, 0b10000000]);
    }

    #[test]
    fn bitio_stream_test() {
        let mut reader = BitReader::new(&[43]);
        if let Some(bit) = reader.next() {
            assert_eq!(bit, false);
        }
        if let Some(bit) = reader.next() {
            assert_eq!(bit, false);
        }
        if let Some(bit) = reader.next() {
            assert_eq!(bit, true);
        }
    }
}
