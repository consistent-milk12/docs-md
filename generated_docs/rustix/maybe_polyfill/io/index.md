*[rustix](../../index.md) / [maybe_polyfill](../index.md) / [io](index.md)*

---

# Module `io`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IoSliceMut`](#ioslicemut) | struct |  |

## Structs

### `IoSliceMut<'ctx, R>`

```rust
struct IoSliceMut<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<alloc::vec::IntoIter<&'ctx crate::function::InlinedFunction<R>>>,
    next: Option<Location<'ctx>>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:43-52`](../../../../.source_1765210505/addr2line-0.25.1/src/frame.rs#L43-L52)*

*Re-exported from `addr2line`*

