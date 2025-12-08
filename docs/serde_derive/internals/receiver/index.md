*[serde_derive](../../index.md) / [internals](../index.md) / [receiver](index.md)*

---

# Module `receiver`

## Structs

### `ReplaceReceiver<'a>`

```rust
struct ReplaceReceiver<'a>(&'a syn::TypePath);
```

#### Implementations

- `fn visit_type_mut(self: &mut Self, ty: &mut Type)`

- `fn visit_type_path_mut(self: &mut Self, ty: &mut TypePath)`

- `fn visit_expr_path_mut(self: &mut Self, expr: &mut ExprPath)`

- `fn visit_type_mut_impl(self: &mut Self, ty: &mut Type)`

- `fn visit_type_path_mut_impl(self: &mut Self, ty: &mut TypePath)`

- `fn visit_expr_path_mut_impl(self: &mut Self, expr: &mut ExprPath)`

- `fn visit_path_mut(self: &mut Self, path: &mut Path)`

- `fn visit_path_arguments_mut(self: &mut Self, arguments: &mut PathArguments)`

- `fn visit_return_type_mut(self: &mut Self, return_type: &mut ReturnType)`

- `fn visit_type_param_bound_mut(self: &mut Self, bound: &mut TypeParamBound)`

- `fn visit_generics_mut(self: &mut Self, generics: &mut Generics)`

- `fn visit_data_mut(self: &mut Self, data: &mut Data)`

- `fn visit_expr_mut(self: &mut Self, expr: &mut Expr)`

- `fn visit_macro_mut(self: &mut Self, _mac: &mut Macro)`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

