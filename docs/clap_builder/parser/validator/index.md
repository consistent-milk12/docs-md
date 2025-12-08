*[clap_builder](../../index.md) / [parser](../index.md) / [validator](index.md)*

---

# Module `validator`

## Structs

### `Validator<'cmd>`

```rust
struct Validator<'cmd> {
    cmd: &'cmd crate::builder::Command,
    required: self::graph::ChildGraph<crate::util::Id>,
}
```

#### Implementations

- `fn new(cmd: &'cmd Command) -> Self` — [`Command`](../../index.md)

- `fn validate(self: &mut Self, matcher: &mut ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn validate_conflicts(self: &mut Self, matcher: &ArgMatcher, conflicts: &Conflicts) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Conflicts`](#conflicts), [`Result`](../../error/index.md)

- `fn validate_exclusive(self: &Self, matcher: &ArgMatcher) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn build_conflict_err(self: &Self, name: &Id, conflict_ids: &[Id], matcher: &ArgMatcher) -> ClapResult<()>` — [`Id`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md), [`Result`](../../error/index.md)

- `fn build_conflict_err_usage(self: &Self, matcher: &ArgMatcher, conflicting_keys: &[Id]) -> Option<StyledStr>` — [`ArgMatcher`](../arg_matcher/index.md), [`Id`](../../index.md), [`StyledStr`](../../builder/index.md)

- `fn gather_requires(self: &mut Self, matcher: &ArgMatcher)` — [`ArgMatcher`](../arg_matcher/index.md)

- `fn validate_required(self: &mut Self, matcher: &ArgMatcher, conflicts: &Conflicts) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Conflicts`](#conflicts), [`Result`](../../error/index.md)

- `fn is_missing_required_ok(self: &Self, a: &Arg, conflicts: &Conflicts) -> bool` — [`Arg`](../../index.md), [`Conflicts`](#conflicts)

- `fn fails_arg_required_unless(self: &Self, a: &Arg, matcher: &ArgMatcher) -> bool` — [`Arg`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md)

- `fn missing_required_error(self: &Self, matcher: &ArgMatcher, raw_req_args: Vec<Id>) -> ClapResult<()>` — [`ArgMatcher`](../arg_matcher/index.md), [`Id`](../../index.md), [`Result`](../../error/index.md)

### `Conflicts`

```rust
struct Conflicts {
    potential: self::flat_map::FlatMap<crate::util::Id, Vec<crate::util::Id>>,
}
```

#### Implementations

- `fn with_args(cmd: &Command, matcher: &ArgMatcher) -> Self` — [`Command`](../../index.md), [`ArgMatcher`](../arg_matcher/index.md)

- `fn gather_conflicts(self: &Self, cmd: &Command, arg_id: &Id) -> Vec<Id>` — [`Command`](../../index.md), [`Id`](../../index.md)

- `fn get_direct_conflicts(self: &Self, arg_id: &Id) -> Option<&[Id]>` — [`Id`](../../index.md)

#### Trait Implementations

##### `impl Clone for Conflicts`

- `fn clone(self: &Self) -> Conflicts` — [`Conflicts`](#conflicts)

##### `impl Debug for Conflicts`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Conflicts`

- `fn default() -> Conflicts` — [`Conflicts`](#conflicts)

## Functions

### `gather_direct_conflicts`

```rust
fn gather_direct_conflicts(cmd: &crate::builder::Command, id: &crate::util::Id) -> Vec<crate::util::Id>
```

### `gather_arg_direct_conflicts`

```rust
fn gather_arg_direct_conflicts(cmd: &crate::builder::Command, arg: &crate::builder::Arg) -> Vec<crate::util::Id>
```

### `gather_group_direct_conflicts`

```rust
fn gather_group_direct_conflicts(group: &crate::builder::ArgGroup) -> Vec<crate::util::Id>
```

### `get_possible_values_cli`

```rust
fn get_possible_values_cli(a: &crate::builder::Arg) -> Vec<crate::builder::PossibleValue>
```

