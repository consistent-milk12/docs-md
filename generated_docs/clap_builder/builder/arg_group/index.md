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

- <span id="arggroup-get-id"></span>`fn get_id(&self) -> &Id` — [`Id`](../../index.md)

- <span id="arggroup-is-required-set"></span>`fn is_required_set(&self) -> bool`

#### Trait Implementations

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](../../index.md)

##### `impl Debug for ArgGroup`

- <span id="arggroup-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](../../index.md)

##### `impl Eq for ArgGroup`

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](../../index.md)

##### `impl StructuralPartialEq for ArgGroup`

