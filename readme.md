Rust port of [simple-js-sha2-384](https://github.com/Azero123/simple-js-sha2-384). No dependencies.

how to import (`Cargo.toml`):
```toml
[dependencies]
simple-rs-sha2-384 = "1"
```
hashing:
```rust
use simple_rs_sha2_384::sha2_384;

sha2_384("hello"); // 59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f
simple_rs_sha2_384::digest(b"hello"); // raw [u8; 48]
```
Accepts anything that is `AsRef<[u8]>` (`&str`, `String`, `&[u8]`, `Vec<u8>`, ...). Strings are hashed as UTF-8.

performance:
```
cargo run --release --example performance
```
```
3836.07 kh/s ~ (Ryzen 9 7950X3D single thread, portable)
```
The portable implementation fully unrolls the rounds and computes the message schedule inline
(3354.48 kh/s -> 3836.07 kh/s, 581 MB/s -> 735 MB/s for large inputs).

On x86_64 CPUs with the SHA512 extensions (Intel Arrow Lake / Lunar Lake and newer) the hardware
`vsha512rnds2`/`vsha512msg*` instructions are used automatically (runtime-detected, still no dependencies).
That path is checked in the test suite against the portable one, on real hardware when available and
otherwise against software models of the instructions; it has not been benchmarked yet.
