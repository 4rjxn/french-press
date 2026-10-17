use crate::{bitio::BitReader, tree::build_tree};

fn read_header(data_bytes: &[u8]) -> ([u64; 256], usize) {
    let mut freq = [0u64; 256];
    if data_bytes.len() < 6 {
        return (freq, 0);
    }
    let frequency_count = u16::from_le_bytes(data_bytes[4..6].try_into().unwrap());
    let mut char_index = 6; //the first charector byte is at 6;
    for _ in 0..frequency_count {
        let index = data_bytes[char_index];
        freq[index as usize] = u64::from_le_bytes(
            data_bytes[char_index + 1..char_index + 9]
                .try_into()
                .unwrap(),
        );
        char_index += 9;
    }
    (freq, char_index)
}

pub fn decompress(data_bytes: &[u8]) -> Vec<u8> {
    let mut decompressed_data: Vec<u8> = Vec::new();
    let (freq, header_end_byte) = read_header(data_bytes);
    let total_chars: usize = freq.iter().map(|&count| count as usize).sum();
    let root = build_tree(&freq).unwrap();
    let reader = BitReader::new(data_bytes);
    let mut pointer = &root;
    let mut bit_index = header_end_byte * 8;
    while bit_index < data_bytes.len() * 8 && decompressed_data.len() < total_chars {
        match pointer {
            crate::tree::Node::Leaf { byte, .. } => {
                decompressed_data.push(*byte);
                pointer = &root;
            }
            crate::tree::Node::Internal { .. } => {
                match reader.read(bit_index) {
                    true => pointer = pointer.right().unwrap(),
                    false => pointer = pointer.left().unwrap(),
                }
                bit_index += 1;
            }
        }
    }
    println!("count: {} total_chars: {}", bit_index / 8, total_chars);
    decompressed_data
}

#[cfg(test)]
mod decompress_tester {
    use crate::compress;

    use super::*;

    #[test]
    fn test_read_header_function() {
        let mut expectied_freq = [0u64; 256];
        expectied_freq[b'a' as usize] = 33;
        expectied_freq[b'b' as usize] = 33;

        let mut header: Vec<u8> = Vec::new();
        header.extend_from_slice(b"FRPS");
        header.extend_from_slice(&2u16.to_le_bytes());

        header.push(b'a');
        header.extend_from_slice(&(33 as u64).to_le_bytes());
        header.push(b'b');
        header.extend_from_slice(&(33 as u64).to_le_bytes());
        let header_freq = read_header(&header);
        assert_eq!(header_freq, expectied_freq);
    }
}
