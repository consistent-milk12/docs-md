*[syn](../index.md) / [buffer](index.md)*

---

# Module `buffer`

A stably addressed token buffer supporting efficient traversal based on a
cheaply copyable cursor.

## Contents

- [Structs](#structs)
  - [`TokenBuffer`](#tokenbuffer)
  - [`Cursor`](#cursor)
- [Enums](#enums)
  - [`Entry`](#entry)
- [Functions](#functions)
  - [`same_scope`](#same_scope)
  - [`same_buffer`](#same_buffer)
  - [`start_of_buffer`](#start_of_buffer)
  - [`cmp_assuming_same_buffer`](#cmp_assuming_same_buffer)
  - [`open_span_of_group`](#open_span_of_group)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TokenBuffer`](#tokenbuffer) | struct | A buffer that can be efficiently traversed multiple times, unlike |
| [`Cursor`](#cursor) | struct | A cheaply copyable cursor into a `TokenBuffer`. |
| [`Entry`](#entry) | enum | Internal type which is used instead of `TokenTree` to represent a token tree |
| [`same_scope`](#same_scope) | fn |  |
| [`same_buffer`](#same_buffer) | fn |  |
| [`start_of_buffer`](#start_of_buffer) | fn |  |
| [`cmp_assuming_same_buffer`](#cmp_assuming_same_buffer) | fn |  |
| [`open_span_of_group`](#open_span_of_group) | fn |  |

## Structs

### `TokenBuffer`

```rust
struct TokenBuffer {
    entries: Box<[Entry]>,
}
```

A buffer that can be efficiently traversed multiple times, unlike
`TokenStream` which requires a deep copy in order to traverse more than
once.

#### Implementations

- <span id="tokenbuffer-recursive-new"></span>`fn recursive_new(entries: &mut Vec<Entry>, stream: TokenStream)` — [`Entry`](#entry)

- <span id="tokenbuffer-new"></span>`fn new(stream: proc_macro::TokenStream) -> Self`

- <span id="tokenbuffer-new2"></span>`fn new2(stream: TokenStream) -> Self`

- <span id="tokenbuffer-begin"></span>`fn begin(&self) -> Cursor<'_>` — [`Cursor`](#cursor)

### `Cursor<'a>`

```rust
struct Cursor<'a> {
    ptr: *const Entry,
    scope: *const Entry,
    marker: std::marker::PhantomData<&'a Entry>,
}
```

A cheaply copyable cursor into a `TokenBuffer`.

This cursor holds a shared reference into the immutable data which is used
internally to represent a `TokenStream`, and can be efficiently manipulated
and copied around.

An empty `Cursor` can be created directly, or one may create a `TokenBuffer`
object and get a cursor to its first token with `begin()`.

#### Implementations

- <span id="cursor-empty"></span>`fn empty() -> Self`

- <span id="cursor-create"></span>`unsafe fn create(ptr: *const Entry, scope: *const Entry) -> Self` — [`Entry`](#entry)

- <span id="cursor-entry"></span>`fn entry(self) -> &'a Entry` — [`Entry`](#entry)

- <span id="cursor-bump-ignore-group"></span>`unsafe fn bump_ignore_group(self) -> Cursor<'a>` — [`Cursor`](#cursor)

- <span id="cursor-ignore-none"></span>`fn ignore_none(&mut self)`

- <span id="cursor-eof"></span>`fn eof(self) -> bool`

- <span id="cursor-ident"></span>`fn ident(self) -> Option<(Ident, Cursor<'a>)>` — [`Ident`](../index.md), [`Cursor`](#cursor)

- <span id="cursor-punct"></span>`fn punct(self) -> Option<(Punct, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-literal"></span>`fn literal(self) -> Option<(Literal, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-lifetime"></span>`fn lifetime(self) -> Option<(Lifetime, Cursor<'a>)>` — [`Lifetime`](../index.md), [`Cursor`](#cursor)

- <span id="cursor-group"></span>`fn group(self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-any-group"></span>`fn any_group(self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-any-group-token"></span>`fn any_group_token(self) -> Option<(Group, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-token-stream"></span>`fn token_stream(self) -> TokenStream`

- <span id="cursor-token-tree"></span>`fn token_tree(self) -> Option<(TokenTree, Cursor<'a>)>` — [`Cursor`](#cursor)

- <span id="cursor-span"></span>`fn span(self) -> Span`

- <span id="cursor-prev-span"></span>`fn prev_span(self) -> Span`

- <span id="cursor-skip"></span>`fn skip(self) -> Option<Cursor<'a>>` — [`Cursor`](#cursor)

- <span id="cursor-scope-delimiter"></span>`fn scope_delimiter(self) -> Delimiter`

#### Trait Implementations

##### `impl<'a> Clone for Cursor<'a>`

- <span id="cursor-clone"></span>`fn clone(&self) -> Self`

##### `impl<'a> Copy for Cursor<'a>`

##### `impl<'a> Eq for Cursor<'a>`

##### `impl<'a> PartialEq for Cursor<'a>`

- <span id="cursor-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<'a> PartialOrd for Cursor<'a>`

- <span id="cursor-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

## Enums

### `Entry`

```rust
enum Entry {
    Group(proc_macro2::Group, usize),
    Ident(proc_macro2::Ident),
    Punct(proc_macro2::Punct),
    Literal(proc_macro2::Literal),
    End(isize, isize),
}
```

Internal type which is used instead of `TokenTree` to represent a token tree
within a `TokenBuffer`.

## Functions

### `same_scope`

```rust
fn same_scope(a: Cursor<'_>, b: Cursor<'_>) -> bool
```

### `same_buffer`

```rust
fn same_buffer(a: Cursor<'_>, b: Cursor<'_>) -> bool
```

### `start_of_buffer`

```rust
fn start_of_buffer(cursor: Cursor<'_>) -> *const Entry
```

### `cmp_assuming_same_buffer`

```rust
fn cmp_assuming_same_buffer(a: Cursor<'_>, b: Cursor<'_>) -> std::cmp::Ordering
```

### `open_span_of_group`

```rust
fn open_span_of_group(cursor: Cursor<'_>) -> proc_macro2::Span
```

