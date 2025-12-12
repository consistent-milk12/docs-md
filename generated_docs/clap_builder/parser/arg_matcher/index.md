*[clap_builder](../../index.md) / [parser](../index.md) / [arg_matcher](index.md)*

---

# Module `arg_matcher`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgMatcher`](#argmatcher) | struct |  |

## Structs

### `ArgMatcher`

```rust
struct ArgMatcher {
    matches: crate::parser::ArgMatches,
    pending: Option<self::parser::PendingArg>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/arg_matcher.rs:17-20`](../../../../.source_1765210505/clap_builder-4.5.53/src/parser/arg_matcher.rs#L17-L20)*

#### Implementations

- <span id="argmatcher-new"></span>`fn new(_cmd: &Command) -> Self` — [`Command`](../../builder/command/index.md#command)

- <span id="argmatcher-into-inner"></span>`fn into_inner(self) -> ArgMatches` — [`ArgMatches`](../matches/arg_matches/index.md#argmatches)

- <span id="argmatcher-propagate-globals"></span>`fn propagate_globals(&mut self, global_arg_vec: &[Id])` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-fill-in-global-values"></span>`fn fill_in_global_values(&mut self, global_arg_vec: &[Id], vals_map: &mut FlatMap<Id, MatchedArg>)` — [`Id`](../../util/id/index.md#id), [`FlatMap`](../../util/flat_map/index.md#flatmap), [`MatchedArg`](../matches/matched_arg/index.md#matchedarg)

- <span id="argmatcher-get"></span>`fn get(&self, arg: &Id) -> Option<&MatchedArg>` — [`Id`](../../util/id/index.md#id), [`MatchedArg`](../matches/matched_arg/index.md#matchedarg)

- <span id="argmatcher-get-mut"></span>`fn get_mut(&mut self, arg: &Id) -> Option<&mut MatchedArg>` — [`Id`](../../util/id/index.md#id), [`MatchedArg`](../matches/matched_arg/index.md#matchedarg)

- <span id="argmatcher-remove"></span>`fn remove(&mut self, arg: &Id) -> bool` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-contains"></span>`fn contains(&self, arg: &Id) -> bool` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-arg-ids"></span>`fn arg_ids(&self) -> std::slice::Iter<'_, Id>` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-args"></span>`fn args(&self) -> crate::util::flat_map::Iter<'_, Id, MatchedArg>` — [`Iter`](../../util/flat_map/index.md#iter), [`Id`](../../util/id/index.md#id), [`MatchedArg`](../matches/matched_arg/index.md#matchedarg)

- <span id="argmatcher-entry"></span>`fn entry(&mut self, arg: Id) -> self::flat_map::Entry<'_, Id, MatchedArg>` — [`Id`](../../util/id/index.md#id), [`Entry`](../../util/flat_map/index.md#entry), [`MatchedArg`](../matches/matched_arg/index.md#matchedarg)

- <span id="argmatcher-subcommand"></span>`fn subcommand(&mut self, sc: SubCommand)` — [`SubCommand`](../matches/arg_matches/index.md#subcommand)

- <span id="argmatcher-subcommand-name"></span>`fn subcommand_name(&self) -> Option<&str>`

- <span id="argmatcher-check-explicit"></span>`fn check_explicit(&self, arg: &Id, predicate: &ArgPredicate) -> bool` — [`Id`](../../util/id/index.md#id), [`ArgPredicate`](../../builder/arg_predicate/index.md#argpredicate)

- <span id="argmatcher-start-custom-arg"></span>`fn start_custom_arg(&mut self, arg: &Arg, source: ValueSource)` — [`Arg`](../../builder/arg/index.md#arg), [`ValueSource`](../matches/value_source/index.md#valuesource)

- <span id="argmatcher-start-custom-group"></span>`fn start_custom_group(&mut self, id: Id, source: ValueSource)` — [`Id`](../../util/id/index.md#id), [`ValueSource`](../matches/value_source/index.md#valuesource)

- <span id="argmatcher-start-occurrence-of-external"></span>`fn start_occurrence_of_external(&mut self, cmd: &Command)` — [`Command`](../../builder/command/index.md#command)

- <span id="argmatcher-add-val-to"></span>`fn add_val_to(&mut self, arg: &Id, val: AnyValue, raw_val: OsString)` — [`Id`](../../util/id/index.md#id), [`AnyValue`](../../util/any_value/index.md#anyvalue)

- <span id="argmatcher-add-index-to"></span>`fn add_index_to(&mut self, arg: &Id, idx: usize)` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-needs-more-vals"></span>`fn needs_more_vals(&self, o: &Arg) -> bool` — [`Arg`](../../builder/arg/index.md#arg)

- <span id="argmatcher-pending-arg-id"></span>`fn pending_arg_id(&self) -> Option<&Id>` — [`Id`](../../util/id/index.md#id)

- <span id="argmatcher-pending-values-mut"></span>`fn pending_values_mut(&mut self, id: &Id, ident: Option<Identifier>, trailing_values: bool) -> &mut Vec<OsString>` — [`Id`](../../util/id/index.md#id), [`Identifier`](../parser/index.md#identifier)

- <span id="argmatcher-start-trailing"></span>`fn start_trailing(&mut self)`

- <span id="argmatcher-take-pending"></span>`fn take_pending(&mut self) -> Option<PendingArg>` — [`PendingArg`](../parser/index.md#pendingarg)

#### Trait Implementations

##### `impl Debug for ArgMatcher`

- <span id="argmatcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgMatcher`

- <span id="argmatcher-default"></span>`fn default() -> ArgMatcher` — [`ArgMatcher`](#argmatcher)

##### `impl Deref for ArgMatcher`

- <span id="argmatcher-deref-type-target"></span>`type Target = ArgMatches`

- <span id="argmatcher-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for ArgMatcher`

- <span id="argmatcher-receiver-type-target"></span>`type Target = T`

