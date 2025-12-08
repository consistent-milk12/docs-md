*[syn](../index.md) / [buffer](index.md)*

---

# Module `buffer`

A stably addressed token buffer supporting efficient traversal based on a
cheaply copyable cursor.

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

- `fn recursive_new(entries: &mut Vec<Entry>, stream: TokenStream)` — [`Entry`](#entry)

- `fn new(stream: proc_macro::TokenStream) -> Self`

- `fn new2(stream: TokenStream) -> Self`

- `fn begin(self: &Self) -> Cursor<'_>` — [`Cursor`](#cursor)

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

- `fn empty() -> Self`

- `unsafe fn create(ptr: *const Entry, scope: *const Entry) -> Self` — [`Entry`](#entry)

- `fn entry(self: Self) -> &'a Entry` — [`Entry`](#entry)

- `unsafe fn bump_ignore_group(self: Self) -> Cursor<'a>` — [`Cursor`](#cursor)

- `fn ignore_none(self: &mut Self)`

- `fn eof(self: Self) -> bool`

- `fn ident(self: Self) -> Option<(Ident, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn punct(self: Self) -> Option<(Punct, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn literal(self: Self) -> Option<(Literal, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn lifetime(self: Self) -> Option<(Lifetime, Cursor<'a>)>` — [`Lifetime`](../lifetime/index.md), [`Cursor`](#cursor)

- `fn group(self: Self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn any_group(self: Self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn any_group_token(self: Self) -> Option<(Group, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn token_stream(self: Self) -> TokenStream`

- `fn token_tree(self: Self) -> Option<(TokenTree, Cursor<'a>)>` — [`Cursor`](#cursor)

- `fn span(self: Self) -> Span`

- `fn prev_span(self: Self) -> Span`

- `fn skip(self: Self) -> Option<Cursor<'a>>` — [`Cursor`](#cursor)

- `fn scope_delimiter(self: Self) -> Delimiter`

#### Trait Implementations

##### `impl<'a> Clone for Cursor<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl<'a> Copy for Cursor<'a>`

##### `impl<'a> Eq for Cursor<'a>`

##### `impl<'a> PartialEq for Cursor<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<'a> PartialOrd for Cursor<'a>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

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

