use std::io::{self, Read, Write};

use crate::BUFFER_SIZE;

pub struct BitWriter<W: Write> {
    writer: W,
    acc: u64,  //accumulator
    nbits: u8, //number of valid bit accumulator has
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            acc: 0,
            nbits: 0,
        }
    }
    pub fn write(&mut self, bits: u64, len: u8) -> io::Result<()> {
        self.acc = (self.acc << len) | bits;
        self.nbits += len;
        while self.nbits >= 8 {
            self.nbits -= 8;
            let byte = (self.acc >> self.nbits) as u8;
            self.writer.write_all(&[byte])?;
        }
        Ok(())
    }

    pub fn finish(mut self) -> io::Result<()> {
        if self.nbits > 0 {
            let byte = (self.acc << (8 - self.nbits)) as u8;
            self.writer.write_all(&[byte])?;
            self.nbits = 0;
        }
        self.writer.flush()?;
        Ok(())
    }
}

pub struct BitReader<R: Read> {
    reader: R,
    buff: [u8; BUFFER_SIZE],
    bit_index: usize,
    bytes_read: usize,
    byte_index: usize,
}

impl<R: Read> BitReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buff: [0u8; BUFFER_SIZE],
            bit_index: 0,
            bytes_read: 0,
            byte_index: 0,
        }
    }
}

impl<R: Read> Iterator for BitReader<R> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index >= self.bytes_read {
            self.bytes_read = self.reader.read(&mut self.buff).ok()?;
            if self.bytes_read == 0 {
                return None;
            }
            self.byte_index = 0;
            self.bit_index = 0;
        }
        let byte = self.buff[self.byte_index];
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
    use std::io::Cursor;

    #[test]
    fn direct_byte_write_test() {
        let byte = 0b000101011 as u64;
        let mut out = Vec::new();
        let mut writer = BitWriter::new(&mut out);
        writer.write(byte, 9).unwrap();
        writer.finish().unwrap();
        assert_eq!(&out[..], [0b00010101, 0b10000000]);
    }

    #[test]
    fn multi_code_write_test() {
        let mut out = Vec::new();
        let mut writer = BitWriter::new(&mut out);
        // Write 3 bits: 0b101
        writer.write(0b101, 3).unwrap();
        // Write 5 bits: 0b01010 (completes first byte: 10101010 = 0xAA)
        writer.write(0b01010, 5).unwrap();
        // Write 4 bits: 0b1111 (padded to 11110000 = 0xF0)
        writer.write(0b1111, 4).unwrap();
        writer.finish().unwrap();
        assert_eq!(&out[..], [0xAA, 0xF0]);
    }

    #[test]
    fn bitio_stream_test() {
        // 42 in binary: 0b00101010
        let mut reader = BitReader::new(Cursor::new([42]));
        let expected_bits = [false, false, true, false, true, false, true, false];
        for expected in expected_bits {
            assert_eq!(reader.next(), Some(expected));
        }
        assert_eq!(reader.next(), None);
    }
}
