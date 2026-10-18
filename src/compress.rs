use std::io::{self, Read, Seek, Write};

use crate::{
    BUFFER_SIZE,
    bitio::BitWriter,
    frequency::frequency,
    huffman::{Code, generate_codes},
    tree::build_tree,
};

fn write_header<W: Write>(frequency: &[u64; 256], writer: &mut W) -> io::Result<()> {
    let used: Vec<(u8, u64)> = frequency
        .iter()
        .enumerate()
        .filter(|(_, f)| **f > 0)
        .map(|(b, &f)| (b as u8, f))
        .collect();

    writer.write_all(b"FRPS")?;
    writer.write_all(&(used.len() as u16).to_le_bytes())?; //2 byte
    for (byte, freq) in used {
        writer.write_all(&[byte])?; //1 byte
        writer.write_all(&freq.to_le_bytes())?; //8 byte
    }
    writer.flush()?;
    Ok(())
}

pub fn compress<R: Read + Seek, W: Write>(data_bytes: &mut R, writer: &mut W) -> io::Result<()> {
    let freq = frequency(data_bytes)?;
    write_header(&freq, writer)?;
    let Some(huffman_root) = build_tree(&freq) else {
        writer.flush()?;
        return Ok(());
    };
    let mut code_table = [Code::default(); 256];
    generate_codes(&huffman_root, 0, 0, &mut code_table);
    let mut writer = BitWriter::new(writer);
    let mut read_buffer = [0u8; BUFFER_SIZE];
    loop {
        let bytes_read = data_bytes.read(&mut read_buffer)?;
        if bytes_read == 0 {
            break;
        }
        for &byte in &read_buffer[..bytes_read] {
            let code = code_table[byte as usize];
            writer.write(code.bits, code.len)?;
        }
    }
    writer.finish()?;
    Ok(())
}

#[cfg(test)]
mod compress_tester {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_writer_one_charector_freq() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 555;

        let expected_header = vec![
            b'F', b'R', b'P', b'S', 1, 0, b'a', 0x2B, 0x02, 0, 0, 0, 0, 0, 0,
        ];

        let mut written_header = Vec::new();
        write_header(&freq, &mut written_header).unwrap();

        assert_eq!(written_header, expected_header);
    }

    #[test]
    fn test_compression() {
        let data_bytes = b"aaabbc";

        // 1. Generate expected header
        let mut freq_cursor = Cursor::new(data_bytes);
        let freq = frequency(&mut freq_cursor).unwrap();
        let mut expected_header = Vec::new();
        write_header(&freq, &mut expected_header).unwrap();

        // 2. Compress using Cursor as Read+Seek and Vec as Write
        let mut input_cursor = Cursor::new(data_bytes);
        let mut compressed_data = Vec::new();
        compress(&mut input_cursor, &mut compressed_data).unwrap();

        // 3. Verify header and bitstream body
        assert_eq!(
            &compressed_data[0..expected_header.len()],
            &expected_header[..]
        );
        assert_eq!(&compressed_data[expected_header.len()..], &[0x1F, 0x0]);
    }
}
