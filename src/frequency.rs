pub fn frequency(data_bytes: &[u8]) -> [u64; 256] {
    let mut map = [0u64; 256];
    for &byte in data_bytes {
        map[byte as usize] += 1;
    }
    map
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn frequency_test_empty_byte() {
        let map = frequency(&[]);
        assert!(map.iter().all(|&e| e == 0));
    }

    #[test]
    fn frequency_test_one_element() {
        let map = frequency(&[2, 2, 2, 2]);
        assert_eq!(map[2], 4);
    }

    #[test]
    fn frequency_test_multiple_elements() {
        let map = frequency(&[2, 5, 2, 3, 3, 5, 77, 2, 202, 2]);
        assert_eq!(map[2], 4);
        assert_eq!(map[5], 2);
        assert_eq!(map[3], 2);
        assert_eq!(map[77], 1);
        assert_eq!(map[202], 1);
    }
}
