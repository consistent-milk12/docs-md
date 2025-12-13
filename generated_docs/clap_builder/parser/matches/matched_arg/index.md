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

*Defined in [`clap_builder-4.5.53/src/parser/matches/matched_arg.rs:16-23`](../../../../../.source_1765633015/clap_builder-4.5.53/src/parser/matches/matched_arg.rs#L16-L23)*

#### Implementations

- <span id="matchedarg-new-arg"></span>`fn new_arg(arg: &crate::Arg) -> Self` — [`Arg`](../../../builder/arg/index.md#arg)

- <span id="matchedarg-new-group"></span>`fn new_group() -> Self`

- <span id="matchedarg-new-external"></span>`fn new_external(cmd: &crate::Command) -> Self` — [`Command`](../../../builder/command/index.md#command)

- <span id="matchedarg-indices"></span>`fn indices(&self) -> Cloned<Iter<'_, usize>>`

- <span id="matchedarg-get-index"></span>`fn get_index(&self, index: usize) -> Option<usize>`

- <span id="matchedarg-push-index"></span>`fn push_index(&mut self, index: usize)`

- <span id="matchedarg-vals"></span>`fn vals(&self) -> Iter<'_, Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-into-vals"></span>`fn into_vals(self) -> Vec<Vec<AnyValue>>` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-vals-flatten"></span>`fn vals_flatten(&self) -> Flatten<Iter<'_, Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-into-vals-flatten"></span>`fn into_vals_flatten(self) -> Flatten<std::vec::IntoIter<Vec<AnyValue>>>` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-raw-vals"></span>`fn raw_vals(&self) -> Iter<'_, Vec<OsString>>`

- <span id="matchedarg-raw-vals-flatten"></span>`fn raw_vals_flatten(&self) -> Flatten<Iter<'_, Vec<OsString>>>`

- <span id="matchedarg-first"></span>`fn first(&self) -> Option<&AnyValue>` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-new-val-group"></span>`fn new_val_group(&mut self)`

- <span id="matchedarg-append-val"></span>`fn append_val(&mut self, val: AnyValue, raw_val: OsString)` — [`AnyValue`](../../../util/any_value/index.md#anyvalue)

- <span id="matchedarg-num-vals"></span>`fn num_vals(&self) -> usize`

- <span id="matchedarg-num-vals-last-group"></span>`fn num_vals_last_group(&self) -> usize`

- <span id="matchedarg-check-explicit"></span>`fn check_explicit(&self, predicate: &ArgPredicate) -> bool` — [`ArgPredicate`](../../../builder/arg_predicate/index.md#argpredicate)

- <span id="matchedarg-source"></span>`fn source(&self) -> Option<ValueSource>` — [`ValueSource`](../value_source/index.md#valuesource)

- <span id="matchedarg-set-source"></span>`fn set_source(&mut self, source: ValueSource)` — [`ValueSource`](../value_source/index.md#valuesource)

- <span id="matchedarg-type-id"></span>`fn type_id(&self) -> Option<AnyValueId>` — [`AnyValueId`](../../../util/any_value/index.md#anyvalueid)

- <span id="matchedarg-infer-type-id"></span>`fn infer_type_id(&self, expected: AnyValueId) -> AnyValueId` — [`AnyValueId`](../../../util/any_value/index.md#anyvalueid)

#### Trait Implementations

##### `impl Any for MatchedArg`

- <span id="matchedarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchedArg`

- <span id="matchedarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchedArg`

- <span id="matchedarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchedArg`

- <span id="matchedarg-clone"></span>`fn clone(&self) -> MatchedArg` — [`MatchedArg`](#matchedarg)

##### `impl CloneToUninit for MatchedArg`

- <span id="matchedarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchedArg`

- <span id="matchedarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchedArg`

##### `impl<T> From for MatchedArg`

- <span id="matchedarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchedArg`

- <span id="matchedarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchedArg`

- <span id="matchedarg-partialeq-eq"></span>`fn eq(&self, other: &MatchedArg) -> bool` — [`MatchedArg`](#matchedarg)

##### `impl ToOwned for MatchedArg`

- <span id="matchedarg-toowned-type-owned"></span>`type Owned = T`

- <span id="matchedarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matchedarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchedArg`

- <span id="matchedarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchedarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchedArg`

- <span id="matchedarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchedarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

