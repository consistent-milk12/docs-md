# Crate `unit_prefix`

This is a library for formatting numbers with numeric prefixes, such as
turning “3000 metres” into “3 kilometres”, or “8705 bytes” into “8.5 KiB”.


# Usage

The function `NumberPrefix::decimal`
returns either a pair of the resulting number and its prefix, or a
notice that the number was too small to have any prefix applied to it. For
example:

```rust
use unit_prefix::NumberPrefix;

let amount = 8542_f32;
let result = match NumberPrefix::decimal(amount) {
    NumberPrefix::Standalone(bytes) => {
        format!("The file is {} bytes in size", bytes)
    },
    NumberPrefix::Prefixed(prefix, n) => {
        format!("The file is {:.1} {}B in size", n, prefix)
    },
};

assert_eq!("The file is 8.5 kB in size", result);
```

The `{:.1}` part of the formatting string tells it to restrict the
output to only one decimal place. This value is calculated by repeatedly
dividing the number by 1000 until it becomes less than that, which in this
case results in 8.542, which gets rounded down. Because only one division
had to take place, the function also returns the decimal prefix `Kilo`,
which gets converted to its internationally-recognised symbol when
formatted as a string.

If the value is too small to have any prefixes applied to it — in this case,
if it’s under 1000 — then the standalone value will be returned:

```rust
use unit_prefix::NumberPrefix;

let amount = 705_f32;
let result = match NumberPrefix::decimal(amount) {
    NumberPrefix::Standalone(bytes) => {
        format!("The file is {} bytes in size", bytes)
    },
    NumberPrefix::Prefixed(prefix, n) => {
        format!("The file is {:.1} {}B in size", n, prefix)
    },
};

assert_eq!("The file is 705 bytes in size", result);
```

In this particular example, the user expects different formatting for
both bytes and kilobytes: while prefixed values are given more precision,
there’s no point using anything other than whole numbers for just byte
amounts. This is why the function pays attention to values without any
prefixes — they often need to be special-cased.


## Binary Prefixes

This library also allows you to use the *binary prefixes*, which use the
number 1024 (2<sup>10</sup>) as the multiplier, rather than the more common 1000
(10<sup>3</sup>). This uses the
`NumberPrefix::binary` function.
For example:

```rust
use unit_prefix::NumberPrefix;

let amount = 8542_f32;
let result = match NumberPrefix::binary(amount) {
    NumberPrefix::Standalone(bytes) => {
        format!("The file is {} bytes in size", bytes)
    },
    NumberPrefix::Prefixed(prefix, n) => {
        format!("The file is {:.1} {}B in size", n, prefix)
    },
};

assert_eq!("The file is 8.3 KiB in size", result);
```

A kibibyte is slightly larger than a kilobyte, so the number is smaller
in the result; but other than that, it works in exactly the same way, with
the binary prefix being converted to a symbol automatically.


## Which type of prefix should I use?

There is no correct answer this question! Common practice is to use
the binary prefixes for numbers of *bytes*, while still using the decimal
prefixes for everything else. Computers work with powers of two, rather than
powers of ten, and by using the binary prefixes, you get a more accurate
representation of the amount of data.


## Prefix Names

If you need to describe your unit in actual words, rather than just with the
symbol, use one of the `upper`, `caps`, `lower`, or `symbol`, which output the
prefix in a variety of formats. For example:

```rust
use unit_prefix::NumberPrefix;

let amount = 8542_f32;
let result = match NumberPrefix::decimal(amount) {
    NumberPrefix::Standalone(bytes) => {
        format!("The file is {} bytes in size", bytes)
    },
    NumberPrefix::Prefixed(prefix, n) => {
        format!("The file is {:.1} {}bytes in size", n, prefix.lower())
    },
};

assert_eq!("The file is 8.5 kilobytes in size", result);
```


## String Parsing

There is a `FromStr` implementation for `NumberPrefix` that parses
strings containing numbers and trailing prefixes, such as `7.5E`.

Currently, the only supported units are `b` and `B` for bytes, and `m` for
metres. Whitespace is allowed between the number and the rest of the string.

```rust
use unit_prefix::{NumberPrefix, Prefix};

assert_eq!(
    "7.05E".parse::<NumberPrefix<_>>(),
    Ok(NumberPrefix::Prefixed(Prefix::Exa, 7.05_f64))
);

assert_eq!(
    "7.05".parse::<NumberPrefix<_>>(),
    Ok(NumberPrefix::Standalone(7.05_f64))
);

assert_eq!(
    "7.05 GiB".parse::<NumberPrefix<_>>(),
    Ok(NumberPrefix::Prefixed(Prefix::Gibi, 7.05_f64))
);
```

## Enums

### `Prefix`

```rust
enum Prefix {
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi,
    Exbi,
    Zebi,
    Yobi,
}
```

A numeric prefix, either binary or decimal.

#### Variants

- **`Kilo`**

  _kilo_, 10<sup>3</sup> or 1000<sup>1</sup>.
  From the Greek ‘χίλιοι’ (‘chilioi’), meaning ‘thousand’.

- **`Mega`**

  _mega_, 10<sup>6</sup> or 1000<sup>2</sup>.
  From the Ancient Greek ‘μέγας’ (‘megas’), meaning ‘great’.

- **`Giga`**

  _giga_, 10<sup>9</sup> or 1000<sup>3</sup>.
  From the Greek ‘γίγας’ (‘gigas’), meaning ‘giant’.

- **`Tera`**

  _tera_, 10<sup>12</sup> or 1000<sup>4</sup>.
  From the Greek ‘τέρας’ (‘teras’), meaning ‘monster’.

- **`Peta`**

  _peta_, 10<sup>15</sup> or 1000<sup>5</sup>.
  From the Greek ‘πέντε’ (‘pente’), meaning ‘five’.

- **`Exa`**

  _exa_, 10<sup>18</sup> or 1000<sup>6</sup>.
  From the Greek ‘ἕξ’ (‘hex’), meaning ‘six’.

- **`Zetta`**

  _zetta_, 10<sup>21</sup> or 1000<sup>7</sup>.
  From the Latin ‘septem’, meaning ‘seven’.

- **`Yotta`**

  _yotta_, 10<sup>24</sup> or 1000<sup>8</sup>.
  From the Green ‘οκτώ’ (‘okto’), meaning ‘eight’.

- **`Kibi`**

  _kibi_, 2<sup>10</sup> or 1024<sup>1</sup>.
  The binary version of _kilo_.

- **`Mebi`**

  _mebi_, 2<sup>20</sup> or 1024<sup>2</sup>.
  The binary version of _mega_.

- **`Gibi`**

  _gibi_, 2<sup>30</sup> or 1024<sup>3</sup>.
  The binary version of _giga_.

- **`Tebi`**

  _tebi_, 2<sup>40</sup> or 1024<sup>4</sup>.
  The binary version of _tera_.

- **`Pebi`**

  _pebi_, 2<sup>50</sup> or 1024<sup>5</sup>.
  The binary version of _peta_.

- **`Exbi`**

  _exbi_, 2<sup>60</sup> or 1024<sup>6</sup>.
  The binary version of _exa_.

- **`Zebi`**

  _zebi_, 2<sup>70</sup> or 1024<sup>7</sup>.
  The binary version of _zetta_.

- **`Yobi`**

  _yobi_, 2<sup>80</sup> or 1024<sup>8</sup>.
  The binary version of _yotta_.

#### Implementations

- `fn upper(self: Self) -> &'static str`

- `fn caps(self: Self) -> &'static str`

- `fn lower(self: Self) -> &'static str`

- `fn symbol(self: Self) -> &'static str`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Prefix` — [`Prefix`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Prefix) -> bool` — [`Prefix`](../index.md)

##### `impl StructuralPartialEq`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `NumberPrefix<F>`

```rust
enum NumberPrefix<F> {
    Standalone(F),
    Prefixed(Prefix, F),
}
```

The result of trying to apply a prefix to a floating-point value.

#### Variants

- **`Standalone`**

  A **standalone** value is returned when the number is too small to
  have any prefixes applied to it. This is commonly a special case, so
  is handled separately.

- **`Prefixed`**

  A **prefixed** value *is* large enough for prefixes. This holds the
  prefix, as well as the resulting value.

#### Implementations

- `fn decimal(amount: F) -> Self`

- `fn binary(amount: F) -> Self`

- `fn format_number(amount: F, kilo: F, prefixes: [Prefix; 8]) -> Self` — [`Prefix`](../index.md)

#### Trait Implementations

##### `impl Clone<F: $crate::clone::Clone>`

- `fn clone(self: &Self) -> NumberPrefix<F>` — [`NumberPrefix`](../index.md)

##### `impl Debug<F: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<F: $crate::cmp::Eq>`

##### `impl FromStr<T: str::FromStr>`

- `type Err = NumberPrefixParseError`

- `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl PartialEq<F: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &NumberPrefix<F>) -> bool` — [`NumberPrefix`](../index.md)

##### `impl StructuralPartialEq<F>`

## Traits

### `Amounts`

```rust
trait Amounts: Copy + Sized + PartialOrd + Div<Output = Self> + Neg<Output = Self> { ... }
```

Traits for floating-point values for both the possible multipliers. They
need to be Copy, have defined 1000 and 1024s, and implement a bunch of
operators.

#### Required Methods

- `const NUM_1000: Self`

- `const NUM_1024: Self`

- `fn is_negative(self: Self) -> bool`

  Whether this number is negative.

