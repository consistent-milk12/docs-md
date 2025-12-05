*[rustix](../index.md) / [ffi](index.md)*

---

# Module `ffi`

Utilities related to FFI bindings.

## Structs

### `c_void`

```rust
struct c_void {
}
```

A non-empty collection of non-empty patterns to search for.

This collection of patterns is what is passed around to both execute
searches and to construct the searchers themselves. Namely, this permits
searches to avoid copying all of the patterns, and allows us to keep only
one copy throughout all packed searchers.

Note that this collection is not a set. The same pattern can appear more
than once.

## Functions

