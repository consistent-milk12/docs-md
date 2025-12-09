*[syn](../../index.md) / [token](../index.md) / [private](index.md)*

---

# Module `private`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WithSpan`](#withspan) | struct | Support writing `token.span` rather than `token.spans[0]` on tokens that hold a single span. |
| [`Sealed`](#sealed) | trait |  |

## Structs

### `WithSpan`

```rust
struct WithSpan {
    pub span: proc_macro2::Span,
}
```

*Defined in [`syn-2.0.111/src/token.rs:147-149`](../../../../.source_1765210505/syn-2.0.111/src/token.rs#L147-L149)*

Support writing `token.span` rather than `token.spans[0]` on tokens that
hold a single span.

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`syn-2.0.111/src/token.rs:141`](../../../../.source_1765210505/syn-2.0.111/src/token.rs#L141)*

#### Implementors

- [`Abstract`](../index.md)
- [`AndAnd`](../index.md)
- [`AndEq`](../index.md)
- [`And`](../index.md)
- [`As`](../index.md)
- [`Async`](../index.md)
- [`At`](../index.md)
- [`Auto`](../index.md)
- [`Await`](../index.md)
- [`Become`](../index.md)
- [`Box`](../index.md)
- [`Brace`](../index.md)
- [`Bracket`](../index.md)
- [`Break`](../index.md)
- [`CaretEq`](../index.md)
- [`Caret`](../index.md)
- [`Colon`](../index.md)
- [`Comma`](../index.md)
- [`Const`](../index.md)
- [`Continue`](../index.md)
- [`Crate`](../index.md)
- [`Default`](../index.md)
- [`Do`](../index.md)
- [`Dollar`](../index.md)
- [`DotDotDot`](../index.md)
- [`DotDotEq`](../index.md)
- [`DotDot`](../index.md)
- [`Dot`](../index.md)
- [`Dyn`](../index.md)
- [`Else`](../index.md)
- [`Enum`](../index.md)
- [`EqEq`](../index.md)
- [`Eq`](../index.md)
- [`Extern`](../index.md)
- [`FatArrow`](../index.md)
- [`Final`](../index.md)
- [`Fn`](../index.md)
- [`For`](../index.md)
- [`Ge`](../index.md)
- [`Group`](../index.md)
- [`Gt`](../index.md)
- [`Ident`](../../ident/index.md)
- [`If`](../index.md)
- [`Impl`](../index.md)
- [`In`](../index.md)
- [`LArrow`](../index.md)
- [`Le`](../index.md)
- [`Let`](../index.md)
- [`Lifetime`](../../lifetime/index.md)
- [`LitBool`](../../lit/index.md)
- [`LitByteStr`](../../lit/index.md)
- [`LitByte`](../../lit/index.md)
- [`LitCStr`](../../lit/index.md)
- [`LitChar`](../../lit/index.md)
- [`LitFloat`](../../lit/index.md)
- [`LitInt`](../../lit/index.md)
- [`LitStr`](../../lit/index.md)
- [`Lit`](../../lit/index.md)
- [`Loop`](../index.md)
- [`Lt`](../index.md)
- [`Macro`](../index.md)
- [`Match`](../index.md)
- [`MinusEq`](../index.md)
- [`Minus`](../index.md)
- [`Mod`](../index.md)
- [`Move`](../index.md)
- [`Mut`](../index.md)
- [`Ne`](../index.md)
- [`Not`](../index.md)
- [`OrEq`](../index.md)
- [`OrOr`](../index.md)
- [`Or`](../index.md)
- [`Override`](../index.md)
- [`Paren`](../index.md)
- [`PathSep`](../index.md)
- [`PercentEq`](../index.md)
- [`Percent`](../index.md)
- [`PlusEq`](../index.md)
- [`Plus`](../index.md)
- [`Pound`](../index.md)
- [`Priv`](../index.md)
- [`Pub`](../index.md)
- [`Question`](../index.md)
- [`RArrow`](../index.md)
- [`Raw`](../index.md)
- [`Ref`](../index.md)
- [`Return`](../index.md)
- [`SelfType`](../index.md)
- [`SelfValue`](../index.md)
- [`Semi`](../index.md)
- [`ShlEq`](../index.md)
- [`Shl`](../index.md)
- [`ShrEq`](../index.md)
- [`Shr`](../index.md)
- [`SlashEq`](../index.md)
- [`Slash`](../index.md)
- [`StarEq`](../index.md)
- [`Star`](../index.md)
- [`Static`](../index.md)
- [`Struct`](../index.md)
- [`Super`](../index.md)
- [`Tilde`](../index.md)
- [`Trait`](../index.md)
- [`Try`](../index.md)
- [`Type`](../index.md)
- [`Typeof`](../index.md)
- [`Underscore`](../index.md)
- [`Union`](../index.md)
- [`Unsafe`](../index.md)
- [`Unsized`](../index.md)
- [`Use`](../index.md)
- [`Virtual`](../index.md)
- [`Where`](../index.md)
- [`While`](../index.md)
- [`Yield`](../index.md)
- `T`
- `proc_macro2::Group`
- `proc_macro2::Literal`
- `proc_macro2::Punct`
- `proc_macro2::TokenTree`

