use crate::hasher::Hasher;
use murmur3::murmur3_x64_128;
use std::io::Cursor;

pub(crate) struct MyMurmur3;

impl Hasher for MyMurmur3 {
    fn new() -> Self {
        Self
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        murmur3_x64_128(&mut Cursor::new(val), 0).unwrap() as u64
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        murmur3_x64_128(&mut Cursor::new(val.to_le_bytes()), 0).unwrap() as u64
    }
}
