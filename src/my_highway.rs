use crate::hasher::Hasher;
use highway::{HighwayHash, HighwayHasher as HighwayHasherImpl};

pub(crate) struct HighwayHasher;

impl Hasher for HighwayHasher {
    fn new() -> Self {
        Self
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        HighwayHasherImpl::default().hash64(&val.to_le_bytes())
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        HighwayHasherImpl::default().hash64(val)
    }
}
