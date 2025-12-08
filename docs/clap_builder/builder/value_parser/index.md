*[clap_builder](../../index.md) / [builder](../index.md) / [value_parser](index.md)*

---

# Module `value_parser`

## Modules

- [`private`](private/index.md) - 

## Structs

### `ValueParser`

```rust
struct ValueParser(ValueParserInner);
```

Parse/validate argument values

Specified with `Arg::value_parser`.

`ValueParser` defines how to convert a raw argument value into a validated and typed value for
use within an application.

See
- `value_parser!` for automatically selecting an implementation for a given type
- `ValueParser::new` for additional [`TypedValueParser`](#typedvalueparser) that can be used

# Example

```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .long("color")
            .value_parser(["always", "auto", "never"])
            .default_value("auto")
    )
    .arg(
        clap::Arg::new("hostname")
            .long("hostname")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .action(clap::ArgAction::Set)
            .required(true)
    )
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(
    ["cmd", "--hostname", "rust-lang.org", "--port", "3001"]
).unwrap();

let color: &String = m.get_one("color")
    .expect("default");
assert_eq!(color, "auto");

let hostname: &String = m.get_one("hostname")
    .expect("required");
assert_eq!(hostname, "rust-lang.org");

let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

#### Implementations

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, source: ValueSource) -> Result<AnyValue, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`ValueSource`](../../parser/matches/value_source/index.md), [`AnyValue`](../../util/any_value/index.md), [`Error`](../../index.md)

- `fn type_id(self: &Self) -> AnyValueId` — [`AnyValueId`](../../util/any_value/index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

- `fn any_value_parser(self: &Self) -> &dyn AnyValueParser` — [`AnyValueParser`](#anyvalueparser)

#### Trait Implementations

##### `impl Clone for ValueParser`

- `fn clone(self: &Self) -> Self`

##### `impl Debug for ValueParser`

- `fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl<I> IntoResettable for ValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

### `StringValueParser`

```rust
struct StringValueParser {
}
```

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for StringValueParser`

- `fn clone(self: &Self) -> StringValueParser` — [`StringValueParser`](#stringvalueparser)

##### `impl Copy for StringValueParser`

##### `impl Debug for StringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for StringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for StringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for StringValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for OsStringValueParser`

- `fn clone(self: &Self) -> OsStringValueParser` — [`OsStringValueParser`](#osstringvalueparser)

##### `impl Copy for OsStringValueParser`

##### `impl Debug for OsStringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for OsStringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for OsStringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for OsStringValueParser`

- `type Value = OsString`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse(self: &Self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for PathBufValueParser`

- `fn clone(self: &Self) -> PathBufValueParser` — [`PathBufValueParser`](#pathbufvalueparser)

##### `impl Copy for PathBufValueParser`

##### `impl Debug for PathBufValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PathBufValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for PathBufValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PathBufValueParser`

- `type Value = PathBuf`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(std::marker::PhantomData<E>);
```

Parse an `ValueEnum` value.

See also:
- [`PossibleValuesParser`](#possiblevaluesparser)

# Example

```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::ColorChoice;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;

// Usage
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::EnumValueParser::<ColorChoice>::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: ColorChoice = *m.get_one("color")
    .expect("required");
assert_eq!(port, ColorChoice::Always);

// Semantics
let value_parser = clap::builder::EnumValueParser::<ColorChoice>::new();
// or
let value_parser = clap::value_parser!(ColorChoice);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), ColorChoice::Always);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), ColorChoice::Auto);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), ColorChoice::Never);
```

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl<E: $crate::clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static> Clone for EnumValueParser<E>`

- `fn clone(self: &Self) -> EnumValueParser<E>` — [`EnumValueParser`](#enumvalueparser)

##### `impl<E: $crate::fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static> Debug for EnumValueParser<E>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> Default for EnumValueParser<E>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for EnumValueParser<E>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> TypedValueParser for EnumValueParser<E>`

- `type Value = E`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `PossibleValuesParser`

```rust
struct PossibleValuesParser(Vec<super::PossibleValue>);
```

Verify the value is from an enumerated set of `PossibleValue`.

See also:
- [`EnumValueParser`](#enumvalueparser) for directly supporting `ValueEnum` types
- `TypedValueParser::map` for adapting values to a more specialized type, like an external
  enums that can't implement `ValueEnum`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::PossibleValuesParser::new(["always", "auto", "never"]))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: &String = m.get_one("color")
    .expect("required");
assert_eq!(port, "always");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::PossibleValuesParser::new(["always", "auto", "never"]);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), "always");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), "auto");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), "never");
```

#### Implementations

- `fn new(values: impl Into<PossibleValuesParser>) -> Self` — [`PossibleValuesParser`](#possiblevaluesparser)

#### Trait Implementations

##### `impl Clone for PossibleValuesParser`

- `fn clone(self: &Self) -> PossibleValuesParser` — [`PossibleValuesParser`](#possiblevaluesparser)

##### `impl Debug for PossibleValuesParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for PossibleValuesParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PossibleValuesParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync>`

```rust
struct RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync> {
    bounds: (std::ops::Bound<i64>, std::ops::Bound<i64>),
    target: std::marker::PhantomData<T>,
}
```

Parse number that fall within a range of values

<div class="warning">

**NOTE:** To capture negative values, you will also need to set
`Arg::allow_negative_numbers` or
`Arg::allow_hyphen_values`.

</div>

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedI64ValueParser::<i32>::new().range(-1..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).unwrap(), -1);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- `fn new() -> Self`

- `fn range<B: RangeBounds<i64>>(self: Self, range: B) -> Self`

- `fn format_bounds(self: &Self) -> String`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + TryFrom<i64> + Clone + Send + Sync> Clone for RangedI64ValueParser<T>`

- `fn clone(self: &Self) -> RangedI64ValueParser<T>` — [`RangedI64ValueParser`](#rangedi64valueparser)

##### `impl<T: $crate::marker::Copy + TryFrom<i64> + Clone + Send + Sync> Copy for RangedI64ValueParser<T>`

##### `impl<T: $crate::fmt::Debug + TryFrom<i64> + Clone + Send + Sync> Debug for RangedI64ValueParser<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync> Default for RangedI64ValueParser<T>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for RangedI64ValueParser<T>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<i64> + Clone + Send + Sync + 'static> TypedValueParser for RangedI64ValueParser<T>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `RangedU64ValueParser<T: TryFrom<u64>>`

```rust
struct RangedU64ValueParser<T: TryFrom<u64>> {
    bounds: (std::ops::Bound<u64>, std::ops::Bound<u64>),
    target: std::marker::PhantomData<T>,
}
```

Parse number that fall within a range of values

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u64).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u64 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::RangedU64ValueParser::<u32>::new().range(0..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

#### Implementations

- `fn new() -> Self`

- `fn range<B: RangeBounds<u64>>(self: Self, range: B) -> Self`

- `fn format_bounds(self: &Self) -> String`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone + TryFrom<u64>> Clone for RangedU64ValueParser<T>`

- `fn clone(self: &Self) -> RangedU64ValueParser<T>` — [`RangedU64ValueParser`](#rangedu64valueparser)

##### `impl<T: $crate::marker::Copy + TryFrom<u64>> Copy for RangedU64ValueParser<T>`

##### `impl<T: $crate::fmt::Debug + TryFrom<u64>> Debug for RangedU64ValueParser<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: TryFrom<u64>> Default for RangedU64ValueParser<T>`

- `fn default() -> Self`

##### `impl<I> IntoResettable for RangedU64ValueParser<T>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<u64> + Clone + Send + Sync + 'static> TypedValueParser for RangedU64ValueParser<T>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolValueParser`

- `fn clone(self: &Self) -> BoolValueParser` — [`BoolValueParser`](#boolvalueparser)

##### `impl Copy for BoolValueParser`

##### `impl Debug for BoolValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BoolValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for BoolValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

Parse false-like string values, everything else is `true`

See also:
- `ValueParser::bool` for assuming non-false is true
- [`BoolishValueParser`](#boolishvalueparser) for different human readable bool representations

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::FalseyValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::FalseyValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for FalseyValueParser`

- `fn clone(self: &Self) -> FalseyValueParser` — [`FalseyValueParser`](#falseyvalueparser)

##### `impl Copy for FalseyValueParser`

##### `impl Debug for FalseyValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for FalseyValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for FalseyValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for FalseyValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

Parse bool-like string values

See also:
- `ValueParser::bool` for different human readable bool representations
- [`FalseyValueParser`](#falseyvalueparser) for assuming non-false is true

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::BoolishValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::BoolishValueParser::new();
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("true")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("Yes")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oN")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("1")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

#### Implementations

- `fn new() -> Self`

- `fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolishValueParser`

- `fn clone(self: &Self) -> BoolishValueParser` — [`BoolishValueParser`](#boolishvalueparser)

##### `impl Copy for BoolishValueParser`

##### `impl Debug for BoolishValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for BoolishValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for BoolishValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolishValueParser`

- `type Value = bool`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `NonEmptyStringValueParser`

```rust
struct NonEmptyStringValueParser {
}
```

Parse non-empty string values

See also:
- `ValueParser::string`

# Example

Usage:
```rust
use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: &String = m.get_one("append")
    .expect("required");
assert_eq!(port, "true");
```

Semantics:
```rust
use clap_builder as clap;
use std::ffi::OsStr;
use clap::builder::TypedValueParser;
let cmd = clap::Command::new("test");
let arg = None;
let value_parser = clap::builder::NonEmptyStringValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), "random");
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
```

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NonEmptyStringValueParser`

- `fn clone(self: &Self) -> NonEmptyStringValueParser` — [`NonEmptyStringValueParser`](#nonemptystringvalueparser)

##### `impl Copy for NonEmptyStringValueParser`

##### `impl Debug for NonEmptyStringValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for NonEmptyStringValueParser`

- `fn default() -> Self`

##### `impl<I> IntoResettable for NonEmptyStringValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for NonEmptyStringValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `MapValueParser<P, F>`

```rust
struct MapValueParser<P, F> {
    parser: P,
    func: F,
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::map`

#### Implementations

- `fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: $crate::clone::Clone, F: $crate::clone::Clone> Clone for MapValueParser<P, F>`

- `fn clone(self: &Self) -> MapValueParser<P, F>` — [`MapValueParser`](#mapvalueparser)

##### `impl<P: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for MapValueParser<P, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for MapValueParser<P, F>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<P, F, T> TypedValueParser for MapValueParser<P, F>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `TryMapValueParser<P, F>`

```rust
struct TryMapValueParser<P, F> {
    parser: P,
    func: F,
}
```

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::try_map`

#### Implementations

- `fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: $crate::clone::Clone, F: $crate::clone::Clone> Clone for TryMapValueParser<P, F>`

- `fn clone(self: &Self) -> TryMapValueParser<P, F>` — [`TryMapValueParser`](#trymapvalueparser)

##### `impl<P: $crate::fmt::Debug, F: $crate::fmt::Debug> Debug for TryMapValueParser<P, F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for TryMapValueParser<P, F>`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<P, F, T, E> TypedValueParser for TryMapValueParser<P, F>`

- `type Value = T`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `UnknownArgumentValueParser`

```rust
struct UnknownArgumentValueParser {
    arg: Option<crate::builder::Str>,
    suggestions: Vec<crate::builder::StyledStr>,
}
```

When encountered, report `ErrorKind::UnknownArgument`

Useful to help users migrate, either from old versions or similar tools.

# Examples

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
let cmd = Command::new("mycmd")
    .args([
        Arg::new("current-dir")
            .short('C'),
        Arg::new("current-dir-unknown")
            .long("cwd")
            .aliases(["current-dir", "directory", "working-directory", "root"])
            .value_parser(clap::builder::UnknownArgumentValueParser::suggest_arg("-C"))
            .hide(true),
    ]);

// Use a supported version of the argument
let matches = cmd.clone().try_get_matches_from(["mycmd", "-C", ".."]).unwrap();
assert!(matches.contains_id("current-dir"));
assert_eq!(
    matches.get_many::<String>("current-dir").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    vec![".."]
);

// Use one of the invalid versions
let err = cmd.try_get_matches_from(["mycmd", "--cwd", ".."]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
```

#### Implementations

- `fn suggest_arg(arg: impl Into<Str>) -> Self` — [`Str`](../str/index.md)

- `fn suggest(text: impl Into<StyledStr>) -> Self` — [`StyledStr`](../styled_str/index.md)

- `fn and_suggest(self: Self, text: impl Into<StyledStr>) -> Self` — [`StyledStr`](../styled_str/index.md)

#### Trait Implementations

##### `impl Clone for UnknownArgumentValueParser`

- `fn clone(self: &Self) -> UnknownArgumentValueParser` — [`UnknownArgumentValueParser`](#unknownargumentvalueparser)

##### `impl Debug for UnknownArgumentValueParser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoResettable for UnknownArgumentValueParser`

- `fn into_resettable(self: Self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for UnknownArgumentValueParser`

- `type Value = String`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`ValueSource`](../../parser/matches/value_source/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

## Enums

### `ValueParserInner`

```rust
enum ValueParserInner {
    Bool,
    String,
    OsString,
    PathBuf,
    Other(Box<dyn AnyValueParser>),
}
```

## Traits

### `AnyValueParser`

```rust
trait AnyValueParser: Send + Sync + 'static { ... }
```

A type-erased wrapper for [`TypedValueParser`](#typedvalueparser).

#### Required Methods

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<AnyValue, crate::Error>`

- `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<AnyValue, crate::Error>`

- `fn type_id(self: &Self) -> AnyValueId`

  Describes the content of `AnyValue`

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

- `fn clone_any(self: &Self) -> Box<dyn AnyValueParser>`

### `TypedValueParser`

```rust
trait TypedValueParser: Clone + Send + Sync + 'static { ... }
```

Parse/validate argument values

As alternatives to implementing `TypedValueParser`,
- Use `Fn(&str) -> Result<T, E>` which implements `TypedValueParser`
- `TypedValueParser::map` or `TypedValueParser::try_map` to adapt an existing `TypedValueParser`

See `ValueParserFactory` to register `TypedValueParser::Value` with
`value_parser!`.

# Example

```rust
#[cfg(feature = "error-context")] {
use clap_builder as clap;
use clap::error::ErrorKind;
use clap::error::ContextKind;
use clap::error::ContextValue;
#[derive(Clone)]
struct Custom(u32);

#[derive(Clone)]
struct CustomValueParser;

impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;

        const INVALID_VALUE: u32 = 10;
        if val == INVALID_VALUE {
            let mut err = clap::Error::new(ErrorKind::ValueValidation)
                .with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(ContextKind::InvalidArg, ContextValue::String(arg.to_string()));
            }
            err.insert(ContextKind::InvalidValue, ContextValue::String(INVALID_VALUE.to_string()));
            return Err(err);
        }

        Ok(Custom(val))
    }
}
}
```

#### Required Methods

- `type Value: 3`

- `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

  Reflect on enumerated value properties

- `fn map<T, F>(self: Self, func: F) -> MapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

- `fn try_map<T, E, F>(self: Self, func: F) -> TryMapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

### `ValueParserFactory`

```rust
trait ValueParserFactory { ... }
```

Register a type with [`value_parser!`][crate::value_parser!]

# Example

```rust
use clap_builder as clap;
#[derive(Copy, Clone, Debug)]
pub struct Custom(u32);

impl clap::builder::ValueParserFactory for Custom {
    type Parser = CustomValueParser;
    fn value_parser() -> Self::Parser {
        CustomValueParser
    }
}

#[derive(Clone, Debug)]
pub struct CustomValueParser;
impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Custom(val))
    }
}

let parser: CustomValueParser = clap::value_parser!(Custom);
```

#### Required Methods

- `type Parser`

- `fn value_parser() -> <Self as >::Parser`

  Create the specified `Self::Parser`

