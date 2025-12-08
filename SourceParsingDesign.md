This document proposes a hybrid approach to Rust documentation generation that combines the existing rustdoc JSON parser with direct source code parsing from `~/.cargo`. The goal is to generate richer documentation that includes implementation details, function bodies, private items, and code examples that rustdoc JSON doesn't expose. Things we can get:

- Access to private items and internal implementation details
- Function body extraction for implementation examples
- Inline code snippets from actual implementations
- Better support for understanding library internals
- Complementary to rustdoc JSON (not a replacement)

The idea is to get full exposure to source code of the dependencies directly in the workspace.

**Current Ideas:** Use `syn` for syntactic parsing, optionally augmented with `ra_ap_` crates for semantic analysis when cross-crate type resolution is needed.
