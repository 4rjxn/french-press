mod bitio;
mod compress;
mod decompress;
mod frequency;
mod huffman;
mod tree;

pub(crate) const BUFFER_SIZE: usize = 64 * 1024;

pub use compress::compress;
pub use decompress::decompress;
