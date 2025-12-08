*[gimli](../../index.md) / [read](../index.md) / [lookup](index.md)*

---

# Module `lookup`

## Structs

### `DebugLookup<R, Parser>`

```rust
struct DebugLookup<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    input_buffer: R,
    phantom: core::marker::PhantomData<Parser>,
}
```

#### Implementations

- `fn items(self: &Self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

- `fn reader(self: &Self) -> &R`

#### Trait Implementations

##### `impl<R, Parser> Clone for DebugLookup<R, Parser>`

- `fn clone(self: &Self) -> DebugLookup<R, Parser>` — [`DebugLookup`](#debuglookup)

##### `impl<R, Parser> Debug for DebugLookup<R, Parser>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `LookupEntryIter<R, Parser>`

```rust
struct LookupEntryIter<R, Parser>
where
    R: Reader,
    Parser: LookupParser<R> {
    current_set: Option<(R, <Parser as >::Header)>,
    remaining_input: R,
}
```

#### Implementations

- `fn next(self: &mut Self) -> Result<Option<<Parser as >::Entry>>` — [`Result`](../../index.md), [`LookupParser`](#lookupparser)

#### Trait Implementations

##### `impl<R, Parser> Clone for LookupEntryIter<R, Parser>`

- `fn clone(self: &Self) -> LookupEntryIter<R, Parser>` — [`LookupEntryIter`](#lookupentryiter)

##### `impl<R, Parser> Debug for LookupEntryIter<R, Parser>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PubStuffHeader<T>`

```rust
struct PubStuffHeader<T> {
    format: crate::common::Format,
    length: T,
    version: u16,
    unit_offset: crate::common::DebugInfoOffset<T>,
    unit_length: T,
}
```

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for PubStuffHeader<T>`

- `fn clone(self: &Self) -> PubStuffHeader<T>` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T: $crate::fmt::Debug> Debug for PubStuffHeader<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for PubStuffHeader<T>`

##### `impl<T: $crate::cmp::PartialEq> PartialEq for PubStuffHeader<T>`

- `fn eq(self: &Self, other: &PubStuffHeader<T>) -> bool` — [`PubStuffHeader`](#pubstuffheader)

##### `impl<T> StructuralPartialEq for PubStuffHeader<T>`

### `PubStuffParser<R, Entry>`

```rust
struct PubStuffParser<R, Entry>
where
    R: Reader,
    Entry: PubStuffEntry<R> {
    phantom: core::marker::PhantomData<(R, Entry)>,
}
```

#### Trait Implementations

##### `impl<R, Entry> Clone for PubStuffParser<R, Entry>`

- `fn clone(self: &Self) -> PubStuffParser<R, Entry>` — [`PubStuffParser`](#pubstuffparser)

##### `impl<R, Entry> Debug for PubStuffParser<R, Entry>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<R, Entry> LookupParser for PubStuffParser<R, Entry>`

- `type Header = PubStuffHeader<<R as Reader>::Offset>`

- `type Entry = Entry`

- `fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>` — [`Result`](../../index.md), [`LookupParser`](#lookupparser)

- `fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>` — [`LookupParser`](#lookupparser), [`Result`](../../index.md)

## Traits

### `LookupParser<R: Reader>`

```rust
trait LookupParser<R: Reader> { ... }
```

#### Required Methods

- `type Header`

- `type Entry`

- `fn parse_header(input: &mut R) -> Result<(R, <Self as >::Header)>`

  Parse a header from `input`. Returns a tuple of `input` sliced to contain just the entries

- `fn parse_entry(input: &mut R, header: &<Self as >::Header) -> Result<Option<<Self as >::Entry>>`

  Parse a single entry from `input`. Returns either a parsed representation of the entry

### `PubStuffEntry<R: Reader>`

```rust
trait PubStuffEntry<R: Reader> { ... }
```

#### Required Methods

- `fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self`

