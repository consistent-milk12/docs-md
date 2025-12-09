//! Source code parser using `syn`.
//!
//! This module provides parsing of Rust source files to extract
//! information that is not available in rustdoc JSON, such as
//! function bodies, private items, and implementation details.

#![expect(
    clippy::unused_self,
    reason = "Will probably be needed for future work."
)]

use std::fs as StdFs;
use std::path::{Path, PathBuf};
use std::string::ToString;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, Fields, File, ImplItem, Item, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemType, Lit, Meta, StaticMutability, Visibility,
};

use super::types::{
    ConstInfo, CrateSource, EnumInfo, FieldInfo, FunctionInfo, ImplInfo, MacroInfo, StaticInfo,
    StructInfo, TraitInfo, TypeAliasInfo, VariantInfo,
};
use crate::error::Error;

/// Parser for Rust source code using `syn`.
#[derive(Debug, Default)]
#[expect(clippy::struct_field_names, reason = "Not really an issue.")]
pub struct SourceParser {
    /// The crate name being parsed.
    crate_name: String,

    /// The crate version.
    crate_version: String,

    /// Root path of the crate.
    crate_root: PathBuf,
}

impl SourceParser {
    /// Create a new source parser for a crate.
    #[must_use]
    pub const fn new(name: String, version: String, root_path: PathBuf) -> Self {
        Self {
            crate_name: name,
            crate_version: version,
            crate_root: root_path,
        }
    }

    /// Parse an entire crate starting from its root.
    ///
    /// # Errors
    ///
    /// Returns an error if any source file cannot be parsed.
    pub fn parse_crate(&self) -> Result<CrateSource, Error> {
        let mut source = CrateSource::new(
            self.crate_name.clone(),
            self.crate_version.clone(),
            self.crate_root.clone(),
        );

        // Find the entry point (lib.rs or main.rs)
        let entry_point = self.find_entry_point()?;

        // Parse starting from the entry point
        self.parse_module_file(&entry_point, &self.crate_name, &mut source)?;

        Ok(source)
    }

    /// Find the crate entry point (lib.rs or main.rs).
    fn find_entry_point(&self) -> Result<PathBuf, Error> {
        let src_dir = self.crate_root.join("src");

        // Try lib.rs first (library crate)
        let lib_rs = src_dir.join("lib.rs");

        if lib_rs.exists() {
            return Ok(lib_rs);
        }

        // Try main.rs (binary crate)
        let main_rs = src_dir.join("main.rs");

        if main_rs.exists() {
            return Ok(main_rs);
        }

        // Try lib.rs in crate root (some crates don't use src/)
        let root_lib = self.crate_root.join("lib.rs");

        if root_lib.exists() {
            return Ok(root_lib);
        }

        Err(Error::SourceParser(format!(
            "No entry point found for crate at {}",
            self.crate_root.display()
        )))
    }

    /// Parse a single module file and its submodules.
    fn parse_module_file(
        &self,
        path: &Path,
        module_path: &str,
        source: &mut CrateSource,
    ) -> Result<(), Error> {
        let content = StdFs::read_to_string(path)
            .map_err(|e| Error::SourceParser(format!("Failed to read {}: {e}", path.display())))?;

        let file = syn::parse_file(&content)
            .map_err(|e| Error::SourceParser(format!("Failed to parse {}: {e}", path.display())))?;

        self.process_file(&file, path, module_path, source)?;

        Ok(())
    }

    /// Process a parsed file, extracting items and following submodules.
    fn process_file(
        &self,
        file: &File,
        file_path: &Path,
        module_path: &str,
        source: &mut CrateSource,
    ) -> Result<(), Error> {
        for item in &file.items {
            self.process_item(item, file_path, module_path, source)?;
        }

        Ok(())
    }

    /// Process a single item from a file.
    fn process_item(
        &self,
        item: &Item,
        file_path: &Path,
        module_path: &str,
        source: &mut CrateSource,
    ) -> Result<(), Error> {
        match item {
            Item::Fn(func) => {
                source
                    .functions
                    .push(self.extract_function(func, file_path, module_path));
            },

            Item::Struct(s) => {
                source
                    .structs
                    .push(self.extract_struct(s, file_path, module_path));
            },

            Item::Enum(e) => {
                source
                    .enums
                    .push(self.extract_enum(e, file_path, module_path));
            },

            Item::Trait(t) => {
                source
                    .traits
                    .push(self.extract_trait(t, file_path, module_path));
            },

            Item::Impl(impl_block) => {
                source
                    .impls
                    .push(self.extract_impl(impl_block, file_path, module_path));
            },

            Item::Const(c) => {
                source
                    .constants
                    .push(self.extract_const(c, file_path, module_path));
            },

            Item::Static(s) => {
                source
                    .statics
                    .push(self.extract_static(s, file_path, module_path));
            },

            Item::Type(t) => {
                source
                    .type_aliases
                    .push(self.extract_type_alias(t, file_path, module_path));
            },

            Item::Macro(m) => {
                if let Some(ident) = &m.ident {
                    source.macros.push(MacroInfo {
                        name: ident.to_string(),
                        module_path: module_path.to_string(),
                        definition: m.to_token_stream().to_string(),
                        doc_comments: Self::extract_doc_comments(&m.attrs),
                        source_file: file_path.to_path_buf(),
                        line_number: Self::line_of(m),
                    });
                }
            },

            Item::Mod(module) => {
                self.process_module(module, file_path, module_path, source)?;
            },

            // Skip other items for now
            _ => {},
        }

        Ok(())
    }

    /// Process a module declaration, potentially following to an external file.
    fn process_module(
        &self,
        module: &ItemMod,
        current_file: &Path,
        parent_module_path: &str,
        source: &mut CrateSource,
    ) -> Result<(), Error> {
        let module_name = module.ident.to_string();
        let new_module_path = format!("{parent_module_path}::{module_name}");

        if let Some((_, items)) = &module.content {
            // Inline module - process items directly
            for item in items {
                self.process_item(item, current_file, &new_module_path, source)?;
            }
        } else {
            // External module - find and parse the file
            if let Some(module_file) = self.find_module_file(current_file, &module_name) {
                self.parse_module_file(&module_file, &new_module_path, source)?;
            }
            // If module file not found, skip silently (might be cfg'd out)
        }

        Ok(())
    }

    /// Find the file for an external module declaration.
    fn find_module_file(&self, current_file: &Path, module_name: &str) -> Option<PathBuf> {
        let current_dir = current_file.parent()?;

        // Check if current file is mod.rs or lib.rs/main.rs
        let file_stem = current_file.file_stem()?.to_str()?;
        let is_mod_file = file_stem == "mod" || file_stem == "lib" || file_stem == "main";

        if is_mod_file {
            // Look for module_name.rs in the same directory
            let sibling = current_dir.join(format!("{module_name}.rs"));

            if sibling.exists() {
                return Some(sibling);
            }

            // Look for module_name/mod.rs
            let subdir = current_dir.join(module_name).join("mod.rs");

            if subdir.exists() {
                return Some(subdir);
            }
        } else {
            // Current file is something like foo.rs
            // Look for foo/module_name.rs
            let parent_dir = current_dir.join(file_stem);

            let sibling = parent_dir.join(format!("{module_name}.rs"));

            if sibling.exists() {
                return Some(sibling);
            }

            // Look for foo/module_name/mod.rs
            let subdir = parent_dir.join(module_name).join("mod.rs");

            if subdir.exists() {
                return Some(subdir);
            }
        }

        None
    }

    /// Extract function information.
    fn extract_function(&self, func: &ItemFn, file_path: &Path, module_path: &str) -> FunctionInfo {
        FunctionInfo {
            name: func.sig.ident.to_string(),
            module_path: module_path.to_string(),
            signature: func.sig.to_token_stream().to_string(),
            body: func.block.to_token_stream().to_string(),
            is_public: matches!(func.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&func.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(func),
        }
    }

    /// Extract struct information.
    fn extract_struct(&self, s: &ItemStruct, file_path: &Path, module_path: &str) -> StructInfo {
        StructInfo {
            name: s.ident.to_string(),
            module_path: module_path.to_string(),
            definition: s.to_token_stream().to_string(),
            is_public: matches!(s.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&s.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(s),
            fields: Self::extract_fields(&s.fields),
        }
    }

    /// Extract enum information.
    fn extract_enum(&self, e: &ItemEnum, file_path: &Path, module_path: &str) -> EnumInfo {
        EnumInfo {
            name: e.ident.to_string(),
            module_path: module_path.to_string(),
            definition: e.to_token_stream().to_string(),
            is_public: matches!(e.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&e.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(e),
            variants: e
                .variants
                .iter()
                .map(|v| VariantInfo {
                    name: v.ident.to_string(),
                    doc_comments: Self::extract_doc_comments(&v.attrs),
                    fields: Self::extract_fields(&v.fields),
                })
                .collect(),
        }
    }

    /// Extract trait information.
    fn extract_trait(&self, t: &ItemTrait, file_path: &Path, module_path: &str) -> TraitInfo {
        TraitInfo {
            name: t.ident.to_string(),
            module_path: module_path.to_string(),
            definition: t.to_token_stream().to_string(),
            is_public: matches!(t.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&t.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(t),
        }
    }

    /// Extract impl block information.
    fn extract_impl(&self, impl_block: &ItemImpl, file_path: &Path, module_path: &str) -> ImplInfo {
        let self_ty = impl_block.self_ty.to_token_stream().to_string();
        let trait_name = impl_block
            .trait_
            .as_ref()
            .map(|(_, path, _)| path.to_token_stream().to_string());

        let methods = impl_block
            .items
            .iter()
            .filter_map(|item| {
                if let ImplItem::Fn(method) = item {
                    Some(FunctionInfo {
                        name: method.sig.ident.to_string(),
                        module_path: module_path.to_string(),
                        signature: method.sig.to_token_stream().to_string(),
                        body: method.block.to_token_stream().to_string(),
                        is_public: matches!(method.vis, Visibility::Public(_)),
                        doc_comments: Self::extract_doc_comments(&method.attrs),
                        source_file: file_path.to_path_buf(),
                        line_number: Self::line_of(method),
                    })
                } else {
                    None
                }
            })
            .collect();

        ImplInfo {
            self_ty,
            trait_name,
            module_path: module_path.to_string(),
            methods,
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(impl_block),
        }
    }

    /// Extract constant information.
    fn extract_const(&self, c: &ItemConst, file_path: &Path, module_path: &str) -> ConstInfo {
        ConstInfo {
            name: c.ident.to_string(),
            module_path: module_path.to_string(),
            ty: c.ty.to_token_stream().to_string(),
            value: c.expr.to_token_stream().to_string(),
            is_public: matches!(c.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&c.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(c),
        }
    }

    /// Extract static information.
    fn extract_static(&self, s: &ItemStatic, file_path: &Path, module_path: &str) -> StaticInfo {
        StaticInfo {
            name: s.ident.to_string(),
            module_path: module_path.to_string(),
            ty: s.ty.to_token_stream().to_string(),
            value: s.expr.to_token_stream().to_string(),
            is_mutable: matches!(s.mutability, StaticMutability::Mut(_)),
            is_public: matches!(s.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&s.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(s),
        }
    }

    /// Extract type alias information.
    fn extract_type_alias(
        &self,
        t: &ItemType,
        file_path: &Path,
        module_path: &str,
    ) -> TypeAliasInfo {
        TypeAliasInfo {
            name: t.ident.to_string(),
            module_path: module_path.to_string(),
            aliased_type: t.ty.to_token_stream().to_string(),
            is_public: matches!(t.vis, Visibility::Public(_)),
            doc_comments: Self::extract_doc_comments(&t.attrs),
            source_file: file_path.to_path_buf(),
            line_number: Self::line_of(t),
        }
    }

    /// Extract the starting line number from a spanned item.
    ///
    /// Uses `proc-macro2`'s span-locations feature to get accurate line numbers.
    fn line_of<T: Spanned>(item: &T) -> usize {
        item.span().start().line
    }

    /// Extract doc comments from attributes.
    ///
    /// Doc comments in Rust are represented as `#[doc = "..."]` attributes.
    fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String> {
        attrs
            .iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("doc") {
                    return None;
                }

                // Try to extract the doc string from #[doc = "..."]
                if let Meta::NameValue(nv) = &attr.meta
                    && let Expr::Lit(expr_lit) = &nv.value
                    && let Lit::Str(lit_str) = &expr_lit.lit
                {
                    return Some(lit_str.value());
                }

                None
            })
            .collect()
    }

    /// Extract field information from struct/enum fields.
    fn extract_fields(fields: &Fields) -> Vec<FieldInfo> {
        match fields {
            Fields::Named(named) => named
                .named
                .iter()
                .map(|f| FieldInfo {
                    name: f.ident.as_ref().map(ToString::to_string),
                    ty: f.ty.to_token_stream().to_string(),
                    is_public: matches!(f.vis, Visibility::Public(_)),
                    doc_comments: Self::extract_doc_comments(&f.attrs),
                })
                .collect(),

            Fields::Unnamed(unnamed) => unnamed
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| FieldInfo {
                    name: Some(format!("{i}")),
                    ty: f.ty.to_token_stream().to_string(),
                    is_public: matches!(f.vis, Visibility::Public(_)),
                    doc_comments: Self::extract_doc_comments(&f.attrs),
                })
                .collect(),

            Fields::Unit => Vec::new(),
        }
    }

    /// Parse a single file without traversing modules.
    ///
    /// Useful for quick parsing of individual files.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn parse_file(path: &Path) -> Result<File, Error> {
        let content = StdFs::read_to_string(path)
            .map_err(|e| Error::SourceParser(format!("Failed to read {}: {e}", path.display())))?;

        syn::parse_file(&content)
            .map_err(|e| Error::SourceParser(format!("Failed to parse {}: {e}", path.display())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_doc_comments() {
        let src = r"
            /// This is a doc comment
            /// with multiple lines
            pub fn foo() {}
        ";

        let file = syn::parse_file(src).unwrap();
        if let Item::Fn(func) = &file.items[0] {
            let docs = SourceParser::extract_doc_comments(&func.attrs);

            assert_eq!(docs.len(), 2);
            assert_eq!(docs[0], " This is a doc comment");
            assert_eq!(docs[1], " with multiple lines");
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_extract_function() {
        let src = r"
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        ";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Fn(func) = &file.items[0] {
            let info = parser.extract_function(func, Path::new("test.rs"), "crate");
            assert_eq!(info.name, "add");
            assert!(info.is_public);
            assert!(info.signature.contains("fn add"));
            assert!(info.body.contains("a + b"));
        }
    }

    #[test]
    fn test_extract_struct_fields() {
        let src = r"
            pub struct Point {
                /// X coordinate
                pub x: f64,
                /// Y coordinate
                pub y: f64,
            }
        ";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Struct(s) = &file.items[0] {
            let info = parser.extract_struct(s, Path::new("test.rs"), "crate");
            assert_eq!(info.name, "Point");
            assert_eq!(info.fields.len(), 2);
            assert_eq!(info.fields[0].name, Some("x".to_string()));
            assert!(info.fields[0].doc_comments[0].contains("X coordinate"));
        }
    }

    #[test]
    fn test_line_numbers_function() {
        // Line 1 is the function
        let src = "pub fn foo() {}";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Fn(func) = &file.items[0] {
            let info = parser.extract_function(func, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 1, "Function should be on line 1");
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_line_numbers_multiple_items() {
        let src = r"pub fn first() {}

pub struct Second;

pub enum Third { A, B }

pub const FOURTH: i32 = 42;
";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        // Function on line 1
        if let Item::Fn(func) = &file.items[0] {
            let info = parser.extract_function(func, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 1, "first() should be on line 1");
        }

        // Struct on line 3
        if let Item::Struct(s) = &file.items[1] {
            let info = parser.extract_struct(s, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 3, "Second should be on line 3");
        }

        // Enum on line 5
        if let Item::Enum(e) = &file.items[2] {
            let info = parser.extract_enum(e, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 5, "Third should be on line 5");
        }

        // Const on line 7
        if let Item::Const(c) = &file.items[3] {
            let info = parser.extract_const(c, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 7, "FOURTH should be on line 7");
        }
    }

    #[test]
    fn test_line_numbers_impl_block() {
        let src = r"struct Foo;

impl Foo {
    pub fn method_one(&self) {}

    pub fn method_two(&self) {}
}
";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Impl(impl_block) = &file.items[1] {
            let info = parser.extract_impl(impl_block, Path::new("test.rs"), "crate");

            // impl block starts on line 3
            assert_eq!(info.line_number, 3, "impl block should be on line 3");

            // Methods have their own line numbers
            assert_eq!(info.methods.len(), 2);
            assert_eq!(
                info.methods[0].line_number, 4,
                "method_one should be on line 4"
            );
            assert_eq!(
                info.methods[1].line_number, 6,
                "method_two should be on line 6"
            );
        } else {
            panic!("Expected impl block");
        }
    }

    #[test]
    fn test_line_numbers_trait() {
        let src = r"
/// A trait
pub trait MyTrait {
    fn required(&self);
}
";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Trait(t) = &file.items[0] {
            let info = parser.extract_trait(t, Path::new("test.rs"), "crate");
            // Doc comment is on line 2, trait keyword on line 3
            assert_eq!(
                info.line_number, 2,
                "Trait should start on line 2 (doc comment)"
            );
        } else {
            panic!("Expected trait");
        }
    }

    #[test]
    fn test_line_numbers_static_and_type_alias() {
        let src = r"pub static FOO: i32 = 1;

pub type Bar = Vec<String>;
";

        let file = syn::parse_file(src).unwrap();
        let parser = SourceParser::new("test".into(), "0.1.0".into(), PathBuf::new());

        if let Item::Static(s) = &file.items[0] {
            let info = parser.extract_static(s, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 1, "static FOO should be on line 1");
        }

        if let Item::Type(t) = &file.items[1] {
            let info = parser.extract_type_alias(t, Path::new("test.rs"), "crate");
            assert_eq!(info.line_number, 3, "type Bar should be on line 3");
        }
    }
}
