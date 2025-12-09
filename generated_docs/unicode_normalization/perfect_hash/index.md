*[unicode_normalization](../index.md) / [perfect_hash](index.md)*

---

# Module `perfect_hash`

Support for lookups based on minimal perfect hashing.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`my_hash`](#my_hash) | fn |  |
| [`mph_lookup`](#mph_lookup) | fn | Do a lookup using minimal perfect hashing. |

## Functions

### `my_hash`

```rust
fn my_hash(key: u32, salt: u32, n: usize) -> usize
```

*Defined in [`unicode-normalization-0.1.25/src/perfect_hash.rs:16-20`](../../../.source_1765210505/unicode-normalization-0.1.25/src/perfect_hash.rs#L16-L20)*

### `mph_lookup`

```rust
fn mph_lookup<KV, V, FK, FV>(x: u32, salt: &[u16], kv: &[KV], fk: FK, fv: FV, default: V) -> V
where
    KV: Copy,
    FK: Fn(KV) -> u32,
    FV: Fn(KV) -> V
```

*Defined in [`unicode-normalization-0.1.25/src/perfect_hash.rs:30-50`](../../../.source_1765210505/unicode-normalization-0.1.25/src/perfect_hash.rs#L30-L50)*

Do a lookup using minimal perfect hashing.

The table is stored as a sequence of "salt" values, then a sequence of
values that contain packed key/value pairs. The strategy is to hash twice.
The first hash retrieves a salt value that makes the second hash unique.
The hash function doesn't have to be very good, just good enough that the
resulting map is unique.

