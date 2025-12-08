*[syn](../index.md) / [tt](index.md)*

---

# Module `tt`

## Structs

### `TokenTreeHelper<'a>`

```rust
struct TokenTreeHelper<'a>(&'a proc_macro2::TokenTree);
```

#### Trait Implementations

##### `impl<'a> Hash for TokenTreeHelper<'a>`

- `fn hash<H: Hasher>(self: &Self, h: &mut H)`

##### `impl<'a> PartialEq for TokenTreeHelper<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

### `TokenStreamHelper<'a>`

```rust
struct TokenStreamHelper<'a>(&'a proc_macro2::TokenStream);
```

#### Trait Implementations

##### `impl<'a> Hash for TokenStreamHelper<'a>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'a> PartialEq for TokenStreamHelper<'a>`

- `fn eq(self: &Self, other: &Self) -> bool`

