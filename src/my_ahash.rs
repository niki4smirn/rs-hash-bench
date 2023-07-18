use crate::hasher::Hasher;

pub(crate) struct AHasher {
    hash_builder: ahash::RandomState,
}

impl Hasher for AHasher {
    fn new() -> Self {
        Self {
            hash_builder: ahash::RandomState::new(),
        }
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        self.hash_builder.hash_one(val)
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        self.hash_builder.hash_one(val)
    }
}
