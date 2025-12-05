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

- `fn recursive_new(entries: &mut Vec<Entry>, stream: TokenStream)` — [`Entry`](../../buffer/index.md)

- `fn new(stream: proc_macro::TokenStream) -> Self`

- `fn new2(stream: TokenStream) -> Self`

- `fn begin(self: &Self) -> Cursor<'_>` — [`Cursor`](../../buffer/index.md)

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

- `unsafe fn create(ptr: *const Entry, scope: *const Entry) -> Self` — [`Entry`](../../buffer/index.md)

- `fn entry(self: Self) -> &'a Entry` — [`Entry`](../../buffer/index.md)

- `unsafe fn bump_ignore_group(self: Self) -> Cursor<'a>` — [`Cursor`](../../buffer/index.md)

- `fn ignore_none(self: &mut Self)`

- `fn eof(self: Self) -> bool`

- `fn ident(self: Self) -> Option<(Ident, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn punct(self: Self) -> Option<(Punct, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn literal(self: Self) -> Option<(Literal, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn lifetime(self: Self) -> Option<(Lifetime, Cursor<'a>)>` — [`Lifetime`](../../lifetime/index.md), [`Cursor`](../../buffer/index.md)

- `fn group(self: Self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn any_group(self: Self) -> Option<(Cursor<'a>, Delimiter, DelimSpan, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn any_group_token(self: Self) -> Option<(Group, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn token_stream(self: Self) -> TokenStream`

- `fn token_tree(self: Self) -> Option<(TokenTree, Cursor<'a>)>` — [`Cursor`](../../buffer/index.md)

- `fn span(self: Self) -> Span`

- `fn prev_span(self: Self) -> Span`

- `fn skip(self: Self) -> Option<Cursor<'a>>` — [`Cursor`](../../buffer/index.md)

- `fn scope_delimiter(self: Self) -> Delimiter`

#### Trait Implementations

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Self`

##### `impl Copy<'a>`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl PartialOrd<'a>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

