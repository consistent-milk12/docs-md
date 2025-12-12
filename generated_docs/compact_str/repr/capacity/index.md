*[compact_str](../../index.md) / [repr](../index.md) / [capacity](index.md)*

---

# Module `capacity`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Capacity`](#capacity) | struct | An integer type that uses `core::mem::size_of::<usize>() - 1` bytes to store the capacity of a heap buffer. |
| [`USIZE_SIZE`](#usize-size) | const |  |
| [`VALID_MASK`](#valid-mask) | const | Mask of bits in [`Capacity`] that encode the value. |
| [`HEAP_MARKER`](#heap-marker) | const | Mask of bits that are set in [`Capacity`] if the string data is stored on the heap. |
| [`MAX_VALUE`](#max-value) | const | The maximum value we're able to store, e.g. on 64-bit arch this is 2^56 - 2. |

## Structs

### `Capacity`

```rust
struct Capacity(usize);
```

*Defined in [`compact_str-0.9.0/src/repr/capacity.rs:55`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/capacity.rs#L55)*

An integer type that uses `core::mem::size_of::<usize>() - 1` bytes to store the capacity of
a heap buffer.

Assuming a 64-bit arch, a `super::BoxString` uses 8 bytes for a pointer, 8 bytes for a
length, and then needs 1 byte for a discriminant. We need to store the capacity somewhere, and
we could store it on the heap, but we also have 7 unused bytes. [`Capacity`](#capacity) handles storing a
value in these 7 bytes, returning an error if it's not possible, at which point we'll store the
capacity on the heap.

# Max Values
* __64-bit:__ `(2 ^ (7 * 8)) - 2 = 72_057_594_037_927_934 ~= 64 petabytes`
* __32-bit:__ `(2 ^ (3 * 8)) - 2 = 16_777_214             ~= 16 megabytes`

Practically speaking, on a 64-bit architecture we'll never need to store the capacity on the
heap, because with it's impossible to create a string that is 64 petabytes or larger. But for
32-bit architectures we need to be able to store a capacity larger than 16 megabytes, since a
string larger than 16 megabytes probably isn't that uncommon.

#### Implementations

- <span id="capacity-new"></span>`const fn new(capacity: usize) -> Self`

- <span id="capacity-as-usize"></span>`unsafe fn as_usize(self) -> usize`

- <span id="capacity-is-heap"></span>`fn is_heap(self) -> bool`

#### Trait Implementations

##### `impl Clone for Capacity`

- <span id="capacity-clone"></span>`fn clone(&self) -> Capacity` — [`Capacity`](#capacity)

##### `impl Copy for Capacity`

##### `impl Debug for Capacity`

- <span id="capacity-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Capacity`

##### `impl PartialEq for Capacity`

- <span id="capacity-eq"></span>`fn eq(&self, other: &Capacity) -> bool` — [`Capacity`](#capacity)

##### `impl StructuralPartialEq for Capacity`

## Constants

### `USIZE_SIZE`
```rust
const USIZE_SIZE: usize = 8usize;
```

*Defined in [`compact_str-0.9.0/src/repr/capacity.rs:6`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/capacity.rs#L6)*

### `VALID_MASK`
```rust
const VALID_MASK: usize = 72_057_594_037_927_935usize;
```

*Defined in [`compact_str-0.9.0/src/repr/capacity.rs:9-13`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/capacity.rs#L9-L13)*

Mask of bits in [`Capacity`](#capacity) that encode the value.

### `HEAP_MARKER`
```rust
const HEAP_MARKER: usize = 15_564_440_312_192_434_176usize;
```

*Defined in [`compact_str-0.9.0/src/repr/capacity.rs:16-20`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/capacity.rs#L16-L20)*

Mask of bits that are set in [`Capacity`](#capacity) if the string data is stored on the heap.

### `MAX_VALUE`
```rust
const MAX_VALUE: usize = 72_057_594_037_927_934usize;
```

*Defined in [`compact_str-0.9.0/src/repr/capacity.rs:30-34`](../../../../.source_1765210505/compact_str-0.9.0/src/repr/capacity.rs#L30-L34)*

The maximum value we're able to store, e.g. on 64-bit arch this is 2^56 - 2.

