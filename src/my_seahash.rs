use crate::hasher::Hasher;

pub(crate) struct SeaHasher;

impl Hasher for SeaHasher {
    fn new() -> Self {
        SeaHasher
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        seahash::hash(val)
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        seahash::hash(&val.to_le_bytes())
    }
}
