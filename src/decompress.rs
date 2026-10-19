use std::io::{self, ErrorKind, Read, Write};

use crate::{BUFFER_SIZE, bitio::BitReader, tree::build_tree};

fn read_header<R: Read>(data_bytes: &mut R) -> io::Result<[u64; 256]> {
    let mut freq = [0u64; 256];
    let mut header_data_buffer = [0u8; 6];
    data_bytes.read_exact(&mut header_data_buffer)?;
    if &header_data_buffer[0..4] != b"FRPS" {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "the magic bytes is not present! (file )",
        ));
    }
    let frequency_count = u16::from_le_bytes([header_data_buffer[4], header_data_buffer[5]]);
    let mut freq_buffer = [0u8; 9];
    for _ in 0..frequency_count {
        data_bytes.read_exact(&mut freq_buffer)?;
        let index = freq_buffer[0];
        freq[index as usize] = u64::from_le_bytes(
            freq_buffer[1..9]
                .try_into()
                .map_err(|_| io::Error::from(ErrorKind::InvalidData))?,
        );
    }
    Ok(freq)
}

pub fn decompress<R: Read, W: Write>(data_bytes: &mut R, output_writer: &mut W) -> io::Result<()> {
    let freq = read_header(data_bytes)?;
    let total_chars: usize = freq.iter().map(|&count| count as usize).sum();
    if total_chars == 0 {
        return Ok(());
    }
    let root = build_tree(&freq).unwrap();
    let mut reader = BitReader::new(data_bytes);
    let mut pointer = &root;
    let mut output_buff = [0u8; BUFFER_SIZE];
    let mut buf_index = 0;
    let mut chars_decoded = 0;
    while chars_decoded < total_chars {
        match pointer {
            crate::tree::Node::Leaf { byte, .. } => {
                output_buff[buf_index] = *byte;
                buf_index += 1;
                chars_decoded += 1;

                if buf_index == BUFFER_SIZE {
                    output_writer.write_all(&output_buff)?;
                    buf_index = 0;
                }
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
    if buf_index > 0 {
        output_writer.write_all(&output_buff[..buf_index])?;
    }
    output_writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod decompress_tester {
    use std::io::Cursor;

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
        let header_freq = read_header(&mut Cursor::new(header)).unwrap();
        assert_eq!(header_freq, expectied_freq);
    }

    #[test]
    fn test_decompress_function() {
        let data = b"french_press";
        let mut compressed_data = Vec::new();
        compress::compress(&mut Cursor::new(data), &mut compressed_data).unwrap();
        let mut decompressed_data = Vec::new();
        decompress(&mut Cursor::new(compressed_data), &mut decompressed_data).unwrap();
        assert_eq!(data.as_slice(), decompressed_data.as_slice());
    }

    #[test]
    fn test_decompress_function_empty_file() {
        let data = b"";
        let mut compressed_data = Vec::new();
        compress::compress(&mut Cursor::new(data), &mut compressed_data).unwrap();
        let mut decompressed_data = Vec::new();
        decompress(&mut Cursor::new(compressed_data), &mut decompressed_data).unwrap();
        assert!(decompressed_data.is_empty());
    }

    #[test]
    fn test_decompress_function_single_char_file() {
        let data = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut compressed_data = Vec::new();
        compress::compress(&mut Cursor::new(data), &mut compressed_data).unwrap();
        let mut decompressed_data = Vec::new();
        decompress(&mut Cursor::new(compressed_data), &mut decompressed_data).unwrap();
        assert_eq!(data.as_slice(), decompressed_data.as_slice());
    }
}
