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

*Defined in [`indicatif-0.18.3/src/format.rs:15`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L15)*

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Debug for FormattedDuration`

- <span id="formattedduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FormattedDuration`

- <span id="formattedduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for FormattedDuration`

- <span id="formattedduration-to-string"></span>`fn to_string(&self) -> String`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

*Defined in [`indicatif-0.18.3/src/format.rs:19`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L19)*

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Debug for HumanDuration`

- <span id="humanduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanDuration`

- <span id="humanduration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanDuration`

- <span id="humanduration-to-string"></span>`fn to_string(&self) -> String`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:34`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L34)*

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

##### `impl Debug for HumanBytes`

- <span id="humanbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanBytes`

- <span id="humanbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanBytes`

- <span id="humanbytes-to-string"></span>`fn to_string(&self) -> String`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:49`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L49)*

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

##### `impl Debug for DecimalBytes`

- <span id="decimalbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for DecimalBytes`

- <span id="decimalbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for DecimalBytes`

- <span id="decimalbytes-to-string"></span>`fn to_string(&self) -> String`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:64`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L64)*

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

##### `impl Debug for BinaryBytes`

- <span id="binarybytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BinaryBytes`

- <span id="binarybytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for BinaryBytes`

- <span id="binarybytes-to-string"></span>`fn to_string(&self) -> String`

### `HumanCount`

```rust
struct HumanCount(u64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:68`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L68)*

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Debug for HumanCount`

- <span id="humancount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanCount`

- <span id="humancount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanCount`

- <span id="humancount-to-string"></span>`fn to_string(&self) -> String`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

*Defined in [`indicatif-0.18.3/src/format.rs:72`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L72)*

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Debug for HumanFloatCount`

- <span id="humanfloatcount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for HumanFloatCount`

- <span id="humanfloatcount-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for HumanFloatCount`

- <span id="humanfloatcount-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `SECOND`
```rust
const SECOND: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:6`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L6)*

### `MINUTE`
```rust
const MINUTE: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:7`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L7)*

### `HOUR`
```rust
const HOUR: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:8`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L8)*

### `DAY`
```rust
const DAY: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:9`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L9)*

### `WEEK`
```rust
const WEEK: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:10`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L10)*

### `YEAR`
```rust
const YEAR: std::time::Duration;
```

*Defined in [`indicatif-0.18.3/src/format.rs:11`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L11)*

### `UNITS`
```rust
const UNITS: &[(std::time::Duration, &str, &str)];
```

*Defined in [`indicatif-0.18.3/src/format.rs:133-140`](../../../.source_1765521767/indicatif-0.18.3/src/format.rs#L133-L140)*

