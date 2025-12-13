*[miniz_oxide](../../index.md) / [inflate](../index.md) / [core](index.md)*

---

# Module `core`

Streaming decompression functionality.

## Contents

- [Modules](#modules)
  - [`inflate_flags`](#inflate-flags)
- [Structs](#structs)
  - [`HuffmanTable`](#huffmantable)
  - [`DecompressorOxide`](#decompressoroxide)
  - [`LocalVars`](#localvars)
- [Enums](#enums)
  - [`State`](#state)
  - [`Action`](#action)
- [Functions](#functions)
  - [`num_extra_bits_for_distance_code`](#num-extra-bits-for-distance-code)
  - [`read_u16_le`](#read-u16-le)
  - [`fill_bit_buffer`](#fill-bit-buffer)
  - [`validate_zlib_header`](#validate-zlib-header)
  - [`decode_huffman_code`](#decode-huffman-code)
  - [`read_byte`](#read-byte)
  - [`read_bits`](#read-bits)
  - [`pad_to_bytes`](#pad-to-bytes)
  - [`end_of_input`](#end-of-input)
  - [`undo_bytes`](#undo-bytes)
  - [`start_static_table`](#start-static-table)
  - [`reverse_bits`](#reverse-bits)
  - [`init_tree`](#init-tree)
  - [`transfer`](#transfer)
  - [`apply_match`](#apply-match)
  - [`decompress_fast`](#decompress-fast)
  - [`decompress`](#decompress)
- [Type Aliases](#type-aliases)
  - [`BitBuffer`](#bitbuffer)
- [Constants](#constants)
  - [`TINFL_LZ_DICT_SIZE`](#tinfl-lz-dict-size)
  - [`MAX_HUFF_TABLES`](#max-huff-tables)
  - [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0)
  - [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1)
  - [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2)
  - [`FAST_LOOKUP_BITS`](#fast-lookup-bits)
  - [`FAST_LOOKUP_SIZE`](#fast-lookup-size)
  - [`MAX_HUFF_TREE_SIZE`](#max-huff-tree-size)
  - [`LITLEN_TABLE`](#litlen-table)
  - [`DIST_TABLE`](#dist-table)
  - [`HUFFLEN_TABLE`](#hufflen-table)
  - [`LEN_CODES_SIZE`](#len-codes-size)
  - [`LEN_CODES_MASK`](#len-codes-mask)
  - [`MIN_TABLE_SIZES`](#min-table-sizes)
  - [`LENGTH_BASE`](#length-base)
  - [`LENGTH_EXTRA`](#length-extra)
  - [`DIST_BASE`](#dist-base)
  - [`BASE_EXTRA_MASK`](#base-extra-mask)
- [Macros](#macros)
  - [`generate_state!`](#generate-state)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`inflate_flags`](#inflate-flags) | mod | Flags to [`decompress()`] to control how inflation works. |
| [`HuffmanTable`](#huffmantable) | struct | A struct containing huffman code lengths and the huffman code tree used by the decompressor. |
| [`DecompressorOxide`](#decompressoroxide) | struct | Main decompression struct. |
| [`LocalVars`](#localvars) | struct |  |
| [`State`](#state) | enum |  |
| [`Action`](#action) | enum |  |
| [`num_extra_bits_for_distance_code`](#num-extra-bits-for-distance-code) | fn | Get the number of extra bits used for a distance code. |
| [`read_u16_le`](#read-u16-le) | fn | Read an le u16 value from the slice iterator. |
| [`fill_bit_buffer`](#fill-bit-buffer) | fn | Ensure that there is data in the bit buffer. |
| [`validate_zlib_header`](#validate-zlib-header) | fn | Check that the zlib header is correct and that there is enough space in the buffer for the window size specified in the header. |
| [`decode_huffman_code`](#decode-huffman-code) | fn | Try to decode the next huffman code, and puts it in the counter field of the decompressor if successful. |
| [`read_byte`](#read-byte) | fn | Try to read one byte from `in_iter` and call `f` with the read byte as an argument, returning the result. |
| [`read_bits`](#read-bits) | fn | Try to read `amount` number of bits from `in_iter` and call the function `f` with the bits as an an argument after reading, returning the result of that function, or `Action::End` if there are not enough bytes left. |
| [`pad_to_bytes`](#pad-to-bytes) | fn |  |
| [`end_of_input`](#end-of-input) | fn |  |
| [`undo_bytes`](#undo-bytes) | fn |  |
| [`start_static_table`](#start-static-table) | fn |  |
| [`reverse_bits`](#reverse-bits) | fn |  |
| [`init_tree`](#init-tree) | fn |  |
| [`transfer`](#transfer) | fn |  |
| [`apply_match`](#apply-match) | fn | Presumes that there is at least match_len bytes in output left. |
| [`decompress_fast`](#decompress-fast) | fn | Fast inner decompression loop which is run  while there is at least 259 bytes left in the output buffer, and at least 6 bytes left in the input buffer (The maximum one match would need + 1). |
| [`decompress`](#decompress) | fn | Main decompression function. |
| [`BitBuffer`](#bitbuffer) | type |  |
| [`TINFL_LZ_DICT_SIZE`](#tinfl-lz-dict-size) | const |  |
| [`MAX_HUFF_TABLES`](#max-huff-tables) | const | The number of huffman tables used. |
| [`MAX_HUFF_SYMBOLS_0`](#max-huff-symbols-0) | const | The length of the first (literal/length) huffman table. |
| [`MAX_HUFF_SYMBOLS_1`](#max-huff-symbols-1) | const | The length of the second (distance) huffman table. |
| [`MAX_HUFF_SYMBOLS_2`](#max-huff-symbols-2) | const | The length of the last (huffman code length) huffman table. |
| [`FAST_LOOKUP_BITS`](#fast-lookup-bits) | const | The maximum length of a code that can be looked up in the fast lookup table. |
| [`FAST_LOOKUP_SIZE`](#fast-lookup-size) | const | The size of the fast lookup table. |
| [`MAX_HUFF_TREE_SIZE`](#max-huff-tree-size) | const |  |
| [`LITLEN_TABLE`](#litlen-table) | const |  |
| [`DIST_TABLE`](#dist-table) | const |  |
| [`HUFFLEN_TABLE`](#hufflen-table) | const |  |
| [`LEN_CODES_SIZE`](#len-codes-size) | const |  |
| [`LEN_CODES_MASK`](#len-codes-mask) | const |  |
| [`MIN_TABLE_SIZES`](#min-table-sizes) | const |  |
| [`LENGTH_BASE`](#length-base) | const | Base length for each length code. |
| [`LENGTH_EXTRA`](#length-extra) | const | Number of extra bits for each length code. |
| [`DIST_BASE`](#dist-base) | const | Base length for each distance code. |
| [`BASE_EXTRA_MASK`](#base-extra-mask) | const | The mask used when indexing the base/extra arrays. |
| [`generate_state!`](#generate-state) | macro |  |

## Modules

- [`inflate_flags`](inflate_flags/index.md) — Flags to [`decompress()`] to control how inflation works.

## Structs

### `HuffmanTable`

```rust
struct HuffmanTable {
    pub look_up: [i16; 1024],
    pub tree: [i16; 576],
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:22-34`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L22-L34)*

A struct containing huffman code lengths and the huffman code tree used by the decompressor.

#### Fields

- **`look_up`**: `[i16; 1024]`

  Fast lookup table for shorter huffman codes.
  
  See `HuffmanTable::fast_lookup`.

- **`tree`**: `[i16; 576]`

  Full huffman tree.
  
  Positive values are edge nodes/symbols, negative values are
  parent nodes/references to other nodes.

#### Implementations

- <span id="huffmantable-new"></span>`const fn new() -> HuffmanTable` — [`HuffmanTable`](#huffmantable)

- <span id="huffmantable-fast-lookup"></span>`fn fast_lookup(&self, bit_buf: u64) -> i16`

  Look for a symbol in the fast lookup table.

  The symbol is stored in the lower 9 bits, the length in the next 6.

  If the returned value is negative, the code wasn't found in the

  fast lookup table and the full tree has to be traversed to find the code.

- <span id="huffmantable-tree-lookup"></span>`fn tree_lookup(&self, fast_symbol: i32, bit_buf: u64, code_len: u8) -> (i32, u32)`

  Get the symbol and the code length from the huffman tree.

- <span id="huffmantable-lookup"></span>`fn lookup(&self, bit_buf: u64) -> (i32, u32)`

  Look up a symbol and code length from the bits in the provided bit buffer.

  

  Returns Some(symbol, length) on success,

  None if the length is 0.

  

  It's possible we could avoid checking for 0 if we can guarantee a sane table.

  TODO: Check if a smaller type for code_len helps performance.

#### Trait Implementations

##### `impl Any for HuffmanTable`

- <span id="huffmantable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HuffmanTable`

- <span id="huffmantable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HuffmanTable`

- <span id="huffmantable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for HuffmanTable`

- <span id="huffmantable-clone"></span>`fn clone(&self) -> HuffmanTable` — [`HuffmanTable`](#huffmantable)

##### `impl CloneToUninit for HuffmanTable`

- <span id="huffmantable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for HuffmanTable`

- <span id="huffmantable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HuffmanTable`

- <span id="huffmantable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for HuffmanTable`

- <span id="huffmantable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="huffmantable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HuffmanTable`

- <span id="huffmantable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="huffmantable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DecompressorOxide`

```rust
struct DecompressorOxide {
    state: core::State,
    num_bits: u32,
    z_header0: u32,
    z_header1: u32,
    z_adler32: u32,
    finish: u8,
    block_type: u8,
    check_adler32: u32,
    dist: u32,
    counter: u32,
    num_extra: u8,
    table_sizes: [u16; 3],
    bit_buf: u64,
    tables: [HuffmanTable; 3],
    code_size_literal: [u8; 288],
    code_size_dist: [u8; 32],
    code_size_huffman: [u8; 19],
    raw_header: [u8; 4],
    len_codes: [u8; 512],
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:236-279`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L236-L279)*

Main decompression struct.


#### Fields

- **`state`**: `core::State`

  Current state of the decompressor.

- **`num_bits`**: `u32`

  Number of bits in the bit buffer.

- **`z_header0`**: `u32`

  Zlib CMF

- **`z_header1`**: `u32`

  Zlib FLG

- **`z_adler32`**: `u32`

  Adler32 checksum from the zlib header.

- **`finish`**: `u8`

  1 if the current block is the last block, 0 otherwise.

- **`block_type`**: `u8`

  The type of the current block.
  or if in a dynamic block, which huffman table we are currently

- **`check_adler32`**: `u32`

  1 if the adler32 value should be checked.

- **`dist`**: `u32`

  Last match distance.

- **`counter`**: `u32`

  Variable used for match length, symbols, and a number of other things.

- **`num_extra`**: `u8`

  Number of extra bits for the last length or distance code.

- **`table_sizes`**: `[u16; 3]`

  Number of entries in each huffman table.

- **`bit_buf`**: `u64`

  Buffer of input data.

- **`tables`**: `[HuffmanTable; 3]`

  Huffman tables.

- **`raw_header`**: `[u8; 4]`

  Raw block header.

- **`len_codes`**: `[u8; 512]`

  Huffman length codes.

#### Implementations

- <span id="decompressoroxide-new"></span>`fn new() -> DecompressorOxide` — [`DecompressorOxide`](#decompressoroxide)

  Create a new tinfl_decompressor with all fields set to 0.

- <span id="decompressoroxide-init"></span>`fn init(&mut self)`

  Set the current state to `Start`.

- <span id="decompressoroxide-adler32"></span>`fn adler32(&self) -> Option<u32>`

  Returns the adler32 checksum of the currently decompressed data.

  Note: Will return Some(1) if decompressing zlib but ignoring adler32.

- <span id="decompressoroxide-adler32-header"></span>`fn adler32_header(&self) -> Option<u32>`

  Returns the adler32 that was read from the zlib header if it exists.

#### Trait Implementations

##### `impl Any for DecompressorOxide`

- <span id="decompressoroxide-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DecompressorOxide`

- <span id="decompressoroxide-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DecompressorOxide`

- <span id="decompressoroxide-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DecompressorOxide`

- <span id="decompressoroxide-clone"></span>`fn clone(&self) -> DecompressorOxide` — [`DecompressorOxide`](#decompressoroxide)

##### `impl CloneToUninit for DecompressorOxide`

- <span id="decompressoroxide-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Default for DecompressorOxide`

- <span id="decompressoroxide-default"></span>`fn default() -> Self`

  Create a new tinfl_decompressor with all fields set to 0.

##### `impl<T> From for DecompressorOxide`

- <span id="decompressoroxide-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DecompressorOxide`

- <span id="decompressoroxide-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for DecompressorOxide`

- <span id="decompressoroxide-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decompressoroxide-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DecompressorOxide`

- <span id="decompressoroxide-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decompressoroxide-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LocalVars`

```rust
struct LocalVars {
    pub bit_buf: u64,
    pub num_bits: u32,
    pub dist: u32,
    pub counter: u32,
    pub num_extra: u8,
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1023-1029`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1023-L1029)*

#### Trait Implementations

##### `impl Any for LocalVars`

- <span id="localvars-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LocalVars`

- <span id="localvars-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalVars`

- <span id="localvars-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LocalVars`

- <span id="localvars-clone"></span>`fn clone(&self) -> LocalVars` — [`LocalVars`](#localvars)

##### `impl CloneToUninit for LocalVars`

- <span id="localvars-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LocalVars`

##### `impl<T> From for LocalVars`

- <span id="localvars-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LocalVars`

- <span id="localvars-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LocalVars`

- <span id="localvars-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localvars-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalVars`

- <span id="localvars-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localvars-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `State`

```rust
enum State {
    Start,
    ReadZlibCmf,
    ReadZlibFlg,
    ReadBlockHeader,
    BlockTypeNoCompression,
    RawHeader,
    RawMemcpy1,
    RawMemcpy2,
    ReadTableSizes,
    ReadHufflenTableCodeSize,
    ReadLitlenDistTablesCodeSize,
    ReadExtraBitsCodeSize,
    DecodeLitlen,
    WriteSymbol,
    ReadExtraBitsLitlen,
    DecodeDistance,
    ReadExtraBitsDistance,
    RawReadFirstByte,
    RawStoreFirstByte,
    WriteLenBytesToEnd,
    BlockDone,
    HuffDecodeOuterLoop1,
    HuffDecodeOuterLoop2,
    ReadAdler32,
    DoneForever,
    BlockTypeUnexpected,
    BadCodeSizeSum,
    BadDistOrLiteralTableLength,
    BadTotalSymbols,
    BadZlibHeader,
    DistanceOutOfBounds,
    BadRawLength,
    BadCodeSizeDistPrevLookup,
    InvalidLitlen,
    InvalidDist,
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:411-450`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L411-L450)*

#### Implementations

- <span id="state-is-failure"></span>`const fn is_failure(self) -> bool`

- <span id="state-begin"></span>`fn begin(&mut self, new_state: State)` — [`State`](#state)

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for State`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Action`

```rust
enum Action {
    None,
    Jump(State),
    End(TINFLStatus),
}
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:594-598`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L594-L598)*

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `num_extra_bits_for_distance_code`

```rust
const fn num_extra_bits_for_distance_code(code: u8) -> u8
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:512-517`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L512-L517)*

Get the number of extra bits used for a distance code.
(Code numbers above `NUM_DISTANCE_CODES` will give some garbage
value.)

### `read_u16_le`

```rust
fn read_u16_le(iter: &mut self::output_buffer::InputWrapper<'_>) -> u16
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:527-534`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L527-L534)*

Read an le u16 value from the slice iterator.

# Panics
Panics if there are less than two bytes left.

### `fill_bit_buffer`

```rust
fn fill_bit_buffer(l: &mut LocalVars, in_iter: &mut self::output_buffer::InputWrapper<'_>)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:543-549`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L543-L549)*

Ensure that there is data in the bit buffer.

On 64-bit platform, we use a 64-bit value so this will
result in there being at least 32 bits in the bit buffer.
This function assumes that there is at least 4 bytes left in the input buffer.

### `validate_zlib_header`

```rust
const fn validate_zlib_header(cmf: u32, flg: u32, flags: u32, mask: usize) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:568-592`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L568-L592)*

Check that the zlib header is correct and that there is enough space in the buffer
for the window size specified in the header.

See https://tools.ietf.org/html/rfc1950

### `decode_huffman_code`

```rust
fn decode_huffman_code<F>(r: &mut DecompressorOxide, l: &mut LocalVars, table: usize, flags: u32, in_iter: &mut self::output_buffer::InputWrapper<'_>, f: F) -> Action
where
    F: FnOnce(&mut DecompressorOxide, &mut LocalVars, i32) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:606-709`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L606-L709)*

Try to decode the next huffman code, and puts it in the counter field of the decompressor
if successful.

# Returns
The specified action returned from `f` on success,
`Action::End` if there are not enough data left to decode a symbol.

### `read_byte`

```rust
fn read_byte<F>(in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(u8) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:715-723`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L715-L723)*

Try to read one byte from `in_iter` and call `f` with the read byte as an argument,
returning the result.
If reading fails, `Action::End is returned`

### `read_bits`

```rust
fn read_bits<F>(l: &mut LocalVars, amount: u32, in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(&mut LocalVars, u64) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:731-760`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L731-L760)*

Try to read `amount` number of bits from `in_iter` and call the function `f` with the bits as an
an argument after reading, returning the result of that function, or `Action::End` if there are
not enough bytes left.

### `pad_to_bytes`

```rust
fn pad_to_bytes<F>(l: &mut LocalVars, in_iter: &mut self::output_buffer::InputWrapper<'_>, flags: u32, f: F) -> Action
where
    F: FnOnce(&mut LocalVars) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:763-769`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L763-L769)*

### `end_of_input`

```rust
const fn end_of_input(flags: u32) -> Action
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:772-778`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L772-L778)*

### `undo_bytes`

```rust
fn undo_bytes(l: &mut LocalVars, max: u32) -> u32
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:781-785`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L781-L785)*

### `start_static_table`

```rust
fn start_static_table(r: &mut DecompressorOxide)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:787-795`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L787-L795)*

### `reverse_bits`

```rust
const fn reverse_bits(n: u16) -> u16
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:805-818`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L805-L818)*

### `init_tree`

```rust
fn init_tree(r: &mut DecompressorOxide, l: &mut LocalVars) -> Option<Action>
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:845-1001`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L845-L1001)*

### `transfer`

```rust
fn transfer(out_slice: &mut [u8], source_pos: usize, out_pos: usize, match_len: usize, out_buf_size_mask: usize)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1032-1108`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1032-L1108)*

### `apply_match`

```rust
fn apply_match(out_slice: &mut [u8], out_pos: usize, dist: usize, match_len: usize, out_buf_size_mask: usize)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1112-1168`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1112-L1168)*

Presumes that there is at least match_len bytes in output left.

### `decompress_fast`

```rust
fn decompress_fast(r: &mut DecompressorOxide, in_iter: &mut self::output_buffer::InputWrapper<'_>, out_buf: &mut self::output_buffer::OutputBuffer<'_>, flags: u32, local_vars: &mut LocalVars, out_buf_size_mask: usize) -> (TINFLStatus, State)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1179-1322`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1179-L1322)*

Fast inner decompression loop which is run  while there is at least
259 bytes left in the output buffer, and at least 6 bytes left in the input buffer
(The maximum one match would need + 1).

This was inspired by a similar optimization in zlib, which uses this info to do
faster unchecked copies of multiple bytes at a time.
Currently we don't do this here, but this function does avoid having to jump through the
big match loop on each state change(as rust does not have fallthrough or gotos at the moment),
and already improves decompression speed a fair bit.

### `decompress`

```rust
fn decompress(r: &mut DecompressorOxide, in_buf: &[u8], out: &mut [u8], out_pos: usize, flags: u32) -> (TINFLStatus, usize, usize)
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1358-2029`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1358-L2029)*

Main decompression function. Keeps decompressing data from `in_buf` until the `in_buf` is
empty, `out` is full, the end of the deflate stream is hit, or there is an error in the
deflate stream.

# Arguments

`r` is a [`DecompressorOxide`](#decompressoroxide) struct with the state of this stream.

`in_buf` is a reference to the compressed data that is to be decompressed. The decompressor will
start at the first byte of this buffer.

`out` is a reference to the buffer that will store the decompressed data, and that
stores previously decompressed data if any.

* The offset given by `out_pos` indicates where in the output buffer slice writing should start.
* If [`TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF`](inflate_flags/index.md) is not set, the output buffer is used in a
  wrapping manner, and it's size is required to be a power of 2.
* The decompression function normally needs access to 32KiB of the previously decompressed data
  (or to the beginning of the decompressed data if less than 32KiB has been decompressed.)
    - If this data is not available, decompression may fail.
    - Some deflate compressors allow specifying a window size which limits match distances to
      less than this, or alternatively an RLE mode where matches will only refer to the previous byte
      and thus allows a smaller output buffer. The window size can be specified in the zlib
      header structure, however, the header data should not be relied on to be correct.

`flags` indicates settings and status to the decompression function.
* The [`TINFL_FLAG_HAS_MORE_INPUT`](inflate_flags/index.md) has to be specified if more compressed data is to be provided
  in a subsequent call to this function.
* See the the [`inflate_flags`](inflate_flags/index.md) module for details on other flags.

# Returns

Returns a tuple containing the status of the compressor, the number of input bytes read, and the
number of bytes output to `out`.

## Type Aliases

### `BitBuffer`

```rust
type BitBuffer = u64;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:176`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L176)*

## Constants

### `TINFL_LZ_DICT_SIZE`
```rust
const TINFL_LZ_DICT_SIZE: usize = 32_768usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:17`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L17)*

### `MAX_HUFF_TABLES`
```rust
const MAX_HUFF_TABLES: usize = 3usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:101`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L101)*

The number of huffman tables used.

### `MAX_HUFF_SYMBOLS_0`
```rust
const MAX_HUFF_SYMBOLS_0: usize = 288usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:103`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L103)*

The length of the first (literal/length) huffman table.

### `MAX_HUFF_SYMBOLS_1`
```rust
const MAX_HUFF_SYMBOLS_1: usize = 32usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:105`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L105)*

The length of the second (distance) huffman table.

### `MAX_HUFF_SYMBOLS_2`
```rust
const MAX_HUFF_SYMBOLS_2: usize = 19usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:107`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L107)*

The length of the last (huffman code length) huffman table.

### `FAST_LOOKUP_BITS`
```rust
const FAST_LOOKUP_BITS: u8 = 10u8;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:109`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L109)*

The maximum length of a code that can be looked up in the fast lookup table.

### `FAST_LOOKUP_SIZE`
```rust
const FAST_LOOKUP_SIZE: u16 = 1_024u16;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:111`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L111)*

The size of the fast lookup table.

### `MAX_HUFF_TREE_SIZE`
```rust
const MAX_HUFF_TREE_SIZE: usize = 576usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:112`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L112)*

### `LITLEN_TABLE`
```rust
const LITLEN_TABLE: usize = 0usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:113`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L113)*

### `DIST_TABLE`
```rust
const DIST_TABLE: usize = 1usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:114`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L114)*

### `HUFFLEN_TABLE`
```rust
const HUFFLEN_TABLE: usize = 2usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:115`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L115)*

### `LEN_CODES_SIZE`
```rust
const LEN_CODES_SIZE: usize = 512usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:116`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L116)*

### `LEN_CODES_MASK`
```rust
const LEN_CODES_MASK: usize = 511usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:117`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L117)*

### `MIN_TABLE_SIZES`
```rust
const MIN_TABLE_SIZES: [u16; 3];
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:173`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L173)*

### `LENGTH_BASE`
```rust
const LENGTH_BASE: [u16; 32];
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:488-491`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L488-L491)*

Base length for each length code.

The base is used together with the value of the extra bits to decode the actual
length/distance values in a match.

### `LENGTH_EXTRA`
```rust
const LENGTH_EXTRA: [u8; 32];
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:495-498`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L495-L498)*

Number of extra bits for each length code.

### `DIST_BASE`
```rust
const DIST_BASE: [u16; 30];
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:502-506`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L502-L506)*

Base length for each distance code.

### `BASE_EXTRA_MASK`
```rust
const BASE_EXTRA_MASK: usize = 31usize;
```

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:520`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L520)*

The mask used when indexing the base/extra arrays.

## Macros

### `generate_state!`

*Defined in [`miniz_oxide-0.8.9/src/inflate/core.rs:1007-1020`](../../../../.source_1765521767/miniz_oxide-0.8.9/src/inflate/core.rs#L1007-L1020)*

