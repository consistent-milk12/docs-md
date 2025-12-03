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

/// Generate markdown documentation from a parsed crate
pub fn generate(krate: &Crate, args: &Args) -> Result<(), Error> {
    // Create output directory
    fs::create_dir_all(&args.output).map_err(Error::CreateDir)?;

    // Get the root module
    let root_item = krate
        .index
        .get(&krate.root)
        .ok_or_else(|| Error::ItemNotFound(krate.root.0.to_string()))?;

    // Build path map for all items
    let path_map = build_path_map(krate);

    // Build impl map for all types
    let impl_map = build_impl_map(krate);

    // Build link registry for cross-references
    let is_flat = matches!(args.format, OutputFormat::Flat);
    let link_registry = LinkRegistry::build(krate, is_flat);

    // Count total modules for progress bar (root + all submodules)
    let total_modules = count_modules(krate, root_item, args) + 1;
    let progress = ProgressBar::new(total_modules as u64);
    progress.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} modules")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Generate documentation based on format
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

/// Count the total number of modules in a crate
fn count_modules(krate: &Crate, item: &Item, args: &Args) -> usize {
    let mut count = 0;
    if let ItemEnum::Module(module) = &item.inner {
        for item_id in &module.items {
            if let Some(child) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &child.inner
                    && should_include_item(child, args) {
                        count += 1;
                        count += count_modules(krate, child, args);
                    }
        }
    }
    count
}

/// Build a map from item ID to its full path
fn build_path_map(krate: &Crate) -> HashMap<Id, Vec<String>> {
    let mut map = HashMap::new();

    for (id, path_info) in &krate.paths {
        map.insert(*id, path_info.path.clone());
    }

    map
}

/// Build a map from type ID to all impl blocks for that type
fn build_impl_map(krate: &Crate) -> HashMap<Id, Vec<&Impl>> {
    let mut map: HashMap<Id, Vec<&Impl>> = HashMap::new();

    for item in krate.index.values() {
        if let ItemEnum::Impl(impl_block) = &item.inner {
            // Get the ID of the type this impl is for
            if let Some(type_id) = get_type_id(&impl_block.for_) {
                map.entry(type_id).or_default().push(impl_block);
            }
        }
    }

    map
}

/// Extract the ID from a Type if it's a resolved path
fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id> {
    match ty {
        rustdoc_types::Type::ResolvedPath(path) => Some(path.id),
        _ => None,
    }
}

/// Generate flat structure (one file per module)
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
    // Generate root module as index.md
    let index_content = generate_module_markdown(
        krate,
        root,
        path_map,
        impl_map,
        link_registry,
        "index.md",
        args,
        true,
    );
    let index_path = output_dir.join("index.md");
    fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Generate each submodule
    if let ItemEnum::Module(module) = &root.inner {
        for item_id in &module.items {
            if let Some(item) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &item.inner
                    && should_include_item(item, args) {
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

/// Generate nested structure (directory per module)
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
    // Generate root module as index.md
    let index_content = generate_module_markdown(
        krate,
        root,
        path_map,
        impl_map,
        link_registry,
        "index.md",
        args,
        true,
    );
    let index_path = output_dir.join("index.md");
    fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Generate each submodule recursively
    if let ItemEnum::Module(module) = &root.inner {
        for item_id in &module.items {
            if let Some(item) = krate.index.get(item_id)
                && let ItemEnum::Module(_) = &item.inner
                    && should_include_item(item, args) {
                        generate_module_file_nested(
                            krate,
                            item,
                            output_dir,
                            path_map,
                            impl_map,
                            link_registry,
                            "",
                            args,
                            progress,
                        )?;
                    }
        }
    }

    Ok(())
}

/// Generate a single module file (flat structure)
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
    let current_file = format!("{}.md", name);
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

    // Recursively generate nested modules with flattened names
    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                    && should_include_item(sub_item, args) {
                        let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                        let flat_name = format!("{}__{}", name, sub_name);
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
    let current_file = format!("{}.md", prefix);
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

    // Continue recursively
    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                    && should_include_item(sub_item, args) {
                        let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                        let new_prefix = format!("{}__{}", prefix, sub_name);
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

/// Generate a single module file (nested structure)
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
    let module_dir = output_dir.join(name);

    fs::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

    let current_file = if path_prefix.is_empty() {
        format!("{}/index.md", name)
    } else {
        format!("{}/{}/index.md", path_prefix, name)
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
    let file_path = module_dir.join("index.md");

    fs::write(&file_path, content).map_err(Error::FileWrite)?;
    progress.inc(1);

    // Recursively generate nested modules
    let new_prefix = if path_prefix.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", path_prefix, name)
    };

    if let ItemEnum::Module(module) = &item.inner {
        for sub_id in &module.items {
            if let Some(sub_item) = krate.index.get(sub_id)
                && let ItemEnum::Module(_) = &sub_item.inner
                    && should_include_item(sub_item, args) {
                        generate_module_file_nested(
                            krate,
                            sub_item,
                            &module_dir,
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

/// Check if an item should be included based on visibility settings
fn should_include_item(item: &Item, args: &Args) -> bool {
    match &item.visibility {
        Visibility::Public => true,
        _ => args.include_private,
    }
}

/// Generate markdown content for a module
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

    // Title
    if is_root {
        md.push_str(&format!("# Crate `{}`\n\n", name));
        if let Some(version) = &krate.crate_version {
            md.push_str(&format!("**Version:** {}\n\n", version));
        }
    } else {
        md.push_str(&format!("# Module `{}`\n\n", name));
    }

    // Documentation
    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // Module contents
    if let ItemEnum::Module(module) = &item.inner {
        let mut modules: Vec<(&Id, &Item)> = Vec::new();
        let mut structs: Vec<(&Id, &Item)> = Vec::new();
        let mut enums: Vec<(&Id, &Item)> = Vec::new();
        let mut traits: Vec<(&Id, &Item)> = Vec::new();
        let mut functions = Vec::new();
        let mut macros = Vec::new();
        let mut constants = Vec::new();
        let mut type_aliases = Vec::new();

        for item_id in &module.items {
            if let Some(child) = krate.index.get(item_id) {
                if !should_include_item(child, args) {
                    continue;
                }

                match &child.inner {
                    ItemEnum::Module(_) => modules.push((item_id, child)),
                    ItemEnum::Struct(_) => structs.push((item_id, child)),
                    ItemEnum::Enum(_) => enums.push((item_id, child)),
                    ItemEnum::Trait(_) => traits.push((item_id, child)),
                    ItemEnum::Function(_) => functions.push(child),
                    ItemEnum::Macro(_) => macros.push(child),
                    ItemEnum::Constant { .. } => constants.push(child),
                    ItemEnum::TypeAlias(_) => type_aliases.push(child),
                    _ => {}
                }
            }
        }

        // Modules section
        if !modules.is_empty() {
            md.push_str("## Modules\n\n");
            for (module_id, module_item) in modules {
                let module_name = module_item.name.as_deref().unwrap_or("unnamed");
                // Create link if available
                if let Some(link) = link_registry.create_link(module_id, current_file) {
                    md.push_str(&format!("- {}", link));
                } else {
                    md.push_str(&format!("- **`{}`**", module_name));
                }
                if let Some(docs) = &module_item.docs
                    && let Some(first_line) = docs.lines().next() {
                        md.push_str(&format!(" - {}", first_line));
                    }
                md.push('\n');
            }
            md.push('\n');
        }

        // Structs section
        if !structs.is_empty() {
            md.push_str("## Structs\n\n");
            for (item_id, struct_item) in structs {
                render_struct(&mut md, krate, item_id, struct_item, path_map, impl_map);
            }
        }

        // Enums section
        if !enums.is_empty() {
            md.push_str("## Enums\n\n");
            for (item_id, enum_item) in enums {
                render_enum(&mut md, krate, item_id, enum_item, path_map, impl_map);
            }
        }

        // Traits section
        if !traits.is_empty() {
            md.push_str("## Traits\n\n");
            for (item_id, trait_item) in traits {
                render_trait(&mut md, krate, item_id, trait_item, path_map, impl_map);
            }
        }

        // Functions section
        if !functions.is_empty() {
            md.push_str("## Functions\n\n");
            for func_item in functions {
                render_function(&mut md, krate, func_item);
            }
        }

        // Macros section
        if !macros.is_empty() {
            md.push_str("## Macros\n\n");
            for macro_item in macros {
                render_macro(&mut md, macro_item);
            }
        }

        // Constants section
        if !constants.is_empty() {
            md.push_str("## Constants\n\n");
            for const_item in constants {
                render_constant(&mut md, krate, const_item);
            }
        }

        // Type aliases section
        if !type_aliases.is_empty() {
            md.push_str("## Type Aliases\n\n");
            for alias_item in type_aliases {
                render_type_alias(&mut md, krate, alias_item);
            }
        }
    }

    md
}

fn render_struct(
    md: &mut String,
    krate: &Crate,
    item_id: &Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Struct(s) = &item.inner {
        // Render full signature
        let generics = render_generics(&s.generics.params, krate);
        let where_clause = render_where_clause(&s.generics.where_predicates, krate);

        md.push_str(&format!("### `{}{}`\n\n", name, generics));

        // Show definition
        md.push_str("```rust\n");
        match &s.kind {
            StructKind::Unit => {
                md.push_str(&format!("struct {}{}{};\n", name, generics, where_clause));
            }
            StructKind::Tuple(fields) => {
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
                md.push_str(&format!(
                    "struct {}{}({}){};\n",
                    name,
                    generics,
                    field_types.join(", "),
                    where_clause
                ));
            }
            StructKind::Plain { fields, .. } => {
                md.push_str(&format!("struct {}{}{} {{\n", name, generics, where_clause));
                for field_id in fields {
                    if let Some(field) = krate.index.get(field_id) {
                        let field_name = field.name.as_deref().unwrap_or("_");
                        if let ItemEnum::StructField(ty) = &field.inner {
                            let vis = match &field.visibility {
                                Visibility::Public => "pub ",
                                _ => "",
                            };
                            md.push_str(&format!(
                                "    {}{}: {},\n",
                                vis,
                                field_name,
                                render_type(ty, krate)
                            ));
                        }
                    }
                }
                md.push_str("}\n");
            }
        }
        md.push_str("```\n\n");
    }

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // Render fields with documentation
    if let ItemEnum::Struct(s) = &item.inner
        && let StructKind::Plain { fields, .. } = &s.kind {
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
                        md.push_str(&format!(
                            "- **`{}`**: `{}`",
                            field_name,
                            render_type(ty, krate)
                        ));
                        if let Some(docs) = &field.docs {
                            md.push_str(&format!("\n\n  {}", docs.replace('\n', "\n  ")));
                        }
                        md.push_str("\n\n");
                    }
                }
            }
        }

    // Render impl blocks
    render_impl_blocks(md, krate, item_id, impl_map);
}

fn render_enum(
    md: &mut String,
    krate: &Crate,
    item_id: &Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Enum(e) = &item.inner {
        let generics = render_generics(&e.generics.params, krate);
        let where_clause = render_where_clause(&e.generics.where_predicates, krate);

        md.push_str(&format!("### `{}{}`\n\n", name, generics));

        // Show definition
        md.push_str("```rust\n");
        md.push_str(&format!("enum {}{}{} {{\n", name, generics, where_clause));
        for variant_id in &e.variants {
            if let Some(variant) = krate.index.get(variant_id) {
                let variant_name = variant.name.as_deref().unwrap_or("_");
                if let ItemEnum::Variant(v) = &variant.inner {
                    match &v.kind {
                        rustdoc_types::VariantKind::Plain => {
                            md.push_str(&format!("    {},\n", variant_name));
                        }
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
                            md.push_str(&format!(
                                "    {}({}),\n",
                                variant_name,
                                field_types.join(", ")
                            ));
                        }
                        rustdoc_types::VariantKind::Struct { fields, .. } => {
                            md.push_str(&format!("    {} {{\n", variant_name));
                            for field_id in fields {
                                if let Some(field) = krate.index.get(field_id) {
                                    let field_name = field.name.as_deref().unwrap_or("_");
                                    if let ItemEnum::StructField(ty) = &field.inner {
                                        md.push_str(&format!(
                                            "        {}: {},\n",
                                            field_name,
                                            render_type(ty, krate)
                                        ));
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

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // Render variants with documentation
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
                md.push_str(&format!("- **`{}`**", variant_name));
                if let Some(docs) = &variant.docs {
                    md.push_str(&format!("\n\n  {}", docs.replace('\n', "\n  ")));
                }
                md.push_str("\n\n");
            }
        }
    }

    // Render impl blocks
    render_impl_blocks(md, krate, item_id, impl_map);
}

fn render_trait(
    md: &mut String,
    krate: &Crate,
    _item_id: &Id,
    item: &Item,
    _path_map: &HashMap<Id, Vec<String>>,
    _impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Trait(t) = &item.inner {
        let generics = render_generics(&t.generics.params, krate);
        let where_clause = render_where_clause(&t.generics.where_predicates, krate);

        md.push_str(&format!("### `{}{}`\n\n", name, generics));

        // Show definition
        md.push_str("```rust\n");
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
        md.push_str(&format!(
            "trait {}{}{}{} {{ ... }}\n",
            name, generics, bounds, where_clause
        ));
        md.push_str("```\n\n");
    }

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }

    // Render trait items
    if let ItemEnum::Trait(t) = &item.inner
        && !t.items.is_empty() {
            md.push_str("#### Required Methods\n\n");
            for method_id in &t.items {
                if let Some(method) = krate.index.get(method_id) {
                    render_trait_item(md, krate, method);
                }
            }
        }
}

fn render_trait_item(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("_");

    match &item.inner {
        ItemEnum::Function(f) => {
            let generics = render_generics(&f.generics.params, krate);

            // Build parameter list
            let params: Vec<String> = f
                .sig
                .inputs
                .iter()
                .map(|(param_name, ty)| format!("{}: {}", param_name, render_type(ty, krate)))
                .collect();

            let ret = f
                .sig
                .output
                .as_ref()
                .map(|ty| format!(" -> {}", render_type(ty, krate)))
                .unwrap_or_default();

            md.push_str(&format!(
                "- `fn {}{}({}){}`",
                name,
                generics,
                params.join(", "),
                ret
            ));

            if let Some(docs) = &item.docs
                && let Some(first_line) = docs.lines().next() {
                    md.push_str(&format!("\n\n  {}", first_line));
                }
            md.push_str("\n\n");
        }
        ItemEnum::AssocType { bounds, type_, .. } => {
            let bounds_str = if bounds.is_empty() {
                String::new()
            } else {
                format!(": {}", bounds.len())
            };
            let default_str = type_
                .as_ref()
                .map(|ty| format!(" = {}", render_type(ty, krate)))
                .unwrap_or_default();
            md.push_str(&format!(
                "- `type {}{}{}`\n\n",
                name, bounds_str, default_str
            ));
        }
        ItemEnum::AssocConst { type_, .. } => {
            md.push_str(&format!(
                "- `const {}: {}`\n\n",
                name,
                render_type(type_, krate)
            ));
        }
        _ => {
            md.push_str(&format!("- `{}`\n\n", name));
        }
    }
}

fn render_function(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Function(f) = &item.inner {
        let generics = render_generics(&f.generics.params, krate);
        let where_clause = render_where_clause(&f.generics.where_predicates, krate);

        // Build parameter list
        let params: Vec<String> = f
            .sig
            .inputs
            .iter()
            .map(|(param_name, ty)| format!("{}: {}", param_name, render_type(ty, krate)))
            .collect();

        let ret = f
            .sig
            .output
            .as_ref()
            .map(|ty| format!(" -> {}", render_type(ty, krate)))
            .unwrap_or_default();

        let is_async = if f.header.is_async { "async " } else { "" };
        let is_const = if f.header.is_const { "const " } else { "" };
        let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

        md.push_str(&format!("### `{}`\n\n", name));
        md.push_str("```rust\n");
        md.push_str(&format!(
            "{}{}{}fn {}{}({}){}{};\n",
            is_const,
            is_async,
            is_unsafe,
            name,
            generics,
            params.join(", "),
            ret,
            where_clause
        ));
        md.push_str("```\n\n");
    }

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

fn render_macro(md: &mut String, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");
    md.push_str(&format!("### `{}!`\n\n", name));

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

fn render_constant(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::Constant { type_, const_ } = &item.inner {
        md.push_str(&format!("### `{}`\n\n", name));
        md.push_str("```rust\n");
        let value = const_
            .value
            .as_ref()
            .map(|v| format!(" = {}", v))
            .unwrap_or_default();
        md.push_str(&format!(
            "const {}: {}{};\n",
            name,
            render_type(type_, krate),
            value
        ));
        md.push_str("```\n\n");
    }

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

fn render_type_alias(md: &mut String, krate: &Crate, item: &Item) {
    let name = item.name.as_deref().unwrap_or("unnamed");

    if let ItemEnum::TypeAlias(ta) = &item.inner {
        let generics = render_generics(&ta.generics.params, krate);
        let where_clause = render_where_clause(&ta.generics.where_predicates, krate);

        md.push_str(&format!("### `{}{}`\n\n", name, generics));
        md.push_str("```rust\n");
        md.push_str(&format!(
            "type {}{}{} = {};\n",
            name,
            generics,
            where_clause,
            render_type(&ta.type_, krate)
        ));
        md.push_str("```\n\n");
    }

    if let Some(docs) = &item.docs {
        md.push_str(docs);
        md.push_str("\n\n");
    }
}

/// Render impl blocks for a given type
fn render_impl_blocks(
    md: &mut String,
    krate: &Crate,
    item_id: &Id,
    impl_map: &HashMap<Id, Vec<&Impl>>,
) {
    let Some(impls) = impl_map.get(item_id) else {
        return;
    };

    // Separate trait impls from inherent impls
    let (trait_impls, inherent_impls): (Vec<_>, Vec<_>) =
        impls.iter().partition(|i| i.trait_.is_some());

    // Render inherent impls (methods defined directly on the type)
    if !inherent_impls.is_empty() {
        md.push_str("#### Implementations\n\n");
        for impl_block in inherent_impls {
            render_impl_methods(md, krate, impl_block);
        }
    }

    // Render trait impls
    if !trait_impls.is_empty() {
        md.push_str("#### Trait Implementations\n\n");
        for impl_block in trait_impls {
            if impl_block.is_synthetic {
                // Skip auto-derived traits like Send, Sync
                continue;
            }

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

            let generics = render_generics(&impl_block.generics.params, krate);
            let for_type = render_type(&impl_block.for_, krate);

            let unsafe_str = if impl_block.is_unsafe { "unsafe " } else { "" };
            let negative_str = if impl_block.is_negative { "!" } else { "" };

            md.push_str(&format!(
                "##### `{}impl{} {}{} for {}`\n\n",
                unsafe_str, generics, negative_str, trait_name, for_type
            ));

            // Render methods in this impl block
            render_impl_methods(md, krate, impl_block);
        }
    }
}

/// Render methods within an impl block
fn render_impl_methods(md: &mut String, krate: &Crate, impl_block: &Impl) {
    for item_id in &impl_block.items {
        if let Some(item) = krate.index.get(item_id) {
            match &item.inner {
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

                    let ret = f
                        .sig
                        .output
                        .as_ref()
                        .map(|ty| format!(" -> {}", render_type(ty, krate)))
                        .unwrap_or_default();

                    let is_async = if f.header.is_async { "async " } else { "" };
                    let is_const = if f.header.is_const { "const " } else { "" };
                    let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

                    md.push_str(&format!(
                        "- `{}{}{}fn {}{}({}){}`",
                        is_const,
                        is_async,
                        is_unsafe,
                        name,
                        generics,
                        params.join(", "),
                        ret
                    ));

                    if let Some(docs) = &item.docs
                        && let Some(first_line) = docs.lines().next() {
                            md.push_str(&format!("\n\n  {}", first_line));
                        }
                    md.push_str("\n\n");
                }
                ItemEnum::AssocConst { type_, .. } => {
                    let name = item.name.as_deref().unwrap_or("_");
                    md.push_str(&format!(
                        "- `const {}: {}`\n\n",
                        name,
                        render_type(type_, krate)
                    ));
                }
                ItemEnum::AssocType { type_, .. } => {
                    let name = item.name.as_deref().unwrap_or("_");
                    if let Some(ty) = type_ {
                        md.push_str(&format!(
                            "- `type {} = {}`\n\n",
                            name,
                            render_type(ty, krate)
                        ));
                    } else {
                        md.push_str(&format!("- `type {}`\n\n", name));
                    }
                }
                _ => {}
            }
        }
    }
}

/// Render generic arguments for impl blocks
fn render_generic_args_for_impl(args: &rustdoc_types::GenericArgs, krate: &Crate) -> String {
    match args {
        rustdoc_types::GenericArgs::AngleBracketed { args, constraints } => {
            let mut parts: Vec<String> = args
                .iter()
                .map(|a| match a {
                    rustdoc_types::GenericArg::Lifetime(lt) => lt.clone(),
                    rustdoc_types::GenericArg::Type(ty) => render_type(ty, krate),
                    rustdoc_types::GenericArg::Const(c) => {
                        c.value.clone().unwrap_or_else(|| c.expr.clone())
                    }
                    rustdoc_types::GenericArg::Infer => "_".to_string(),
                })
                .collect();
            parts.extend(constraints.iter().map(|c| {
                let constraint_args = c
                    .args
                    .as_ref()
                    .map(|a| render_generic_args_for_impl(a, krate))
                    .unwrap_or_default();
                match &c.binding {
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        let term_str = match term {
                            rustdoc_types::Term::Type(ty) => render_type(ty, krate),
                            rustdoc_types::Term::Constant(c) => {
                                c.value.clone().unwrap_or_else(|| c.expr.clone())
                            }
                        };
                        format!("{}{} = {}", c.name, constraint_args, term_str)
                    }
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                        let bound_strs: Vec<String> = bounds
                            .iter()
                            .map(|b| render_generic_bound(b, krate))
                            .collect();
                        format!("{}{}: {}", c.name, constraint_args, bound_strs.join(" + "))
                    }
                }
            }));
            if parts.is_empty() {
                String::new()
            } else {
                format!("<{}>", parts.join(", "))
            }
        }
        rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
            let input_strs: Vec<String> = inputs.iter().map(|t| render_type(t, krate)).collect();
            let ret = output
                .as_ref()
                .map(|t| format!(" -> {}", render_type(t, krate)))
                .unwrap_or_default();
            format!("({}){}", input_strs.join(", "), ret)
        }
        rustdoc_types::GenericArgs::ReturnTypeNotation => " (..)".to_string(),
    }
}
