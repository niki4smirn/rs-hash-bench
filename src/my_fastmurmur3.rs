use crate::hasher::Hasher;

pub(crate) struct FastMurmur3Hasher;

impl Hasher for FastMurmur3Hasher {
    fn new() -> Self {
        Self
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        fastmurmur3::hash(&val.to_le_bytes()) as u64
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        fastmurmur3::hash(val) as u64
    }
}
