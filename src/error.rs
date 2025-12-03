use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum Error {
    #[error("Failed to read file")]
    #[diagnostic(
        code(docs_md::io::read),
        help("Check that the file exists and is readable")
    )]
    FileRead(#[source] std::io::Error),

    #[error("Failed to parse rustdoc JSON")]
    #[diagnostic(
        code(docs_md::parse::json),
        help("Ensure the file is valid rustdoc JSON output (generated with --output-format json)")
    )]
    JsonParse(#[source] serde_json::Error),

    #[error("Failed to create output directory")]
    #[diagnostic(code(docs_md::io::mkdir))]
    CreateDir(#[source] std::io::Error),

    #[error("Failed to write file")]
    #[diagnostic(code(docs_md::io::write))]
    FileWrite(#[source] std::io::Error),

    #[error("Item not found in index: {0}")]
    #[diagnostic(code(docs_md::parse::item_not_found))]
    ItemNotFound(String),
}
