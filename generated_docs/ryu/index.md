# Crate `ryu`

[![github]](https://github.com/dtolnay/ryu)&ensp;[![crates-io]](https://crates.io/crates/ryu)&ensp;[![docs-rs]](https://docs.rs/ryu)



<br>

Pure Rust implementation of Ryū, an algorithm to quickly convert floating
point numbers to decimal strings.

The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper] by Ulf
Adams includes a complete correctness proof of the algorithm. The paper is
available under the creative commons CC-BY-SA license.

This Rust implementation is a line-by-line port of Ulf Adams' implementation
in C, [https://github.com/ulfjack/ryu][upstream].


# Example

```rust
fn main() {
    let mut buffer = ryu::Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");
}
```

## Performance (lower is better)

![performance](https://raw.githubusercontent.com/dtolnay/ryu/master/performance.png)

You can run upstream's benchmarks with:

```console
$ git clone https://github.com/ulfjack/ryu c-ryu
$ cd c-ryu
$ bazel run -c opt //ryu/benchmark
```

And the same benchmark against our implementation with:

```console
$ git clone https://github.com/dtolnay/ryu rust-ryu
$ cd rust-ryu
$ cargo run --example upstream_benchmark --release
```

These benchmarks measure the average time to print a 32-bit float and average
time to print a 64-bit float, where the inputs are distributed as uniform random
bit patterns 32 and 64 bits wide.

The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
all perform the same, taking around 21 nanoseconds to format a 32-bit float and
31 nanoseconds to format a 64-bit float.

There is also a Rust-specific benchmark comparing this implementation to the
standard library which you can run with:

```console
$ cargo bench
```

The benchmark shows Ryū approximately 2-5x faster than the standard library
across a range of f32 and f64 inputs. Measurements are in nanoseconds per
iteration; smaller is better.

## Formatting

This library tends to produce more human-readable output than the standard
library's to\_string, which never uses scientific notation. Here are two
examples:

- *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
- *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123

Both libraries print short decimals such as 0.0000123 without scientific
notation.

## Modules

- [`buffer`](buffer/index.md) - 
- [`common`](common/index.md) - 
- [`d2s`](d2s/index.md) - 
- [`d2s_full_table`](d2s_full_table/index.md) - 
- [`d2s_intrinsics`](d2s_intrinsics/index.md) - 
- [`digit_table`](digit_table/index.md) - 
- [`f2s`](f2s/index.md) - 
- [`f2s_intrinsics`](f2s_intrinsics/index.md) - 
- [`pretty`](pretty/index.md) - 
- [`raw`](raw/index.md) - Unsafe functions that mirror the API of the C implementation of Ryū.

## Structs

### `Buffer`

```rust
struct Buffer {
    bytes: [core::mem::MaybeUninit<u8>; 24],
}
```

Safe API for formatting floating point numbers to text.

## Example

```rust
let mut buffer = ryu::Buffer::new();
let printed = buffer.format_finite(1.234);
assert_eq!(printed, "1.234");
```

#### Implementations

- `fn new() -> Self`

- `fn format<F: Float>(self: &mut Self, f: F) -> &str`

- `fn format_finite<F: Float>(self: &mut Self, f: F) -> &str`

#### Trait Implementations

##### `impl Clone for Buffer`

- `fn clone(self: &Self) -> Self`

##### `impl Copy for Buffer`

##### `impl Default for Buffer`

- `fn default() -> Self`

## Traits

