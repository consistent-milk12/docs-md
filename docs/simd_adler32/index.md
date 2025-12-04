# Crate `simd_adler32`

A SIMD-accelerated Adler-32 hash algorithm implementation.

## Features

- No dependencies
- Support `no_std` (with `default-features = false`)
- Runtime CPU feature detection (when `std` enabled)
- Blazing fast performance on as many targets as possible (currently only x86 and x86_64)
- Default to scalar implementation when simd not available

## Quick start

> Cargo.toml

```toml
[dependencies](#dependencies)

simd-adler32 = "*"
```

> example.rs

```rust
use simd_adler32::Adler32;

let mut adler = Adler32::new();
adler.write(b"rust is pretty cool, man");
let hash = adler.finish();

println!("{}", hash);
// 1921255656
```

## Feature flags

* `std` - Enabled by default

Enables std support, see [CPU Feature Detection](#cpu-feature-detection) for runtime
detection support.
* `nightly`

Enables nightly features required for avx512 support.

* `const-generics` - Enabled by default

Enables const-generics support allowing for user-defined array hashing by value.  See
[`Adler32Hash`](simd_adler32/index.md) for details.

## Support

**CPU Features**

| impl | arch             | feature |
| ---- | ---------------- | ------- |
| ✅   | `x86`, `x86_64`  | avx512  |
| ✅   | `x86`, `x86_64`  | avx2    |
| ✅   | `x86`, `x86_64`  | ssse3   |
| ✅   | `x86`, `x86_64`  | sse2    |
| 🚧   | `arm`, `aarch64` | neon    |
|      | `wasm32`         | simd128 |

**MSRV** `1.36.0`\*\*

Minimum supported rust version is tested before a new version is published. [**] Feature
`const-generics` needs to disabled to build on rustc versions `<1.51` which can be done
by updating your dependency definition to the following.

## CPU Feature Detection
simd-adler32 supports both runtime and compile time CPU feature detection using the
`std::is_x86_feature_detected` macro when the `Adler32` struct is instantiated with
the `new` fn.  

Without `std` feature enabled simd-adler32 falls back to compile time feature detection
using `target-feature` or `target-cpu` flags supplied to rustc. See [https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html)
for more information.

Feature detection tries to use the fastest supported feature first.

## Structs

### `Adler32`

```rust
struct Adler32 {
}
```

An adler32 hash generator type.

#### Implementations

- `fn new() -> Self`
  Constructs a new `Adler32`.

- `fn from_checksum(checksum: u32) -> Self`
  Constructs a new `Adler32` using existing checksum.

- `fn write(self: &mut Self, data: &[u8])`
  Computes hash for supplied data and stores results in internal state.

- `fn finish(self: &Self) -> u32`
  Returns the hash value for the values written so far.

- `fn reset(self: &mut Self)`
  Resets the internal state.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Adler32`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Default`

- `fn default() -> Self`

## Traits

### `Adler32Hash`

```rust
trait Adler32Hash { ... }
```

A Adler-32 hash-able type.

#### Required Methods

- `fn hash(self: &Self) -> u32`

  Feeds this value into `Adler32`.

## Functions

### `adler32`

```rust
fn adler32<H: Adler32Hash>(hash: &H) -> u32
```

Compute Adler-32 hash on `Adler32Hash` type.

# Arguments
* `hash` - A Adler-32 hash-able type.

# Examples
```rust
use simd_adler32::adler32;

let hash = adler32(b"Adler-32");
println!("{}", hash); // 800813569
```

