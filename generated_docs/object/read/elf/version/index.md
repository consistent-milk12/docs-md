*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [version](index.md)*

---

# Module `version`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VersionIndex`](#versionindex) | struct | A version index. |
| [`Version`](#version) | struct | A version definition or requirement. |
| [`VersionTable`](#versiontable) | struct | A table of version definitions and requirements. |
| [`VerdefIterator`](#verdefiterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerdauxIterator`](#verdauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerneedIterator`](#verneediterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section. |
| [`VernauxIterator`](#vernauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section. |

## Structs

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:10`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L10)*

A version index.

#### Implementations

- <span id="versionindex-index"></span>`fn index(&self) -> u16`

  Return the version index.

- <span id="versionindex-is-local"></span>`fn is_local(&self) -> bool`

  Return true if it is the local index.

- <span id="versionindex-is-global"></span>`fn is_global(&self) -> bool`

  Return true if it is the global index.

- <span id="versionindex-is-hidden"></span>`fn is_hidden(&self) -> bool`

  Return the hidden flag.

#### Trait Implementations

##### `impl Any for VersionIndex`

- <span id="versionindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VersionIndex`

- <span id="versionindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VersionIndex`

- <span id="versionindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for VersionIndex`

- <span id="versionindex-clone"></span>`fn clone(&self) -> VersionIndex` — [`VersionIndex`](../index.md#versionindex)

##### `impl CloneToUninit for VersionIndex`

- <span id="versionindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- <span id="versionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionIndex`

- <span id="versionindex-default"></span>`fn default() -> VersionIndex` — [`VersionIndex`](../index.md#versionindex)

##### `impl<T> From for VersionIndex`

- <span id="versionindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VersionIndex`

- <span id="versionindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for VersionIndex`

- <span id="versionindex-toowned-type-owned"></span>`type Owned = T`

- <span id="versionindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="versionindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VersionIndex`

- <span id="versionindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="versionindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VersionIndex`

- <span id="versionindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="versionindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:38-44`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L38-L44)*

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

#### Implementations

- <span id="version-name"></span>`fn name(&self) -> &'data [u8]`

  Return the version name.

- <span id="version-hash"></span>`fn hash(&self) -> u32`

  Return hash of the version name.

- <span id="version-file"></span>`fn file(&self) -> Option<&'data [u8]>`

  Return the filename of the library containing this version.

  

  This is the `vn_file` field of the associated entry in [`elf::SHT_GNU_VERNEED`](../../../elf/index.md).

  or `None` if the version info was parsed from a [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Trait Implementations

##### `impl Any for Version<'data>`

- <span id="version-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Version<'data>`

- <span id="version-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Version<'data>`

- <span id="version-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Version<'data>`

- <span id="version-clone"></span>`fn clone(&self) -> Version<'data>` — [`Version`](../index.md#version)

##### `impl CloneToUninit for Version<'data>`

- <span id="version-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Version<'data>`

##### `impl Debug for Version<'data>`

- <span id="version-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Version<'data>`

- <span id="version-default"></span>`fn default() -> Version<'data>` — [`Version`](../index.md#version)

##### `impl<T> From for Version<'data>`

- <span id="version-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Version<'data>`

- <span id="version-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Version<'data>`

- <span id="version-toowned-type-owned"></span>`type Owned = T`

- <span id="version-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="version-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Version<'data>`

- <span id="version-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="version-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Version<'data>`

- <span id="version-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="version-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:75-78`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L75-L78)*

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- <span id="versiontable-parse"></span>`fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Versym`](../../../elf/index.md#versym), [`VerdefIterator`](../index.md#verdefiterator), [`VerneedIterator`](../index.md#verneediterator), [`StringTable`](../../index.md#stringtable), [`Result`](../../../index.md#result)

  Parse the version sections.

- <span id="versiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the version table is empty.

- <span id="versiontable-version-index"></span>`fn version_index(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex), [`VersionIndex`](../index.md#versionindex)

  Return version index for a given symbol index.

- <span id="versiontable-version"></span>`fn version(&self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](../index.md#versionindex), [`Result`](../../../index.md#result), [`Version`](../index.md#version)

  Return version information for a given symbol version index.

  

  Returns `Ok(None)` for local and global versions.

  Returns `Err(_)` if index is invalid.

- <span id="versiontable-matches"></span>`fn matches(&self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex), [`Version`](../index.md#version)

  Return true if the given symbol index satisfies the requirements of `need`.

  

  Returns false for any error.

  

  Note: this function hasn't been fully tested and is likely to be incomplete.

#### Trait Implementations

##### `impl Any for VersionTable<'data, Elf>`

- <span id="versiontable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VersionTable<'data, Elf>`

- <span id="versiontable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VersionTable<'data, Elf>`

- <span id="versiontable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- <span id="versiontable-clone"></span>`fn clone(&self) -> VersionTable<'data, Elf>` — [`VersionTable`](../index.md#versiontable)

##### `impl CloneToUninit for VersionTable<'data, Elf>`

- <span id="versiontable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- <span id="versiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader> Default for VersionTable<'data, Elf>`

- <span id="versiontable-default"></span>`fn default() -> Self`

##### `impl<T> From for VersionTable<'data, Elf>`

- <span id="versiontable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VersionTable<'data, Elf>`

- <span id="versiontable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for VersionTable<'data, Elf>`

- <span id="versiontable-toowned-type-owned"></span>`type Owned = T`

- <span id="versiontable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="versiontable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VersionTable<'data, Elf>`

- <span id="versiontable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="versiontable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VersionTable<'data, Elf>`

- <span id="versiontable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="versiontable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:234-237`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L234-L237)*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdefiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md#result), [`Verdef`](../../../elf/index.md#verdef), [`FileHeader`](../index.md#fileheader), [`VerdauxIterator`](../index.md#verdauxiterator)

  Return the next `Verdef` entry.

- <span id="verdefiterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../../index.md#result), [`Verdef`](../../../elf/index.md#verdef), [`FileHeader`](../index.md#fileheader), [`VerdauxIterator`](../index.md#verdauxiterator)

#### Trait Implementations

##### `impl Any for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clone"></span>`fn clone(&self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](../index.md#verdefiterator)

##### `impl CloneToUninit for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdefiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdefiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-iterator-type-item"></span>`type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- <span id="verdefiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verdefiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verdefiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verdefiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verdefiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:297-301`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L297-L301)*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md#result), [`Verdaux`](../../../elf/index.md#verdaux), [`FileHeader`](../index.md#fileheader)

  Return the next `Verdaux` entry.

- <span id="verdauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../../index.md#result), [`Verdaux`](../../../elf/index.md#verdaux), [`FileHeader`](../index.md#fileheader)

#### Trait Implementations

##### `impl Any for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clone"></span>`fn clone(&self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](../index.md#verdauxiterator)

##### `impl CloneToUninit for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-iterator-type-item"></span>`type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="verdauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verdauxiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verdauxiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verdauxiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verdauxiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:350-353`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L350-L353)*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="verneediterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md#result), [`Verneed`](../../../elf/index.md#verneed), [`FileHeader`](../index.md#fileheader), [`VernauxIterator`](../index.md#vernauxiterator)

  Return the next `Verneed` entry.

- <span id="verneediterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../../index.md#result), [`Verneed`](../../../elf/index.md#verneed), [`FileHeader`](../index.md#fileheader), [`VernauxIterator`](../index.md#vernauxiterator)

#### Trait Implementations

##### `impl Any for VerneedIterator<'data, Elf>`

- <span id="verneediterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VerneedIterator<'data, Elf>`

- <span id="verneediterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VerneedIterator<'data, Elf>`

- <span id="verneediterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clone"></span>`fn clone(&self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](../index.md#verneediterator)

##### `impl CloneToUninit for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- <span id="verneediterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VerneedIterator<'data, Elf>`

- <span id="verneediterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VerneedIterator<'data, Elf>`

- <span id="verneediterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verneediterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verneediterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-iterator-type-item"></span>`type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- <span id="verneediterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VerneedIterator<'data, Elf>`

- <span id="verneediterator-toowned-type-owned"></span>`type Owned = T`

- <span id="verneediterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="verneediterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VerneedIterator<'data, Elf>`

- <span id="verneediterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="verneediterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VerneedIterator<'data, Elf>`

- <span id="verneediterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="verneediterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

*Defined in [`object-0.37.3/src/read/elf/version.rs:426-430`](../../../../../.source_1765633015/object-0.37.3/src/read/elf/version.rs#L426-L430)*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="vernauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md#result), [`Vernaux`](../../../elf/index.md#vernaux), [`FileHeader`](../index.md#fileheader)

  Return the next `Vernaux` entry.

- <span id="vernauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../../index.md#result), [`Vernaux`](../../../elf/index.md#vernaux), [`FileHeader`](../index.md#fileheader)

#### Trait Implementations

##### `impl Any for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<Elf: clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clone"></span>`fn clone(&self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](../index.md#vernauxiterator)

##### `impl CloneToUninit for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="vernauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="vernauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-iterator-type-item"></span>`type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="vernauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-toowned-type-owned"></span>`type Owned = T`

- <span id="vernauxiterator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="vernauxiterator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="vernauxiterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="vernauxiterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

