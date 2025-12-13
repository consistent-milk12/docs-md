//! Generator integration for source-enhanced documentation.
//!
//! This module provides the bridge between parsed source code and the
//! documentation generator, enabling features like:
//!
//! - Function body rendering in `<details>` blocks
//! - Private items section in modules
//! - Actual constant/static values
//! - Source file and line number references
//!
//! # Architecture
//!
//! Instead of modifying existing generator types, this module provides:
//!
//! - `SourceAccess` trait (in `traits`) - For accessing parsed source data
//! - Context and render helpers for source-enhanced content

mod context;
mod render;
mod traits;
