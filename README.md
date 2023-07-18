## Small research on fast non-cryptographic hash algorithms in Rust

Algorithms were tested on random u64 values and random size (10..100) strings consisting of random lowercase English letters.

*Note: each iteration is hashing 1000 objects*

String:
| Algorithm   | ns/iter | +- ns/iter |
|-------------|---------|------------|
| ahash       | 638     | 12         |
| fastmurmur3 | 26,966  | 1,069      |
| highway     | 45,237  | 1,412      |
| murmur3     | 60,530  | 2,353      |
| rustc_hash  | 637     | 10         |
| seahash     | 29,387  | 877        |
| xxhash      | 12,068  | 379        |

u64:
| Algorithm   | ns/iter | +- ns/iter |
|-------------|---------|------------|
| ahash       | 883     | 28         |
| fastmurmur3 | 5,161   | 160        |
| highway     | 39,604  | 552        |
| murmur3     | 24,121  | 1,118      |
| rustc_hash  | 892     | 24         |
| seahash     | 12,960  | 332        |
| xxhash      | 3,880   | 201        |


As you can see, ahash and rustc-hash significantly outperform other algorithms. However, these algorithms are not *silver bullets*:
- Ahash is intended exclusively for use in in-memory hashmaps. Also, it's claimed to be DOS resistant, but this fact is not proven, as far as I know.
- rustc-hash is not DOS resistant.

If DOS resistance is crucial for you, consider highway.

## Contribution
Feel free to add more algorihtms or improve the quality of results.
