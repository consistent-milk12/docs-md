//! Source access trait definition.

use crate::source::{
    ConstInfo, CrateSource, EnumInfo, FunctionInfo, PrivateItem, StaticInfo, StructInfo, TraitInfo,
};

/// Access to parsed source code information.
///
/// This trait provides access to source code details not available in
/// rustdoc JSON, including function bodies, private items and actual
/// constant values.
#[expect(dead_code, reason = "TODO: Reserved for future use.")]
pub trait SourceAccess {
    /// Get parsed source for the current crate.
    fn crate_source(&self) -> Option<&CrateSource>;

    /// Find function source by full path (e.g., `"crate::module::func_name"`).
    fn find_function_source(&self, path: &str) -> Option<&FunctionInfo> {
        self.crate_source()?.find_function(path)
    }

    /// Find struct source by full path.
    fn find_struct_source(&self, path: &str) -> Option<&StructInfo> {
        self.crate_source()?.find_struct(path)
    }

    /// Find enum source by full path.
    fn find_enum_source(&self, path: &str) -> Option<&EnumInfo> {
        self.crate_source()?.find_enum(path)
    }

    /// Find trait source by full path.
    fn find_trait_source(&self, path: &str) -> Option<&TraitInfo> {
        self.crate_source()?.find_trait(path)
    }

    /// Find constant source by full path.
    fn find_const_source(&self, path: &str) -> Option<&ConstInfo> {
        self.crate_source()?.find_const(path)
    }

    /// Find static source by full path.
    fn find_static_source(&self, path: &str) -> Option<&StaticInfo> {
        self.crate_source()?.find_static(path)
    }

    /// Get all private items in a module.
    fn private_items_in_module(&self, module_path: &str) -> Vec<PrivateItem<'_>> {
        self.crate_source()
            .map(|s| s.private_items_in_module(module_path))
            .unwrap_or_default()
    }
}
