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

