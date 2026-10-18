use std::io::{self, Read, Seek};

const BUFFER_SIZE: usize = 64 * 1024;

pub fn frequency<R: Read + Seek>(data_bytes: &mut R) -> io::Result<[u64; 256]> {
    let mut map = [0u64; 256];
    let mut read_buffer = [0u8; BUFFER_SIZE];
    loop {
        let bytes_read = data_bytes.read(&mut read_buffer)?;
        if bytes_read == 0 {
            break;
        }
        for &byte in &read_buffer[..bytes_read] {
            map[byte as usize] += 1
        }
    }
    data_bytes.rewind()?;
    Ok(map)
}

#[cfg(test)]

mod frequency_tester {
    use std::io::Cursor;

    use super::*;
    #[test]
    fn frequency_test_empty_byte() {
        let mut reader = Cursor::new(&[][..]);
        let map = frequency(&mut reader).unwrap();
        assert!(map.iter().all(|&e| e == 0));
    }

    #[test]
    fn frequency_test_one_element() {
        let mut reader = Cursor::new(&[2, 2, 2, 2]);
        let map = frequency(&mut reader).unwrap();
        assert_eq!(map[2], 4);
    }

    #[test]
    fn frequency_test_multiple_elements() {
        let mut reader = Cursor::new([2, 5, 2, 3, 3, 5, 77, 2, 202, 2]);
        let map = frequency(&mut reader).unwrap();
        assert_eq!(map[2], 4);
        assert_eq!(map[5], 2);
        assert_eq!(map[3], 2);
        assert_eq!(map[77], 1);
        assert_eq!(map[202], 1);
    }
}
