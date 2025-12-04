*[gimli](../../index.md) / [leb128](../index.md) / [read](index.md)*

---

# Module `read`

A module for reading signed and unsigned integers that have been LEB128
encoded.

## Functions

### `skip`

```rust
fn skip<R: Reader>(r: &mut R) -> crate::read::Result<()>
```

Read bytes until the LEB128 continuation bit is not set.

### `unsigned`

```rust
fn unsigned<R: Reader>(r: &mut R) -> crate::read::Result<u64>
```

Read an unsigned LEB128 number from the given `Reader` and
return it or an error if reading failed.

### `u16`

```rust
fn u16<R: Reader>(r: &mut R) -> crate::read::Result<u16>
```

Read an LEB128 u16 from the given `Reader` and
return it or an error if reading failed.

### `signed`

```rust
fn signed<R: Reader>(r: &mut R) -> crate::read::Result<i64>
```

Read a signed LEB128 number from the given `Reader` and
return it or an error if reading failed.

