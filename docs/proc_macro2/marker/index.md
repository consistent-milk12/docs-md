*[proc_macro2](../index.md) / [marker](index.md)*

---

# Module `marker`

## Structs

### `ProcMacroAutoTraits`

```rust
struct ProcMacroAutoTraits(core::marker::PhantomData<alloc::rc::Rc<()>>);
```

#### Trait Implementations

##### `impl Clone for ProcMacroAutoTraits`

- `fn clone(self: &Self) -> ProcMacroAutoTraits` — [`ProcMacroAutoTraits`](#procmacroautotraits)

##### `impl Copy for ProcMacroAutoTraits`

##### `impl RefUnwindSafe for ProcMacroAutoTraits`

##### `impl UnwindSafe for ProcMacroAutoTraits`

## Constants

### `MARKER`

```rust
const MARKER: ProcMacroAutoTraits;
```

