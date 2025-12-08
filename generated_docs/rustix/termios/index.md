*[rustix](../index.md) / [termios](index.md)*

---

# Module `termios`

Terminal I/O stream operations.

This API automatically supports setting arbitrary I/O speeds, on any
platform that supports them, including Linux and the BSDs.

The [`speed`](#speed) module contains various predefined speed constants which are
more likely to be portable, however any `u32` value can be passed to
`Termios::set_speed`, `Termios::set_input_speed`, and
`Termios::set_output_speed`, and they will simply fail if the speed is
not supported by the platform or the device.

## Modules

- [`ioctl`](ioctl/index.md) - Terminal-related `ioctl` functions.
- [`tc`](tc/index.md) - 
- [`tty`](tty/index.md) - Functions which operate on file descriptors which might be terminals.
- [`types`](types/index.md) - 
- [`speed`](speed/index.md) - Speeds for use with [`Termios::set_speed`], [`Termios::set_input_speed`],

## Structs

### `Termios`

```rust
struct Termios {
    pub input_modes: InputModes,
    pub output_modes: OutputModes,
    pub control_modes: ControlModes,
    pub local_modes: LocalModes,
    pub line_discipline: u8,
    pub special_codes: SpecialCodes,
    input_speed: linux_raw_sys::general::speed_t,
    output_speed: linux_raw_sys::general::speed_t,
}
```

`struct termios` for use with [`tcgetattr`](#tcgetattr) and [`tcsetattr`](#tcsetattr).



#### Fields

- **`input_modes`**: `InputModes`

  How is input interpreted?

- **`output_modes`**: `OutputModes`

  How is output translated?

- **`control_modes`**: `ControlModes`

  Low-level configuration flags.

- **`local_modes`**: `LocalModes`

  High-level configuration flags.

- **`line_discipline`**: `u8`

  Line discipline.

- **`special_codes`**: `SpecialCodes`

  How are various special control codes handled?

- **`input_speed`**: `linux_raw_sys::general::speed_t`

  See the `input_speed` and `set_input_seed` functions.
  
  On Linux and BSDs, this is the arbitrary integer speed value. On all
  other platforms, this is the encoded speed value.

- **`output_speed`**: `linux_raw_sys::general::speed_t`

  See the `output_speed` and `set_output_seed` functions.
  
  On Linux and BSDs, this is the integer speed value. On all other
  platforms, this is the encoded speed value.

#### Implementations

- `fn make_raw(self: &mut Self)`

- `fn input_speed(self: &Self) -> u32`

- `fn output_speed(self: &Self) -> u32`

- `fn set_speed(self: &mut Self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/index.md)

- `fn set_input_speed(self: &mut Self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/index.md)

- `fn set_output_speed(self: &mut Self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/index.md)

#### Trait Implementations

##### `impl Clone for Termios`

- `fn clone(self: &Self) -> Termios` — [`Termios`](#termios)

##### `impl Debug for Termios`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `InputModes`

```rust
struct InputModes(<InputModes as $crate::__private::PublicFlags>::Internal);
```

Flags controlling terminal input.

#### Implementations

- `const IGNBRK: Self`

- `const BRKINT: Self`

- `const IGNPAR: Self`

- `const PARMRK: Self`

- `const INPCK: Self`

- `const ISTRIP: Self`

- `const INLCR: Self`

- `const IGNCR: Self`

- `const ICRNL: Self`

- `const IUCLC: Self`

- `const IXON: Self`

- `const IXANY: Self`

- `const IXOFF: Self`

- `const IMAXBEL: Self`

- `const IUTF8: Self`

#### Trait Implementations

##### `impl Binary for InputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for InputModes`

- `type Output = InputModes`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for InputModes`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for InputModes`

- `type Output = InputModes`

- `fn bitor(self: Self, other: InputModes) -> Self` — [`InputModes`](#inputmodes)

##### `impl BitOrAssign for InputModes`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for InputModes`

- `type Output = InputModes`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for InputModes`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for InputModes`

- `fn clone(self: &Self) -> InputModes` — [`InputModes`](#inputmodes)

##### `impl Copy for InputModes`

##### `impl Debug for InputModes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for InputModes`

##### `impl Extend for InputModes`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for InputModes`

- `const FLAGS: &'static [$crate::Flag<InputModes>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> InputModes` — [`c_uint`](../ffi/index.md), [`InputModes`](#inputmodes)

##### `impl FromIterator for InputModes`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for InputModes`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for InputModes`

- `type Item = InputModes`

- `type IntoIter = Iter<InputModes>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for InputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for InputModes`

- `type Output = InputModes`

- `fn not(self: Self) -> Self`

##### `impl Octal for InputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for InputModes`

- `fn eq(self: &Self, other: &InputModes) -> bool` — [`InputModes`](#inputmodes)

##### `impl PublicFlags for InputModes`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for InputModes`

##### `impl Sub for InputModes`

- `type Output = InputModes`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for InputModes`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for InputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

### `OutputModes`

```rust
struct OutputModes(<OutputModes as $crate::__private::PublicFlags>::Internal);
```

Flags controlling terminal output.

#### Implementations

- `const fn iter(self: &Self) -> $crate::iter::Iter<OutputModes>` — [`OutputModes`](#outputmodes)

- `const fn iter_names(self: &Self) -> $crate::iter::IterNames<OutputModes>` — [`OutputModes`](#outputmodes)

#### Trait Implementations

##### `impl Binary for OutputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for OutputModes`

- `type Output = OutputModes`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for OutputModes`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for OutputModes`

- `type Output = OutputModes`

- `fn bitor(self: Self, other: OutputModes) -> Self` — [`OutputModes`](#outputmodes)

##### `impl BitOrAssign for OutputModes`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for OutputModes`

- `type Output = OutputModes`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for OutputModes`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for OutputModes`

- `fn clone(self: &Self) -> OutputModes` — [`OutputModes`](#outputmodes)

##### `impl Copy for OutputModes`

##### `impl Debug for OutputModes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for OutputModes`

##### `impl Extend for OutputModes`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for OutputModes`

- `const FLAGS: &'static [$crate::Flag<OutputModes>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> OutputModes` — [`c_uint`](../ffi/index.md), [`OutputModes`](#outputmodes)

##### `impl FromIterator for OutputModes`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for OutputModes`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for OutputModes`

- `type Item = OutputModes`

- `type IntoIter = Iter<OutputModes>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for OutputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for OutputModes`

- `type Output = OutputModes`

- `fn not(self: Self) -> Self`

##### `impl Octal for OutputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for OutputModes`

- `fn eq(self: &Self, other: &OutputModes) -> bool` — [`OutputModes`](#outputmodes)

##### `impl PublicFlags for OutputModes`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for OutputModes`

##### `impl Sub for OutputModes`

- `type Output = OutputModes`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for OutputModes`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for OutputModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

### `ControlModes`

```rust
struct ControlModes(<ControlModes as $crate::__private::PublicFlags>::Internal);
```

Flags controlling special terminal modes.

`CBAUD`, `CBAUDEX`, `CIBAUD`, `CIBAUDEX`, and various `B*` speed
constants are often included in the control modes, however rustix
handles them separately, in `Termios::set_speed` and related
functions. If you see extra bits in the `Debug` output, they're
probably these flags.

#### Implementations

- `const CSIZE: Self`

- `const CS5: Self`

- `const CS6: Self`

- `const CS7: Self`

- `const CS8: Self`

- `const CSTOPB: Self`

- `const CREAD: Self`

- `const PARENB: Self`

- `const PARODD: Self`

- `const HUPCL: Self`

- `const CLOCAL: Self`

- `const CRTSCTS: Self`

- `const CMSPAR: Self`

#### Trait Implementations

##### `impl Binary for ControlModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for ControlModes`

- `type Output = ControlModes`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for ControlModes`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for ControlModes`

- `type Output = ControlModes`

- `fn bitor(self: Self, other: ControlModes) -> Self` — [`ControlModes`](#controlmodes)

##### `impl BitOrAssign for ControlModes`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for ControlModes`

- `type Output = ControlModes`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for ControlModes`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for ControlModes`

- `fn clone(self: &Self) -> ControlModes` — [`ControlModes`](#controlmodes)

##### `impl Copy for ControlModes`

##### `impl Debug for ControlModes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for ControlModes`

##### `impl Extend for ControlModes`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for ControlModes`

- `const FLAGS: &'static [$crate::Flag<ControlModes>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> ControlModes` — [`c_uint`](../ffi/index.md), [`ControlModes`](#controlmodes)

##### `impl FromIterator for ControlModes`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for ControlModes`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for ControlModes`

- `type Item = ControlModes`

- `type IntoIter = Iter<ControlModes>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for ControlModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for ControlModes`

- `type Output = ControlModes`

- `fn not(self: Self) -> Self`

##### `impl Octal for ControlModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for ControlModes`

- `fn eq(self: &Self, other: &ControlModes) -> bool` — [`ControlModes`](#controlmodes)

##### `impl PublicFlags for ControlModes`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ControlModes`

##### `impl Sub for ControlModes`

- `type Output = ControlModes`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for ControlModes`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for ControlModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

### `LocalModes`

```rust
struct LocalModes(<LocalModes as $crate::__private::PublicFlags>::Internal);
```

Flags controlling “local” terminal modes.

#### Implementations

- `const fn iter(self: &Self) -> $crate::iter::Iter<LocalModes>` — [`LocalModes`](#localmodes)

- `const fn iter_names(self: &Self) -> $crate::iter::IterNames<LocalModes>` — [`LocalModes`](#localmodes)

#### Trait Implementations

##### `impl Binary for LocalModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl BitAnd for LocalModes`

- `type Output = LocalModes`

- `fn bitand(self: Self, other: Self) -> Self`

##### `impl BitAndAssign for LocalModes`

- `fn bitand_assign(self: &mut Self, other: Self)`

##### `impl BitOr for LocalModes`

- `type Output = LocalModes`

- `fn bitor(self: Self, other: LocalModes) -> Self` — [`LocalModes`](#localmodes)

##### `impl BitOrAssign for LocalModes`

- `fn bitor_assign(self: &mut Self, other: Self)`

##### `impl BitXor for LocalModes`

- `type Output = LocalModes`

- `fn bitxor(self: Self, other: Self) -> Self`

##### `impl BitXorAssign for LocalModes`

- `fn bitxor_assign(self: &mut Self, other: Self)`

##### `impl Clone for LocalModes`

- `fn clone(self: &Self) -> LocalModes` — [`LocalModes`](#localmodes)

##### `impl Copy for LocalModes`

##### `impl Debug for LocalModes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for LocalModes`

##### `impl Extend for LocalModes`

- `fn extend<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T)`

##### `impl Flags for LocalModes`

- `const FLAGS: &'static [$crate::Flag<LocalModes>]`

- `type Bits = u32`

- `fn bits(self: &Self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md)

- `fn from_bits_retain(bits: ffi::c_uint) -> LocalModes` — [`c_uint`](../ffi/index.md), [`LocalModes`](#localmodes)

##### `impl FromIterator for LocalModes`

- `fn from_iter<T: $crate::__private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for LocalModes`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl IntoIterator for LocalModes`

- `type Item = LocalModes`

- `type IntoIter = Iter<LocalModes>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl LowerHex for LocalModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl Not for LocalModes`

- `type Output = LocalModes`

- `fn not(self: Self) -> Self`

##### `impl Octal for LocalModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

##### `impl PartialEq for LocalModes`

- `fn eq(self: &Self, other: &LocalModes) -> bool` — [`LocalModes`](#localmodes)

##### `impl PublicFlags for LocalModes`

- `type Primitive = u32`

- `type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for LocalModes`

##### `impl Sub for LocalModes`

- `type Output = LocalModes`

- `fn sub(self: Self, other: Self) -> Self`

##### `impl SubAssign for LocalModes`

- `fn sub_assign(self: &mut Self, other: Self)`

##### `impl UpperHex for LocalModes`

- `fn fmt(self: &Self, f: &mut $crate::__private::core::fmt::Formatter<'_>) -> $crate::__private::core::fmt::Result`

### `SpecialCodes`

```rust
struct SpecialCodes([linux_raw_sys::general::cc_t; 19]);
```

An array indexed by [`SpecialCodeIndex`](#specialcodeindex) indicating the current values of
various special control codes.

#### Trait Implementations

##### `impl Clone for SpecialCodes`

- `fn clone(self: &Self) -> SpecialCodes` — [`SpecialCodes`](#specialcodes)

##### `impl Debug for SpecialCodes`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Index for SpecialCodes`

- `type Output = u8`

- `fn index(self: &Self, index: SpecialCodeIndex) -> &<Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl IndexMut for SpecialCodes`

- `fn index_mut(self: &mut Self, index: SpecialCodeIndex) -> &mut <Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

### `SpecialCode`

```rust
struct SpecialCode(u8);
```

A newtype for pretty printing.

#### Trait Implementations

##### `impl Debug for SpecialCode`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SpecialCodeIndex`

```rust
struct SpecialCodeIndex(usize);
```

Indices for use with `Termios::special_codes`.

#### Implementations

- `const VINTR: Self`

- `const VQUIT: Self`

- `const VERASE: Self`

- `const VKILL: Self`

- `const VEOF: Self`

- `const VTIME: Self`

- `const VMIN: Self`

- `const VSWTC: Self`

- `const VSTART: Self`

- `const VSTOP: Self`

- `const VSUSP: Self`

- `const VEOL: Self`

- `const VREPRINT: Self`

- `const VDISCARD: Self`

- `const VWERASE: Self`

- `const VLNEXT: Self`

- `const VEOL2: Self`

#### Trait Implementations

##### `impl Clone for SpecialCodeIndex`

- `fn clone(self: &Self) -> SpecialCodeIndex` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl Copy for SpecialCodeIndex`

##### `impl Debug for SpecialCodeIndex`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SpecialCodeIndex`

##### `impl Hash for SpecialCodeIndex`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for SpecialCodeIndex`

- `fn eq(self: &Self, other: &SpecialCodeIndex) -> bool` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl StructuralPartialEq for SpecialCodeIndex`

### `Winsize`

```rust
struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
```

`struct winsize` for use with [`tcgetwinsize`](#tcgetwinsize).


#### Fields

- **`ws_row`**: `u16`

  The number of rows the terminal has.

- **`ws_col`**: `u16`

  The number of columns the terminal has.

#### Trait Implementations

##### `impl Clone for Winsize`

- `fn clone(self: &Self) -> Winsize` — [`Winsize`](#winsize)

##### `impl Copy for Winsize`

##### `impl Debug for Winsize`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Winsize`

##### `impl Hash for Winsize`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Winsize`

- `fn eq(self: &Self, other: &Winsize) -> bool` — [`Winsize`](#winsize)

##### `impl StructuralPartialEq for Winsize`

## Enums

### `OptionalActions`

```rust
enum OptionalActions {
    Now,
    Drain,
    Flush,
}
```

`TCSA*` values for use with [`tcsetattr`](#tcsetattr).


#### Variants

- **`Now`**

  `TCSANOW`—Make the change immediately.

- **`Drain`**

  `TCSADRAIN`—Make the change after all output has been transmitted.

- **`Flush`**

  `TCSAFLUSH`—Discard any pending input and then make the change
  after all output has been transmitted.

#### Trait Implementations

##### `impl Clone for OptionalActions`

- `fn clone(self: &Self) -> OptionalActions` — [`OptionalActions`](#optionalactions)

##### `impl Copy for OptionalActions`

##### `impl Debug for OptionalActions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for OptionalActions`

##### `impl Hash for OptionalActions`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for OptionalActions`

- `fn eq(self: &Self, other: &OptionalActions) -> bool` — [`OptionalActions`](#optionalactions)

##### `impl StructuralPartialEq for OptionalActions`

### `QueueSelector`

```rust
enum QueueSelector {
    IFlush,
    OFlush,
    IOFlush,
}
```

`TC*` values for use with [`tcflush`](../backend/termios/syscalls/index.md).


#### Variants

- **`IFlush`**

  `TCIFLUSH`—Flush data received but not read.

- **`OFlush`**

  `TCOFLUSH`—Flush data written but not transmitted.

- **`IOFlush`**

  `TCIOFLUSH`—`IFlush` and `OFlush` combined.

#### Trait Implementations

##### `impl Clone for QueueSelector`

- `fn clone(self: &Self) -> QueueSelector` — [`QueueSelector`](#queueselector)

##### `impl Copy for QueueSelector`

##### `impl Debug for QueueSelector`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for QueueSelector`

##### `impl Hash for QueueSelector`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for QueueSelector`

- `fn eq(self: &Self, other: &QueueSelector) -> bool` — [`QueueSelector`](#queueselector)

##### `impl StructuralPartialEq for QueueSelector`

### `Action`

```rust
enum Action {
    OOff,
    OOn,
    IOff,
    IOn,
}
```

`TC*` values for use with [`tcflow`](#tcflow).


#### Variants

- **`OOff`**

  `TCOOFF`—Suspend output.

- **`OOn`**

  `TCOON`—Restart suspended output.

- **`IOff`**

  `TCIOFF`—Transmits a STOP byte.

- **`IOn`**

  `TCION`—Transmits a START byte.

#### Trait Implementations

##### `impl Clone for Action`

- `fn clone(self: &Self) -> Action` — [`Action`](#action)

##### `impl Copy for Action`

##### `impl Debug for Action`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Action`

##### `impl Hash for Action`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Action`

- `fn eq(self: &Self, other: &Action) -> bool` — [`Action`](#action)

##### `impl StructuralPartialEq for Action`

## Functions

### `ioctl_tiocexcl`

```rust
fn ioctl_tiocexcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal.

In exclusive mode, subsequent unprivileged `open` calls on the terminal
device fail with `io::Errno::BUSY`.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





### `ioctl_tiocnxcl`

```rust
fn ioctl_tiocnxcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





### `tcgetattr`

```rust
fn tcgetattr<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Termios>
```

`tcgetattr(fd)`—Get terminal attributes.

Also known as the `TCGETS` (or `TCGETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCGETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCGETS`, manually
initializes the fields that `TCGETS` doesn't initialize, and fails with
`io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcgetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetwinsize`

```rust
fn tcgetwinsize<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Winsize>
```

`tcgetwinsize(fd)`—Get the current terminal window size.

Also known as the `TIOCGWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `tcgetpgrp`

```rust
fn tcgetpgrp<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

`tcgetpgrp(fd)`—Get the terminal foreground process group.

Also known as the `TIOCGPGRP` operation with `ioctl`.

On Linux, if `fd` is a pseudo-terminal, the underlying system call here can
return a pid of 0, which rustix's `Pid` type doesn't support. So rustix
instead handles this case by failing with `io::Errno::OPNOTSUPP` if the
pid is 0.

# References
 - [POSIX]
 - [Linux]



### `tcsetpgrp`

```rust
fn tcsetpgrp<Fd: AsFd>(fd: Fd, pid: Pid) -> io::Result<()>
```

`tcsetpgrp(fd, pid)`—Set the terminal foreground process group.

Also known as the `TIOCSPGRP` operation with `ioctl`.

# References
 - [POSIX]
 - [Linux]



### `tcsetattr`

```rust
fn tcsetattr<Fd: AsFd>(fd: Fd, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

`tcsetattr(fd)`—Set terminal attributes.

Also known as the `TCSETS` (or `TCSETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCSETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCSETS`, and fails
with `io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcsetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcsendbreak`

```rust
fn tcsendbreak<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`tcsendbreak(fd, 0)`—Transmit zero-valued bits.

This transmits zero-valued bits for at least 0.25 seconds.

This function does not have a `duration` parameter, and always uses the
implementation-defined value, which transmits for at least 0.25 seconds.

Also known as the `TCSBRK` operation with `ioctl`, with a duration
parameter of 0.

# References
 - [POSIX `tcsendbreak`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcdrain`

```rust
fn tcdrain<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

`tcdrain(fd, duration)`—Wait until all pending output has been written.

# References
 - [POSIX `tcdrain`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflush`

```rust
fn tcflush<Fd: AsFd>(fd: Fd, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

`tcflush(fd, queue_selector)`—Wait until all pending output has been
written.

# References
 - [POSIX `tcflush`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflow`

```rust
fn tcflow<Fd: AsFd>(fd: Fd, action: crate::termios::Action) -> io::Result<()>
```

`tcflow(fd, action)`—Suspend or resume transmission or reception.

# References
 - [POSIX `tcflow`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetsid`

```rust
fn tcgetsid<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

`tcgetsid(fd)`—Return the session ID of the current session with `fd` as
its controlling terminal.

# References
 - [POSIX]
 - [Linux]



### `tcsetwinsize`

```rust
fn tcsetwinsize<Fd: AsFd>(fd: Fd, winsize: crate::termios::Winsize) -> io::Result<()>
```

`tcsetwinsize(fd)`—Set the current terminal window size.

Also known as the `TIOCSWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `isatty`

```rust
fn isatty<Fd: AsFd>(fd: Fd) -> bool
```

`isatty(fd)`—Tests whether a file descriptor refers to a terminal.

# References
 - [POSIX]
 - [Linux]



