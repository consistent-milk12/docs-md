*[anstyle_parse](../index.md) / [params](index.md)*

---

# Module `params`

Fixed size parameters list with optional subparameters.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Params`](#params) | struct |  |
| [`ParamsIter`](#paramsiter) | struct | Immutable subparameter iterator. |
| [`MAX_PARAMS`](#max_params) | const |  |

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

*Defined in [`anstyle-parse-0.2.7/src/params.rs:8-25`](../../../.source_1765210505/anstyle-parse-0.2.7/src/params.rs#L8-L25)*

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

- <span id="params-len"></span>`fn len(&self) -> usize`

- <span id="params-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="params-iter"></span>`fn iter(&self) -> ParamsIter<'_>` — [`ParamsIter`](#paramsiter)

- <span id="params-is-full"></span>`fn is_full(&self) -> bool`

- <span id="params-clear"></span>`fn clear(&mut self)`

- <span id="params-push"></span>`fn push(&mut self, item: u16)`

- <span id="params-extend"></span>`fn extend(&mut self, item: u16)`

#### Trait Implementations

##### `impl Clone for Params`

- <span id="params-clone"></span>`fn clone(&self) -> Params` — [`Params`](#params)

##### `impl Debug for Params`

- <span id="params-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Default for Params`

- <span id="params-default"></span>`fn default() -> Params` — [`Params`](#params)

##### `impl Eq for Params`

##### `impl IntoIterator for &'a Params`

- <span id="a-params-type-intoiter"></span>`type IntoIter = ParamsIter<'a>`

- <span id="a-params-type-item"></span>`type Item = &'a [u16]`

- <span id="a-params-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for Params`

- <span id="params-eq"></span>`fn eq(&self, other: &Params) -> bool` — [`Params`](#params)

##### `impl StructuralPartialEq for Params`

### `ParamsIter<'a>`

```rust
struct ParamsIter<'a> {
    params: &'a Params,
    index: usize,
}
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:88-91`](../../../.source_1765210505/anstyle-parse-0.2.7/src/params.rs#L88-L91)*

Immutable subparameter iterator.

#### Implementations

- <span id="paramsiter-new"></span>`fn new(params: &'a Params) -> Self` — [`Params`](#params)

#### Trait Implementations

##### `impl IntoIterator for ParamsIter<'a>`

- <span id="paramsiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="paramsiter-type-intoiter"></span>`type IntoIter = I`

- <span id="paramsiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ParamsIter<'a>`

- <span id="paramsiter-type-item"></span>`type Item = &'a [u16]`

- <span id="paramsiter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="paramsiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Constants

### `MAX_PARAMS`
```rust
const MAX_PARAMS: usize = 32usize;
```

*Defined in [`anstyle-parse-0.2.7/src/params.rs:5`](../../../.source_1765210505/anstyle-parse-0.2.7/src/params.rs#L5)*

