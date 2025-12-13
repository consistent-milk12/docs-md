*[aho_corasick](../../index.md) / [automaton](../index.md) / [private](index.md)*

---

# Module `private`

We seal the `Automaton` trait for now. It's a big trait, and it's
conceivable that I might want to add new required methods, and sealing the
trait permits doing that in a backwards compatible fashion. On other the
hand, if you have a solid use case for implementing the trait yourself,
please file an issue and we can discuss it. This was *mostly* done as a
conservative step.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait |  |

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:29`](../../../../.source_1765633015/aho-corasick-1.1.4/src/automaton.rs#L29)*

#### Implementors

- [`DFA`](../../dfa/index.md#dfa)
- [`NFA`](../../nfa/contiguous/index.md#nfa)
- [`NFA`](../../nfa/noncontiguous/index.md#nfa)
- `&'a T`
- `alloc::sync::Arc<dyn AcAutomaton>`

