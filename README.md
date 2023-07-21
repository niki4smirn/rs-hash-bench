## Small research on fast non-cryptographic hash algorithms in Rust

Algorithms were tested on random u64 values and random size (10..100) strings consisting of random lowercase English letters.

*Note: each iteration is hashing 1000 objects. All measurements are in ns/iter*

### string
| Name | Median | Deviation |
|------|-------:|----------:|
| ahash | 634 | 7 |
| fastmurmur3 | 27948 | 338 |
| highway | 44904 | 1170 |
| murmur3 | 57447 | 736 |
| rustc_hash | 634 | 6 |
| seahash | 29717 | 730 |
| xxhash | 12003 | 230 |

### u64
| Name | Median | Deviation |
|------|-------:|----------:|
| ahash | 882 | 11 |
| fastmurmur3 | 3839 | 106 |
| highway | 39459 | 920 |
| murmur3 | 23009 | 373 |
| rustc_hash | 882 | 13 |
| seahash | 13301 | 319 |
| xxhash | 3593 | 117 |


As you can see, ahash and rustc-hash significantly outperform other algorithms. However, these algorithms are not *silver bullets*:
- Ahash is intended exclusively for use in in-memory hashmaps. Also, it's claimed to be DOS resistant, but this fact is not proven, as far as I know.
- rustc-hash is not DOS resistant.

If DOS resistance is crucial for you, consider highway.

## Contribution
Feel free to add more algorihtms or improve the quality of results.
