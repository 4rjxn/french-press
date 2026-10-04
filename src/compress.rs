use crate::{
    bitio::BitWriter,
    frequency::frequency,
    huffman::{Code, generate_codes},
    tree::build_tree,
};

fn write_header(frequency: &[u64; 256]) -> Vec<u8> {
    let used: Vec<(u8, u64)> = frequency
        .iter()
        .enumerate()
        .filter(|(_, f)| **f > 0)
        .map(|(b, &f)| (b as u8, f))
        .collect();

    let mut out = Vec::new();
    out.extend_from_slice(b"FRPS"); //4 byte
    out.extend_from_slice(&(used.len() as u16).to_le_bytes()); //2 byte
    for (byte, freq) in used {
        out.push(byte); //1 byte
        out.extend_from_slice(&freq.to_le_bytes()); //8 byte
    }
    out
}

pub fn compress(data_bytes: &[u8]) -> Vec<u8> {
    let freq = frequency(data_bytes);
    let mut header = write_header(&freq);
    let Some(huffman_root) = build_tree(&freq) else {
        return header;
    };
    let mut code_table = [Code::default(); 256];
    generate_codes(&huffman_root, 0, 0, &mut code_table);
    let mut writer = BitWriter::new();
    for &byte in data_bytes {
        let code = code_table[byte as usize];
        writer.write(code.bits, code.len);
    }
    header.extend(writer.finish());
    header
}

#[cfg(test)]
mod compress_tester {
    use super::*;

    #[test]
    fn test_header_writer_one_charector_freq() {
        let mut freq = [0u64; 256];
        freq[b'a' as usize] = 555;

        let header = vec![
            b'F', b'R', b'P', b'S', 1, 0, b'a', 0x2B, 0x02, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(write_header(&freq), header);
    }

    #[test]
    fn test_compression() {
        let data_bytes = b"aaabbc";
        let header = write_header(&frequency(data_bytes));
        let compressed_data = compress(data_bytes);
        assert_eq!(&compressed_data[0..header.len()], &header[..]);
        assert_eq!(&compressed_data[header.len()..], &[0x1F, 0x0]);
    }
}
