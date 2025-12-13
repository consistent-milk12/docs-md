# Crate `unit_prefix`

This is a library for formatting numbers with numeric prefixes, such as
turning “3000 metres” into “3 kilometres”, or “8705 bytes” into “8.5 KiB”.


# Usage

The function [`NumberPrefix::decimal`](#numberprefix-decimal)
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
[`NumberPrefix::binary`](#numberprefix-binary) function.
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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse`](#parse) | mod |  |
| [`Prefix`](#prefix) | enum | A numeric prefix, either binary or decimal. |
| [`NumberPrefix`](#numberprefix) | enum | The result of trying to apply a prefix to a floating-point value. |
| [`Amounts`](#amounts) | trait | Traits for floating-point values for both the possible multipliers. |

## Modules

- [`parse`](parse/index.md)

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

*Defined in [`unit-prefix-0.5.2/src/lib.rs:171-235`](../../.source_1765521767/unit-prefix-0.5.2/src/lib.rs#L171-L235)*

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

- <span id="prefix-upper"></span>`fn upper(self) -> &'static str`

  Returns the name in uppercase, such as “KILO”.

  

  # Examples

  

  ```rust

  use unit_prefix::Prefix;

  

  assert_eq!("GIGA", Prefix::Giga.upper());

  assert_eq!("GIBI", Prefix::Gibi.upper());

  ```

- <span id="prefix-caps"></span>`fn caps(self) -> &'static str`

  Returns the name with the first letter capitalised, such as “Mega”.

  

  # Examples

  

  ```rust

  use unit_prefix::Prefix;

  

  assert_eq!("Giga", Prefix::Giga.caps());

  assert_eq!("Gibi", Prefix::Gibi.caps());

  ```

- <span id="prefix-lower"></span>`fn lower(self) -> &'static str`

  Returns the name in lowercase, such as “giga”.

  

  # Examples

  

  ```rust

  use unit_prefix::Prefix;

  

  assert_eq!("giga", Prefix::Giga.lower());

  assert_eq!("gibi", Prefix::Gibi.lower());

  ```

- <span id="prefix-symbol"></span>`fn symbol(self) -> &'static str`

  Returns the short-hand symbol, such as “T” (for “tera”).

  

  # Examples

  

  ```rust

  use unit_prefix::Prefix;

  

  assert_eq!("G", Prefix::Giga.symbol());

  assert_eq!("Gi", Prefix::Gibi.symbol());

  ```

#### Trait Implementations

##### `impl Any for Prefix`

- <span id="prefix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Prefix`

- <span id="prefix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Prefix`

- <span id="prefix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Prefix`

- <span id="prefix-clone"></span>`fn clone(&self) -> Prefix` — [`Prefix`](#prefix)

##### `impl CloneToUninit for Prefix`

- <span id="prefix-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Prefix`

##### `impl Debug for Prefix`

- <span id="prefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Prefix`

- <span id="prefix-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Prefix`

##### `impl<T> From for Prefix`

- <span id="prefix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Prefix`

- <span id="prefix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Prefix`

- <span id="prefix-partialeq-eq"></span>`fn eq(&self, other: &Prefix) -> bool` — [`Prefix`](#prefix)

##### `impl StructuralPartialEq for Prefix`

##### `impl ToOwned for Prefix`

- <span id="prefix-toowned-type-owned"></span>`type Owned = T`

- <span id="prefix-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefix-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Prefix`

- <span id="prefix-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Prefix`

- <span id="prefix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Prefix`

- <span id="prefix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NumberPrefix<F>`

```rust
enum NumberPrefix<F> {
    Standalone(F),
    Prefixed(Prefix, F),
}
```

*Defined in [`unit-prefix-0.5.2/src/lib.rs:240-249`](../../.source_1765521767/unit-prefix-0.5.2/src/lib.rs#L240-L249)*

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

- <span id="numberprefix-decimal"></span>`fn decimal(amount: F) -> Self`

  Formats the given floating-point number using **decimal** prefixes.

  

  This function accepts both `f32` and `f64` values. If you’re trying to

  format an integer, you’ll have to cast it first.

  

  # Examples

  

  ```rust

  use unit_prefix::{NumberPrefix, Prefix};

  

  assert_eq!(

      NumberPrefix::decimal(1_000_000_000_f32),

      NumberPrefix::Prefixed(Prefix::Giga, 1_f32)

  );

  ```

- <span id="numberprefix-binary"></span>`fn binary(amount: F) -> Self`

  Formats the given floating-point number using **binary** prefixes.

  

  This function accepts both `f32` and `f64` values. If you’re trying to

  format an integer, you’ll have to cast it first.

  

  # Examples

  

  ```rust

  use unit_prefix::{NumberPrefix, Prefix};

  

  assert_eq!(

      NumberPrefix::binary(1_073_741_824_f64),

      NumberPrefix::Prefixed(Prefix::Gibi, 1_f64)

  );

  ```

- <span id="numberprefix-format-number"></span>`fn format_number(amount: F, kilo: F, prefixes: [Prefix; 8]) -> Self` — [`Prefix`](#prefix)

#### Trait Implementations

##### `impl Any for NumberPrefix<F>`

- <span id="numberprefix-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NumberPrefix<F>`

- <span id="numberprefix-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NumberPrefix<F>`

- <span id="numberprefix-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<F: clone::Clone> Clone for NumberPrefix<F>`

- <span id="numberprefix-clone"></span>`fn clone(&self) -> NumberPrefix<F>` — [`NumberPrefix`](#numberprefix)

##### `impl CloneToUninit for NumberPrefix<F>`

- <span id="numberprefix-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<F: fmt::Debug> Debug for NumberPrefix<F>`

- <span id="numberprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<F: cmp::Eq> Eq for NumberPrefix<F>`

##### `impl<T> From for NumberPrefix<F>`

- <span id="numberprefix-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: str::FromStr> FromStr for super::NumberPrefix<T>`

- <span id="supernumberprefix-fromstr-type-err"></span>`type Err = NumberPrefixParseError`

- <span id="supernumberprefix-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl<U> Into for NumberPrefix<F>`

- <span id="numberprefix-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<F: cmp::PartialEq> PartialEq for NumberPrefix<F>`

- <span id="numberprefix-partialeq-eq"></span>`fn eq(&self, other: &NumberPrefix<F>) -> bool` — [`NumberPrefix`](#numberprefix)

##### `impl<F> StructuralPartialEq for NumberPrefix<F>`

##### `impl ToOwned for NumberPrefix<F>`

- <span id="numberprefix-toowned-type-owned"></span>`type Owned = T`

- <span id="numberprefix-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="numberprefix-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NumberPrefix<F>`

- <span id="numberprefix-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="numberprefix-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NumberPrefix<F>`

- <span id="numberprefix-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="numberprefix-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Amounts`

```rust
trait Amounts: Copy + Sized + PartialOrd + Div<Output = Self> + Neg<Output = Self> { ... }
```

*Defined in [`unit-prefix-0.5.2/src/lib.rs:467-477`](../../.source_1765521767/unit-prefix-0.5.2/src/lib.rs#L467-L477)*

Traits for floating-point values for both the possible multipliers. They
need to be Copy, have defined 1000 and 1024s, and implement a bunch of
operators.

#### Associated Constants

- `const NUM_1000: Self`

- `const NUM_1024: Self`

#### Required Methods

- `fn is_negative(self) -> bool`

  Whether this number is negative.

#### Implementors

- `f32`
- `f64`

