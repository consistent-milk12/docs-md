*[unicode_normalization](../index.md) / [char](index.md)*

---

# Module `char`

Methods for composing and decomposing characters.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`compose`](#compose) | fn |  |
| [`decompose_canonical`](#decompose_canonical) | fn |  |
| [`decompose_cjk_compat_variants`](#decompose_cjk_compat_variants) | fn |  |
| [`decompose_compatible`](#decompose_compatible) | fn |  |
| [`canonical_combining_class`](#canonical_combining_class) | fn |  |
| [`is_combining_mark`](#is_combining_mark) | fn |  |
| [`is_public_assigned`](#is_public_assigned) | fn | Return whether the given character is assigned (`General_Category` != `Unassigned`) |

## Functions

Return whether the given character is assigned (`General_Category` != `Unassigned`)
and not Private-Use (`General_Category` != `Private_Use`), in the supported version
of Unicode.

