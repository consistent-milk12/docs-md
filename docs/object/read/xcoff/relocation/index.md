*[object](../../../index.md) / [read](../../index.md) / [xcoff](../index.md) / [relocation](index.md)*

---

# Module `relocation`

## Structs

### `XcoffRelocationIterator<'data, 'file, Xcoff, R>`

```rust
struct XcoffRelocationIterator<'data, 'file, Xcoff, R>
where
    Xcoff: FileHeader,
    R: ReadRef<'data> {
    file: &'file super::XcoffFile<'data, Xcoff, R>,
    relocations: slice::Iter<'data, <<Xcoff as FileHeader>::SectionHeader as SectionHeader>::Rel>,
}
```

An iterator for the relocations in an [`XcoffSection`](super::XcoffSection).

#### Trait Implementations

##### `impl<'data, 'file, Xcoff, R> Debug for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'data, 'file, Xcoff, R> Iterator for XcoffRelocationIterator<'data, 'file, Xcoff, R>`

- `type Item = (u64, Relocation)`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

### `Rel`

```rust
trait Rel: Debug + Pod { ... }
```

A trait for generic access to [`xcoff::Rel32`](../../../xcoff/index.md) and [`xcoff::Rel64`](../../../xcoff/index.md).

#### Required Methods

- `type Word: 1`

- `fn r_vaddr(self: &Self) -> <Self as >::Word`

- `fn r_symndx(self: &Self) -> u32`

- `fn r_rsize(self: &Self) -> u8`

- `fn r_rtype(self: &Self) -> u8`

- `fn symbol(self: &Self) -> SymbolIndex`

## Type Aliases

### `XcoffRelocationIterator32<'data, 'file, R>`

```rust
type XcoffRelocationIterator32<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader32, R>;
```

An iterator for the relocations in an [`XcoffSection32`](super::XcoffSection32).

### `XcoffRelocationIterator64<'data, 'file, R>`

```rust
type XcoffRelocationIterator64<'data, 'file, R> = XcoffRelocationIterator<'data, 'file, xcoff::FileHeader64, R>;
```

An iterator for the relocations in an [`XcoffSection64`](super::XcoffSection64).

