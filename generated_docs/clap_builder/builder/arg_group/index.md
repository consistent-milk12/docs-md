*[clap_builder](../../index.md) / [builder](../index.md) / [arg_group](index.md)*

---

# Module `arg_group`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArgGroup`](#arggroup) | struct | Specifies a logical group of [arguments] |

## Structs

### `ArgGroup`

```rust
struct ArgGroup {
    id: crate::util::Id,
    args: Vec<crate::util::Id>,
    required: bool,
    requires: Vec<crate::util::Id>,
    conflicts: Vec<crate::util::Id>,
    multiple: bool,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/arg_group.rs:68-75`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/arg_group.rs#L68-L75)*

Specifies a logical group of [arguments]

You can use this for
- applying validation to an entire group, like `ArgGroup::multiple`
- validate relationships between an argument and a group, like [conflicts] or [requirements]
- check which argument in a group was specified on the command-line

For visually grouping arguments in help, see instead
`Arg::help_heading`.

# Examples

The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
the arguments from the specified group is present at runtime.

```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, error::ErrorKind};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor", "patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major", "--patch"]);
// Because we used two args in the group it's an error
assert!(result.is_err());
let err = result.unwrap_err();
assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
```

This next example shows a passing parse of the same scenario
```rust
use clap_builder as clap;
use clap::{Command, arg, ArgGroup, Id};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor","patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major"]);
assert!(result.is_ok());
let matches = result.unwrap();
// We may not know which of the args was used, so we can test for the group...
assert!(matches.contains_id("vers"));
// We can also ask the group which arg was used
assert_eq!(matches
    .get_one::<Id>("vers")
    .expect("`vers` is required")
    .as_str(),
    "major"
);
// we could also alternatively check each arg individually (not shown here)
```




#### Implementations

- <span id="arggroup-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md)

- <span id="arggroup-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md)

- <span id="arggroup-arg"></span>`fn arg(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md), [`Id`](../../util/id/index.md)

- <span id="arggroup-args"></span>`fn args(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md)

- <span id="arggroup-get-args"></span>`fn get_args(&self) -> impl Iterator<Item = &Id>` — [`Id`](../../util/id/index.md)

- <span id="arggroup-multiple"></span>`fn multiple(self, yes: bool) -> Self`

- <span id="arggroup-is-multiple"></span>`fn is_multiple(&mut self) -> bool`

- <span id="arggroup-required"></span>`fn required(self, yes: bool) -> Self`

- <span id="arggroup-requires"></span>`fn requires(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md), [`Id`](../../util/id/index.md)

- <span id="arggroup-requires-all"></span>`fn requires_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md)

- <span id="arggroup-conflicts-with"></span>`fn conflicts_with(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md), [`Id`](../../util/id/index.md)

- <span id="arggroup-conflicts-with-all"></span>`fn conflicts_with_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md)

#### Trait Implementations

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl Debug for ArgGroup`

- <span id="arggroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](#arggroup)

##### `impl StructuralPartialEq for ArgGroup`

