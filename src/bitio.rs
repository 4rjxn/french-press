pub struct BitWriter {
    bytes: Vec<u8>,
    acc: u8,   //accumulator
    nbits: u8, //number of valid bit accumulator has
}

impl BitWriter {
    pub fn new() -> BitWriter {
        BitWriter {
            bytes: Vec::new(),
            acc: 0,
            nbits: 0,
        }
    }
    pub fn write(&mut self, bits: u64, len: u8) {
        for i in (0..len).rev() {
            let bit = ((bits >> i) & 1) as u8;
            self.acc = (self.acc << 1) | bit;
            self.nbits += 1;
            if self.nbits == 8 {
                self.bytes.push(self.acc);
                self.acc = 0;
                self.nbits = 0;
            }
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 {
            self.acc <<= 8 - self.nbits;
            self.bytes.push(self.acc);
        }
        self.bytes
    }
}

pub struct BitReader {
    bytes: Vec<u8>,
}

impl BitReader {
    pub fn new(bytes: &[u8]) -> BitReader {
        return BitReader {
            bytes: bytes.to_vec(),
        };
    }

    pub fn read(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let shift_count = index % 8;
        (self.bytes.get(byte_index).unwrap() << shift_count & 0x80) == 0x80
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
    fn bitio_reader_test() {
        let reader = BitReader::new(&[43]);
        let first_value = reader.read(0);
        let second_value = reader.read(7);
        assert_eq!(first_value, false);
        assert_eq!(second_value, true);
    }
}
