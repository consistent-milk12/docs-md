*[proc_macro2](../index.md) / [marker](index.md)*

---

# Module `marker`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProcMacroAutoTraits`](#procmacroautotraits) | struct |  |
| [`MARKER`](#marker) | const |  |

## Structs

### `ProcMacroAutoTraits`

```rust
struct ProcMacroAutoTraits(core::marker::PhantomData<alloc::rc::Rc<()>>);
```

*Defined in [`proc-macro2-1.0.103/src/marker.rs:12`](../../../.source_1765521767/proc-macro2-1.0.103/src/marker.rs#L12)*

#### Trait Implementations

##### `impl Clone for ProcMacroAutoTraits`

- <span id="procmacroautotraits-clone"></span>`fn clone(&self) -> ProcMacroAutoTraits` — [`ProcMacroAutoTraits`](#procmacroautotraits)

##### `impl Copy for ProcMacroAutoTraits`

##### `impl RefUnwindSafe for ProcMacroAutoTraits`

##### `impl UnwindSafe for ProcMacroAutoTraits`

## Constants

### `MARKER`
```rust
const MARKER: ProcMacroAutoTraits;
```

*Defined in [`proc-macro2-1.0.103/src/marker.rs:14`](../../../.source_1765521767/proc-macro2-1.0.103/src/marker.rs#L14)*

