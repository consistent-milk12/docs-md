*[compact_str](../index.md) / [repr](index.md)*

---

# Module `repr`

## Modules

- [`capacity`](capacity/index.md) - 
- [`heap`](heap/index.md) - 
- [`inline`](inline/index.md) - 
- [`iter`](iter/index.md) - Implementations of the [`FromIterator`] trait to make building [`Repr`]s more ergonomic
- [`last_utf8_char`](last_utf8_char/index.md) - 
- [`num`](num/index.md) - Implementations for efficiently converting a number into a [`Repr`]
- [`static_str`](static_str/index.md) - 
- [`traits`](traits/index.md) - 

## Structs

### `Repr`

```rust
struct Repr(*const (), usize, u32, u16, u8, last_utf8_char::LastByte);
```

#### Implementations

- `fn new(text: &str) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `const fn const_new(text: &'static str) -> Self`

- `fn with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `fn from_utf8<B: AsRef<[u8]>>(buf: B) -> Result<Self, Utf8Error>`

- `unsafe fn from_utf8_unchecked<B: AsRef<[u8]>>(buf: B) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `fn from_string(s: String, should_inline: bool) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `fn into_string(self: Self) -> String`

- `fn reserve(self: &mut Self, additional: usize) -> Result<(), ReserveError>` — [`ReserveError`](../index.md)

- `fn shrink_to(self: &mut Self, min_capacity: usize)`

- `fn push_str(self: &mut Self, s: &str)`

- `fn pop(self: &mut Self) -> Option<char>`

- `fn as_slice(self: &Self) -> &[u8]`

- `fn as_str(self: &Self) -> &str`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn capacity(self: &Self) -> usize`

- `fn is_heap_allocated(self: &Self) -> bool`

- `const fn is_static_str(self: &Self) -> bool`

- `const fn as_static_str(self: &Self) -> Option<&'static str>`

- `fn as_static_variant_mut(self: &mut Self) -> Option<&mut StaticStr>` — [`StaticStr`](static_str/index.md)

- `unsafe fn as_mut_buf(self: &mut Self) -> &mut [u8]`

- `unsafe fn set_len(self: &mut Self, len: usize)`

- `const fn last_byte(self: &Self) -> u8`

- `const fn from_inline(inline: InlineBuffer) -> Self` — [`InlineBuffer`](inline/index.md)

- `const fn from_heap(heap: HeapBuffer) -> Self` — [`HeapBuffer`](heap/index.md)

- `const fn from_static(heap: StaticStr) -> Self` — [`StaticStr`](static_str/index.md)

- `const unsafe fn into_heap(self: Self) -> HeapBuffer` — [`HeapBuffer`](heap/index.md)

- `unsafe fn as_mut_heap(self: &mut Self) -> &mut HeapBuffer` — [`HeapBuffer`](heap/index.md)

- `unsafe fn as_heap(self: &Self) -> &HeapBuffer` — [`HeapBuffer`](heap/index.md)

- `unsafe fn as_mut_inline(self: &mut Self) -> &mut InlineBuffer` — [`InlineBuffer`](inline/index.md)

#### Trait Implementations

##### `impl Clone for Repr`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Drop for Repr`

- `fn drop(self: &mut Self)`

##### `impl<'a> Extend for Repr`

- `fn extend<T: IntoIterator<Item = &'a char>>(self: &mut Self, iter: T)`

##### `impl FromIterator for super::Repr`

- `fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self`

##### `impl LifetimeFree for super::repr::Repr`

##### `impl Send for Repr`

##### `impl Sync for Repr`

## Functions

### `ensure_read`

```rust
fn ensure_read(value: usize) -> usize
```

Returns the supplied value, and ensures that the value is eagerly loaded into a register.

## Constants

### `MAX_SIZE`

```rust
const MAX_SIZE: usize = 24usize;
```

The max size of a string we can fit inline

### `HEAP_MASK`

```rust
const HEAP_MASK: u8 = 216u8;
```

Used as a discriminant to identify different variants

### `STATIC_STR_MASK`

```rust
const STATIC_STR_MASK: u8 = 217u8;
```

Used for `StaticStr` variant

### `LENGTH_MASK`

```rust
const LENGTH_MASK: u8 = 192u8;
```

When our string is stored inline, we represent the length of the string in the last byte, offset
by `LENGTH_MASK`

### `EMPTY`

```rust
const EMPTY: Repr;
```

