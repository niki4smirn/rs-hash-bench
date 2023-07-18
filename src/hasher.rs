pub(crate) trait Hasher {
    fn new() -> Self;
    fn hash_str(&self, val: &[u8]) -> u64;
    fn hash_u64(&self, val: u64) -> u64;
}
