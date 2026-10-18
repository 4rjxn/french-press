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
    if total_chars == 0 {
        return Vec::new();
    }
    let root = build_tree(&freq).unwrap();
    let mut reader = BitReader::new(&data_bytes[header_end_byte..]);
    let mut pointer = &root;
    while decompressed_data.len() < total_chars {
        match pointer {
            crate::tree::Node::Leaf { byte, .. } => {
                decompressed_data.push(*byte);
                pointer = &root;
                continue;
            }
            crate::tree::Node::Internal { .. } => match reader.next() {
                Some(bit) => {
                    if bit {
                        pointer = pointer.right().unwrap();
                    } else {
                        pointer = pointer.left().unwrap();
                    }
                }
                None => break,
            },
        }
    }

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
        let (header_freq, read_byte_count) = read_header(&header);
        assert_eq!(header_freq, expectied_freq);
        assert_eq!(read_byte_count, 24);
    }

    #[test]
    fn test_decompress_function() {
        let data = b"french_press";
        let compressed_data = compress::compress(data);
        let decompressed_data = decompress(&compressed_data);
        assert_eq!(data.len(), decompressed_data.len());
        assert_eq!(decompressed_data[0], b'f');
    }
    #[test]
    fn test_decompress_function_empty_file() {
        let data = b"";
        let compressed_data = compress::compress(data);
        let decompressed_data = decompress(&compressed_data);
        assert_eq!(data.len(), decompressed_data.len());
    }

    #[test]
    fn test_decompress_function_single_char_file() {
        let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let compressed_data = compress::compress(data);
        let decompressed_data = decompress(&compressed_data);
        assert_eq!(data.len(), decompressed_data.len());
        assert_eq!(decompressed_data[0], b'a');
    }
}
