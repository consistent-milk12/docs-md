*[clap_builder](../../index.md) / [parser](../index.md) / [validator](index.md)*

---

# Module `validator`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Validator`](#validator) | struct |  |
| [`Conflicts`](#conflicts) | struct |  |
| [`gather_direct_conflicts`](#gather-direct-conflicts) | fn |  |
| [`gather_arg_direct_conflicts`](#gather-arg-direct-conflicts) | fn |  |
| [`gather_group_direct_conflicts`](#gather-group-direct-conflicts) | fn |  |
| [`get_possible_values_cli`](#get-possible-values-cli) | fn |  |

## Structs

### `Validator<'cmd>`

```rust
struct Validator<'cmd> {
    cmd: &'cmd crate::builder::Command,
    required: self::graph::ChildGraph<crate::util::Id>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:13-16`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L13-L16)*

#### Implementations

- <span id="validator-new"></span>`fn new(cmd: &'cmd Command) -> Self` — [`Command`](../../builder/command/index.md#command)

- <span id="validator-validate"></span>`fn validate(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Result`](../../error/index.md#result)

- <span id="validator-validate-conflicts"></span>`fn validate_conflicts(&mut self, matcher: &ArgMatcher, conflicts: &Conflicts) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Conflicts`](#conflicts), [`Result`](../../error/index.md#result)

- <span id="validator-validate-exclusive"></span>`fn validate_exclusive(&self, matcher: &ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Result`](../../error/index.md#result)

- <span id="validator-build-conflict-err"></span>`fn build_conflict_err(&self, name: &Id, conflict_ids: &[Id], matcher: &ArgMatcher) -> ClapResult<()>` — [`Id`](../../util/id/index.md#id), [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Result`](../../error/index.md#result)

- <span id="validator-build-conflict-err-usage"></span>`fn build_conflict_err_usage(&self, matcher: &ArgMatcher, conflicting_keys: &[Id]) -> Option<StyledStr>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Id`](../../util/id/index.md#id), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

- <span id="validator-gather-requires"></span>`fn gather_requires(&mut self, matcher: &ArgMatcher)` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher)

- <span id="validator-validate-required"></span>`fn validate_required(&mut self, matcher: &ArgMatcher, conflicts: &Conflicts) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Conflicts`](#conflicts), [`Result`](../../error/index.md#result)

- <span id="validator-is-missing-required-ok"></span>`fn is_missing_required_ok(&self, a: &Arg, conflicts: &Conflicts) -> bool` — [`Arg`](../../builder/arg/index.md#arg), [`Conflicts`](#conflicts)

- <span id="validator-fails-arg-required-unless"></span>`fn fails_arg_required_unless(&self, a: &Arg, matcher: &ArgMatcher) -> bool` — [`Arg`](../../builder/arg/index.md#arg), [`ArgMatcher`](../arg_matcher/index.md#argmatcher)

- <span id="validator-missing-required-error"></span>`fn missing_required_error(&self, matcher: &ArgMatcher, raw_req_args: Vec<Id>) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md#argmatcher), [`Id`](../../util/id/index.md#id), [`Result`](../../error/index.md#result)

### `Conflicts`

```rust
struct Conflicts {
    potential: self::flat_map::FlatMap<crate::util::Id, Vec<crate::util::Id>>,
}
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:437-439`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L437-L439)*

#### Implementations

- <span id="conflicts-with-args"></span>`fn with_args(cmd: &Command, matcher: &ArgMatcher) -> Self` — [`Command`](../../builder/command/index.md#command), [`ArgMatcher`](../arg_matcher/index.md#argmatcher)

- <span id="conflicts-gather-conflicts"></span>`fn gather_conflicts(&self, cmd: &Command, arg_id: &Id) -> Vec<Id>` — [`Command`](../../builder/command/index.md#command), [`Id`](../../util/id/index.md#id)

- <span id="conflicts-get-direct-conflicts"></span>`fn get_direct_conflicts(&self, arg_id: &Id) -> Option<&[Id]>` — [`Id`](../../util/id/index.md#id)

#### Trait Implementations

##### `impl Clone for Conflicts`

- <span id="conflicts-clone"></span>`fn clone(&self) -> Conflicts` — [`Conflicts`](#conflicts)

##### `impl Debug for Conflicts`

- <span id="conflicts-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Conflicts`

- <span id="conflicts-default"></span>`fn default() -> Conflicts` — [`Conflicts`](#conflicts)

## Functions

### `gather_direct_conflicts`

```rust
fn gather_direct_conflicts(cmd: &crate::builder::Command, id: &crate::util::Id) -> Vec<crate::util::Id>
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:490-501`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L490-L501)*

### `gather_arg_direct_conflicts`

```rust
fn gather_arg_direct_conflicts(cmd: &crate::builder::Command, arg: &crate::builder::Arg) -> Vec<crate::util::Id>
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:503-521`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L503-L521)*

### `gather_group_direct_conflicts`

```rust
fn gather_group_direct_conflicts(group: &crate::builder::ArgGroup) -> Vec<crate::util::Id>
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:523-525`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L523-L525)*

### `get_possible_values_cli`

```rust
fn get_possible_values_cli(a: &crate::builder::Arg) -> Vec<crate::builder::PossibleValue>
```

*Defined in [`clap_builder-4.5.53/src/parser/validator.rs:527-536`](../../../../.source_1765521767/clap_builder-4.5.53/src/parser/validator.rs#L527-L536)*

