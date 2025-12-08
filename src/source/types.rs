//! Types for representing parsed source code information.
//!
//! These types complement rustdoc JSON by providing information
//! that is only available from parsing source code directly.

use std::path::PathBuf;

/// Information about a parsed function, including its body.
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// The function name.
    pub name: String,

    /// Full module path (e.g., `crate::module::submodule`).
    pub module_path: String,

    /// The function signature as a string.
    pub signature: String,

    /// The function body as source code.
    pub body: String,

    /// Whether this function is public.
    pub is_public: bool,

    /// Doc comments extracted from `///` or `//!` attributes.
    pub doc_comments: Vec<String>,

    /// Source file where this function is defined.
    pub source_file: PathBuf,

    /// Line number where the function starts.
    pub line_number: usize,
}

/// Information about a parsed struct.
#[derive(Debug, Clone)]
pub struct StructInfo {
    /// The struct name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The full struct definition as source code.
    pub definition: String,

    /// Whether this struct is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,

    /// Field information (for struct fields).
    pub fields: Vec<FieldInfo>,
}

/// Information about a struct or enum field.
#[derive(Debug, Clone)]
pub struct FieldInfo {
    /// Field name (None for tuple struct fields).
    pub name: Option<String>,

    /// Field type as a string.
    pub ty: String,

    /// Whether this field is public.
    pub is_public: bool,

    /// Doc comments for this field.
    pub doc_comments: Vec<String>,
}

/// Information about a parsed enum.
#[derive(Debug, Clone)]
pub struct EnumInfo {
    /// The enum name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The full enum definition as source code.
    pub definition: String,

    /// Whether this enum is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,

    /// Variant information.
    pub variants: Vec<VariantInfo>,
}

/// Information about an enum variant.
#[derive(Debug, Clone)]
pub struct VariantInfo {
    /// Variant name.
    pub name: String,

    /// Doc comments for this variant.
    pub doc_comments: Vec<String>,

    /// Fields (for tuple or struct variants).
    pub fields: Vec<FieldInfo>,
}

/// Information about a parsed trait.
#[derive(Debug, Clone)]
pub struct TraitInfo {
    /// The trait name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The full trait definition as source code.
    pub definition: String,

    /// Whether this trait is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Information about an impl block.
#[derive(Debug, Clone)]
pub struct ImplInfo {
    /// The type being implemented for (e.g., `MyStruct`).
    pub self_ty: String,

    /// The trait being implemented (if any).
    pub trait_name: Option<String>,

    /// Full module path where this impl is defined.
    pub module_path: String,

    /// Methods in this impl block.
    pub methods: Vec<FunctionInfo>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Information about a constant.
#[derive(Debug, Clone)]
pub struct ConstInfo {
    /// The constant name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The type of the constant.
    pub ty: String,

    /// The value expression as source code.
    pub value: String,

    /// Whether this constant is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Information about a static variable.
#[derive(Debug, Clone)]
pub struct StaticInfo {
    /// The static name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The type of the static.
    pub ty: String,

    /// The value expression as source code.
    pub value: String,

    /// Whether this static is mutable.
    pub is_mutable: bool,

    /// Whether this static is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Information about a type alias.
#[derive(Debug, Clone)]
pub struct TypeAliasInfo {
    /// The type alias name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The aliased type as a string.
    pub aliased_type: String,

    /// Whether this type alias is public.
    pub is_public: bool,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Information about a macro definition.
#[derive(Debug, Clone)]
pub struct MacroInfo {
    /// The macro name.
    pub name: String,

    /// Full module path.
    pub module_path: String,

    /// The full macro definition as source code.
    pub definition: String,

    /// Doc comments.
    pub doc_comments: Vec<String>,

    /// Source file location.
    pub source_file: PathBuf,

    /// Line number.
    pub line_number: usize,
}

/// Aggregated source information for an entire crate.
#[derive(Debug, Default)]
pub struct CrateSource {
    /// Crate name.
    pub name: String,

    /// Crate version (from Cargo.toml).
    pub version: String,

    /// Root path of the crate source.
    pub root_path: PathBuf,

    /// All parsed functions (including methods).
    pub functions: Vec<FunctionInfo>,

    /// All parsed structs.
    pub structs: Vec<StructInfo>,

    /// All parsed enums.
    pub enums: Vec<EnumInfo>,

    /// All parsed traits.
    pub traits: Vec<TraitInfo>,

    /// All parsed impl blocks.
    pub impls: Vec<ImplInfo>,

    /// All parsed constants.
    pub constants: Vec<ConstInfo>,

    /// All parsed statics.
    pub statics: Vec<StaticInfo>,

    /// All parsed type aliases.
    pub type_aliases: Vec<TypeAliasInfo>,

    /// All parsed macro definitions.
    pub macros: Vec<MacroInfo>,
}

impl CrateSource {
    /// Create a new empty `CrateSource`.
    #[must_use]
    pub fn new(name: String, version: String, root_path: PathBuf) -> Self {
        Self {
            name,
            version,
            root_path,
            ..Default::default()
        }
    }

    /// Look up a function by its full path.
    #[must_use]
    pub fn find_function(&self, path: &str) -> Option<&FunctionInfo> {
        self.functions.iter().find(|f| {
            let full_path = format!("{}::{}", f.module_path, f.name);
            full_path == path
        })
    }

    /// Look up a struct by its full path.
    #[must_use]
    pub fn find_struct(&self, path: &str) -> Option<&StructInfo> {
        self.structs.iter().find(|s| {
            let full_path = format!("{}::{}", s.module_path, s.name);
            full_path == path
        })
    }

    /// Get all private items (functions, structs, etc.) in a module.
    #[must_use]
    pub fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>> {
        let mut items = Vec::new();

        for f in &self.functions {
            if !f.is_public && f.module_path == module_path {
                items.push(PrivateItem::Function(f));
            }
        }

        for s in &self.structs {
            if !s.is_public && s.module_path == module_path {
                items.push(PrivateItem::Struct(s));
            }
        }

        for e in &self.enums {
            if !e.is_public && e.module_path == module_path {
                items.push(PrivateItem::Enum(e));
            }
        }

        items
    }
}

/// A reference to a private item for rendering.
#[derive(Debug)]
pub enum PrivateItem<'a> {
    /// A private function.
    Function(&'a FunctionInfo),

    /// A private struct.
    Struct(&'a StructInfo),

    /// A private enum.
    Enum(&'a EnumInfo),

    /// A private constant.
    Const(&'a ConstInfo),

    /// A private type alias.
    TypeAlias(&'a TypeAliasInfo),
}
