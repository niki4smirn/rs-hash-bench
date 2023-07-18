use crate::hasher::Hasher;
use std::hash::Hasher as StdHasher;

use rustc_hash::FxHasher;

pub(crate) struct MyRustcHash;

impl Hasher for MyRustcHash {
    fn new() -> Self {
        MyRustcHash
    }

    #[inline]
    fn hash_str(&self, val: &[u8]) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write(val);
        hasher.finish()
    }

    #[inline]
    fn hash_u64(&self, val: u64) -> u64 {
        let mut hasher = FxHasher::default();
        hasher.write(&val.to_le_bytes());
        hasher.finish()
    }
}
