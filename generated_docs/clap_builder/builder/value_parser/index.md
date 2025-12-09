*[clap_builder](../../index.md) / [builder](../index.md) / [value_parser](index.md)*

---

# Module `value_parser`

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`ValueParser`](#valueparser)
  - [`StringValueParser`](#stringvalueparser)
  - [`OsStringValueParser`](#osstringvalueparser)
  - [`PathBufValueParser`](#pathbufvalueparser)
  - [`EnumValueParser`](#enumvalueparser)
  - [`PossibleValuesParser`](#possiblevaluesparser)
  - [`RangedI64ValueParser`](#rangedi64valueparser)
  - [`RangedU64ValueParser`](#rangedu64valueparser)
  - [`BoolValueParser`](#boolvalueparser)
  - [`FalseyValueParser`](#falseyvalueparser)
  - [`BoolishValueParser`](#boolishvalueparser)
  - [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
  - [`MapValueParser`](#mapvalueparser)
  - [`TryMapValueParser`](#trymapvalueparser)
  - [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
- [Enums](#enums)
  - [`ValueParserInner`](#valueparserinner)
- [Traits](#traits)
  - [`AnyValueParser`](#anyvalueparser)
  - [`TypedValueParser`](#typedvalueparser)
  - [`ValueParserFactory`](#valueparserfactory)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`ValueParser`](#valueparser) | struct | Parse/validate argument values |
| [`StringValueParser`](#stringvalueparser) | struct | Implementation for [`ValueParser::string`] |
| [`OsStringValueParser`](#osstringvalueparser) | struct | Implementation for [`ValueParser::os_string`] |
| [`PathBufValueParser`](#pathbufvalueparser) | struct | Implementation for [`ValueParser::path_buf`] |
| [`EnumValueParser`](#enumvalueparser) | struct | Parse an [`ValueEnum`][crate::ValueEnum] value. |
| [`PossibleValuesParser`](#possiblevaluesparser) | struct | Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue]. |
| [`RangedI64ValueParser`](#rangedi64valueparser) | struct | Parse number that fall within a range of values |
| [`RangedU64ValueParser`](#rangedu64valueparser) | struct | Parse number that fall within a range of values |
| [`BoolValueParser`](#boolvalueparser) | struct | Implementation for [`ValueParser::bool`] |
| [`FalseyValueParser`](#falseyvalueparser) | struct | Parse false-like string values, everything else is `true` |
| [`BoolishValueParser`](#boolishvalueparser) | struct | Parse bool-like string values |
| [`NonEmptyStringValueParser`](#nonemptystringvalueparser) | struct | Parse non-empty string values |
| [`MapValueParser`](#mapvalueparser) | struct | Adapt a `TypedValueParser` from one value to another |
| [`TryMapValueParser`](#trymapvalueparser) | struct | Adapt a `TypedValueParser` from one value to another |
| [`UnknownArgumentValueParser`](#unknownargumentvalueparser) | struct | When encountered, report [`ErrorKind::UnknownArgument`][crate::error::ErrorKind::UnknownArgument] |
| [`ValueParserInner`](#valueparserinner) | enum |  |
| [`AnyValueParser`](#anyvalueparser) | trait | A type-erased wrapper for [`TypedValueParser`]. |
| [`TypedValueParser`](#typedvalueparser) | trait | Parse/validate argument values |
| [`ValueParserFactory`](#valueparserfactory) | trait | Register a type with [`value_parser!`][crate::value_parser!] |

## Modules

- [`private`](private/index.md)

## Structs

### `ValueParser`

```rust
struct ValueParser(ValueParserInner);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:63`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L63)*

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

- <span id="valueparser-new"></span>`fn new<P>(other: P) -> Self`

- <span id="valueparser-bool"></span>`const fn bool() -> Self`

- <span id="valueparser-string"></span>`const fn string() -> Self`

- <span id="valueparser-os-string"></span>`const fn os_string() -> Self`

- <span id="valueparser-path-buf"></span>`const fn path_buf() -> Self`

#### Trait Implementations

##### `impl Clone for ValueParser`

- <span id="valueparser-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for ValueParser`

- <span id="valueparser-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>`

##### `impl IntoResettable for ValueParser`

- <span id="valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

### `StringValueParser`

```rust
struct StringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:905`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L905)*

Implementation for `ValueParser::string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="stringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for StringValueParser`

- <span id="stringvalueparser-clone"></span>`fn clone(&self) -> StringValueParser` — [`StringValueParser`](#stringvalueparser)

##### `impl Copy for StringValueParser`

##### `impl Debug for StringValueParser`

- <span id="stringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StringValueParser`

- <span id="stringvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for StringValueParser`

- <span id="stringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for StringValueParser`

- <span id="stringvalueparser-type-value"></span>`type Value = String`

- <span id="stringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="stringvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `OsStringValueParser`

```rust
struct OsStringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:953`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L953)*

Implementation for `ValueParser::os_string`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="osstringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for OsStringValueParser`

- <span id="osstringvalueparser-clone"></span>`fn clone(&self) -> OsStringValueParser` — [`OsStringValueParser`](#osstringvalueparser)

##### `impl Copy for OsStringValueParser`

##### `impl Debug for OsStringValueParser`

- <span id="osstringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for OsStringValueParser`

- <span id="osstringvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for OsStringValueParser`

- <span id="osstringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for OsStringValueParser`

- <span id="osstringvalueparser-type-value"></span>`type Value = OsString`

- <span id="osstringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="osstringvalueparser-parse"></span>`fn parse(&self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `PathBufValueParser`

```rust
struct PathBufValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:995`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L995)*

Implementation for `ValueParser::path_buf`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="pathbufvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for PathBufValueParser`

- <span id="pathbufvalueparser-clone"></span>`fn clone(&self) -> PathBufValueParser` — [`PathBufValueParser`](#pathbufvalueparser)

##### `impl Copy for PathBufValueParser`

##### `impl Debug for PathBufValueParser`

- <span id="pathbufvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PathBufValueParser`

- <span id="pathbufvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for PathBufValueParser`

- <span id="pathbufvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PathBufValueParser`

- <span id="pathbufvalueparser-type-value"></span>`type Value = PathBuf`

- <span id="pathbufvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="pathbufvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>`

```rust
struct EnumValueParser<E: crate::ValueEnum + Clone + Send + Sync + 'static>(std::marker::PhantomData<E>);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1079-1081`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1079-L1081)*

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

- <span id="enumvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<E: clone::Clone + crate::ValueEnum + Clone + Send + Sync + 'static> Clone for EnumValueParser<E>`

- <span id="enumvalueparser-clone"></span>`fn clone(&self) -> EnumValueParser<E>` — [`EnumValueParser`](#enumvalueparser)

##### `impl<E: fmt::Debug + crate::ValueEnum + Clone + Send + Sync + 'static> Debug for EnumValueParser<E>`

- <span id="enumvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> Default for EnumValueParser<E>`

- <span id="enumvalueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for EnumValueParser<E>`

- <span id="enumvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<E: crate::ValueEnum + Clone + Send + Sync + 'static> TypedValueParser for EnumValueParser<E>`

- <span id="enumvalueparser-type-value"></span>`type Value = E`

- <span id="enumvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="enumvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `PossibleValuesParser`

```rust
struct PossibleValuesParser(Vec<super::PossibleValue>);
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1196`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1196)*

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

- <span id="possiblevaluesparser-new"></span>`fn new(values: impl Into<PossibleValuesParser>) -> Self` — [`PossibleValuesParser`](#possiblevaluesparser)

#### Trait Implementations

##### `impl Clone for PossibleValuesParser`

- <span id="possiblevaluesparser-clone"></span>`fn clone(&self) -> PossibleValuesParser` — [`PossibleValuesParser`](#possiblevaluesparser)

##### `impl Debug for PossibleValuesParser`

- <span id="possiblevaluesparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoResettable for PossibleValuesParser`

- <span id="possiblevaluesparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for PossibleValuesParser`

- <span id="possiblevaluesparser-type-value"></span>`type Value = String`

- <span id="possiblevaluesparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="possiblevaluesparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`Error`](../../index.md)

- <span id="possiblevaluesparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync>`

```rust
struct RangedI64ValueParser<T: TryFrom<i64> + Clone + Send + Sync> {
    bounds: (std::ops::Bound<i64>, std::ops::Bound<i64>),
    target: std::marker::PhantomData<T>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1315-1318`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1315-L1318)*

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

- <span id="rangedi64valueparser-new"></span>`fn new() -> Self`

- <span id="rangedi64valueparser-range"></span>`fn range<B: RangeBounds<i64>>(self, range: B) -> Self`

- <span id="rangedi64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T: clone::Clone + TryFrom<i64> + Clone + Send + Sync> Clone for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-clone"></span>`fn clone(&self) -> RangedI64ValueParser<T>` — [`RangedI64ValueParser`](#rangedi64valueparser)

##### `impl<T: marker::Copy + TryFrom<i64> + Clone + Send + Sync> Copy for RangedI64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<i64> + Clone + Send + Sync> Debug for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<i64> + Clone + Send + Sync> Default for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<i64> + Clone + Send + Sync + 'static> TypedValueParser for RangedI64ValueParser<T>`

- <span id="rangedi64valueparser-type-value"></span>`type Value = T`

- <span id="rangedi64valueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `RangedU64ValueParser<T: TryFrom<u64>>`

```rust
struct RangedU64ValueParser<T: TryFrom<u64>> {
    bounds: (std::ops::Bound<u64>, std::ops::Bound<u64>),
    target: std::marker::PhantomData<T>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1514-1517`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1514-L1517)*

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

- <span id="rangedu64valueparser-new"></span>`fn new() -> Self`

- <span id="rangedu64valueparser-range"></span>`fn range<B: RangeBounds<u64>>(self, range: B) -> Self`

- <span id="rangedu64valueparser-format-bounds"></span>`fn format_bounds(&self) -> String`

#### Trait Implementations

##### `impl<T: clone::Clone + TryFrom<u64>> Clone for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-clone"></span>`fn clone(&self) -> RangedU64ValueParser<T>` — [`RangedU64ValueParser`](#rangedu64valueparser)

##### `impl<T: marker::Copy + TryFrom<u64>> Copy for RangedU64ValueParser<T>`

##### `impl<T: fmt::Debug + TryFrom<u64>> Debug for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: TryFrom<u64>> Default for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-default"></span>`fn default() -> Self`

##### `impl<I> IntoResettable for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<T: TryFrom<u64> + Clone + Send + Sync + 'static> TypedValueParser for RangedU64ValueParser<T>`

- <span id="rangedu64valueparser-type-value"></span>`type Value = T`

- <span id="rangedu64valueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `BoolValueParser`

```rust
struct BoolValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1677`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1677)*

Implementation for `ValueParser::bool`

Useful for composing new [`TypedValueParser`](#typedvalueparser)s

#### Implementations

- <span id="boolvalueparser-new"></span>`fn new() -> Self`

- <span id="boolvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolValueParser`

- <span id="boolvalueparser-clone"></span>`fn clone(&self) -> BoolValueParser` — [`BoolValueParser`](#boolvalueparser)

##### `impl Copy for BoolValueParser`

##### `impl Debug for BoolValueParser`

- <span id="boolvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolValueParser`

- <span id="boolvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for BoolValueParser`

- <span id="boolvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolValueParser`

- <span id="boolvalueparser-type-value"></span>`type Value = bool`

- <span id="boolvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="boolvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `FalseyValueParser`

```rust
struct FalseyValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1778`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1778)*

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

- <span id="falseyvalueparser-new"></span>`fn new() -> Self`

- <span id="falseyvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for FalseyValueParser`

- <span id="falseyvalueparser-clone"></span>`fn clone(&self) -> FalseyValueParser` — [`FalseyValueParser`](#falseyvalueparser)

##### `impl Copy for FalseyValueParser`

##### `impl Debug for FalseyValueParser`

- <span id="falseyvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FalseyValueParser`

- <span id="falseyvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for FalseyValueParser`

- <span id="falseyvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for FalseyValueParser`

- <span id="falseyvalueparser-type-value"></span>`type Value = bool`

- <span id="falseyvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="falseyvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `BoolishValueParser`

```rust
struct BoolishValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1877`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1877)*

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

- <span id="boolishvalueparser-new"></span>`fn new() -> Self`

- <span id="boolishvalueparser-possible-values"></span>`fn possible_values() -> impl Iterator<Item = crate::builder::PossibleValue>` — [`PossibleValue`](../possible_value/index.md)

#### Trait Implementations

##### `impl Clone for BoolishValueParser`

- <span id="boolishvalueparser-clone"></span>`fn clone(&self) -> BoolishValueParser` — [`BoolishValueParser`](#boolishvalueparser)

##### `impl Copy for BoolishValueParser`

##### `impl Debug for BoolishValueParser`

- <span id="boolishvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BoolishValueParser`

- <span id="boolishvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for BoolishValueParser`

- <span id="boolishvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for BoolishValueParser`

- <span id="boolishvalueparser-type-value"></span>`type Value = bool`

- <span id="boolishvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="boolishvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `NonEmptyStringValueParser`

```rust
struct NonEmptyStringValueParser {
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:1968`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L1968)*

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

- <span id="nonemptystringvalueparser-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Clone for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-clone"></span>`fn clone(&self) -> NonEmptyStringValueParser` — [`NonEmptyStringValueParser`](#nonemptystringvalueparser)

##### `impl Copy for NonEmptyStringValueParser`

##### `impl Debug for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-default"></span>`fn default() -> Self`

##### `impl IntoResettable for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for NonEmptyStringValueParser`

- <span id="nonemptystringvalueparser-type-value"></span>`type Value = String`

- <span id="nonemptystringvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

### `MapValueParser<P, F>`

```rust
struct MapValueParser<P, F> {
    parser: P,
    func: F,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2014-2017`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L2014-L2017)*

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::map`

#### Implementations

- <span id="mapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: clone::Clone, F: clone::Clone> Clone for MapValueParser<P, F>`

- <span id="mapvalueparser-clone"></span>`fn clone(&self) -> MapValueParser<P, F>` — [`MapValueParser`](#mapvalueparser)

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for MapValueParser<P, F>`

- <span id="mapvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for MapValueParser<P, F>`

- <span id="mapvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<P, F, T> TypedValueParser for MapValueParser<P, F>`

- <span id="mapvalueparser-type-value"></span>`type Value = T`

- <span id="mapvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="mapvalueparser-parse"></span>`fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="mapvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `TryMapValueParser<P, F>`

```rust
struct TryMapValueParser<P, F> {
    parser: P,
    func: F,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2073-2076`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L2073-L2076)*

Adapt a `TypedValueParser` from one value to another

See `TypedValueParser::try_map`

#### Implementations

- <span id="trymapvalueparser-new"></span>`fn new(parser: P, func: F) -> Self`

#### Trait Implementations

##### `impl<P: clone::Clone, F: clone::Clone> Clone for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-clone"></span>`fn clone(&self) -> TryMapValueParser<P, F>` — [`TryMapValueParser`](#trymapvalueparser)

##### `impl<P: fmt::Debug, F: fmt::Debug> Debug for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoResettable for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl<P, F, T, E> TypedValueParser for TryMapValueParser<P, F>`

- <span id="trymapvalueparser-type-value"></span>`type Value = T`

- <span id="trymapvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="trymapvalueparser-possible-values"></span>`fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>` — [`PossibleValue`](../possible_value/index.md)

### `UnknownArgumentValueParser`

```rust
struct UnknownArgumentValueParser {
    arg: Option<crate::builder::Str>,
    suggestions: Vec<crate::builder::StyledStr>,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2159-2162`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L2159-L2162)*

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

- <span id="unknownargumentvalueparser-suggest-arg"></span>`fn suggest_arg(arg: impl Into<Str>) -> Self` — [`Str`](../str/index.md)

- <span id="unknownargumentvalueparser-suggest"></span>`fn suggest(text: impl Into<StyledStr>) -> Self` — [`StyledStr`](../styled_str/index.md)

- <span id="unknownargumentvalueparser-and-suggest"></span>`fn and_suggest(self, text: impl Into<StyledStr>) -> Self` — [`StyledStr`](../styled_str/index.md)

#### Trait Implementations

##### `impl Clone for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-clone"></span>`fn clone(&self) -> UnknownArgumentValueParser` — [`UnknownArgumentValueParser`](#unknownargumentvalueparser)

##### `impl Debug for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoResettable for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueParser>` — [`Resettable`](../resettable/index.md), [`ValueParser`](#valueparser)

##### `impl TypedValueParser for UnknownArgumentValueParser`

- <span id="unknownargumentvalueparser-type-value"></span>`type Value = String`

- <span id="unknownargumentvalueparser-parse-ref"></span>`fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

- <span id="unknownargumentvalueparser-parse-ref"></span>`fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>` — [`Command`](../command/index.md), [`Arg`](../arg/index.md), [`ValueSource`](../../parser/matches/value_source/index.md), [`TypedValueParser`](#typedvalueparser), [`Error`](../../index.md)

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

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:65-75`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L65-L75)*

## Traits

### `AnyValueParser`

```rust
trait AnyValueParser: Send + Sync + 'static { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:591-617`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L591-L617)*

A type-erased wrapper for [`TypedValueParser`](#typedvalueparser).

#### Required Methods

- `fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<AnyValue, crate::Error>`

- `fn type_id(&self) -> AnyValueId`

  Describes the content of `AnyValue`

- `fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

- `fn clone_any(&self) -> Box<dyn AnyValueParser>`

#### Provided Methods

- `fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<AnyValue, crate::Error>`

#### Implementors

- `P`

### `TypedValueParser`

```rust
trait TypedValueParser: Clone + Send + Sync + 'static { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:711-868`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L711-L868)*

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

#### Associated Types

- `type Value: 3`

#### Required Methods

- `fn parse_ref(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

#### Provided Methods

- `fn parse_ref_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn parse_(&self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString, _source: ValueSource) -> Result<<Self as >::Value, crate::Error>`

  Parse the argument value

- `fn possible_values(&self) -> Option<Box<dyn Iterator<Item = crate::builder::PossibleValue>>>`

  Reflect on enumerated value properties

- `fn map<T, F>(self, func: F) -> MapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

- `fn try_map<T, E, F>(self, func: F) -> TryMapValueParser<Self, F>`

  Adapt a `TypedValueParser` from one value to another

#### Implementors

- [`BoolValueParser`](#boolvalueparser)
- [`BoolishValueParser`](#boolishvalueparser)
- [`EnumValueParser`](#enumvalueparser)
- [`FalseyValueParser`](#falseyvalueparser)
- [`MapValueParser`](#mapvalueparser)
- [`NonEmptyStringValueParser`](#nonemptystringvalueparser)
- [`OsStringValueParser`](#osstringvalueparser)
- [`PathBufValueParser`](#pathbufvalueparser)
- [`PossibleValuesParser`](#possiblevaluesparser)
- [`RangedI64ValueParser`](#rangedi64valueparser)
- [`RangedU64ValueParser`](#rangedu64valueparser)
- [`StringValueParser`](#stringvalueparser)
- [`TryMapValueParser`](#trymapvalueparser)
- [`UnknownArgumentValueParser`](#unknownargumentvalueparser)
- `F`

### `ValueParserFactory`

```rust
trait ValueParserFactory { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/value_parser.rs:2276-2285`](../../../../.source_1765210505/clap_builder-4.5.53/src/builder/value_parser.rs#L2276-L2285)*

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

#### Associated Types

- `type Parser`

#### Required Methods

- `fn value_parser() -> <Self as >::Parser`

  Create the specified `Self::Parser`

#### Implementors

- `Box<T>`
- `Box<std::ffi::OsStr>`
- `Box<std::path::Path>`
- `Box<str>`
- `String`
- `bool`
- `i16`
- `i32`
- `i64`
- `i8`
- `std::ffi::OsString`
- `std::num::Saturating<T>`
- `std::num::Wrapping<T>`
- `std::path::PathBuf`
- `std::sync::Arc<T>`
- `u16`
- `u32`
- `u64`
- `u8`

