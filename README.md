# RC5 block-cipher in Rust

Library implementation of the basic RC5 block cipher in Rust. RC5 is different
from the classical ciphers (like AES) in the sense that allows to parametrize
the algorithm and optimize both security and efficiency on different hardware.

These parameters are:

* word length `w` (in bytes)
* number of rounds `r`
* key length `b` (in bytes)

The selection of each of them should be preferably done by choosing standards
from other use cases. For example the word length `w` could be any number of
bytes but the recommendation for performance and security is that should be a
power of 2, or even better, a power of 8. In that way one can use the hardware
registers more efficiently, e.g. 32-bits or 64-bits registers, with
vectorization possibilies (AVX on Intel or SVE on ARM).

This RC5 implementation is designed only for the standard values of `w` (powers
of 8) making use of the standard Rust types: u8, u16, u32, u64, u128.

## Error Handling

Since a cipher should be efficient and secure the way we handle errors is
through unrecoverable errors through panicking when something undersired occurs.
This means that when something unexpected happens during the program execution,
e.g. a bad length in an array we stop the program calling the `panic!` macro.
The users of this library need to have this into account when they pass
arguments to this library.

# Testing

The application runs in release mode only since arithmetic overflow should be
allowed: 

```
cargo test --release
```

# Benchmarking

To measure the performance of the application we provide different benchmarks,
this is using the `test` crate which is only available on the nichtly rust
version. To run the benchmarks do:

```
cargo +nightly bench

test tests::bench_encode_8_12_4           ... bench:         376 ns/iter (+/- 38)
test tests::bench_decode_8_12_4           ... bench:         353 ns/iter (+/- 40)
test tests::bench_encode_kernel_8_12_4    ... bench:         311 ns/iter (+/- 8)
test tests::bench_decode_kernel_8_12_4    ... bench:         317 ns/iter (+/- 21)

test tests::bench_encode_16_16_8          ... bench:         476 ns/iter (+/- 69)
test tests::bench_decode_16_16_8          ... bench:         450 ns/iter (+/- 8)
test tests::bench_encode_kernel_16_16_8   ... bench:         405 ns/iter (+/- 24)
test tests::bench_decode_kernel_16_16_8   ... bench:         410 ns/iter (+/- 9)

test tests::bench_encode_32_20_16         ... bench:         581 ns/iter (+/- 46)
test tests::bench_decode_32_20_16         ... bench:         563 ns/iter (+/- 37)
test tests::bench_encode_kernel_32_20_16  ... bench:         537 ns/iter (+/- 34)
test tests::bench_decode_kernel_32_20_16  ... bench:         519 ns/iter (+/- 11)

test tests::bench_encode_64_24_24         ... bench:         687 ns/iter (+/- 19)
test tests::bench_decode_64_24_24         ... bench:         674 ns/iter (+/- 93)
test tests::bench_encode_kernel_64_24_24  ... bench:         638 ns/iter (+/- 24)
test tests::bench_decode_kernel_64_24_24  ... bench:         637 ns/iter (+/- 42)

test tests::bench_encode_128_28_32        ... bench:       1,089 ns/iter (+/- 30)
test tests::bench_decode_128_28_32        ... bench:       1,057 ns/iter (+/- 27)
test tests::bench_encode_kernel_128_28_32 ... bench:         906 ns/iter (+/- 33)
test tests::bench_decode_kernel_128_28_32 ... bench:         928 ns/iter (+/- 65)
```

## TODO
 - [ ] Use case with a main to encrypt/decrypt a file (here an encryption mode
 should be selected)
 - [ ] Publish a crate

## Bibliography

* Rivest original paper: https://www.grc.com/r&d/rc5.pdf
* C implementation and tests: https://tools.ietf.org/id/draft-krovetz-rc6-rc5-vectors-00.html#rfc.section.4
* Haskell implementation: https://hackage.haskell.org/package/cipher-rc5-0.1.0.2/docs/src/Crypto-Cipher-RC5.html

