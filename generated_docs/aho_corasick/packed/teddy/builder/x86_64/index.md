*[aho_corasick](../../../../index.md) / [packed](../../../index.md) / [teddy](../../index.md) / [builder](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Contents

- [Structs](#structs)
  - [`SlimSSSE3`](#slimssse3)
  - [`SlimAVX2`](#slimavx2)
  - [`FatAVX2`](#fatavx2)
- [Functions](#functions)
  - [`is_available_ssse3`](#is-available-ssse3)
  - [`is_available_avx2`](#is-available-avx2)
- [Macros](#macros)
  - [`slim_ssse3!`](#slim-ssse3)
  - [`slim_avx2!`](#slim-avx2)
  - [`fat_avx2!`](#fat-avx2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SlimSSSE3`](#slimssse3) | struct |  |
| [`SlimAVX2`](#slimavx2) | struct |  |
| [`FatAVX2`](#fatavx2) | struct |  |
| [`is_available_ssse3`](#is-available-ssse3) | fn |  |
| [`is_available_avx2`](#is-available-avx2) | fn |  |
| [`slim_ssse3!`](#slim-ssse3) | macro |  |
| [`slim_avx2!`](#slim-avx2) | macro |  |
| [`fat_avx2!`](#fat-avx2) | macro |  |

## Structs

### `SlimSSSE3<const BYTES: usize>`

```rust
struct SlimSSSE3<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:465-467`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L465-L467)*

#### Implementations

- <span id="slimssse3-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 128-bit

  vectors. If SSSE3 is not available in the current

  environment, then this returns `None`.

- <span id="slimssse3-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 256-bit

  vectors without checking whether SSSE3 is available or not.

  

  # Safety

  

  Callers must ensure that SSSE3 is available in the current

  environment.

#### Trait Implementations

##### `impl Any for SlimSSSE3<BYTES>`

- <span id="slimssse3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlimSSSE3<BYTES>`

- <span id="slimssse3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlimSSSE3<BYTES>`

- <span id="slimssse3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SlimSSSE3<BYTES>`

- <span id="slimssse3-clone"></span>`fn clone(&self) -> SlimSSSE3<BYTES>` — [`SlimSSSE3`](#slimssse3)

##### `impl CloneToUninit for SlimSSSE3<BYTES>`

- <span id="slimssse3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SlimSSSE3<BYTES>`

- <span id="slimssse3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SlimSSSE3<BYTES>`

- <span id="slimssse3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlimSSSE3<BYTES>`

- <span id="slimssse3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SearcherT for SlimSSSE3<1>`

- <span id="slimssse3-searchert-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

##### `impl ToOwned for SlimSSSE3<BYTES>`

- <span id="slimssse3-toowned-type-owned"></span>`type Owned = T`

- <span id="slimssse3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slimssse3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SlimSSSE3<BYTES>`

- <span id="slimssse3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slimssse3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlimSSSE3<BYTES>`

- <span id="slimssse3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slimssse3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SlimAVX2<const BYTES: usize>`

```rust
struct SlimAVX2<const BYTES: usize> {
    slim128: generic::Slim<core::arch::x86_64::__m128i, BYTES>,
    slim256: generic::Slim<core::arch::x86_64::__m256i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:528-531`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L528-L531)*

#### Implementations

- <span id="slimavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 256-bit

  vectors. If AVX2 is not available in the current

  environment, then this returns `None`.

- <span id="slimavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 256-bit

  vectors without checking whether AVX2 is available or not.

  

  # Safety

  

  Callers must ensure that AVX2 is available in the current

  environment.

#### Trait Implementations

##### `impl Any for SlimAVX2<BYTES>`

- <span id="slimavx2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SlimAVX2<BYTES>`

- <span id="slimavx2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SlimAVX2<BYTES>`

- <span id="slimavx2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SlimAVX2<BYTES>`

- <span id="slimavx2-clone"></span>`fn clone(&self) -> SlimAVX2<BYTES>` — [`SlimAVX2`](#slimavx2)

##### `impl CloneToUninit for SlimAVX2<BYTES>`

- <span id="slimavx2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SlimAVX2<BYTES>`

- <span id="slimavx2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SlimAVX2<BYTES>`

- <span id="slimavx2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SlimAVX2<BYTES>`

- <span id="slimavx2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SearcherT for SlimAVX2<1>`

- <span id="slimavx2-searchert-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

##### `impl ToOwned for SlimAVX2<BYTES>`

- <span id="slimavx2-toowned-type-owned"></span>`type Owned = T`

- <span id="slimavx2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="slimavx2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SlimAVX2<BYTES>`

- <span id="slimavx2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slimavx2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SlimAVX2<BYTES>`

- <span id="slimavx2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slimavx2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FatAVX2<const BYTES: usize>`

```rust
struct FatAVX2<const BYTES: usize> {
    fat256: generic::Fat<core::arch::x86_64::__m256i, BYTES>,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:601-603`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L601-L603)*

#### Implementations

- <span id="fatavx2-new"></span>`fn new(patterns: &Arc<Patterns>) -> Option<Searcher>` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 256-bit

  vectors. If AVX2 is not available in the current

  environment, then this returns `None`.

- <span id="fatavx2-new-unchecked"></span>`unsafe fn new_unchecked(patterns: &Arc<Patterns>) -> Searcher` — [`Patterns`](../../../pattern/index.md#patterns), [`Searcher`](../index.md#searcher)

  Creates a new searcher using "slim" Teddy with 256-bit

  vectors without checking whether AVX2 is available or not.

  

  # Safety

  

  Callers must ensure that AVX2 is available in the current

  environment.

#### Trait Implementations

##### `impl Any for FatAVX2<BYTES>`

- <span id="fatavx2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FatAVX2<BYTES>`

- <span id="fatavx2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FatAVX2<BYTES>`

- <span id="fatavx2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FatAVX2<BYTES>`

- <span id="fatavx2-clone"></span>`fn clone(&self) -> FatAVX2<BYTES>` — [`FatAVX2`](#fatavx2)

##### `impl CloneToUninit for FatAVX2<BYTES>`

- <span id="fatavx2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FatAVX2<BYTES>`

- <span id="fatavx2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FatAVX2<BYTES>`

- <span id="fatavx2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FatAVX2<BYTES>`

- <span id="fatavx2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl SearcherT for FatAVX2<1>`

- <span id="fatavx2-searchert-find"></span>`unsafe fn find(&self, start: *const u8, end: *const u8) -> Option<Match>` — [`Match`](../../generic/index.md#match)

##### `impl ToOwned for FatAVX2<BYTES>`

- <span id="fatavx2-toowned-type-owned"></span>`type Owned = T`

- <span id="fatavx2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="fatavx2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for FatAVX2<BYTES>`

- <span id="fatavx2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fatavx2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FatAVX2<BYTES>`

- <span id="fatavx2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fatavx2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_available_ssse3`

```rust
fn is_available_ssse3() -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:664-687`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L664-L687)*

### `is_available_avx2`

```rust
fn is_available_avx2() -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:690-713`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L690-L713)*

## Macros

### `slim_ssse3!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:470-520`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L470-L520)*

### `slim_avx2!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:534-593`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L534-L593)*

### `fat_avx2!`

*Defined in [`aho-corasick-1.1.4/src/packed/teddy/builder.rs:606-656`](../../../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/teddy/builder.rs#L606-L656)*

