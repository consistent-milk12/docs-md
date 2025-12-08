*[serde_derive](../../index.md) / [internals](../index.md) / [receiver](index.md)*

---

# Module `receiver`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReplaceReceiver`](#replacereceiver) | struct |  |
| [`replace_receiver`](#replace_receiver) | fn |  |

## Structs

### `ReplaceReceiver<'a>`

```rust
struct ReplaceReceiver<'a>(&'a syn::TypePath);
```

#### Implementations

- <span id="replacereceiver-visit-type-mut"></span>`fn visit_type_mut(&mut self, ty: &mut Type)`

- <span id="replacereceiver-visit-type-path-mut"></span>`fn visit_type_path_mut(&mut self, ty: &mut TypePath)`

- <span id="replacereceiver-visit-expr-path-mut"></span>`fn visit_expr_path_mut(&mut self, expr: &mut ExprPath)`

- <span id="replacereceiver-visit-type-mut-impl"></span>`fn visit_type_mut_impl(&mut self, ty: &mut Type)`

- <span id="replacereceiver-visit-type-path-mut-impl"></span>`fn visit_type_path_mut_impl(&mut self, ty: &mut TypePath)`

- <span id="replacereceiver-visit-expr-path-mut-impl"></span>`fn visit_expr_path_mut_impl(&mut self, expr: &mut ExprPath)`

- <span id="replacereceiver-visit-path-mut"></span>`fn visit_path_mut(&mut self, path: &mut Path)`

- <span id="replacereceiver-visit-path-arguments-mut"></span>`fn visit_path_arguments_mut(&mut self, arguments: &mut PathArguments)`

- <span id="replacereceiver-visit-return-type-mut"></span>`fn visit_return_type_mut(&mut self, return_type: &mut ReturnType)`

- <span id="replacereceiver-visit-type-param-bound-mut"></span>`fn visit_type_param_bound_mut(&mut self, bound: &mut TypeParamBound)`

- <span id="replacereceiver-visit-generics-mut"></span>`fn visit_generics_mut(&mut self, generics: &mut Generics)`

- <span id="replacereceiver-visit-data-mut"></span>`fn visit_data_mut(&mut self, data: &mut Data)`

- <span id="replacereceiver-visit-expr-mut"></span>`fn visit_expr_mut(&mut self, expr: &mut Expr)`

- <span id="replacereceiver-visit-macro-mut"></span>`fn visit_macro_mut(&mut self, _mac: &mut Macro)`

## Functions

### `replace_receiver`

```rust
fn replace_receiver(input: &mut syn::DeriveInput)
```

