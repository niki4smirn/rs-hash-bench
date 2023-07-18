use crate::hasher::Hasher;

pub(crate) struct XXHasher;

impl Hasher for XXHasher {
    fn new() -> Self {
        XXHasher
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        xxhash_rust::xxh3::xxh3_64(val)
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        xxhash_rust::xxh3::xxh3_64(&val.to_le_bytes())
    }
}
