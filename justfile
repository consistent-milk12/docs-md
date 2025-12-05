# Justfile for docs-md development

# Default recipe: regenerate docs
default: docs

# Clean everything and regenerate documentation
docs: clean build rustdoc generate

# Remove docs directory and run cargo clean
clean:
    rm -rf docs/
    cargo clean

# Build the release binary
build:
    cargo build --release

# Generate rustdoc JSON with private items
rustdoc:
    RUSTDOCFLAGS='-Z unstable-options --output-format json --document-private-items' cargo +nightly doc

# Generate markdown documentation (assumes binary is built)
generate:
    ./target/release/docs_md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate docs_md

# Quick regenerate (skip cargo clean, just rebuild docs)
quick:
    rm -rf docs/
    cargo build --release
    ./target/release/docs_md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate docs_md

# Run tests
test:
    cargo test

# Run clippy
lint:
    cargo clippy -- -W clippy::pedantic

# Run benchmarks
bench:
    cargo bench
