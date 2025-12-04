//! Markdown documentation generator for rustdoc JSON.
//!
//! This is the core module that transforms rustdoc JSON data into markdown files.
//! It handles the complete generation pipeline: traversing modules, rendering
//! different item types, and creating cross-reference links.
//!
//! # Architecture
//!
//! The generation process follows these steps:
//!
//! 1. **Setup**: Create output directory, build path and impl maps
//! 2. **Link Registry**: Build a registry mapping item IDs to file paths
//! 3. **Generation**: Recursively traverse modules and write markdown files
//!
//! # Output Formats
//!
//! Two output formats are supported:
//!
//! - **Flat**: All files in one directory (`module.md`, `parent__child.md`)
//! - **Nested**: Directory hierarchy (`module/index.md`, `parent/child/index.md`)
//!
//! # Item Rendering
//!
//! Each item type has a dedicated render function:
//!
//! - [`render_struct`] - Structs with fields and impls
//! - [`render_enum`] - Enums with variants and impls
//! - [`render_trait`] - Traits with methods and associated types
//! - [`render_function`] - Standalone functions
//! - [`render_macro`] - Declarative and procedural macros
//! - [`render_constant`] - Constants and statics
//! - [`render_type_alias`] - Type aliases

use crate::Args;
use crate::OutputFormat;
use crate::error::Error;
use crate::linker::LinkRegistry;
use crate::types::{render_generic_bound, render_generics, render_type, render_where_clause};
use fs_err as fs;
use indicatif::{ProgressBar, ProgressStyle};
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, StructKind, Visibility};
use std::collections::HashMap;
use std::path::Path;

use std::fmt::Write;

/// Generate markdown documentation from a parsed rustdoc JSON crate.
///
/// This is the main entry point for documentation generation. It orchestrates
/// the entire process: creating the output directory, building lookup tables,
/// and generating markdown files for each module.
///
/// # Arguments
///
/// * `krate` - The parsed rustdoc JSON crate
/// * `args` - CLI arguments containing output path, format, and options
///
/// # Returns
///
/// `Ok(())` on success, or an error if any file operation fails.
///
/// # Process
///
/// 1. Create the output directory
/// 2. Build the path map (item ID → module path)
/// 3. Build the impl map (type ID → impl blocks)
/// 4. Build the link registry for cross-references
/// 5. Generate markdown files (flat or nested format)
pub fn generate(krate: &Crate, args: &Args) -> Result<(), Error> {
    // Ensure the output directory exists (creates parent dirs as needed)
    fs::create_dir_all(&args.output).map_err(Error::CreateDir)?;

    // Get the root module - this is the crate's top-level module containing
    // all other items. The root ID is stored in krate.root.
    let root_item = krate
        .index
        .get(&krate.root)
        .ok_or_else(|| Error::ItemNotFound(krate.root.0.to_string()))?;

    // Build path map: maps item IDs to their full module paths.
    // Used for generating "defined in" information.
    let path_map = build_path_map(krate);

    // Build impl map: maps type IDs to all impl blocks for that type.
    // Used for rendering "Implementations" and "Trait Implementations" sections.
    let impl_map = build_impl_map(krate);

    // Build link registry: maps item IDs to their documentation file paths.
    // Used for creating cross-reference links between items.
    let is_flat = matches!(args.format, OutputFormat::Flat);
    let link_registry = LinkRegistry::build(krate, is_flat);

    // Set up progress bar for user feedback during generation.
    // Count includes root module plus all nested modules.
    let total_modules = count_modules(krate, root_item, args) + 1;
    let progress = ProgressBar::new(total_modules as u64);
    progress.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} modules")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Dispatch to format-specific generation function.
    // Both follow the same pattern: generate root index.md, then recurse into submodules.
    match args.format {
        OutputFormat::Flat => generate_flat(
            krate,
            root_item,
            &args.output,
            &path_map,
            &impl_map,
            &link_registry,
            args,
            &progress,
        )?,
        OutputFormat::Nested => generate_nested(
            krate,
            root_item,
            &args.output,
            &path_map,
            &impl_map,
            &link_registry,
            args,
            &progress,
        )?,
    }

    progress.finish_with_message("done");
    Ok(())
}

/// Count the total number of modules that will be generated.
///
/// Used to initialize the progress bar with the correct total.
/// Respects the `--include-private` flag when counting.
///
/// # Arguments
///
/// * `krate` - The crate to count modules in
/// * `item` - The current item (module) to start counting from
/// * `args` - CLI args for visibility filtering
fn count_modules(krate: &Crate, item: &Item, args: &Args) -> usize {
    let mut count = 0;

    // Only modules have child items to count
    if let ItemEnum::Module(module) = &item.inner {
        for item_id in &module.items {
            // Check if this child is a module we'll include
            if let Some(child) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &child.inner
                && should_include_item(child, args)
            {
                count += 1; // Count this module
                count += count_modules(krate, child, args); // Recurse into its children
            }
        }
    }

    count
}

/// Build a map from item ID to its full module path.
///
/// The rustdoc JSON `paths` field contains this information for all
/// items that can be linked to. We copy it into a `HashMap` for fast lookup.
///
/// # Example
///
/// For `std::collections::HashMap`, the path would be `["std", "collections", "HashMap"]`.
fn build_path_map(krate: &Crate) -> HashMap<Id, Vec<String>> {
    let mut map = HashMap::new();

    // The paths field in rustdoc JSON maps IDs to ItemSummary which contains the path
    for (id, path_info) in &krate.paths {
        map.insert(*id, path_info.path.clone());
    }

    map
}

/// Build a map from type ID to all impl blocks for that type.
///
/// This enables rendering the "Implementations" and "Trait Implementations"
/// sections for structs, enums, and other types.
///
/// # How it works
///
/// Rustdoc JSON stores impl blocks as separate items in the index.
/// Each impl has a `for_` field indicating what type it implements for.
/// We walk all items, find impls, and group them by their target type ID.
fn build_impl_map(krate: &Crate) -> HashMap<Id, Vec<&Impl>> {
    let mut map: HashMap<Id, Vec<&Impl>> = HashMap::new();

    // Scan all items in the crate index
    for item in krate.index.values() {
        // Only process impl blocks
        if let ItemEnum::Impl(impl_block) = &item.inner {
            // Try to get the ID of the type this impl is for.
            // This only works for ResolvedPath types (named types).
            if let Some(type_id) = get_type_id(&impl_block.for_) {
                map.entry(type_id).or_default().push(impl_block);
            }
        }
    }

    map
}

/// Extract the item ID from a Type if it's a resolved path.
///
/// Only `ResolvedPath` types (named types like `Vec`, `String`, `MyStruct`)
/// have associated IDs. Other types (primitives, references, etc.) return None.
fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id> {
    match ty {
        // ResolvedPath represents a named type with an ID we can link to
        rustdoc_types::Type::ResolvedPath(path) => Some(path.id),
        // Other type variants don't have IDs (primitives, tuples, references, etc.)
        _ => None,
    }
}

/// Generate documentation with flat file structure.
///
/// All markdown files are placed in a single directory. Module hierarchy
/// is encoded in filenames using double underscores as separators.
///
/// # Output Structure
///
/// ```text
/// output/
/// ├── index.md              # Crate root
/// ├── module_a.md           # Top-level module
/// ├── module_b.md           # Top-level module
/// ├── module_a__child.md    # Nested module (module_a::child)
/// └── module_a__child__deep.md  # Deeply nested
/// ```
fn generate_flat(
    krate: &Crate,
    root: &Item,
    output_dir: &Path,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    args: &Args,
    progress: &ProgressBar,
) -> Result<(), Error> {
    // Generate root module as index.md (the crate's main documentation page)
    let index_content = generate_module_markdown(
        krate,
        root,
        path_map,
        impl_map,
        link_registry,
        "index.md",
        args,
        true, // is_root = true
    );
    let index_path = output_dir.join("index.md");
    fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Generate each top-level submodule (which recurses into nested modules)
    if let ItemEnum::Module(module) = &root.inner {
        for item_id in &module.items {
            if let Some(item) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &item.inner
                && should_include_item(item, args)
            {
                generate_module_file_flat(
                    krate,
                    item,
                    output_dir,
                    path_map,
                    impl_map,
                    link_registry,
                    args,
                    progress,
                )?;
            }
        }
    }

    Ok(())
}

/// Generate documentation with nested directory structure.
///
/// Each module gets its own directory with an `index.md` file.
/// This mirrors the Rust module hierarchy in the filesystem.
///
/// # Output Structure
///
/// ```text
/// output/
/// ├── index.md                  # Crate root
/// ├── module_a/
/// │   ├── index.md              # module_a docs
/// │   └── child/
/// │       └── index.md          # module_a::child docs
/// └── module_b/
///     └── index.md              # module_b docs
/// ```
fn generate_nested(
    krate: &Crate,
    root: &Item,
    output_dir: &Path,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    args: &Args,
    progress: &ProgressBar,
) -> Result<(), Error> {
    // Generate root module as index.md in the output directory
    let index_content = generate_module_markdown(
        krate,
        root,
        path_map,
        impl_map,
        link_registry,
        "index.md",
        args,
        true, // is_root = true
    );
    let index_path = output_dir.join("index.md");
    fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Generate each top-level submodule (creates directories and recurses)
    if let ItemEnum::Module(module) = &root.inner {
        for item_id in &module.items {
            if let Some(item) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &item.inner
                && should_include_item(item, args)
            {
                generate_module_file_nested(
                    krate,
                    item,
                    output_dir,
                    path_map,
                    impl_map,
                    link_registry,
                    "", // Start with empty path prefix
                    args,
                    progress,
                )?;
            }
        }
    }

    Ok(())
}

/// Generate a single module file in flat format.
///
/// Creates `{module_name}.md` in the output directory and recursively
/// generates child modules with flattened names (e.g., `parent__child.md`).
fn generate_module_file_flat(
    krate: &Crate,
    item: &Item,
    output_dir: &Path,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    args: &Args,
    progress: &ProgressBar,
) -> Result<(), Error> {
    let name = item.name.as_deref().unwrap_or("unnamed");
    let current_file = format!("{name}.md");

    // Generate the markdown content for this module
    let content = generate_module_markdown(
        krate,
        item,
        path_map,
        impl_map,
        link_registry,
        &current_file,
        args,
        false, // is_root = false
    );

    // Write the file
    let file_path = output_dir.join(&current_file);
    fs::write(&file_path, content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Recursively generate nested modules with flattened names.
    // For module "foo" with child "bar", generates "foo__bar.md".
    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                && should_include_item(sub_item, args)
            {
                let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                // Build flattened name: parent__child
                let flat_name = format!("{name}__{sub_name}");
                generate_module_file_flat_recursive(
                    krate,
                    sub_item,
                    output_dir,
                    path_map,
                    impl_map,
                    link_registry,
                    args,
                    &flat_name,
                    progress,
                )?;
            }
        }
    }

    Ok(())
}

/// Recursively generate nested module files in flat format.
///
/// This is called for modules beyond the first level, using the accumulated
/// prefix to build the flattened filename.
///
/// # Arguments
///
/// * `prefix` - Accumulated path prefix (e.g., "`parent__child`")
fn generate_module_file_flat_recursive(
    krate: &Crate,
    item: &Item,
    output_dir: &Path,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    args: &Args,
    prefix: &str,
    progress: &ProgressBar,
) -> Result<(), Error> {
    // The prefix IS the filename (without .md extension)
    let current_file = format!("{prefix}.md");

    let content = generate_module_markdown(
        krate,
        item,
        path_map,
        impl_map,
        link_registry,
        &current_file,
        args,
        false,
    );

    let file_path = output_dir.join(&current_file);
    fs::write(&file_path, content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Continue recursively for any child modules
    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                && should_include_item(sub_item, args)
            {
                let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                // Extend the prefix: parent__child -> parent__child__grandchild
                let new_prefix = format!("{prefix}__{sub_name}");
                generate_module_file_flat_recursive(
                    krate,
                    sub_item,
                    output_dir,
                    path_map,
                    impl_map,
                    link_registry,
                    args,
                    &new_prefix,
                    progress,
                )?;
            }
        }
    }

    Ok(())
}

/// Generate a single module file in nested format.
///
/// Creates a directory for the module and an `index.md` file inside it.
/// Recursively creates subdirectories for child modules.
///
/// # Arguments
///
/// * `output_dir` - Parent directory to create module directory in
/// * `path_prefix` - Accumulated path for link resolution (e.g., "parent/child")
fn generate_module_file_nested(
    krate: &Crate,
    item: &Item,
    output_dir: &Path,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    path_prefix: &str,
    args: &Args,
    progress: &ProgressBar,
) -> Result<(), Error> {
    let name = item.name.as_deref().unwrap_or("unnamed");

    // Create directory for this module: output_dir/module_name/
    let module_dir = output_dir.join(name);
    fs::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

    // Compute the file path for link resolution
    // This tells the LinkRegistry where this file is relative to the output root
    let current_file = if path_prefix.is_empty() {
        format!("{name}/index.md")
    } else {
        format!("{path_prefix}/{name}/index.md")
    };

    let content = generate_module_markdown(
        krate,
        item,
        path_map,
        impl_map,
        link_registry,
        &current_file,
        args,
        false,
    );

    // Write index.md inside the module directory
    let file_path = module_dir.join("index.md");
    fs::write(&file_path, content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Update the path prefix for child modules
    let new_prefix = if path_prefix.is_empty() {
        name.to_string()
    } else {
        format!("{path_prefix}/{name}")
    };

    // Recursively generate child modules as subdirectories
    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                && should_include_item(sub_item, args)
            {
                generate_module_file_nested(
                    krate,
                    sub_item,
                    &module_dir, // Child's parent is this module's directory
                    path_map,
                    impl_map,
                    link_registry,
                    &new_prefix,
                    args,
                    progress,
                )?;
            }
        }
    }

    Ok(())
}

/// Check if an item should be included based on visibility settings.
///
/// By default, only public items are included. If `--include-private`
/// is set, all items are included regardless of visibility.
///
/// # Visibility Levels
///
/// - `Public` - Always included
/// - `Crate`, `Restricted`, `Default` - Only with `--include-private`
fn should_include_item(item: &Item, args: &Args) -> bool {
    match &item.visibility {
        Visibility::Public => true,
        _ => args.include_private,
    }
}

/// Generate the markdown content for a module.
///
/// This is the core rendering function that produces the markdown text
/// for a single module's documentation page. It handles:
///
/// - Title (Crate or Module heading)
/// - Module-level documentation
/// - Sections for each item type (Modules, Structs, Enums, etc.)
///
/// # Arguments
///
/// * `krate` - The full crate for looking up items
/// * `item` - The module item to generate docs for
/// * `path_map` - Maps item IDs to their full module paths
/// * `impl_map` - Maps type IDs to their impl blocks
/// * `link_registry` - For creating cross-reference links
/// * `current_file` - Path of this file (for relative link calculation)
/// * `args` - CLI args for visibility filtering
/// * `is_root` - True if this is the crate root module
///
/// # Output Structure
///
/// ```markdown
/// # Crate `name` (or Module `name`)
///
/// [module documentation]
///
/// ## Modules
/// - [submodule](link) - first line of docs
///
/// ## Structs
/// ### `StructName`
/// [struct definition and docs]
///
/// ## Enums
/// ...
/// ```
fn generate_module_markdown(
    krate: &Crate,
    item: &Item,
    path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
    link_registry: &LinkRegistry,
    current_file: &str,
    args: &Args,
    is_root: bool,
) -> String {
    let mut md = String::new();

    let name = item.name.as_deref().unwrap_or("crate");

    // === Title Section ===
    // Root module gets "Crate" prefix, others get "Module"
    if is_root {
        _ = write!(md, "# Crate `{name}`\n\n");
        // Show version if available (from Cargo.toml)
        if let Some(version) = &krate.crate_version {
            _ = write!(md, "**Version:** {version}\n\n");
        }
    } else {
        _ = write!(md, "# Module `{name}`\n\n");
    }

    // === Documentation Section ===
    // Module-level doc comments (//! or /*! */)
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // === Module Contents ===
    // Collect and categorize all items in this module by type.
    // Items are sorted into buckets for organized rendering.
    if let ItemEnum::Module(module) = &item.inner {
        // Buckets for each item type we render.
        // Modules and types store (Id, Item) for cross-reference linking.
        // Simple items just store the Item reference.
        let mut modules: Vec<(&Id, &Item)> = Vec::new();
        let mut structs: Vec<(&Id, &Item)> = Vec::new();
        let mut enums: Vec<(&Id, &Item)> = Vec::new();
        let mut traits: Vec<(&Id, &Item)> = Vec::new();
        let mut functions = Vec::new();
        let mut macros = Vec::new();
        let mut constants = Vec::new();
        let mut type_aliases = Vec::new();

        // Categorize each item in the module
        for item_id in &module.items {
            if let Some(child) = krate.index.get(item_id) {
                // Apply visibility filtering (--include-private flag)
                if !should_include_item(child, args) {
                    continue;
                }

                // Sort into the appropriate bucket based on item type
                match &child.inner {
                    ItemEnum::Module(_) => modules.push((item_id, child)),
                    ItemEnum::Struct(_) => structs.push((item_id, child)),
                    ItemEnum::Enum(_) => enums.push((item_id, child)),
                    ItemEnum::Trait(_) => traits.push((item_id, child)),
                    ItemEnum::Function(_) => functions.push(child),
                    ItemEnum::Macro(_) => macros.push(child),
                    ItemEnum::Constant { .. } => constants.push(child),
                    ItemEnum::TypeAlias(_) => type_aliases.push(child),
                    // Other items (imports, impl blocks, etc.) are not rendered directly
                    _ => {}
                }
            }
        }

        // === Render Each Section ===
        // Sections are rendered in a consistent order: Modules, Structs, Enums,
        // Traits, Functions, Macros, Constants, Type Aliases

        // --- Modules Section ---
        // List child modules with links and brief descriptions
        if !modules.is_empty() {
            md.push_str("## Modules\n\n");
            for (module_id, module_item) in modules {
                let module_name = module_item.name.as_deref().unwrap_or("unnamed");
                // Try to create a cross-reference link to the module's page
                if let Some(link) = link_registry.create_link(*module_id, current_file) {
                    _ = write!(md, "- {link}");
                } else {
                    // Fallback: just show the name without a link
                    _ = write!(md, "- **`{module_name}`**");
                }
                // Add first line of docs as a brief description
                if let Some(docs) = &module_item.docs
                    && let Some(first_line) = docs.lines().next()
                {
                    _ = write!(md, " - {first_line}");
                }
                md.push('\n');
            }
            md.push('\n');
        }

        // --- Structs Section ---
        if !structs.is_empty() {
            md.push_str("## Structs\n\n");
            for (item_id, struct_item) in structs {
                render_struct(&mut md, krate, *item_id, struct_item, path_map, impl_map);
            }
        }

        // --- Enums Section ---
        if !enums.is_empty() {
            md.push_str("## Enums\n\n");
            for (item_id, enum_item) in enums {
                render_enum(&mut md, krate, *item_id, enum_item, path_map, impl_map);
            }
        }

        // --- Traits Section ---
        if !traits.is_empty() {
            md.push_str("## Traits\n\n");
            for (item_id, trait_item) in traits {
                render_trait(&mut md, krate, *item_id, trait_item, path_map, impl_map);
            }
        }

        // --- Functions Section ---
        if !functions.is_empty() {
            md.push_str("## Functions\n\n");
            for func_item in functions {
                render_function(&mut md, krate, func_item);
            }
        }

        // --- Macros Section ---
        if !macros.is_empty() {
            md.push_str("## Macros\n\n");
            for macro_item in macros {
                render_macro(&mut md, macro_item);
            }
        }

        // --- Constants Section ---
        if !constants.is_empty() {
            md.push_str("## Constants\n\n");
            for const_item in constants {
                render_constant(&mut md, krate, const_item);
            }
        }

        // --- Type Aliases Section ---
        if !type_aliases.is_empty() {
            md.push_str("## Type Aliases\n\n");
            for alias_item in type_aliases {
                render_type_alias(&mut md, krate, alias_item);
            }
        }
    }

    md
}

/// Render a struct definition to markdown.
///
/// Produces a section with:
/// - Heading with struct name and generics
/// - Rust code block showing the struct definition
/// - Documentation from doc comments
/// - Fields section (for structs with documented fields)
/// - Implementations section (inherent and trait impls)
///
/// # Struct Kinds
///
/// Rust has three kinds of structs, each rendered differently:
/// - **Unit**: `struct Foo;`
/// - **Tuple**: `struct Foo(T, U);`
/// - **Plain** (named fields): `struct Foo { field: T }`
fn render_struct(
    md: &mut String,
    krate: &Crate,
    item_id: Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Struct(s) = &item.inner {
        // === Signature Section ===
        // Render generic parameters (e.g., `<T, U>`) and where clauses
        let generics = render_generics(&s.generics.params, krate);
        let where_clause = render_where_clause(&s.generics.where_predicates, krate);

        // Level 3 heading with name and generics (without where clause for brevity)
        _ = write!(md, "### `{name}{generics}`\n\n");

        // === Definition Code Block ===
        // Show the full struct definition in a fenced code block
        md.push_str("```rust\n");
        match &s.kind {
            // Unit struct: no fields, just a name
            StructKind::Unit => {
                _ = writeln!(md, "struct {name}{generics}{where_clause};");
            }

            // Tuple struct: unnamed fields accessed by index (e.g., self.0)
            StructKind::Tuple(fields) => {
                // Look up each field to get its type
                let field_types: Vec<String> = fields
                    .iter()
                    .filter_map(|id| id.as_ref()) // Skip None entries (private fields)
                    .filter_map(|id| krate.index.get(id))
                    .filter_map(|item| {
                        if let ItemEnum::StructField(ty) = &item.inner {
                            Some(render_type(ty, krate))
                        } else {
                            None
                        }
                    })
                    .collect();
                _ = writeln!(
                    md,
                    "struct {}{}({}){};",
                    name,
                    generics,
                    field_types.join(", "),
                    where_clause
                );
            }

            // Plain struct: named fields with visibility
            StructKind::Plain { fields, .. } => {
                _ = writeln!(md, "struct {name}{generics}{where_clause} {{");

                // Render each field with its visibility, name, and type
                for field_id in fields {
                    if let Some(field) = krate.index.get(field_id) {
                        let field_name = field.name.as_deref().unwrap_or("_");
                        if let ItemEnum::StructField(ty) = &field.inner {
                            // Show "pub" for public fields, nothing for private
                            let vis = match &field.visibility {
                                Visibility::Public => "pub ",
                                _ => "",
                            };
                            _ = writeln!(
                                md,
                                "    {}{}: {},",
                                vis,
                                field_name,
                                render_type(ty, krate)
                            );
                        }
                    }
                }
                md.push_str("}\n");
            }
        }
        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    // Include the struct's doc comment
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // === Fields Documentation ===
    // For plain structs, list fields that have their own doc comments
    if let ItemEnum::Struct(s) = &item.inner
        && let StructKind::Plain { fields, .. } = &s.kind
    {
        // Filter to only fields with documentation
        let documented_fields: Vec<_> = fields
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|f| f.docs.is_some())
            .collect();

        if !documented_fields.is_empty() {
            md.push_str("#### Fields\n\n");
            for field in documented_fields {
                let field_name = field.name.as_deref().unwrap_or("_");
                if let ItemEnum::StructField(ty) = &field.inner {
                    // Bold field name with its type
                    _ = write!(md, "- **`{}`**: `{}`", field_name, render_type(ty, krate));

                    // Indent the field's documentation
                    if let Some(docs) = &field.docs {
                        _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
                    }

                    md.push_str("\n\n");
                }
            }
        }
    }

    // === Implementations Section ===
    // Render inherent impls and trait implementations
    render_impl_blocks(md, krate, item_id, impl_map);
}

/// Render an enum definition to markdown.
///
/// Produces a section with:
/// - Heading with enum name and generics
/// - Rust code block showing the enum definition with all variants
/// - Documentation from doc comments
/// - Variants section (for variants with documentation)
/// - Implementations section (inherent and trait impls)
///
/// # Variant Kinds
///
/// Rust enums support three variant kinds:
/// - **Plain**: `Variant` (no data)
/// - **Tuple**: `Variant(T, U)` (positional data)
/// - **Struct**: `Variant { field: T }` (named fields)
fn render_enum(
    md: &mut String,
    krate: &Crate,
    item_id: Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Enum(e) = &item.inner {
        // === Signature Section ===
        let generics = render_generics(&e.generics.params, krate);
        let where_clause = render_where_clause(&e.generics.where_predicates, krate);

        _ = write!(md, "### `{name}{generics}`\n\n");

        // === Definition Code Block ===
        md.push_str("```rust\n");
        _ = writeln!(md, "enum {name}{generics}{where_clause} {{");

        // Render each variant based on its kind
        for variant_id in &e.variants {
            if let Some(variant) = krate.index.get(variant_id) {
                let variant_name = variant.name.as_deref().unwrap_or("_");
                if let ItemEnum::Variant(v) = &variant.inner {
                    match &v.kind {
                        // Plain variant: just a name, no data
                        rustdoc_types::VariantKind::Plain => {
                            _ = writeln!(md, "    {variant_name},");
                        }

                        // Tuple variant: positional fields like Option::Some(T)
                        rustdoc_types::VariantKind::Tuple(fields) => {
                            let field_types: Vec<String> = fields
                                .iter()
                                .filter_map(|id| id.as_ref())
                                .filter_map(|id| krate.index.get(id))
                                .filter_map(|item| {
                                    if let ItemEnum::StructField(ty) = &item.inner {
                                        Some(render_type(ty, krate))
                                    } else {
                                        None
                                    }
                                })
                                .collect();

                            _ = writeln!(md, "    {}({}),", variant_name, field_types.join(", "));
                        }

                        // Struct variant: named fields like Result::Err { source, ... }
                        rustdoc_types::VariantKind::Struct { fields, .. } => {
                            _ = writeln!(md, "    {variant_name} {{");

                            for field_id in fields {
                                if let Some(field) = krate.index.get(field_id) {
                                    let field_name = field.name.as_deref().unwrap_or("_");
                                    if let ItemEnum::StructField(ty) = &field.inner {
                                        _ = writeln!(
                                            md,
                                            "        {}: {},",
                                            field_name,
                                            render_type(ty, krate)
                                        );
                                    }
                                }
                            }
                            md.push_str("    },\n");
                        }
                    }
                }
            }
        }
        md.push_str("}\n");
        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // === Variants Documentation ===
    // List variants that have their own doc comments
    if let ItemEnum::Enum(e) = &item.inner {
        let documented_variants: Vec<_> = e
            .variants
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|v| v.docs.is_some())
            .collect();

        if !documented_variants.is_empty() {
            md.push_str("#### Variants\n\n");
            for variant in documented_variants {
                let variant_name = variant.name.as_deref().unwrap_or("_");
                _ = write!(md, "- **`{variant_name}`**");

                // Indent variant documentation for proper markdown formatting
                if let Some(docs) = &variant.docs {
                    _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
                }

                md.push_str("\n\n");
            }
        }
    }

    // === Implementations Section ===
    render_impl_blocks(md, krate, item_id, impl_map);
}

/// Render a trait definition to markdown.
///
/// Produces a section with:
/// - Heading with trait name and generics
/// - Rust code block showing trait signature with supertraits
/// - Documentation from doc comments
/// - Required methods section listing all trait items
///
/// # Trait Items
///
/// Traits can contain:
/// - **Methods**: `fn method(&self) -> T`
/// - **Associated Types**: `type Item;`
/// - **Associated Constants**: `const VALUE: T;`
///
/// Note: We don't render implementors here; that would require scanning
/// all impl blocks in the crate.
fn render_trait(
    md: &mut String,
    krate: &Crate,
    _item_id: Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    _impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Trait(t) = &item.inner {
        // === Signature Section ===
        let generics = render_generics(&t.generics.params, krate);
        let where_clause = render_where_clause(&t.generics.where_predicates, krate);

        _ = write!(md, "### `{name}{generics}`\n\n");

        // === Definition Code Block ===
        md.push_str("```rust\n");

        // Render supertrait bounds (e.g., `trait Foo: Clone + Send`)
        let bounds = if t.bounds.is_empty() {
            String::new()
        } else {
            let bound_strs: Vec<String> = t
                .bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!(": {}", bound_strs.join(" + "))
        };

        // Show abbreviated definition (... for body since items listed below)
        _ = writeln!(md, "trait {name}{generics}{bounds}{where_clause} {{ ... }}");
        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // === Required Methods Section ===
    // List all items defined in the trait (methods, associated types, etc.)
    if let ItemEnum::Trait(t) = &item.inner
        && !t.items.is_empty()
    {
        md.push_str("#### Required Methods\n\n");
        for method_id in &t.items {
            if let Some(method) = krate.index.get(method_id) {
                render_trait_item(md, krate, method);
            }
        }
    }
}

/// Render a single trait item (method, associated type, or associated constant).
///
/// Each item is rendered as a bullet point with its signature in backticks.
/// The first line of documentation is included for methods.
///
/// # Item Types
///
/// - **Function**: `fn method(&self, arg: T) -> U`
/// - **Associated Type**: `type Item: Bound = Default`
/// - **Associated Constant**: `const VALUE: T`
fn render_trait_item(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("_");

    match &item.inner {
        // Trait method: render full signature
        ItemEnum::Function(f) => {
            let generics = render_generics(&f.generics.params, krate);

            // Build parameter list: "name: Type, name2: Type2"
            let params: Vec<String> = f
                .sig
                .inputs
                .iter()
                .map(|(param_name, ty)| format!("{}: {}", param_name, render_type(ty, krate)))
                .collect();

            // Optional return type
            let ret = f
                .sig
                .output
                .as_ref()
                .map(|ty| format!(" -> {}", render_type(ty, krate)))
                .unwrap_or_default();

            _ = write!(
                md,
                "- `fn {}{}({}){}`",
                name,
                generics,
                params.join(", "),
                ret
            );

            // Include first line of docs as a brief description
            if let Some(docs) = &item.docs
                && let Some(first_line) = docs.lines().next()
            {
                _ = write!(md, "\n\n  {first_line}");
            }

            md.push_str("\n\n");
        }

        // Associated type: `type Item: Bound = DefaultType`
        ItemEnum::AssocType { bounds, type_, .. } => {
            // Show number of bounds (simplified)
            let bounds_str = if bounds.is_empty() {
                String::new()
            } else {
                format!(": {}", bounds.len())
            };
            // Show default type if provided
            let default_str = type_
                .as_ref()
                .map(|ty| format!(" = {}", render_type(ty, krate)))
                .unwrap_or_default();

            _ = write!(md, "- `type {name}{bounds_str}{default_str}`\n\n");
        }

        // Associated constant: `const VALUE: Type`
        ItemEnum::AssocConst { type_, .. } => {
            _ = write!(md, "- `const {}: {}`\n\n", name, render_type(type_, krate));
        }

        // Fallback for any other item type (shouldn't happen in traits)
        _ => {
            _ = write!(md, "- `{name}`\n\n");
        }
    }
}

/// Render a standalone function to markdown.
///
/// Produces a section with:
/// - Heading with function name
/// - Rust code block showing full signature
/// - Documentation from doc comments
///
/// # Function Modifiers
///
/// The signature includes applicable modifiers:
/// - `const fn` - Compile-time evaluable
/// - `async fn` - Returns a Future
/// - `unsafe fn` - Requires unsafe block to call
fn render_function(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Function(f) = &item.inner {
        // === Signature Components ===
        let generics = render_generics(&f.generics.params, krate);
        let where_clause = render_where_clause(&f.generics.where_predicates, krate);

        // Build parameter list: "name: Type, name2: Type2"
        let params: Vec<String> = f
            .sig
            .inputs
            .iter()
            .map(|(param_name, ty)| format!("{}: {}", param_name, render_type(ty, krate)))
            .collect();

        // Optional return type (unit functions omit it)
        let ret = f
            .sig
            .output
            .as_ref()
            .map(|ty| format!(" -> {}", render_type(ty, krate)))
            .unwrap_or_default();

        // Function modifiers from the header
        let is_async = if f.header.is_async { "async " } else { "" };
        let is_const = if f.header.is_const { "const " } else { "" };
        let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

        // === Render Output ===
        _ = write!(md, "### `{name}`\n\n");
        md.push_str("```rust\n");

        // Full signature: modifiers + fn + name + generics + params + return + where
        _ = writeln!(
            md,
            "{}{}{}fn {}{}({}){}{};",
            is_const,
            is_async,
            is_unsafe,
            name,
            generics,
            params.join(", "),
            ret,
            where_clause
        );

        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

/// Render a macro definition to markdown.
///
/// Produces a section with:
/// - Heading with macro name and `!` suffix
/// - Documentation from doc comments
///
/// Note: We don't show macro rules/implementation since rustdoc JSON
/// doesn't provide the full macro definition, only metadata.
fn render_macro(md: &mut String, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    // Macros are shown with the ! invocation syntax
    _ = write!(md, "### `{name}!`\n\n");

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

/// Render a constant definition to markdown.
///
/// Produces a section with:
/// - Heading with constant name
/// - Rust code block showing `const NAME: Type = value;`
/// - Documentation from doc comments
///
/// The value may be omitted if rustdoc couldn't determine it
/// (e.g., for complex const expressions).
fn render_constant(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Constant { type_, const_ } = &item.inner {
        _ = write!(md, "### `{name}`\n\n");

        md.push_str("```rust\n");

        // Include the value if known, otherwise just show type
        let value = const_
            .value
            .as_ref()
            .map(|v| format!(" = {v}"))
            .unwrap_or_default();

        _ = writeln!(
            md,
            "const {}: {}{};",
            name,
            render_type(type_, krate),
            value
        );

        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

/// Render a type alias to markdown.
///
/// Produces a section with:
/// - Heading with alias name and generics
/// - Rust code block showing `type Name<T> = TargetType;`
/// - Documentation from doc comments
///
/// # Example Output
///
/// ```markdown
/// ### `Result<T>`
///
/// ```rust
/// type Result<T> = std::result::Result<T, Error>;
/// ```
/// ```
fn render_type_alias(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::TypeAlias(ta) = &item.inner {
        // === Signature Components ===
        let generics = render_generics(&ta.generics.params, krate);
        let where_clause = render_where_clause(&ta.generics.where_predicates, krate);

        _ = write!(md, "### `{name}{generics}`\n\n");
        md.push_str("```rust\n");

        // Full definition: type Name<T> where ... = Target;
        _ = writeln!(
            md,
            "type {}{}{} = {};",
            name,
            generics,
            where_clause,
            render_type(&ta.type_, krate)
        );
        md.push_str("```\n\n");
    }

    // === Documentation Section ===
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

/// Render impl blocks for a given type.
///
/// This function looks up all impl blocks for a type and renders them
/// in two sections:
///
/// 1. **Implementations** - Inherent impls (methods defined directly on the type)
/// 2. **Trait Implementations** - Trait impls (`impl Trait for Type`)
///
/// # Impl Block Categories
///
/// - **Inherent**: `impl MyType { fn method(&self) {} }`
/// - **Trait**: `impl Clone for MyType { ... }`
/// - **Synthetic**: Auto-derived by compiler (Send, Sync) - skipped
///
/// # Arguments
///
/// * `item_id` - The ID of the type to render impls for
/// * `impl_map` - Pre-built map from type ID to impl blocks
fn render_impl_blocks(
    md: &mut String,
    krate: &Crate,
    item_id: Id,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    // Look up impl blocks for this type; return early if none
    let Some(impls) = impl_map.get(&item_id) else {
        return;
    };

    // === Partition Impls ===
    // Separate trait impls from inherent impls based on presence of trait_ field
    let (trait_impls, inherent_impls): (Vec<_>, Vec<_>) =
        impls.iter().partition(|i| i.trait_.is_some());

    // === Inherent Implementations ===
    // Methods defined directly on the type (e.g., `impl Vec<T> { fn new() ... }`)
    if !inherent_impls.is_empty() {
        md.push_str("#### Implementations\n\n");
        for impl_block in inherent_impls {
            render_impl_methods(md, krate, impl_block);
        }
    }

    // === Trait Implementations ===
    // Trait impls like `impl Clone for MyType`
    if !trait_impls.is_empty() {
        md.push_str("#### Trait Implementations\n\n");
        for impl_block in trait_impls {
            // Skip synthetic impls (auto-traits like Send, Sync, Unpin)
            // These are compiler-generated and usually not useful in docs
            if impl_block.is_synthetic {
                continue;
            }

            // Build the trait name with generic args (e.g., "Iterator<Item = T>")
            let trait_name = impl_block
                .trait_
                .as_ref()
                .map(|t| {
                    let mut name = t.path.clone();
                    if let Some(args) = &t.args {
                        name.push_str(&render_generic_args_for_impl(args, krate));
                    }
                    name
                })
                .unwrap_or_default();

            // Render generics and the target type
            let generics = render_generics(&impl_block.generics.params, krate);
            let for_type = render_type(&impl_block.for_, krate);

            // Handle special impl modifiers
            let unsafe_str = if impl_block.is_unsafe { "unsafe " } else { "" };
            let negative_str = if impl_block.is_negative { "!" } else { "" }; // e.g., `impl !Send`

            // Level 5 heading with full impl signature
            _ = write!(
                md,
                "##### `{unsafe_str}impl{generics} {negative_str}{trait_name} for {for_type}`\n\n"
            );

            // Render the methods/items provided by this trait impl
            render_impl_methods(md, krate, impl_block);
        }
    }
}

/// Render the items (methods, constants, types) within an impl block.
///
/// Each item is rendered as a bullet point. Items can be:
/// - **Functions/Methods**: Full signature with modifiers
/// - **Associated Constants**: `const NAME: Type`
/// - **Associated Types**: `type Name = Type`
///
/// For methods, the first line of documentation is included as a brief summary.
fn render_impl_methods(md: &mut String, krate: &Crate, impl_block: &Impl) {
    // Iterate over all items in the impl block
    for item_id in &impl_block.items {
        if let Some(item) = krate.index.get(item_id) {
            match &item.inner {
                // === Method ===
                // Render full signature: `[modifiers] fn name<T>(params) -> ReturnType`
                ItemEnum::Function(f) => {
                    let name = item.name.as_deref().unwrap_or("_");
                    let generics = render_generics(&f.generics.params, krate);

                    // Build parameter list
                    let params: Vec<String> = f
                        .sig
                        .inputs
                        .iter()
                        .map(|(param_name, ty)| {
                            format!("{}: {}", param_name, render_type(ty, krate))
                        })
                        .collect();

                    // Optional return type
                    let ret = f
                        .sig
                        .output
                        .as_ref()
                        .map(|ty| format!(" -> {}", render_type(ty, krate)))
                        .unwrap_or_default();

                    // Function modifiers
                    let is_async = if f.header.is_async { "async " } else { "" };
                    let is_const = if f.header.is_const { "const " } else { "" };
                    let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

                    // Render as bullet with signature in backticks
                    _ = write!(
                        md,
                        "- `{}{}{}fn {}{}({}){}`",
                        is_const,
                        is_async,
                        is_unsafe,
                        name,
                        generics,
                        params.join(", "),
                        ret
                    );

                    // Include first line of docs as brief description
                    if let Some(docs) = &item.docs
                        && let Some(first_line) = docs.lines().next()
                    {
                        _ = write!(md, "\n\n  {first_line}");
                    }

                    md.push_str("\n\n");
                }

                // === Associated Constant ===
                // `const NAME: Type` (value not shown in impl context)
                ItemEnum::AssocConst { type_, .. } => {
                    let name = item.name.as_deref().unwrap_or("_");
                    _ = write!(md, "- `const {}: {}`\n\n", name, render_type(type_, krate));
                }

                // === Associated Type ===
                // `type Name = ConcreteType` or just `type Name` if not specified
                ItemEnum::AssocType { type_, .. } => {
                    let name = item.name.as_deref().unwrap_or("_");

                    if let Some(ty) = type_ {
                        _ = write!(md, "- `type {} = {}`\n\n", name, render_type(ty, krate));
                    } else {
                        _ = write!(md, "- `type {name}`\n\n");
                    }
                }

                // Other item types in impl blocks are ignored
                _ => {}
            }
        }
    }
}

/// Render generic arguments for impl block signatures.
///
/// This is similar to `render_generic_args` in the types module, but
/// is used specifically for rendering trait names in impl blocks.
///
/// # Argument Forms
///
/// - **Angle bracketed**: `<T, U, Item = V>` (most common)
/// - **Parenthesized**: `(A, B) -> C` (for Fn traits)
/// - **Return type notation**: `(..)` (experimental)
///
/// # Example
///
/// For `impl Iterator<Item = u32> for MyType`, this renders the
/// `<Item = u32>` portion after "Iterator".
fn render_generic_args_for_impl(args: &rustdoc_types::GenericArgs, krate: &Crate) -> String {
    match args {
        // Standard angle bracket syntax: `<T, U, Item = V>`
        rustdoc_types::GenericArgs::AngleBracketed { args, constraints } => {
            // Render each generic argument (lifetime, type, const, or inferred)
            let mut parts: Vec<String> = args
                .iter()
                .map(|a| match a {
                    rustdoc_types::GenericArg::Lifetime(lt) => lt.clone(),
                    rustdoc_types::GenericArg::Type(ty) => render_type(ty, krate),
                    rustdoc_types::GenericArg::Const(c) => {
                        // Prefer computed value, fall back to expression
                        c.value.clone().unwrap_or_else(|| c.expr.clone())
                    }
                    rustdoc_types::GenericArg::Infer => "_".to_string(),
                })
                .collect();

            // Add associated type constraints (e.g., `Item = u32`, `Item: Display`)
            parts.extend(constraints.iter().map(|c| {
                // Constraint may have its own generic args (rare)
                let constraint_args = c
                    .args
                    .as_ref()
                    .map(|a| render_generic_args_for_impl(a, krate))
                    .unwrap_or_default();

                match &c.binding {
                    // Equality: `Item = SomeType`
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        let term_str = match term {
                            rustdoc_types::Term::Type(ty) => render_type(ty, krate),
                            rustdoc_types::Term::Constant(c) => {
                                c.value.clone().unwrap_or_else(|| c.expr.clone())
                            }
                        };
                        format!("{}{} = {}", c.name, constraint_args, term_str)
                    }
                    // Bound: `Item: Display + Debug`
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                        let bound_strs: Vec<String> = bounds
                            .iter()
                            .map(|b| render_generic_bound(b, krate))
                            .collect();
                        format!("{}{}: {}", c.name, constraint_args, bound_strs.join(" + "))
                    }
                }
            }));

            // Return empty string if no args, otherwise wrap in angle brackets
            if parts.is_empty() {
                String::new()
            } else {
                format!("<{}>", parts.join(", "))
            }
        }

        // Parenthesized syntax for Fn traits: `Fn(A, B) -> C`
        rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
            let input_strs: Vec<String> = inputs.iter().map(|t| render_type(t, krate)).collect();
            let ret = output
                .as_ref()
                .map(|t| format!(" -> {}", render_type(t, krate)))
                .unwrap_or_default();
            format!("({}){}", input_strs.join(", "), ret)
        }

        // Return type notation (experimental feature)
        rustdoc_types::GenericArgs::ReturnTypeNotation => " (..)".to_string(),
    }
}
