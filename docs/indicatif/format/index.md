*[indicatif](../index.md) / [format](index.md)*

---

# Module `format`

## Structs

### `FormattedDuration`

```rust
struct FormattedDuration(std::time::Duration);
```

Wraps an std duration for human basic formatting.

#### Trait Implementations

##### `impl Debug for FormattedDuration`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for FormattedDuration`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for FormattedDuration`

- `fn to_string(self: &Self) -> String`

### `HumanDuration`

```rust
struct HumanDuration(std::time::Duration);
```

Wraps an std duration for human readable formatting.

#### Trait Implementations

##### `impl Debug for HumanDuration`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanDuration`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanDuration`

- `fn to_string(self: &Self) -> String`

### `HumanBytes`

```rust
struct HumanBytes(u64);
```

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanBytes`

- `fn to_string(self: &Self) -> String`

### `DecimalBytes`

```rust
struct DecimalBytes(u64);
```

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for DecimalBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for DecimalBytes`

- `fn to_string(self: &Self) -> String`

### `BinaryBytes`

```rust
struct BinaryBytes(u64);
```

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

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BinaryBytes`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for BinaryBytes`

- `fn to_string(self: &Self) -> String`

### `HumanCount`

```rust
struct HumanCount(u64);
```

Formats counts for human readability using commas

#### Trait Implementations

##### `impl Debug for HumanCount`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanCount`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanCount`

- `fn to_string(self: &Self) -> String`

### `HumanFloatCount`

```rust
struct HumanFloatCount(f64);
```

Formats counts for human readability using commas for floats

#### Trait Implementations

##### `impl Debug for HumanFloatCount`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for HumanFloatCount`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for HumanFloatCount`

- `fn to_string(self: &Self) -> String`

## Constants

### `SECOND`

```rust
const SECOND: std::time::Duration;
```

### `MINUTE`

```rust
const MINUTE: std::time::Duration;
```

### `HOUR`

```rust
const HOUR: std::time::Duration;
```

### `DAY`

```rust
const DAY: std::time::Duration;
```

### `WEEK`

```rust
const WEEK: std::time::Duration;
```

### `YEAR`

```rust
const YEAR: std::time::Duration;
```

### `UNITS`

```rust
const UNITS: &[(std::time::Duration, &str, &str)];
```

