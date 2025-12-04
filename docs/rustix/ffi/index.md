*[rustix](../index.md) / [ffi](index.md)*

---

# Module `ffi`

Utilities related to FFI bindings.

## Modules

- [`c_ushort`](c_ushort/index.md) - 

## Structs

### `c_uint<'a, A, R>`

```rust
struct c_uint<'a, A, R> {
}
```

An iterator that reports matches in a stream.

This iterator yields elements of type `io::Result<Match>`, where an error
is reported if there was a problem reading from the underlying stream.
The iterator terminates only when the underlying stream reaches `EOF`.

This iterator is constructed via the [`Automaton::try_stream_find_iter`](#try-stream-find-iter)
method.

The type variable `A` refers to the implementation of the [`Automaton`](../../aho_corasick/aho_corasick/automaton/index.md)
trait used to execute the search.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the [`Automaton`](../../aho_corasick/aho_corasick/automaton/index.md)
implementation.

## Functions

