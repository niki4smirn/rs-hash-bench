#![feature(test)]
extern crate rand;
extern crate test;

mod hasher;
mod my_ahash;
mod my_fastmurmur3;
mod my_highway;
mod my_murmur3;
mod my_rustc_hash;
mod my_seahash;
mod my_xxhash;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use test::Bencher;

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use hasher::Hasher;

    const INPUT_SIZE: usize = 1000000;

    #[test]
    fn write_u64_input() {
        if Path::new("input_u64.txt").exists() {
            return;
        }
        let mut file = File::create("input_u64.txt").unwrap();
        for _ in 0..INPUT_SIZE {
            let val = rand::thread_rng().gen::<u64>();
            file.write_all(val.to_string().as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
        }
    }

    #[test]
    fn write_string_input() {
        if Path::new("input_string.txt").exists() {
            return;
        }
        let mut file = File::create("input_string.txt").unwrap();
        for _ in 0..INPUT_SIZE {
            let str_len = rand::thread_rng().gen_range(10..100);
            let mut val = String::new();
            for _ in 0..str_len {
                val.push(rand::thread_rng().gen_range('a'..'z'));
            }
            file.write_all(val.as_bytes()).unwrap();
            file.write_all("\n".as_bytes()).unwrap();
        }
    }

    const OPS_PER_ITER: usize = 1000;

    fn bench_u64<H: Hasher>(b: &mut Bencher) {
        let mut file = File::open("input_u64.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let nums = contents
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut idx = 0;
        let hasher = H::new();
        b.iter(|| {
            for _ in 0..OPS_PER_ITER {
                if idx == INPUT_SIZE {
                    idx = 0;
                }
                let _ = hasher.hash_u64(nums[idx]);
                idx += 1;
            }
        });
    }

    fn bench_string<H: Hasher>(b: &mut Bencher) {
        let mut file = File::open("input_string.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let lines = contents
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();
        let mut idx = 0;
        let hasher = H::new();
        b.iter(|| {
            for _ in 0..OPS_PER_ITER {
                if idx == INPUT_SIZE {
                    idx = 0;
                }
                let _ = hasher.hash_str(lines[idx]);
                idx += 1;
            }
        });
    }

    #[bench]
    fn bench_u64_ahash(b: &mut Bencher) {
        bench_u64::<my_ahash::AHasher>(b);
    }

    #[bench]
    fn bench_string_ahash(b: &mut Bencher) {
        bench_string::<my_ahash::AHasher>(b);
    }

    #[bench]
    fn bench_u64_fastmurmur3(b: &mut Bencher) {
        bench_u64::<my_fastmurmur3::FastMurmur3Hasher>(b);
    }

    #[bench]
    fn bench_string_fastmurmur3(b: &mut Bencher) {
        bench_string::<my_fastmurmur3::FastMurmur3Hasher>(b);
    }

    #[bench]
    fn bench_u64_highway(b: &mut Bencher) {
        bench_u64::<my_highway::HighwayHasher>(b);
    }

    #[bench]
    fn bench_string_highway(b: &mut Bencher) {
        bench_string::<my_highway::HighwayHasher>(b);
    }

    #[bench]
    fn bench_u64_murmur3(b: &mut Bencher) {
        bench_u64::<my_murmur3::MyMurmur3>(b);
    }

    #[bench]
    fn bench_string_murmur3(b: &mut Bencher) {
        bench_string::<my_murmur3::MyMurmur3>(b);
    }

    #[bench]
    fn bench_u64_rustc_hash(b: &mut Bencher) {
        bench_u64::<my_rustc_hash::MyRustcHash>(b);
    }

    #[bench]
    fn bench_string_rustc_hash(b: &mut Bencher) {
        bench_string::<my_rustc_hash::MyRustcHash>(b);
    }

    #[bench]
    fn bench_u64_seahash(b: &mut Bencher) {
        bench_u64::<my_seahash::SeaHasher>(b);
    }

    #[bench]
    fn bench_string_seahash(b: &mut Bencher) {
        bench_string::<my_seahash::SeaHasher>(b);
    }

    #[bench]
    fn bench_u64_xxhash(b: &mut Bencher) {
        bench_u64::<my_xxhash::XXHasher>(b);
    }

    #[bench]
    fn bench_string_xxhash(b: &mut Bencher) {
        bench_string::<my_xxhash::XXHasher>(b);
    }
}
