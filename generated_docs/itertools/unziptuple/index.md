*[itertools](../index.md) / [unziptuple](index.md)*

---

# Module `unziptuple`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MultiUnzip`](#multiunzip) | trait | An iterator that can be unzipped into multiple collections. |
| [`multiunzip`](#multiunzip) | fn | Converts an iterator of tuples into a tuple of containers. |
| [`impl_unzip_iter!`](#impl-unzip-iter) | macro |  |

## Traits

### `MultiUnzip<FromI>`

```rust
trait MultiUnzip<FromI>: Iterator { ... }
```

*Defined in [`itertools-0.14.0/src/unziptuple.rs:32-35`](../../../.source_1765894658/itertools-0.14.0/src/unziptuple.rs#L32-L35)*

An iterator that can be unzipped into multiple collections.

See [`.multiunzip()`](crate::Itertools::multiunzip) for more information.

#### Required Methods

- `fn multiunzip(self) -> FromI`

  Unzip this iterator into multiple collections.

#### Implementors

- `IT`

## Functions

### `multiunzip`

```rust
fn multiunzip<FromI, I>(i: I) -> FromI
where
    I: IntoIterator,
    <I as >::IntoIter: MultiUnzip<FromI>
```

*Defined in [`itertools-0.14.0/src/unziptuple.rs:21-27`](../../../.source_1765894658/itertools-0.14.0/src/unziptuple.rs#L21-L27)*

Converts an iterator of tuples into a tuple of containers.

`multiunzip()` consumes an entire iterator of n-ary tuples, producing `n` collections, one for each
column.

This function is, in some sense, the opposite of [`multizip`](../ziptuple/index.md).

```rust
use itertools::multiunzip;

let inputs = vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)];

let (a, b, c): (Vec<_>, Vec<_>, Vec<_>) = multiunzip(inputs);

assert_eq!(a, vec![1, 4, 7]);
assert_eq!(b, vec![2, 5, 8]);
assert_eq!(c, vec![3, 6, 9]);
```


## Macros

### `impl_unzip_iter!`

*Defined in [`itertools-0.14.0/src/unziptuple.rs:37-66`](../../../.source_1765894658/itertools-0.14.0/src/unziptuple.rs#L37-L66)*

