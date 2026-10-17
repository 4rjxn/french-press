use std::fs::read;

use french_press::{compress, decompress};

fn main() {
    match read("blobs/mobby_dick.txt") {
        Ok(data_bytes) => {
            let compressed_data = compress::compress(&data_bytes);
            let decompressed_bytes = decompress::decompress(&compressed_data);
            println!(
                "before compression: {} compressed_size: {}",
                data_bytes.len(),
                compressed_data.len()
            );
            println!(
                "compression: {} decompressed_size: {}",
                compressed_data.len(),
                decompressed_bytes.len()
            );
        }
        Err(_) => {
            println!("invlaid file ")
        }
    }
}
