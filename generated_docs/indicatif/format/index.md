*[indicatif](../index.md) / [format](index.md)*

---

# Module `format`

## Contents

- [Structs](#structs)
  - [`FormattedDuration`](#formattedduration)
  - [`HumanDuration`](#humanduration)
  - [`HumanBytes`](#humanbytes)
  - [`DecimalBytes`](#decimalbytes)
  - [`BinaryBytes`](#binarybytes)
  - [`HumanCount`](#humancount)
  - [`HumanFloatCount`](#humanfloatcount)
- [Constants](#constants)
  - [`SECOND`](#second)
  - [`MINUTE`](#minute)
  - [`HOUR`](#hour)
  - [`DAY`](#day)
  - [`WEEK`](#week)
  - [`YEAR`](#year)
  - [`UNITS`](#units)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FormattedDuration`](#formattedduration) | struct | Wraps an std duration for human basic formatting. |
| [`HumanDuration`](#humanduration) | struct | Wraps an std duration for human readable formatting. |
| [`HumanBytes`](#humanbytes) | struct | Formats bytes for human readability |
| [`DecimalBytes`](#decimalbytes) | struct | Formats bytes for human readability using SI prefixes |
| [`BinaryBytes`](#binarybytes) | struct | Formats bytes for human readability using ISO/IEC prefixes |
| [`HumanCount`](#humancount) | struct | Formats counts for human readability using commas |
| [`HumanFloatCount`](#humanfloatcount) | struct | Formats counts for human readability using commas for floats |
| [`SECOND`](#second) | const |  |
| [`MINUTE`](#minute) | const |  |
| [`HOUR`](#hour) | const |  |
| [`DAY`](#day) | const |  |
| [`WEEK`](#week) | const |  |
| [`YEAR`](#year) | const |  |
| [`UNITS`](#units) | const |  |

## Structs

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:15`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L15)*

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Any for FormattedDuration`

- <span id="formattedduration-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FormattedDuration`

- <span id="formattedduration-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FormattedDuration`

- <span id="formattedduration-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FormattedDuration`

- <span id="formattedduration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FormattedDuration`

- <span id="formattedduration-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FormattedDuration`

- <span id="formattedduration-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FormattedDuration`

- <span id="formattedduration-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for FormattedDuration`

- <span id="formattedduration-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FormattedDuration`

- <span id="formattedduration-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formattedduration-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FormattedDuration`

- <span id="formattedduration-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formattedduration-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:19`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L19)*

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Any for HumanDuration`

- <span id="humanduration-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanDuration`

- <span id="humanduration-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanDuration`

- <span id="humanduration-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanDuration`

- <span id="humanduration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanDuration`

- <span id="humanduration-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanDuration`

- <span id="humanduration-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanDuration`

- <span id="humanduration-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanDuration`

- <span id="humanduration-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanDuration`

- <span id="humanduration-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanduration-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanDuration`

- <span id="humanduration-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanduration-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:34`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L34)*

Formats bytes for human readability

# Examples
```rust
use indicatif::HumanBytes;
assert_eq!("15 B",     format!("{}", HumanBytes(15)));
assert_eq!("1.46 KiB", format!("{}", HumanBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", HumanBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", HumanBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", HumanBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", HumanBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for HumanBytes`

- <span id="humanbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanBytes`

- <span id="humanbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanBytes`

- <span id="humanbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanBytes`

- <span id="humanbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanBytes`

- <span id="humanbytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanBytes`

- <span id="humanbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanBytes`

- <span id="humanbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanBytes`

- <span id="humanbytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanBytes`

- <span id="humanbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanBytes`

- <span id="humanbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:49`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L49)*

Formats bytes for human readability using SI prefixes

# Examples
```rust
use indicatif::DecimalBytes;
assert_eq!("15 B",    format!("{}", DecimalBytes(15)));
assert_eq!("1.50 kB", format!("{}", DecimalBytes(1_500)));
assert_eq!("1.50 MB", format!("{}", DecimalBytes(1_500_000)));
assert_eq!("1.50 GB", format!("{}", DecimalBytes(1_500_000_000)));
assert_eq!("1.50 TB", format!("{}", DecimalBytes(1_500_000_000_000)));
assert_eq!("1.50 PB", format!("{}", DecimalBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for DecimalBytes`

- <span id="decimalbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DecimalBytes`

- <span id="decimalbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DecimalBytes`

- <span id="decimalbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for DecimalBytes`

- <span id="decimalbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecimalBytes`

- <span id="decimalbytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DecimalBytes`

- <span id="decimalbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DecimalBytes`

- <span id="decimalbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for DecimalBytes`

- <span id="decimalbytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for DecimalBytes`

- <span id="decimalbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="decimalbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DecimalBytes`

- <span id="decimalbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="decimalbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:64`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L64)*

Formats bytes for human readability using ISO/IEC prefixes

# Examples
```rust
use indicatif::BinaryBytes;
assert_eq!("15 B",     format!("{}", BinaryBytes(15)));
assert_eq!("1.46 KiB", format!("{}", BinaryBytes(1_500)));
assert_eq!("1.43 MiB", format!("{}", BinaryBytes(1_500_000)));
assert_eq!("1.40 GiB", format!("{}", BinaryBytes(1_500_000_000)));
assert_eq!("1.36 TiB", format!("{}", BinaryBytes(1_500_000_000_000)));
assert_eq!("1.33 PiB", format!("{}", BinaryBytes(1_500_000_000_000_000)));
```

#### Trait Implementations

##### `impl Any for BinaryBytes`

- <span id="binarybytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BinaryBytes`

- <span id="binarybytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BinaryBytes`

- <span id="binarybytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for BinaryBytes`

- <span id="binarybytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryBytes`

- <span id="binarybytes-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for BinaryBytes`

- <span id="binarybytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BinaryBytes`

- <span id="binarybytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for BinaryBytes`

- <span id="binarybytes-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BinaryBytes`

- <span id="binarybytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="binarybytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BinaryBytes`

- <span id="binarybytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="binarybytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanCount`

```rust
struct HumanCount(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:68`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L68)*

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Any for HumanCount`

- <span id="humancount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanCount`

- <span id="humancount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanCount`

- <span id="humancount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanCount`

- <span id="humancount-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanCount`

- <span id="humancount-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanCount`

- <span id="humancount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanCount`

- <span id="humancount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanCount`

- <span id="humancount-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanCount`

- <span id="humancount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humancount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanCount`

- <span id="humancount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humancount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:72`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L72)*

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Any for HumanFloatCount`

- <span id="humanfloatcount-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for HumanFloatCount`

- <span id="humanfloatcount-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for HumanFloatCount`

- <span id="humanfloatcount-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for HumanFloatCount`

- <span id="humanfloatcount-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanFloatCount`

- <span id="humanfloatcount-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for HumanFloatCount`

- <span id="humanfloatcount-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for HumanFloatCount`

- <span id="humanfloatcount-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for HumanFloatCount`

- <span id="humanfloatcount-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for HumanFloatCount`

- <span id="humanfloatcount-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="humanfloatcount-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for HumanFloatCount`

- <span id="humanfloatcount-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="humanfloatcount-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `SECOND`
```rust
const SECOND: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:6`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L6)*

### `MINUTE`
```rust
const MINUTE: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:7`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L7)*

### `HOUR`
```rust
const HOUR: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:8`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L8)*

### `DAY`
```rust
const DAY: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:9`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L9)*

### `WEEK`
```rust
const WEEK: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:10`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L10)*

### `YEAR`
```rust
const YEAR: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:11`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L11)*

### `UNITS`
```rust
const UNITS: &[(std::time::Duration, &str, &str)];
```

*Defined in [`indicatif-0.18.3/src/format.rs:133-140`](../../../.source_1765633015/indicatif-0.18.3/src/format.rs#L133-L140)*

