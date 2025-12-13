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

*Defined in [`clap_builder-4.5.53/src/builder/arg_group.rs:68-75`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/arg_group.rs#L68-L75)*

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

- <span id="arggroup-new"></span>`fn new(id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

  Create a `ArgGroup` using a unique name.

  

  The name will be used to get values from the group or refer to the group inside of conflict

  and requirement rules.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::new("config")

  ;

  ```

- <span id="arggroup-id"></span>`fn id(self, id: impl Into<Id>) -> Self` — [`Id`](../../util/id/index.md#id)

  Sets the group name.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, ArgGroup};

  ArgGroup::default().id("config")

  ;

  ```

- <span id="arggroup-arg"></span>`fn arg(self, arg_id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Id`](../../util/id/index.md#id)

  Adds an [argument] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .arg("flag")

          .arg("color"))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-args"></span>`fn args(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md#id)

  Adds multiple [arguments] to this group by name

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .get_matches_from(vec!["myprog", "-f"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  // but we can also check individually if needed

  assert!(m.contains_id("flag"));

  ```

- <span id="arggroup-get-args"></span>`fn get_args(&self) -> impl Iterator<Item = &Id>` — [`Id`](../../util/id/index.md#id)

  Getters for all args. It will return a vector of `Id`

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let args: Vec<&str> = vec!["a1".into(), "a4".into()];

  let grp = ArgGroup::new("program").args(&args);

  

  for (pos, arg) in grp.get_args().enumerate() {

      assert_eq!(*arg, args[pos]);

  }

  ```

- <span id="arggroup-multiple"></span>`fn multiple(self, yes: bool) -> Self`

  Allows more than one of the [`Arg`](../../index.md)s in this group to be used. (Default: `false`)

  

  # Examples

  

  Notice in this example we use *both* the `-f` and `-c` flags which are both part of the

  group

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, ArgAction};

  let m = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .multiple(true))

      .get_matches_from(vec!["myprog", "-f", "-c"]);

  // maybe we don't know which of the two flags was used...

  assert!(m.contains_id("req_flags"));

  ```

  In this next example, we show the default behavior (i.e. `multiple(false)`) which will throw

  an error if more than one of the args in the group was used.

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"]))

      .try_get_matches_from(vec!["myprog", "-f", "-c"]);

  // Because we used both args in the group it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-is-multiple"></span>`fn is_multiple(&mut self) -> bool`

  Return true if the group allows more than one of the arguments

  in this group to be used. (Default: `false`)

  

  # Example

  

  ```rust

  use clap_builder as clap;

  use clap::{ArgGroup};

  let mut group = ArgGroup::new("myprog")

      .args(["f", "c"])

      .multiple(true);

  

  assert!(group.is_multiple());

  ```

- <span id="arggroup-required"></span>`fn required(self, yes: bool) -> Self`

  Require an argument from the group to be present when parsing.

  

  This is unless conflicting with another argument.  A required group will be displayed in

  the usage string of the application in the format `<arg|arg2|arg3>`.

  

  <div class="warning">

  

  **NOTE:** This setting only applies to the current [`Command`](../command/index.md) / [`Subcommand`](../../derive/index.md)s, and not

  globally.

  

  </div>

  

  <div class="warning">

  

  **NOTE:** By default, `ArgGroup::multiple` is set to `false` which when combined with

  `ArgGroup::required(true)` states, "One and *only one* arg must be used from this group.

  Use of more than one arg is an error." Vice setting `ArgGroup::multiple(true)` which

  states, '*At least* one arg from this group must be used. Using multiple is OK."

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .required(true))

      .try_get_matches_from(vec!["myprog"]);

  // Because we didn't use any of the args in the group, it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

  

- <span id="arggroup-requires"></span>`fn requires(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Id`](../../util/id/index.md#id)

  Specify an argument or group that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when any one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires("debug"))

      .try_get_matches_from(vec!["myprog", "-c"]);

  // because we used an arg from the group, and the group requires "-d" to be used, it's an

  // error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-requires-all"></span>`fn requires_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md#id)

  Specify arguments or groups that must be present when this group is.

  

  This is not to be confused with a [required group]. Requirement rules function just like

  [argument requirement rules], you can name other arguments or groups that must be present

  when one of the arguments from this group is used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .requires_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group requires "-d" and "-v" to be used,

  // yet we only used "-d" it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);

  ```

  

- <span id="arggroup-conflicts-with"></span>`fn conflicts_with(self, id: impl IntoResettable<Id>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Id`](../../util/id/index.md#id)

  Specify an argument or group that must **not** be present when this group is.

  

  Exclusion (aka conflict) rules function just like [argument exclusion rules], you can name

  other arguments or groups that must *not* be present when one of the arguments from this

  group are used.

  

  <div class="warning">

  

  **NOTE:** The name provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with("debug"))

      .try_get_matches_from(vec!["myprog", "-c", "-d"]);

  // because we used an arg from the group, and the group conflicts with "-d", it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

- <span id="arggroup-conflicts-with-all"></span>`fn conflicts_with_all(self, ns: impl IntoIterator<Item = impl Into<Id>>) -> Self` — [`Id`](../../util/id/index.md#id)

  Specify arguments or groups that must **not** be present when this group is.

  

  Exclusion rules function just like [argument exclusion rules], you can name other arguments

  or groups that must *not* be present when one of the arguments from this group are used.

  

  <div class="warning">

  

  **NOTE:** The names provided may be an argument, or group name

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::{Command, Arg, ArgGroup, error::ErrorKind, ArgAction};

  let result = Command::new("myprog")

      .arg(Arg::new("flag")

          .short('f')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("color")

          .short('c')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("debug")

          .short('d')

          .action(ArgAction::SetTrue))

      .arg(Arg::new("verb")

          .short('v')

          .action(ArgAction::SetTrue))

      .group(ArgGroup::new("req_flags")

          .args(["flag", "color"])

          .conflicts_with_all(["debug", "verb"]))

      .try_get_matches_from(vec!["myprog", "-c", "-v"]);

  // because we used an arg from the group, and the group conflicts with either "-v" or "-d"

  // it's an error

  assert!(result.is_err());

  let err = result.unwrap_err();

  assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

  ```

#### Trait Implementations

##### `impl Any for ArgGroup`

- <span id="arggroup-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgGroup`

- <span id="arggroup-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgGroup`

- <span id="arggroup-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgGroup`

- <span id="arggroup-clone"></span>`fn clone(&self) -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl CloneToUninit for ArgGroup`

- <span id="arggroup-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgGroup`

- <span id="arggroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ArgGroup`

- <span id="arggroup-default"></span>`fn default() -> ArgGroup` — [`ArgGroup`](#arggroup)

##### `impl Eq for ArgGroup`

##### `impl<T> From for ArgGroup`

- <span id="arggroup-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgGroup`

- <span id="arggroup-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for ArgGroup`

- <span id="arggroup-partialeq-eq"></span>`fn eq(&self, other: &ArgGroup) -> bool` — [`ArgGroup`](#arggroup)

##### `impl StructuralPartialEq for ArgGroup`

##### `impl ToOwned for ArgGroup`

- <span id="arggroup-toowned-type-owned"></span>`type Owned = T`

- <span id="arggroup-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arggroup-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgGroup`

- <span id="arggroup-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arggroup-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgGroup`

- <span id="arggroup-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arggroup-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

