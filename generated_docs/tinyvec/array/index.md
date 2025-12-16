*[tinyvec](../index.md) / [array](index.md)*

---

# Module `array`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`const_generic_impl`](#const-generic-impl) | mod |  |
| [`Array`](#array) | trait | A trait for types that are an array. |

## Modules

- [`const_generic_impl`](const_generic_impl/index.md)

## Traits

### `Array`

```rust
trait Array { ... }
```

*Defined in [`tinyvec-1.10.0/src/array.rs:18-41`](../../../.source_1765900590/tinyvec-1.10.0/src/array.rs#L18-L41)*

A trait for types that are an array.

An "array", for our purposes, has the following properties:
* Owns some number of elements.
* The element type can be generic, but must implement [`Default`](#default).
* The capacity is fixed at compile time, based on the implementing type.
* You can get a shared or mutable slice to the elements.

You are generally **not** expected to need to implement this yourself. It is
already implemented for all array lengths.

**Additional lengths can easily be added upon request.**

## Safety Reminder

Just a reminder: this trait is 100% safe, which means that `unsafe` code
**must not** rely on an instance of this trait being correct.

#### Associated Types

- `type Item: 1`

#### Associated Constants

- `const CAPACITY: usize`

#### Required Methods

- `fn Array::as_slice(&self) -> &[<Self as >::Item]`

  Gives a shared slice over the whole thing.
  
  A correct implementation will return a slice with a length equal to the
  `CAPACITY` value.

- `fn Array::as_slice_mut(&mut self) -> &mut [<Self as >::Item]`

  Gives a unique slice over the whole thing.
  
  A correct implementation will return a slice with a length equal to the
  `CAPACITY` value.

- `fn Array::default() -> Self`

  Create a default-initialized instance of ourself, similar to the
  [`Default`](#default) trait, but implemented for the same range of sizes as
  [`Array`](../index.md).

#### Implementors

- `[T; N]`

