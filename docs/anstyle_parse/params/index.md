*[anstyle_parse](../index.md) / [params](index.md)*

---

# Module `params`

Fixed size parameters list with optional subparameters.

## Structs

### `Params`

```rust
struct Params {
    subparams: [u8; 32],
    params: [u16; 32],
    current_subparams: u8,
    len: usize,
}
```

#### Fields

- **`subparams`**: `[u8; 32]`

  Number of subparameters for each parameter.
  
  For each entry in the `params` slice, this stores the length of the param as number of
  subparams at the same index as the param in the `params` slice.
  
  At the subparam positions the length will always be `0`.

- **`params`**: `[u16; 32]`

  All parameters and subparameters.

- **`current_subparams`**: `u8`

  Number of suparameters in the current parameter.

- **`len`**: `usize`

  Total number of parameters and subparameters.

#### Implementations

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn iter(self: &Self) -> ParamsIter<'_>` — [`ParamsIter`](#paramsiter)

- `fn is_full(self: &Self) -> bool`

- `fn clear(self: &mut Self)`

- `fn push(self: &mut Self, item: u16)`

- `fn extend(self: &mut Self, item: u16)`

#### Trait Implementations

##### `impl Clone for Params`

- `fn clone(self: &Self) -> Params` — [`Params`](#params)

##### `impl Debug for Params`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default for Params`

- `fn default() -> Params` — [`Params`](#params)

##### `impl Eq for Params`

##### `impl PartialEq for Params`

- `fn eq(self: &Self, other: &Params) -> bool` — [`Params`](#params)

##### `impl StructuralPartialEq for Params`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
    params: &'a Params,
    index: usize,
}
```

Immutable subparameter iterator.

#### Implementations

- `fn new(params: &'a Params) -> Self` — [`Params`](#params)

#### Trait Implementations

##### `impl<I> IntoIterator for ParamsIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for ParamsIter<'a>`

- `type Item = &'a [u16]`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Constants

### `MAX_PARAMS`

```rust
const MAX_PARAMS: usize = 32usize;
```

