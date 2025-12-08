# Justfile for docs-md development

# Colors
cyan := '\033[36m'
green := '\033[32m'
yellow := '\033[33m'
red := '\033[31m'
reset := '\033[0m'

# Default recipe: regenerate docs
default: docs

# Show available recipes
help:
    @echo "{{cyan}}docs-md development recipes:{{reset}}"
    @echo "  {{green}}just{{reset}}        - Regenerate docs (clean + build + rustdoc + generate)"
    @echo "  {{green}}just quick{{reset}}  - Quick rebuild (skip cargo clean)"
    @echo "  {{green}}just build{{reset}}  - Build release binary"
    @echo "  {{green}}just test{{reset}}   - Run tests"
    @echo "  {{green}}just lint{{reset}}   - Run clippy"
    @echo "  {{green}}just bench{{reset}}  - Run benchmarks"
    @echo "  {{green}}just clean{{reset}}  - Remove docs/ and cargo clean"

# Check that cargo is available
[private]
check-cargo:
    @command -v cargo >/dev/null 2>&1 || { \
        echo "{{red}}Error: cargo is not installed{{reset}}"; \
        echo "Install Rust from https://rustup.rs"; \
        exit 1; \
    }

# Check that nightly toolchain is available
[private]
check-nightly: check-cargo
    @rustup toolchain list | grep -q nightly || { \
        echo "{{red}}Error: Rust nightly toolchain is not installed{{reset}}"; \
        echo ""; \
        echo "rustdoc JSON output requires the nightly toolchain."; \
        echo "Install it with:"; \
        echo ""; \
        echo "  {{cyan}}rustup toolchain install nightly{{reset}}"; \
        echo ""; \
        exit 1; \
    }

# Clean everything and regenerate documentation
docs: clean build rustdoc generate
    @echo "{{green}}Documentation generated successfully in docs/{{reset}}"

# Remove docs directory and run cargo clean
clean: check-cargo
    @echo "{{yellow}}Cleaning...{{reset}}"
    rm -rf docs/
    cargo clean
    @echo "{{green}}Clean complete{{reset}}"

# Build the release binary
build: check-cargo
    @echo "{{yellow}}Building release binary...{{reset}}"
    cargo build --release
    @echo "{{green}}Build complete: target/release/cargo-docs-md{{reset}}"

# Generate rustdoc JSON with private items
rustdoc: check-nightly
    @echo "{{yellow}}Generating rustdoc JSON (this may take a while)...{{reset}}"
    RUSTDOCFLAGS='-Z unstable-options --output-format json --document-private-items' cargo +nightly doc
    @echo "{{green}}Rustdoc JSON generated in target/doc/{{reset}}"

# Generate markdown documentation (assumes binary is built)
generate:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! -f ./target/release/cargo-docs-md ]]; then
        echo -e "{{red}}Error: Binary not found at target/release/cargo-docs-md{{reset}}"
        echo "Run 'just build' first"
        exit 1
    fi
    if [[ ! -d ./target/doc ]]; then
        echo -e "{{red}}Error: Rustdoc output not found at target/doc/{{reset}}"
        echo "Run 'just rustdoc' first"
        exit 1
    fi
    echo -e "{{yellow}}Generating markdown documentation...{{reset}}"
    ./target/release/cargo-docs-md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate cargo_docs_md
    echo -e "{{green}}Markdown docs generated in docs/{{reset}}"

# Quick regenerate (skip cargo clean, just rebuild docs)
quick: check-nightly
    @echo "{{yellow}}Quick rebuild starting...{{reset}}"
    rm -rf docs/
    @echo "{{yellow}}Building release binary...{{reset}}"
    cargo build --release
    @echo "{{yellow}}Generating markdown documentation...{{reset}}"
    ./target/release/cargo-docs-md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate cargo_docs_md
    @echo "{{green}}Quick rebuild complete - docs in docs/{{reset}}"

# Run tests
test: check-cargo
    @echo "{{yellow}}Running tests...{{reset}}"
    cargo test
    @echo "{{green}}All tests passed{{reset}}"

# Run clippy
lint: check-cargo
    @echo "{{yellow}}Running clippy...{{reset}}"
    cargo clippy -- -W clippy::pedantic
    @echo "{{green}}Lint complete{{reset}}"

# Run benchmarks
bench: check-cargo
    @echo "{{yellow}}Running benchmarks...{{reset}}"
    cargo bench
