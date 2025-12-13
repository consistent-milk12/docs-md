*[gimli](../../index.md) / [leb128](../index.md) / [read](index.md)*

---

# Module `read`

A module for reading signed and unsigned integers that have been LEB128
encoded.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`skip`](#skip) | fn | Read bytes until the LEB128 continuation bit is not set. |
| [`unsigned`](#unsigned) | fn | Read an unsigned LEB128 number from the given `Reader` and return it or an error if reading failed. |
| [`u16`](#u16) | fn | Read an LEB128 u16 from the given `Reader` and return it or an error if reading failed. |
| [`signed`](#signed) | fn | Read a signed LEB128 number from the given `Reader` and return it or an error if reading failed. |

## Functions

### `skip`

```rust
fn skip<R: Reader>(r: &mut R) -> crate::read::Result<()>
```

*Defined in [`gimli-0.32.3/src/leb128.rs:71-78`](../../../../.source_1765633015/gimli-0.32.3/src/leb128.rs#L71-L78)*

Read bytes until the LEB128 continuation bit is not set.

### `unsigned`

```rust
fn unsigned<R: Reader>(r: &mut R) -> crate::read::Result<u64>
```

*Defined in [`gimli-0.32.3/src/leb128.rs:82-101`](../../../../.source_1765633015/gimli-0.32.3/src/leb128.rs#L82-L101)*

Read an unsigned LEB128 number from the given `Reader` and
return it or an error if reading failed.

### `u16`

```rust
fn u16<R: Reader>(r: &mut R) -> crate::read::Result<u16>
```

*Defined in [`gimli-0.32.3/src/leb128.rs:105-124`](../../../../.source_1765633015/gimli-0.32.3/src/leb128.rs#L105-L124)*

Read an LEB128 u16 from the given `Reader` and
return it or an error if reading failed.

### `signed`

```rust
fn signed<R: Reader>(r: &mut R) -> crate::read::Result<i64>
```

*Defined in [`gimli-0.32.3/src/leb128.rs:128-155`](../../../../.source_1765633015/gimli-0.32.3/src/leb128.rs#L128-L155)*

Read a signed LEB128 number from the given `Reader` and
return it or an error if reading failed.

