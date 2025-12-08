*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [matched_arg](index.md)*

---

# Module `matched_arg`

## Structs

### `MatchedArg`

```rust
struct MatchedArg {
    source: Option<crate::parser::ValueSource>,
    indices: Vec<usize>,
    type_id: Option<self::any_value::AnyValueId>,
    vals: Vec<Vec<self::any_value::AnyValue>>,
    raw_vals: Vec<Vec<std::ffi::OsString>>,
    ignore_case: bool,
}
```

#### Implementations

- `fn new_arg(arg: &crate::Arg) -> Self` — [`Arg`](../../../index.md)

- `fn new_group() -> Self`

- `fn new_external(cmd: &crate::Command) -> Self` — [`Command`](../../../index.md)

- `fn indices(self: &Self) -> Cloned<Iter<'_, usize>>`

- `fn get_index(self: &Self, index: usize) -> Option<usize>`

- `fn push_index(self: &mut Self, index: usize)`

- `fn vals(self: &Self) -> Iter<'_, Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md)

- `fn into_vals(self: Self) -> Vec<Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md)

- `fn vals_flatten(self: &Self) -> Flatten<Iter<'_, Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md)

- `fn into_vals_flatten(self: Self) -> Flatten<std::vec::IntoIter<Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md)

- `fn raw_vals(self: &Self) -> Iter<'_, Vec<OsString>>`

- `fn raw_vals_flatten(self: &Self) -> Flatten<Iter<'_, Vec<OsString>>>`

- `fn first(self: &Self) -> Option<&AnyValue>` — [`AnyValue`](../../../util/any_value/index.md)

- `fn new_val_group(self: &mut Self)`

- `fn append_val(self: &mut Self, val: AnyValue, raw_val: OsString)` — [`AnyValue`](../../../util/any_value/index.md)

- `fn num_vals(self: &Self) -> usize`

- `fn num_vals_last_group(self: &Self) -> usize`

- `fn check_explicit(self: &Self, predicate: &ArgPredicate) -> bool` — [`ArgPredicate`](../../../builder/index.md)

- `fn source(self: &Self) -> Option<ValueSource>` — [`ValueSource`](../../index.md)

- `fn set_source(self: &mut Self, source: ValueSource)` — [`ValueSource`](../../index.md)

- `fn type_id(self: &Self) -> Option<AnyValueId>` — [`AnyValueId`](../../../util/any_value/index.md)

- `fn infer_type_id(self: &Self, expected: AnyValueId) -> AnyValueId` — [`AnyValueId`](../../../util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for MatchedArg`

- `fn clone(self: &Self) -> MatchedArg` — [`MatchedArg`](#matchedarg)

##### `impl Debug for MatchedArg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for MatchedArg`

##### `impl PartialEq for MatchedArg`

- `fn eq(self: &Self, other: &MatchedArg) -> bool` — [`MatchedArg`](#matchedarg)

