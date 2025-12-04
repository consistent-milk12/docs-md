# Crate `pathdiff`

## Functions

### `diff_paths`

```rust
fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>
```

Construct a relative path from a provided base directory path to the provided path.

```rust
use pathdiff::diff_paths;
use std::path::*;

assert_eq!(diff_paths("/foo/bar",      "/foo/bar/baz"),  Some("../".into()));
assert_eq!(diff_paths("/foo/bar/baz",  "/foo/bar"),      Some("baz".into()));
assert_eq!(diff_paths("/foo/bar/quux", "/foo/bar/baz"),  Some("../quux".into()));
assert_eq!(diff_paths("/foo/bar/baz",  "/foo/bar/quux"), Some("../baz".into()));
assert_eq!(diff_paths("/foo/bar",      "/foo/bar/quux"), Some("../".into()));

assert_eq!(diff_paths("/foo/bar",      "baz"),           Some("/foo/bar".into()));
assert_eq!(diff_paths("/foo/bar",      "/baz"),          Some("../foo/bar".into()));
assert_eq!(diff_paths("foo",           "bar"),           Some("../foo".into()));

assert_eq!(
    diff_paths(&"/foo/bar/baz", "/foo/bar".to_string()),
    Some("baz".into())
);
assert_eq!(
    diff_paths(Path::new("/foo/bar/baz"), Path::new("/foo/bar").to_path_buf()),
    Some("baz".into())
);
```
