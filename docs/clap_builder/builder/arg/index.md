*[clap_builder](../../index.md) / [builder](../index.md) / [arg](index.md)*

---

# Module `arg`

## Structs

### `Arg`

```rust
struct Arg {
    id: crate::Id,
    help: Option<crate::builder::StyledStr>,
    long_help: Option<crate::builder::StyledStr>,
    action: Option<crate::ArgAction>,
    value_parser: Option<super::ValueParser>,
    blacklist: Vec<crate::Id>,
    settings: arg_settings::ArgFlags,
    overrides: Vec<crate::Id>,
    groups: Vec<crate::Id>,
    requires: Vec<(crate::builder::ArgPredicate, crate::Id)>,
    r_ifs: Vec<(crate::Id, crate::builder::OsStr)>,
    r_ifs_all: Vec<(crate::Id, crate::builder::OsStr)>,
    r_unless: Vec<crate::Id>,
    r_unless_all: Vec<crate::Id>,
    short: Option<char>,
    long: Option<crate::builder::Str>,
    aliases: Vec<(crate::builder::Str, bool)>,
    short_aliases: Vec<(char, bool)>,
    disp_ord: Option<usize>,
    val_names: Vec<crate::builder::Str>,
    num_vals: Option<crate::builder::ValueRange>,
    val_delim: Option<char>,
    default_vals: Vec<crate::builder::OsStr>,
    default_vals_ifs: Vec<(crate::Id, crate::builder::ArgPredicate, Option<Vec<crate::builder::OsStr>>)>,
    default_missing_vals: Vec<crate::builder::OsStr>,
    terminator: Option<crate::builder::Str>,
    index: Option<usize>,
    help_heading: Option<Option<crate::builder::Str>>,
    ext: crate::builder::ext::Extensions,
}
```

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`](#arg)s, using the builder pattern and setting options
manually, or using a usage string which is far less verbose but has fewer options. You can also
use a combination of the two methods to achieve the best of both worlds.

- [Basic API][crate::Arg#basic-api]
- [Value Handling][crate::Arg#value-handling]
- [Help][crate::Arg#help-1]
- [Advanced Argument Relations][crate::Arg#advanced-argument-relations]
- [Reflection][crate::Arg#reflection]

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, arg, ArgAction};
// Using the traditional builder pattern and setting each option manually
let cfg = Arg::new("config")
      .short('c')
      .long("config")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("Provides a config file to myprog");
// Using a usage string (setting a similar argument to the one above)
let input = arg!(-i --input <FILE> "Provides an input file to the program");
```

#### Implementations

- `fn _build(self: &mut Self)`

- `fn name_no_brackets(self: &Self) -> String`

- `fn stylized(self: &Self, styles: &Styles, required: Option<bool>) -> StyledStr` — [`Styles`](../styling/index.md), [`StyledStr`](../styled_str/index.md)

- `fn stylize_arg_suffix(self: &Self, styles: &Styles, required: Option<bool>) -> StyledStr` — [`Styles`](../styling/index.md), [`StyledStr`](../styled_str/index.md)

- `fn render_arg_val(self: &Self, required: bool) -> String`

- `fn is_multiple(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Arg`

- `fn clone(self: &Self) -> Arg` — [`Arg`](#arg)

##### `impl Debug for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Default for Arg`

- `fn default() -> Arg` — [`Arg`](#arg)

##### `impl Display for Arg`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for Arg`

##### `impl Ord for Arg`

- `fn cmp(self: &Self, other: &Arg) -> Ordering` — [`Arg`](#arg)

##### `impl PartialEq for Arg`

- `fn eq(self: &Self, other: &Arg) -> bool` — [`Arg`](#arg)

##### `impl PartialOrd for Arg`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl<T> ToString for Arg`

- `fn to_string(self: &Self) -> String`

