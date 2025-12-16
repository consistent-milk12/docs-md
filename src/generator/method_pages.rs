//! Separate method page generation for large traits.
//!
//! This module provides [`MethodPageGenerator`] for generating individual markdown
//! pages for each method when a trait exceeds the configured method threshold.
//! This improves navigation and readability for traits with many methods.

use std::fmt::Write;

use rustdoc_types::{Item, ItemEnum};

use crate::generator::render_shared::RendererInternals;
use crate::types::TypeRenderer;

/// A generated method page ready to be written to disk.
#[derive(Debug, Clone)]
pub struct MethodPage {
    /// Relative path for the method page (e.g., "itertools/methods/coalesce.md")
    pub path: String,

    /// The markdown content for the method page.
    pub content: String,
}

/// Generator for individual method pages.
///
/// When a trait has more methods than the configured threshold, this generator
/// creates separate markdown pages for each method, improving navigation in
/// large documentation sets.
pub struct MethodPageGenerator<'a> {
    /// Type renderer for signatures.
    type_renderer: &'a TypeRenderer<'a>,

    /// Trait name for breadcrumbs and titles.
    trait_name: &'a str,

    /// Base directory for method pages (e.g., "itertools/methods/").
    base_dir: String,

    /// Module path for breadcrumbs.
    module_path: &'a [String],
}

impl<'a> MethodPageGenerator<'a> {
    /// Create a new method page generator.
    ///
    /// # Arguments
    ///
    /// * `type_renderer` - Type renderer for method signatures
    /// * `trait_name` - Name of the trait containing the methods
    /// * `base_dir` - Base directory for method pages
    /// * `module_path` - Module path for breadcrumb generation
    #[must_use]
    pub const fn new(
        type_renderer: &'a TypeRenderer<'a>,
        trait_name: &'a str,
        base_dir: String,
        module_path: &'a [String],
    ) -> Self {
        Self {
            type_renderer,
            trait_name,
            base_dir,
            module_path,
        }
    }

    /// Generate a method page for a single method item.
    ///
    /// # Arguments
    ///
    /// * `method` - The method item from rustdoc
    /// * `docs` - Processed documentation string (with resolved links)
    ///
    /// # Returns
    ///
    /// A `MethodPage` containing the path and content, or `None` if not a function.
    #[must_use]
    pub fn generate_method_page(&self, method: &Item, docs: Option<String>) -> Option<MethodPage> {
        let name = method.name.as_deref()?;

        let ItemEnum::Function(f) = &method.inner else {
            return None;
        };

        let mut content = String::new();

        // Breadcrumb navigation
        self.write_breadcrumbs(&mut content, name);

        // Title with full qualified name
        _ = writeln!(content, "# `{}::{}`\n", self.trait_name, name);

        // Method signature in code block
        let generics = self.type_renderer.render_generics(&f.generics.params);
        let params: Vec<String> = f
            .sig
            .inputs
            .iter()
            .map(|(param_name, ty)| format!("{}: {}", param_name, self.type_renderer.render_type(ty)))
            .collect();
        let ret = f
            .sig
            .output
            .as_ref()
            .map(|ty| format!(" -> {}", self.type_renderer.render_type(ty)))
            .unwrap_or_default();

        _ = writeln!(content, "```rust");
        _ = writeln!(
            content,
            "fn {}{}({}){}",
            name,
            generics,
            params.join(", "),
            ret
        );
        _ = writeln!(content, "```\n");

        // Full documentation with adjusted heading levels
        if let Some(doc_content) = docs {
            // Adjust headings: method page titles are H1, so doc headings should be H2+
            let adjusted = RendererInternals::adjust_heading_levels(&doc_content, 1);
            _ = writeln!(content, "{adjusted}");
        }

        // Back link
        _ = writeln!(content, "\n---\n");
        _ = writeln!(
            content,
            "[← Back to `{}`](../index.md#{})",
            self.trait_name,
            crate::linker::AnchorUtils::slugify_anchor(self.trait_name)
        );

        let path = format!("{}{}.md", self.base_dir, name);

        Some(MethodPage { path, content })
    }

    /// Generate a summary link for a method (used in the main trait page).
    ///
    /// # Arguments
    ///
    /// * `method` - The method item
    /// * `first_line_doc` - First line of documentation for summary
    ///
    /// # Returns
    ///
    /// Markdown link to the method page with summary.
    #[must_use]
    pub fn generate_method_link(&self, method: &Item, first_line_doc: Option<&str>) -> Option<String> {
        let name = method.name.as_deref()?;

        let ItemEnum::Function(f) = &method.inner else {
            return None;
        };

        let generics = self.type_renderer.render_generics(&f.generics.params);
        let params: Vec<String> = f
            .sig
            .inputs
            .iter()
            .map(|(param_name, ty)| format!("{}: {}", param_name, self.type_renderer.render_type(ty)))
            .collect();
        let ret = f
            .sig
            .output
            .as_ref()
            .map(|ty| format!(" -> {}", self.type_renderer.render_type(ty)))
            .unwrap_or_default();

        let mut link = format!(
            "- [`{}::{}{}({}){}`](methods/{}.md)",
            self.trait_name,
            name,
            generics,
            params.join(", "),
            ret,
            name
        );

        if let Some(doc) = first_line_doc {
            _ = write!(link, "\n\n  {doc}");
        }

        _ = writeln!(link);

        Some(link)
    }

    /// Write breadcrumb navigation to the content.
    fn write_breadcrumbs(&self, content: &mut String, method_name: &str) {
        let mut crumbs = Vec::new();

        // Depth from method page to crate root:
        // methods/method.md -> .. -> module -> .. -> crate
        let depth = self.module_path.len() + 2; // +1 for trait dir, +1 for methods dir

        // Crate root link
        crumbs.push(format!("[crate]({}index.md)", "../".repeat(depth)));

        // Module path links
        for (i, segment) in self.module_path.iter().enumerate() {
            let levels_up = depth - i - 1;
            crumbs.push(format!("[{}]({}index.md)", segment, "../".repeat(levels_up)));
        }

        // Trait link (parent of methods dir)
        crumbs.push(format!("[{}](../index.md)", self.trait_name));

        // Current method (no link)
        crumbs.push(format!("`{method_name}`"));

        _ = writeln!(content, "*{}*\n", crumbs.join(" / "));
        _ = writeln!(content, "---\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{Crate, FunctionSignature, Target, Abi, FunctionHeader};
    use std::collections::HashMap;

    fn make_test_crate() -> Crate {
        Crate {
            root: rustdoc_types::Id(0),
            crate_version: None,
            includes_private: false,
            index: HashMap::new(),
            paths: HashMap::new(),
            external_crates: HashMap::new(),
            format_version: 0,
            target: Target {
                triple: String::new(),
                target_features: vec![],
            },
        }
    }

    fn make_function_item(name: &str) -> Item {
        Item {
            id: rustdoc_types::Id(1),
            crate_id: 0,
            name: Some(name.to_string()),
            span: None,
            visibility: rustdoc_types::Visibility::Public,
            docs: Some("Test documentation.".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: ItemEnum::Function(rustdoc_types::Function {
                sig: FunctionSignature {
                    inputs: vec![("self".to_string(), rustdoc_types::Type::Primitive("Self".to_string()))],
                    output: Some(rustdoc_types::Type::Primitive("bool".to_string())),
                    is_c_variadic: false,
                },
                generics: rustdoc_types::Generics {
                    params: vec![],
                    where_predicates: vec![],
                },
                header: FunctionHeader {
                    is_const: false,
                    is_unsafe: false,
                    is_async: false,
                    abi: Abi::Rust,
                },
                has_body: true,
            }),
        }
    }

    #[test]
    fn generate_method_page_creates_valid_content() {
        let krate = make_test_crate();
        let type_renderer = TypeRenderer::new(&krate);
        let module_path = vec!["itertools".to_string()];

        let generator = MethodPageGenerator::new(
            &type_renderer,
            "Itertools",
            "itertools/methods/".to_string(),
            &module_path,
        );

        let method = make_function_item("test_method");
        let page = generator.generate_method_page(&method, Some("Full docs here.".to_string()));

        assert!(page.is_some());
        let page = page.unwrap();

        assert_eq!(page.path, "itertools/methods/test_method.md");
        assert!(page.content.contains("# `Itertools::test_method`"));
        assert!(page.content.contains("Full docs here."));
        assert!(page.content.contains("Back to `Itertools`"));
    }

    #[test]
    fn generate_method_link_creates_valid_markdown() {
        let krate = make_test_crate();
        let type_renderer = TypeRenderer::new(&krate);
        let module_path = vec![];

        let generator = MethodPageGenerator::new(
            &type_renderer,
            "Iterator",
            "methods/".to_string(),
            &module_path,
        );

        let method = make_function_item("next");
        let link = generator.generate_method_link(&method, Some("Returns the next item."));

        assert!(link.is_some());
        let link = link.unwrap();

        assert!(link.contains("[`Iterator::next"));
        assert!(link.contains("](methods/next.md)"));
        assert!(link.contains("Returns the next item."));
    }
}
