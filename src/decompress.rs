fn read_header(data_bytes: &[u8]) -> [u64; 256] {
    let mut freq = [0u64; 256];
    if data_bytes.len() < 6 {
        return freq;
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
    freq
}

#[cfg(test)]
mod decompress_tester {
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
