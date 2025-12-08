*[clap_builder](../../../index.md) / [parser](../../index.md) / [matches](../index.md) / [matched_arg](index.md)*

---

# Module `matched_arg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MatchedArg`](#matchedarg) | struct |  |

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

- <span id="matchedarg-new-arg"></span>`fn new_arg(arg: &crate::Arg) -> Self` — [`Arg`](../../../index.md)

- <span id="matchedarg-new-group"></span>`fn new_group() -> Self`

- <span id="matchedarg-new-external"></span>`fn new_external(cmd: &crate::Command) -> Self` — [`Command`](../../../index.md)

- <span id="matchedarg-indices"></span>`fn indices(&self) -> Cloned<Iter<'_, usize>>`

- <span id="matchedarg-get-index"></span>`fn get_index(&self, index: usize) -> Option<usize>`

- <span id="matchedarg-push-index"></span>`fn push_index(&mut self, index: usize)`

- <span id="matchedarg-vals"></span>`fn vals(&self) -> Iter<'_, Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-into-vals"></span>`fn into_vals(self) -> Vec<Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-vals-flatten"></span>`fn vals_flatten(&self) -> Flatten<Iter<'_, Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-into-vals-flatten"></span>`fn into_vals_flatten(self) -> Flatten<std::vec::IntoIter<Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-raw-vals"></span>`fn raw_vals(&self) -> Iter<'_, Vec<OsString>>`

- <span id="matchedarg-raw-vals-flatten"></span>`fn raw_vals_flatten(&self) -> Flatten<Iter<'_, Vec<OsString>>>`

- <span id="matchedarg-first"></span>`fn first(&self) -> Option<&AnyValue>` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-new-val-group"></span>`fn new_val_group(&mut self)`

- <span id="matchedarg-append-val"></span>`fn append_val(&mut self, val: AnyValue, raw_val: OsString)` — [`AnyValue`](../../../util/any_value/index.md)

- <span id="matchedarg-num-vals"></span>`fn num_vals(&self) -> usize`

- <span id="matchedarg-num-vals-last-group"></span>`fn num_vals_last_group(&self) -> usize`

- <span id="matchedarg-check-explicit"></span>`fn check_explicit(&self, predicate: &ArgPredicate) -> bool` — [`ArgPredicate`](../../../builder/index.md)

- <span id="matchedarg-source"></span>`fn source(&self) -> Option<ValueSource>` — [`ValueSource`](../../index.md)

- <span id="matchedarg-set-source"></span>`fn set_source(&mut self, source: ValueSource)` — [`ValueSource`](../../index.md)

- <span id="matchedarg-type-id"></span>`fn type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](../../../util/any_value/index.md)

- <span id="matchedarg-infer-type-id"></span>`fn infer_type_id(&self, expected: AnyValueId) -> AnyValueId` — [`AnyValueId`](../../../util/any_value/index.md)

#### Trait Implementations

##### `impl Clone for MatchedArg`

- <span id="matchedarg-clone"></span>`fn clone(&self) -> MatchedArg` — [`MatchedArg`](#matchedarg)

##### `impl Debug for MatchedArg`

- <span id="matchedarg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchedArg`

##### `impl PartialEq for MatchedArg`

- <span id="matchedarg-eq"></span>`fn eq(&self, other: &MatchedArg) -> bool` — [`MatchedArg`](#matchedarg)

