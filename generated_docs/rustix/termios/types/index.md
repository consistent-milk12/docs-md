*[rustix](../../index.md) / [termios](../index.md) / [types](index.md)*

---

# Module `types`

## Contents

- [Modules](#modules)
  - [`speed`](#speed)
- [Structs](#structs)
  - [`Termios`](#termios)
  - [`InputModes`](#inputmodes)
  - [`OutputModes`](#outputmodes)
  - [`ControlModes`](#controlmodes)
  - [`LocalModes`](#localmodes)
  - [`SpecialCodes`](#specialcodes)
  - [`SpecialCode`](#specialcode)
  - [`SpecialCodeIndex`](#specialcodeindex)
  - [`Winsize`](#winsize)
- [Enums](#enums)
  - [`OptionalActions`](#optionalactions)
  - [`QueueSelector`](#queueselector)
  - [`Action`](#action)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`speed`](#speed) | mod | Speeds for use with [`Termios::set_speed`], [`Termios::set_input_speed`] |
| [`Termios`](#termios) | struct | `struct termios` for use with [`tcgetattr`] and [`tcsetattr`]. |
| [`InputModes`](#inputmodes) | struct | Flags controlling terminal input. |
| [`OutputModes`](#outputmodes) | struct | Flags controlling terminal output. |
| [`ControlModes`](#controlmodes) | struct | Flags controlling special terminal modes. |
| [`LocalModes`](#localmodes) | struct | Flags controlling “local” terminal modes. |
| [`SpecialCodes`](#specialcodes) | struct | An array indexed by [`SpecialCodeIndex`] indicating the current values of |
| [`SpecialCode`](#specialcode) | struct | A newtype for pretty printing. |
| [`SpecialCodeIndex`](#specialcodeindex) | struct | Indices for use with [`Termios::special_codes`]. |
| [`Winsize`](#winsize) | struct | `struct winsize` for use with [`tcgetwinsize`]. |
| [`OptionalActions`](#optionalactions) | enum | `TCSA*` values for use with [`tcsetattr`]. |
| [`QueueSelector`](#queueselector) | enum | `TC*` values for use with [`tcflush`]. |
| [`Action`](#action) | enum | `TC*` values for use with [`tcflow`]. |

## Modules

- [`speed`](speed/index.md) — Speeds for use with [`Termios::set_speed`], [`Termios::set_input_speed`],

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

`struct termios` for use with [`tcgetattr`](../../backend/termios/syscalls/index.md) and [`tcsetattr`](../index.md).



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

- <span id="termios-make-raw"></span>`fn make_raw(&mut self)`

- <span id="termios-input-speed"></span>`fn input_speed(&self) -> u32`

- <span id="termios-output-speed"></span>`fn output_speed(&self) -> u32`

- <span id="termios-set-speed"></span>`fn set_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../../io/index.md)

- <span id="termios-set-input-speed"></span>`fn set_input_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../../io/index.md)

- <span id="termios-set-output-speed"></span>`fn set_output_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../../io/index.md)

#### Trait Implementations

##### `impl Clone for Termios`

- <span id="termios-clone"></span>`fn clone(&self) -> Termios` — [`Termios`](../index.md)

##### `impl Debug for Termios`

- <span id="termios-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `InputModes`

```rust
struct InputModes(<InputModes as __private::PublicFlags>::Internal);
```

Flags controlling terminal input.

#### Implementations

- <span id="inputmodes-empty"></span>`const fn empty() -> Self`

- <span id="inputmodes-all"></span>`const fn all() -> Self`

- <span id="inputmodes-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="inputmodes-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../ffi/index.md)

- <span id="inputmodes-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="inputmodes-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="inputmodes-from-name"></span>`fn from_name(name: &str) -> __private::core::option::Option<Self>`

- <span id="inputmodes-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="inputmodes-is-all"></span>`const fn is_all(&self) -> bool`

- <span id="inputmodes-intersects"></span>`const fn intersects(&self, other: Self) -> bool`

- <span id="inputmodes-contains"></span>`const fn contains(&self, other: Self) -> bool`

- <span id="inputmodes-insert"></span>`fn insert(&mut self, other: Self)`

- <span id="inputmodes-remove"></span>`fn remove(&mut self, other: Self)`

- <span id="inputmodes-toggle"></span>`fn toggle(&mut self, other: Self)`

- <span id="inputmodes-set"></span>`fn set(&mut self, other: Self, value: bool)`

- <span id="inputmodes-intersection"></span>`const fn intersection(self, other: Self) -> Self`

- <span id="inputmodes-union"></span>`const fn union(self, other: Self) -> Self`

- <span id="inputmodes-difference"></span>`const fn difference(self, other: Self) -> Self`

- <span id="inputmodes-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

- <span id="inputmodes-complement"></span>`const fn complement(self) -> Self`

#### Trait Implementations

##### `impl Binary for InputModes`

- <span id="inputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for InputModes`

- <span id="inputmodes-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for InputModes`

- <span id="inputmodes-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for InputModes`

- <span id="inputmodes-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitor"></span>`fn bitor(self, other: InputModes) -> Self` — [`InputModes`](../index.md)

##### `impl BitOrAssign for InputModes`

- <span id="inputmodes-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for InputModes`

- <span id="inputmodes-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for InputModes`

- <span id="inputmodes-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for InputModes`

- <span id="inputmodes-clone"></span>`fn clone(&self) -> InputModes` — [`InputModes`](../index.md)

##### `impl Copy for InputModes`

##### `impl Debug for InputModes`

- <span id="inputmodes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InputModes`

##### `impl Extend for InputModes`

- <span id="inputmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for InputModes`

- <span id="inputmodes-flags"></span>`const FLAGS: &'static [Flag<InputModes>]`

- <span id="inputmodes-bits"></span>`type Bits = u32`

- <span id="inputmodes-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="inputmodes-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> InputModes` — [`c_uint`](../../ffi/index.md), [`InputModes`](../index.md)

##### `impl FromIterator for InputModes`

- <span id="inputmodes-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for InputModes`

- <span id="inputmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for InputModes`

- <span id="inputmodes-item"></span>`type Item = InputModes`

- <span id="inputmodes-intoiter"></span>`type IntoIter = Iter<InputModes>`

- <span id="inputmodes-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for InputModes`

- <span id="inputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for InputModes`

- <span id="inputmodes-output"></span>`type Output = InputModes`

- <span id="inputmodes-not"></span>`fn not(self) -> Self`

##### `impl Octal for InputModes`

- <span id="inputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for InputModes`

- <span id="inputmodes-eq"></span>`fn eq(&self, other: &InputModes) -> bool` — [`InputModes`](../index.md)

##### `impl PublicFlags for InputModes`

- <span id="inputmodes-primitive"></span>`type Primitive = u32`

- <span id="inputmodes-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for InputModes`

##### `impl Sub for InputModes`

- <span id="inputmodes-output"></span>`type Output = InputModes`

- <span id="inputmodes-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for InputModes`

- <span id="inputmodes-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for InputModes`

- <span id="inputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `OutputModes`

```rust
struct OutputModes(<OutputModes as __private::PublicFlags>::Internal);
```

Flags controlling terminal output.

#### Implementations

- <span id="outputmodes-iter"></span>`const fn iter(&self) -> iter::Iter<OutputModes>` — [`OutputModes`](../index.md)

- <span id="outputmodes-iter-names"></span>`const fn iter_names(&self) -> iter::IterNames<OutputModes>` — [`OutputModes`](../index.md)

#### Trait Implementations

##### `impl Binary for OutputModes`

- <span id="outputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for OutputModes`

- <span id="outputmodes-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for OutputModes`

- <span id="outputmodes-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for OutputModes`

- <span id="outputmodes-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitor"></span>`fn bitor(self, other: OutputModes) -> Self` — [`OutputModes`](../index.md)

##### `impl BitOrAssign for OutputModes`

- <span id="outputmodes-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for OutputModes`

- <span id="outputmodes-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for OutputModes`

- <span id="outputmodes-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for OutputModes`

- <span id="outputmodes-clone"></span>`fn clone(&self) -> OutputModes` — [`OutputModes`](../index.md)

##### `impl Copy for OutputModes`

##### `impl Debug for OutputModes`

- <span id="outputmodes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OutputModes`

##### `impl Extend for OutputModes`

- <span id="outputmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for OutputModes`

- <span id="outputmodes-flags"></span>`const FLAGS: &'static [Flag<OutputModes>]`

- <span id="outputmodes-bits"></span>`type Bits = u32`

- <span id="outputmodes-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="outputmodes-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> OutputModes` — [`c_uint`](../../ffi/index.md), [`OutputModes`](../index.md)

##### `impl FromIterator for OutputModes`

- <span id="outputmodes-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for OutputModes`

- <span id="outputmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for OutputModes`

- <span id="outputmodes-item"></span>`type Item = OutputModes`

- <span id="outputmodes-intoiter"></span>`type IntoIter = Iter<OutputModes>`

- <span id="outputmodes-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for OutputModes`

- <span id="outputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for OutputModes`

- <span id="outputmodes-output"></span>`type Output = OutputModes`

- <span id="outputmodes-not"></span>`fn not(self) -> Self`

##### `impl Octal for OutputModes`

- <span id="outputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for OutputModes`

- <span id="outputmodes-eq"></span>`fn eq(&self, other: &OutputModes) -> bool` — [`OutputModes`](../index.md)

##### `impl PublicFlags for OutputModes`

- <span id="outputmodes-primitive"></span>`type Primitive = u32`

- <span id="outputmodes-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for OutputModes`

##### `impl Sub for OutputModes`

- <span id="outputmodes-output"></span>`type Output = OutputModes`

- <span id="outputmodes-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for OutputModes`

- <span id="outputmodes-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for OutputModes`

- <span id="outputmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `ControlModes`

```rust
struct ControlModes(<ControlModes as __private::PublicFlags>::Internal);
```

Flags controlling special terminal modes.

`CBAUD`, `CBAUDEX`, `CIBAUD`, `CIBAUDEX`, and various `B*` speed
constants are often included in the control modes, however rustix
handles them separately, in `Termios::set_speed` and related
functions. If you see extra bits in the `Debug` output, they're
probably these flags.

#### Implementations

- <span id="controlmodes-csize"></span>`const CSIZE: Self`

- <span id="controlmodes-cs5"></span>`const CS5: Self`

- <span id="controlmodes-cs6"></span>`const CS6: Self`

- <span id="controlmodes-cs7"></span>`const CS7: Self`

- <span id="controlmodes-cs8"></span>`const CS8: Self`

- <span id="controlmodes-cstopb"></span>`const CSTOPB: Self`

- <span id="controlmodes-cread"></span>`const CREAD: Self`

- <span id="controlmodes-parenb"></span>`const PARENB: Self`

- <span id="controlmodes-parodd"></span>`const PARODD: Self`

- <span id="controlmodes-hupcl"></span>`const HUPCL: Self`

- <span id="controlmodes-clocal"></span>`const CLOCAL: Self`

- <span id="controlmodes-crtscts"></span>`const CRTSCTS: Self`

- <span id="controlmodes-cmspar"></span>`const CMSPAR: Self`

#### Trait Implementations

##### `impl Binary for ControlModes`

- <span id="controlmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for ControlModes`

- <span id="controlmodes-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for ControlModes`

- <span id="controlmodes-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for ControlModes`

- <span id="controlmodes-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitor"></span>`fn bitor(self, other: ControlModes) -> Self` — [`ControlModes`](../index.md)

##### `impl BitOrAssign for ControlModes`

- <span id="controlmodes-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for ControlModes`

- <span id="controlmodes-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for ControlModes`

- <span id="controlmodes-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for ControlModes`

- <span id="controlmodes-clone"></span>`fn clone(&self) -> ControlModes` — [`ControlModes`](../index.md)

##### `impl Copy for ControlModes`

##### `impl Debug for ControlModes`

- <span id="controlmodes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ControlModes`

##### `impl Extend for ControlModes`

- <span id="controlmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for ControlModes`

- <span id="controlmodes-flags"></span>`const FLAGS: &'static [Flag<ControlModes>]`

- <span id="controlmodes-bits"></span>`type Bits = u32`

- <span id="controlmodes-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="controlmodes-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ControlModes` — [`c_uint`](../../ffi/index.md), [`ControlModes`](../index.md)

##### `impl FromIterator for ControlModes`

- <span id="controlmodes-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for ControlModes`

- <span id="controlmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for ControlModes`

- <span id="controlmodes-item"></span>`type Item = ControlModes`

- <span id="controlmodes-intoiter"></span>`type IntoIter = Iter<ControlModes>`

- <span id="controlmodes-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for ControlModes`

- <span id="controlmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for ControlModes`

- <span id="controlmodes-output"></span>`type Output = ControlModes`

- <span id="controlmodes-not"></span>`fn not(self) -> Self`

##### `impl Octal for ControlModes`

- <span id="controlmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for ControlModes`

- <span id="controlmodes-eq"></span>`fn eq(&self, other: &ControlModes) -> bool` — [`ControlModes`](../index.md)

##### `impl PublicFlags for ControlModes`

- <span id="controlmodes-primitive"></span>`type Primitive = u32`

- <span id="controlmodes-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ControlModes`

##### `impl Sub for ControlModes`

- <span id="controlmodes-output"></span>`type Output = ControlModes`

- <span id="controlmodes-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for ControlModes`

- <span id="controlmodes-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for ControlModes`

- <span id="controlmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `LocalModes`

```rust
struct LocalModes(<LocalModes as __private::PublicFlags>::Internal);
```

Flags controlling “local” terminal modes.

#### Implementations

- <span id="localmodes-empty"></span>`const fn empty() -> Self`

- <span id="localmodes-all"></span>`const fn all() -> Self`

- <span id="localmodes-bits"></span>`const fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="localmodes-from-bits"></span>`const fn from_bits(bits: ffi::c_uint) -> __private::core::option::Option<Self>` — [`c_uint`](../../ffi/index.md)

- <span id="localmodes-from-bits-truncate"></span>`const fn from_bits_truncate(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="localmodes-from-bits-retain"></span>`const fn from_bits_retain(bits: ffi::c_uint) -> Self` — [`c_uint`](../../ffi/index.md)

- <span id="localmodes-from-name"></span>`fn from_name(name: &str) -> __private::core::option::Option<Self>`

- <span id="localmodes-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="localmodes-is-all"></span>`const fn is_all(&self) -> bool`

- <span id="localmodes-intersects"></span>`const fn intersects(&self, other: Self) -> bool`

- <span id="localmodes-contains"></span>`const fn contains(&self, other: Self) -> bool`

- <span id="localmodes-insert"></span>`fn insert(&mut self, other: Self)`

- <span id="localmodes-remove"></span>`fn remove(&mut self, other: Self)`

- <span id="localmodes-toggle"></span>`fn toggle(&mut self, other: Self)`

- <span id="localmodes-set"></span>`fn set(&mut self, other: Self, value: bool)`

- <span id="localmodes-intersection"></span>`const fn intersection(self, other: Self) -> Self`

- <span id="localmodes-union"></span>`const fn union(self, other: Self) -> Self`

- <span id="localmodes-difference"></span>`const fn difference(self, other: Self) -> Self`

- <span id="localmodes-symmetric-difference"></span>`const fn symmetric_difference(self, other: Self) -> Self`

- <span id="localmodes-complement"></span>`const fn complement(self) -> Self`

#### Trait Implementations

##### `impl Binary for LocalModes`

- <span id="localmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for LocalModes`

- <span id="localmodes-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

##### `impl BitAndAssign for LocalModes`

- <span id="localmodes-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

##### `impl BitOr for LocalModes`

- <span id="localmodes-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitor"></span>`fn bitor(self, other: LocalModes) -> Self` — [`LocalModes`](../index.md)

##### `impl BitOrAssign for LocalModes`

- <span id="localmodes-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl BitXor for LocalModes`

- <span id="localmodes-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

##### `impl BitXorAssign for LocalModes`

- <span id="localmodes-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

##### `impl Clone for LocalModes`

- <span id="localmodes-clone"></span>`fn clone(&self) -> LocalModes` — [`LocalModes`](../index.md)

##### `impl Copy for LocalModes`

##### `impl Debug for LocalModes`

- <span id="localmodes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocalModes`

##### `impl Extend for LocalModes`

- <span id="localmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

##### `impl Flags for LocalModes`

- <span id="localmodes-flags"></span>`const FLAGS: &'static [Flag<LocalModes>]`

- <span id="localmodes-bits"></span>`type Bits = u32`

- <span id="localmodes-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../../ffi/index.md)

- <span id="localmodes-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> LocalModes` — [`c_uint`](../../ffi/index.md), [`LocalModes`](../index.md)

##### `impl FromIterator for LocalModes`

- <span id="localmodes-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

##### `impl Hash for LocalModes`

- <span id="localmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for LocalModes`

- <span id="localmodes-item"></span>`type Item = LocalModes`

- <span id="localmodes-intoiter"></span>`type IntoIter = Iter<LocalModes>`

- <span id="localmodes-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for LocalModes`

- <span id="localmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for LocalModes`

- <span id="localmodes-output"></span>`type Output = LocalModes`

- <span id="localmodes-not"></span>`fn not(self) -> Self`

##### `impl Octal for LocalModes`

- <span id="localmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for LocalModes`

- <span id="localmodes-eq"></span>`fn eq(&self, other: &LocalModes) -> bool` — [`LocalModes`](../index.md)

##### `impl PublicFlags for LocalModes`

- <span id="localmodes-primitive"></span>`type Primitive = u32`

- <span id="localmodes-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for LocalModes`

##### `impl Sub for LocalModes`

- <span id="localmodes-output"></span>`type Output = LocalModes`

- <span id="localmodes-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for LocalModes`

- <span id="localmodes-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

##### `impl UpperHex for LocalModes`

- <span id="localmodes-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `SpecialCodes`

```rust
struct SpecialCodes([linux_raw_sys::general::cc_t; 19]);
```

An array indexed by [`SpecialCodeIndex`](../index.md) indicating the current values of
various special control codes.

#### Trait Implementations

##### `impl Clone for SpecialCodes`

- <span id="specialcodes-clone"></span>`fn clone(&self) -> SpecialCodes` — [`SpecialCodes`](../index.md)

##### `impl Debug for SpecialCodes`

- <span id="specialcodes-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Index for SpecialCodes`

- <span id="specialcodes-output"></span>`type Output = u8`

- <span id="specialcodes-index"></span>`fn index(&self, index: SpecialCodeIndex) -> &<Self as >::Output` — [`SpecialCodeIndex`](../index.md)

##### `impl IndexMut for SpecialCodes`

- <span id="specialcodes-index-mut"></span>`fn index_mut(&mut self, index: SpecialCodeIndex) -> &mut <Self as >::Output` — [`SpecialCodeIndex`](../index.md)

### `SpecialCode`

```rust
struct SpecialCode(u8);
```

A newtype for pretty printing.

#### Trait Implementations

##### `impl Debug for SpecialCode`

- <span id="specialcode-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SpecialCodeIndex`

```rust
struct SpecialCodeIndex(usize);
```

Indices for use with `Termios::special_codes`.

#### Implementations

- <span id="specialcodeindex-vintr"></span>`const VINTR: Self`

- <span id="specialcodeindex-vquit"></span>`const VQUIT: Self`

- <span id="specialcodeindex-verase"></span>`const VERASE: Self`

- <span id="specialcodeindex-vkill"></span>`const VKILL: Self`

- <span id="specialcodeindex-veof"></span>`const VEOF: Self`

- <span id="specialcodeindex-vtime"></span>`const VTIME: Self`

- <span id="specialcodeindex-vmin"></span>`const VMIN: Self`

- <span id="specialcodeindex-vswtc"></span>`const VSWTC: Self`

- <span id="specialcodeindex-vstart"></span>`const VSTART: Self`

- <span id="specialcodeindex-vstop"></span>`const VSTOP: Self`

- <span id="specialcodeindex-vsusp"></span>`const VSUSP: Self`

- <span id="specialcodeindex-veol"></span>`const VEOL: Self`

- <span id="specialcodeindex-vreprint"></span>`const VREPRINT: Self`

- <span id="specialcodeindex-vdiscard"></span>`const VDISCARD: Self`

- <span id="specialcodeindex-vwerase"></span>`const VWERASE: Self`

- <span id="specialcodeindex-vlnext"></span>`const VLNEXT: Self`

- <span id="specialcodeindex-veol2"></span>`const VEOL2: Self`

#### Trait Implementations

##### `impl Clone for SpecialCodeIndex`

- <span id="specialcodeindex-clone"></span>`fn clone(&self) -> SpecialCodeIndex` — [`SpecialCodeIndex`](../index.md)

##### `impl Copy for SpecialCodeIndex`

##### `impl Debug for SpecialCodeIndex`

- <span id="specialcodeindex-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SpecialCodeIndex`

##### `impl Hash for SpecialCodeIndex`

- <span id="specialcodeindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SpecialCodeIndex`

- <span id="specialcodeindex-eq"></span>`fn eq(&self, other: &SpecialCodeIndex) -> bool` — [`SpecialCodeIndex`](../index.md)

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

`struct winsize` for use with [`tcgetwinsize`](../../backend/termios/syscalls/index.md).


#### Fields

- **`ws_row`**: `u16`

  The number of rows the terminal has.

- **`ws_col`**: `u16`

  The number of columns the terminal has.

#### Trait Implementations

##### `impl Clone for Winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> Winsize` — [`Winsize`](../index.md)

##### `impl Copy for Winsize`

##### `impl Debug for Winsize`

- <span id="winsize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Winsize`

##### `impl Hash for Winsize`

- <span id="winsize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Winsize`

- <span id="winsize-eq"></span>`fn eq(&self, other: &Winsize) -> bool` — [`Winsize`](../index.md)

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

`TCSA*` values for use with [`tcsetattr`](../index.md).


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

- <span id="optionalactions-clone"></span>`fn clone(&self) -> OptionalActions` — [`OptionalActions`](../index.md)

##### `impl Copy for OptionalActions`

##### `impl Debug for OptionalActions`

- <span id="optionalactions-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OptionalActions`

##### `impl Hash for OptionalActions`

- <span id="optionalactions-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for OptionalActions`

- <span id="optionalactions-eq"></span>`fn eq(&self, other: &OptionalActions) -> bool` — [`OptionalActions`](../index.md)

##### `impl StructuralPartialEq for OptionalActions`

### `QueueSelector`

```rust
enum QueueSelector {
    IFlush,
    OFlush,
    IOFlush,
}
```

`TC*` values for use with [`tcflush`](../../backend/termios/syscalls/index.md).


#### Variants

- **`IFlush`**

  `TCIFLUSH`—Flush data received but not read.

- **`OFlush`**

  `TCOFLUSH`—Flush data written but not transmitted.

- **`IOFlush`**

  `TCIOFLUSH`—`IFlush` and `OFlush` combined.

#### Trait Implementations

##### `impl Clone for QueueSelector`

- <span id="queueselector-clone"></span>`fn clone(&self) -> QueueSelector` — [`QueueSelector`](../index.md)

##### `impl Copy for QueueSelector`

##### `impl Debug for QueueSelector`

- <span id="queueselector-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for QueueSelector`

##### `impl Hash for QueueSelector`

- <span id="queueselector-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for QueueSelector`

- <span id="queueselector-eq"></span>`fn eq(&self, other: &QueueSelector) -> bool` — [`QueueSelector`](../index.md)

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

`TC*` values for use with [`tcflow`](../../backend/termios/syscalls/index.md).


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

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](../index.md)

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Action`

##### `impl Hash for Action`

- <span id="action-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Action`

- <span id="action-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](../index.md)

##### `impl StructuralPartialEq for Action`

