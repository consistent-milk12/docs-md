//! Breadcrumb navigation generation for nested module pages.
//!
//! This module provides [`BreadcrumbGenerator`] which creates navigation
//! links showing the path from crate root to the current module.

/// Generates breadcrumb navigation for nested module pages.
///
/// Creates a navigation line showing the path from the crate root to
/// the current module, with each segment being a clickable link.
pub struct BreadcrumbGenerator<'a> {
    /// The module path segments (e.g., `["error", "types"]`).
    module_path: &'a [String],

    /// The name of the crate for the root link.
    crate_name: &'a str,
}

impl<'a> BreadcrumbGenerator<'a> {
    /// Create a new breadcrumb generator.
    ///
    /// # Arguments
    ///
    /// * `module_path` - The module path segments
    /// * `crate_name` - The name of the crate for the root link
    #[must_use]
    pub const fn new(module_path: &'a [String], crate_name: &'a str) -> Self {
        Self {
            module_path,
            crate_name,
        }
    }

    /// Generate breadcrumb navigation markdown.
    ///
    /// Returns empty string for root module.
    ///
    /// # Example Output
    ///
    /// For `module_path = ["error", "types"]` and `crate_name = "docs_md"`:
    /// ```markdown
    /// *[docs_md](../../index.md) / [error](../index.md) / [types](index.md)*
    ///
    /// ---
    /// ```
    #[must_use]
    pub fn generate(&self) -> String {
        if self.module_path.is_empty() {
            return String::new();
        }

        let mut crumbs = Vec::new();
        let depth = self.module_path.len();

        // Link to crate root
        crumbs.push(format!(
            "[{}]({}index.md)",
            self.crate_name,
            "../".repeat(depth)
        ));

        // Links to each intermediate module
        for (i, segment) in self.module_path.iter().enumerate() {
            let levels_up = depth - i - 1;
            let link = if levels_up == 0 {
                format!("[{segment}](index.md)")
            } else {
                format!("[{segment}]({}index.md)", "../".repeat(levels_up))
            };
            crumbs.push(link);
        }

        format!("*{}*\n\n---\n\n", crumbs.join(" / "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumbs_empty_path() {
        let generator = BreadcrumbGenerator::new(&[], "my_crate");
        assert_eq!(generator.generate(), "");
    }

    #[test]
    fn test_breadcrumbs_single_level() {
        let path = vec!["module".to_string()];
        let generator = BreadcrumbGenerator::new(&path, "my_crate");
        assert_eq!(
            generator.generate(),
            "*[my_crate](../index.md) / [module](index.md)*\n\n---\n\n"
        );
    }

    #[test]
    fn test_breadcrumbs_two_levels() {
        let path = vec!["parent".to_string(), "child".to_string()];
        let generator = BreadcrumbGenerator::new(&path, "my_crate");
        assert_eq!(
            generator.generate(),
            "*[my_crate](../../index.md) / [parent](../index.md) / [child](index.md)*\n\n---\n\n"
        );
    }

    #[test]
    fn test_breadcrumbs_three_levels() {
        let path = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let generator = BreadcrumbGenerator::new(&path, "crate");
        assert_eq!(
            generator.generate(),
            "*[crate](../../../index.md) / [a](../../index.md) / [b](../index.md) / [c](index.md)*\n\n---\n\n"
        );
    }
}
