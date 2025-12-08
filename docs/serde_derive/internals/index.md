*[serde_derive](../index.md) / [internals](index.md)*

---

# Module `internals`

## Modules

- [`ast`](ast/index.md) - A Serde ast, parsed from the Syn ast and ready to generate Rust code.
- [`attr`](attr/index.md) - 
- [`name`](name/index.md) - 
- [`case`](case/index.md) - Code to convert the Rust-styled field/variant (e.g. `my_field`, `MyType`) to the
- [`check`](check/index.md) - 
- [`ctxt`](ctxt/index.md) - 
- [`receiver`](receiver/index.md) - 
- [`respan`](respan/index.md) - 
- [`symbol`](symbol/index.md) - 

## Structs

### `Ctxt`

```rust
struct Ctxt {
    errors: std::cell::RefCell<Option<Vec<syn::Error>>>,
}
```

A type to collect errors together and format them.

Dropping this object will cause a panic. It must be consumed using `check`.

References can be shared since this type uses run-time exclusive mut checking.

#### Implementations

- `fn new() -> Self`

- `fn error_spanned_by<A: ToTokens, T: Display>(self: &Self, obj: A, msg: T)`

- `fn syn_error(self: &Self, err: syn::Error)`

- `fn check(self: Self) -> syn::Result<()>`

#### Trait Implementations

##### `impl Default for Ctxt`

- `fn default() -> Ctxt` — [`Ctxt`](ctxt/index.md)

##### `impl Drop for Ctxt`

- `fn drop(self: &mut Self)`

## Enums

### `Derive`

```rust
enum Derive {
    Serialize,
    Deserialize,
}
```

#### Trait Implementations

##### `impl Clone for Derive`

- `fn clone(self: &Self) -> Derive` — [`Derive`](#derive)

##### `impl Copy for Derive`

## Functions

### `ungroup`

```rust
fn ungroup(ty: &syn::Type) -> &syn::Type
```

