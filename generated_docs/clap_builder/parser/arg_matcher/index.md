*[clap_builder](../../index.md) / [parser](../index.md) / [arg_matcher](index.md)*

---

# Module `arg_matcher`

## Structs

### `ArgMatcher`

```rust
struct ArgMatcher {
    matches: crate::parser::ArgMatches,
    pending: Option<self::parser::PendingArg>,
}
```

#### Implementations

- `fn new(_cmd: &Command) -> Self` — [`Command`](../../index.md)

- `fn into_inner(self: Self) -> ArgMatches` — [`ArgMatches`](../../index.md)

- `fn propagate_globals(self: &mut Self, global_arg_vec: &[Id])` — [`Id`](../../index.md)

- `fn fill_in_global_values(self: &mut Self, global_arg_vec: &[Id], vals_map: &mut FlatMap<Id, MatchedArg>)` — [`Id`](../../index.md), [`FlatMap`](../../util/flat_map/index.md), [`MatchedArg`](../matches/matched_arg/index.md)

- `fn get(self: &Self, arg: &Id) -> Option<&MatchedArg>` — [`Id`](../../index.md), [`MatchedArg`](../matches/matched_arg/index.md)

- `fn get_mut(self: &mut Self, arg: &Id) -> Option<&mut MatchedArg>` — [`Id`](../../index.md), [`MatchedArg`](../matches/matched_arg/index.md)

- `fn remove(self: &mut Self, arg: &Id) -> bool` — [`Id`](../../index.md)

- `fn contains(self: &Self, arg: &Id) -> bool` — [`Id`](../../index.md)

- `fn arg_ids(self: &Self) -> std::slice::Iter<'_, Id>` — [`Id`](../../index.md)

- `fn args(self: &Self) -> crate::util::flat_map::Iter<'_, Id, MatchedArg>` — [`Iter`](../../util/flat_map/index.md), [`Id`](../../index.md), [`MatchedArg`](../matches/matched_arg/index.md)

- `fn entry(self: &mut Self, arg: Id) -> self::flat_map::Entry<'_, Id, MatchedArg>` — [`Id`](../../index.md), [`Entry`](../../util/flat_map/index.md), [`MatchedArg`](../matches/matched_arg/index.md)

- `fn subcommand(self: &mut Self, sc: SubCommand)` — [`SubCommand`](../matches/arg_matches/index.md)

- `fn subcommand_name(self: &Self) -> Option<&str>`

- `fn check_explicit(self: &Self, arg: &Id, predicate: &ArgPredicate) -> bool` — [`Id`](../../index.md), [`ArgPredicate`](../../builder/index.md)

- `fn start_custom_arg(self: &mut Self, arg: &Arg, source: ValueSource)` — [`Arg`](../../index.md), [`ValueSource`](../index.md)

- `fn start_custom_group(self: &mut Self, id: Id, source: ValueSource)` — [`Id`](../../index.md), [`ValueSource`](../index.md)

- `fn start_occurrence_of_external(self: &mut Self, cmd: &Command)` — [`Command`](../../index.md)

- `fn add_val_to(self: &mut Self, arg: &Id, val: AnyValue, raw_val: OsString)` — [`Id`](../../index.md), [`AnyValue`](../../util/any_value/index.md)

- `fn add_index_to(self: &mut Self, arg: &Id, idx: usize)` — [`Id`](../../index.md)

- `fn needs_more_vals(self: &Self, o: &Arg) -> bool` — [`Arg`](../../index.md)

- `fn pending_arg_id(self: &Self) -> Option<&Id>` — [`Id`](../../index.md)

- `fn pending_values_mut(self: &mut Self, id: &Id, ident: Option<Identifier>, trailing_values: bool) -> &mut Vec<OsString>` — [`Id`](../../index.md), [`Identifier`](../parser/index.md)

- `fn start_trailing(self: &mut Self)`

- `fn take_pending(self: &mut Self) -> Option<PendingArg>` — [`PendingArg`](../parser/index.md)

#### Trait Implementations

##### `impl Debug for ArgMatcher`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ArgMatcher`

- `fn default() -> ArgMatcher` — [`ArgMatcher`](#argmatcher)

##### `impl Deref for ArgMatcher`

- `type Target = ArgMatches`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<P, T> Receiver for ArgMatcher`

- `type Target = T`

