*[compact_str](../index.md) / [repr](index.md)*

---

# Module `repr`

## Contents

- [Modules](#modules)
  - [`capacity`](#capacity)
  - [`heap`](#heap)
  - [`inline`](#inline)
  - [`iter`](#iter)
  - [`last_utf8_char`](#last-utf8-char)
  - [`num`](#num)
  - [`static_str`](#static-str)
  - [`traits`](#traits)
- [Structs](#structs)
  - [`Repr`](#repr)
- [Functions](#functions)
  - [`ensure_read`](#ensure-read)
- [Constants](#constants)
  - [`MAX_SIZE`](#max-size)
  - [`HEAP_MASK`](#heap-mask)
  - [`STATIC_STR_MASK`](#static-str-mask)
  - [`LENGTH_MASK`](#length-mask)
  - [`EMPTY`](#empty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`capacity`](#capacity) | mod |  |
| [`heap`](#heap) | mod |  |
| [`inline`](#inline) | mod |  |
| [`iter`](#iter) | mod | Implementations of the [`FromIterator`] trait to make building [`Repr`]s more ergonomic |
| [`last_utf8_char`](#last-utf8-char) | mod |  |
| [`num`](#num) | mod | Implementations for efficiently converting a number into a [`Repr`] |
| [`static_str`](#static-str) | mod |  |
| [`traits`](#traits) | mod |  |
| [`Repr`](#repr) | struct |  |
| [`ensure_read`](#ensure-read) | fn | Returns the supplied value, and ensures that the value is eagerly loaded into a register. |
| [`MAX_SIZE`](#max-size) | const | The max size of a string we can fit inline |
| [`HEAP_MASK`](#heap-mask) | const | Used as a discriminant to identify different variants |
| [`STATIC_STR_MASK`](#static-str-mask) | const | Used for `StaticStr` variant |
| [`LENGTH_MASK`](#length-mask) | const | When our string is stored inline, we represent the length of the string in the last byte, offset by `LENGTH_MASK` |
| [`EMPTY`](#empty) | const |  |

## Modules

- [`capacity`](capacity/index.md)
- [`heap`](heap/index.md)
- [`inline`](inline/index.md)
- [`iter`](iter/index.md) — Implementations of the [`FromIterator`] trait to make building [`Repr`]s more ergonomic
- [`last_utf8_char`](last_utf8_char/index.md)
- [`num`](num/index.md) — Implementations for efficiently converting a number into a [`Repr`]
- [`static_str`](static_str/index.md)
- [`traits`](traits/index.md)

## Structs

### `Repr`

```rust
struct Repr(*const (), usize, u32, u16, u8, last_utf8_char::LastByte);
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:44-57`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L44-L57)*

#### Implementations

- <span id="repr-new"></span>`fn new(text: &str) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md#reserveerror)

- <span id="repr-const-new"></span>`const fn const_new(text: &'static str) -> Self`

- <span id="repr-with-capacity"></span>`fn with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md#reserveerror)

- <span id="repr-from-utf8"></span>`fn from_utf8<B: AsRef<[u8]>>(buf: B) -> Result<Self, Utf8Error>`

- <span id="repr-from-utf8-unchecked"></span>`unsafe fn from_utf8_unchecked<B: AsRef<[u8]>>(buf: B) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md#reserveerror)

- <span id="repr-from-string"></span>`fn from_string(s: String, should_inline: bool) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md#reserveerror)

- <span id="repr-into-string"></span>`fn into_string(self) -> String`

- <span id="repr-reserve"></span>`fn reserve(&mut self, additional: usize) -> Result<(), ReserveError>` — [`ReserveError`](../index.md#reserveerror)

- <span id="repr-shrink-to"></span>`fn shrink_to(&mut self, min_capacity: usize)`

- <span id="repr-push-str"></span>`fn push_str(&mut self, s: &str)`

- <span id="repr-pop"></span>`fn pop(&mut self) -> Option<char>`

- <span id="repr-as-slice"></span>`fn as_slice(&self) -> &[u8]`

- <span id="repr-as-str"></span>`fn as_str(&self) -> &str`

- <span id="repr-len"></span>`fn len(&self) -> usize`

- <span id="repr-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="repr-capacity"></span>`fn capacity(&self) -> usize`

- <span id="repr-is-heap-allocated"></span>`fn is_heap_allocated(&self) -> bool`

- <span id="repr-is-static-str"></span>`const fn is_static_str(&self) -> bool`

- <span id="repr-as-static-str"></span>`const fn as_static_str(&self) -> Option<&'static str>`

- <span id="repr-as-static-variant-mut"></span>`fn as_static_variant_mut(&mut self) -> Option<&mut StaticStr>` — [`StaticStr`](static_str/index.md#staticstr)

- <span id="repr-as-mut-buf"></span>`unsafe fn as_mut_buf(&mut self) -> &mut [u8]`

- <span id="repr-set-len"></span>`unsafe fn set_len(&mut self, len: usize)`

- <span id="repr-last-byte"></span>`const fn last_byte(&self) -> u8`

- <span id="repr-from-inline"></span>`const fn from_inline(inline: InlineBuffer) -> Self` — [`InlineBuffer`](inline/index.md#inlinebuffer)

- <span id="repr-from-heap"></span>`const fn from_heap(heap: HeapBuffer) -> Self` — [`HeapBuffer`](heap/index.md#heapbuffer)

- <span id="repr-from-static"></span>`const fn from_static(heap: StaticStr) -> Self` — [`StaticStr`](static_str/index.md#staticstr)

- <span id="repr-into-heap"></span>`const unsafe fn into_heap(self) -> HeapBuffer` — [`HeapBuffer`](heap/index.md#heapbuffer)

- <span id="repr-as-mut-heap"></span>`unsafe fn as_mut_heap(&mut self) -> &mut HeapBuffer` — [`HeapBuffer`](heap/index.md#heapbuffer)

- <span id="repr-as-heap"></span>`unsafe fn as_heap(&self) -> &HeapBuffer` — [`HeapBuffer`](heap/index.md#heapbuffer)

- <span id="repr-as-mut-inline"></span>`unsafe fn as_mut_inline(&mut self) -> &mut InlineBuffer` — [`InlineBuffer`](inline/index.md#inlinebuffer)

#### Trait Implementations

##### `impl Clone for Repr`

- <span id="repr-clone"></span>`fn clone(&self) -> Self`

- <span id="repr-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl Drop for Repr`

- <span id="repr-drop"></span>`fn drop(&mut self)`

##### `impl Extend for Repr`

- <span id="repr-extend"></span>`fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T)`

##### `impl FromIterator for super::Repr`

- <span id="superrepr-from-iter"></span>`fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self`

##### `impl LifetimeFree for super::repr::Repr`

##### `impl Send for Repr`

##### `impl Sync for Repr`

## Functions

### `ensure_read`

```rust
fn ensure_read(value: usize) -> usize
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:841-863`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L841-L863)*

Returns the supplied value, and ensures that the value is eagerly loaded into a register.

## Constants

### `MAX_SIZE`
```rust
const MAX_SIZE: usize = 24usize;
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:32`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L32)*

The max size of a string we can fit inline

### `HEAP_MASK`
```rust
const HEAP_MASK: u8 = 216u8;
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:34`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L34)*

Used as a discriminant to identify different variants

### `STATIC_STR_MASK`
```rust
const STATIC_STR_MASK: u8 = 217u8;
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:36`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L36)*

Used for `StaticStr` variant

### `LENGTH_MASK`
```rust
const LENGTH_MASK: u8 = 192u8;
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:39`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L39)*

When our string is stored inline, we represent the length of the string in the last byte, offset
by `LENGTH_MASK`

### `EMPTY`
```rust
const EMPTY: Repr;
```

*Defined in [`compact_str-0.9.0/src/repr/mod.rs:41`](../../../.source_1765210505/compact_str-0.9.0/src/repr/mod.rs#L41)*

