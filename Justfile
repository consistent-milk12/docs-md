# Justfile for docs-md development

# Colors
cyan := '\033[36m'
green := '\033[32m'
yellow := '\033[33m'
red := '\033[31m'
reset := '\033[0m'

# Default recipe: show help
default: help

# Show available recipes
help:
    @echo "{{cyan}}docs-md development recipes:{{reset}}"
    @echo ""
    @echo "{{yellow}}Documentation:{{reset}}"
    @echo "  {{green}}just docs{{reset}}         - Full docs (clean + sources + build + generate with max detail)"
    @echo "  {{green}}just regen{{reset}}        - Regenerate generated_docs/ (quick, uses debug build)"
    @echo "  {{green}}just quick{{reset}}        - Quick rebuild generated_docs/ (uses release build)"
    @echo "  {{green}}just sources{{reset}}      - Collect dependency sources to .source_*/"
    @echo "  {{green}}just walkdir{{reset}}      - Generate only walkdir docs (for testing traits)"
    @echo ""
    @echo "{{yellow}}Testing:{{reset}}"
    @echo "  {{green}}just test{{reset}}         - Run all tests"
    @echo "  {{green}}just test-lib{{reset}}     - Run unit tests only"
    @echo "  {{green}}just test-int{{reset}}     - Run integration tests only"
    @echo "  {{green}}just test-filter P{{reset}} - Run tests matching pattern P"
    @echo ""
    @echo "{{yellow}}Development:{{reset}}"
    @echo "  {{green}}just build{{reset}}        - Build debug binary"
    @echo "  {{green}}just release{{reset}}      - Build release binary"
    @echo "  {{green}}just check{{reset}}        - Quick cargo check"
    @echo "  {{green}}just lint{{reset}}         - Run clippy (pedantic + nursery)"
    @echo "  {{green}}just errors{{reset}}       - Build and show only errors/warnings"
    @echo "  {{green}}just bench{{reset}}        - Run benchmarks"
    @echo "  {{green}}just clean{{reset}}        - Remove generated_docs/, .source_*/, and cargo clean"

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

# Clean everything and regenerate documentation with max detail
docs: clean release sources rustdoc generate
    @echo "{{green}}Documentation generated successfully in generated_docs/{{reset}}"

# Collect dependency sources to .source_*/
sources: release
    @echo "{{yellow}}Collecting dependency sources...{{reset}}"
    ./target/release/cargo-docs-md docs-md collect-sources
    @echo "{{green}}Sources collected{{reset}}"

# Remove generated_docs, .source_* directories and run cargo clean
clean: check-cargo
    @echo "{{yellow}}Cleaning...{{reset}}"
    rm -rf generated_docs/
    rm -rf .source_*/
    cargo clean
    @echo "{{green}}Clean complete{{reset}}"

# Build debug binary
build: check-cargo
    @echo "{{yellow}}Building debug binary...{{reset}}"
    cargo build
    @echo "{{green}}Build complete: target/debug/cargo-docs-md{{reset}}"

# Build release binary
release: check-cargo
    @echo "{{yellow}}Building release binary...{{reset}}"
    cargo build --release
    @echo "{{green}}Build complete: target/release/cargo-docs-md{{reset}}"

# Quick cargo check (fastest feedback)
check: check-cargo
    @echo "{{yellow}}Running cargo check...{{reset}}"
    cargo check
    @echo "{{green}}Check complete{{reset}}"

# Build and show only errors/warnings
errors: check-cargo
    @echo "{{yellow}}Building and filtering errors/warnings...{{reset}}"
    cargo build 2>&1 | grep -E "^error|^warning|^\s+-->" || echo "{{green}}No errors or warnings{{reset}}"

# Generate rustdoc JSON with private items
rustdoc: check-nightly
    @echo "{{yellow}}Generating rustdoc JSON (this may take a while)...{{reset}}"
    RUSTDOCFLAGS='-Z unstable-options --output-format json --document-private-items' cargo +nightly doc
    @echo "{{green}}Rustdoc JSON generated in target/doc/{{reset}}"

# Generate markdown documentation with max detail (assumes binary is built)
generate:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! -f ./target/release/cargo-docs-md ]]; then
        echo -e "{{red}}Error: Binary not found at target/release/cargo-docs-md{{reset}}"
        echo "Run 'just release' first"
        exit 1
    fi
    if [[ ! -d ./target/doc ]]; then
        echo -e "{{red}}Error: Rustdoc output not found at target/doc/{{reset}}"
        echo "Run 'just rustdoc' first"
        exit 1
    fi
    echo -e "{{yellow}}Generating markdown documentation...{{reset}}"
    ./target/release/cargo-docs-md docs-md --dir target/doc/ -o generated_docs/ \
        --primary-crate cargo_docs_md \
        --include-blanket-impls \
        --source-locations \
        --full-method-docs
    echo -e "{{green}}Markdown docs generated in generated_docs/{{reset}}"

# Quick regenerate (skip cargo clean, just rebuild docs with max detail)
quick: check-nightly
    @echo "{{yellow}}Quick rebuild starting...{{reset}}"
    rm -rf generated_docs/
    @echo "{{yellow}}Building release binary...{{reset}}"
    cargo build --release
    @echo "{{yellow}}Generating markdown documentation...{{reset}}"
    ./target/release/cargo-docs-md docs-md --dir target/doc/ -o generated_docs/ \
        --primary-crate cargo_docs_md \
        --include-blanket-impls \
        --source-locations \
        --full-method-docs
    @echo "{{green}}Quick rebuild complete - docs in generated_docs/{{reset}}"

# Run all tests
test: check-cargo
    @echo "{{yellow}}Running all tests...{{reset}}"
    cargo test
    @echo "{{green}}All tests passed{{reset}}"

# Run unit tests only
test-lib: check-cargo
    @echo "{{yellow}}Running unit tests...{{reset}}"
    cargo test --lib
    @echo "{{green}}Unit tests passed{{reset}}"

# Run integration tests only
test-int: check-cargo
    @echo "{{yellow}}Running integration tests...{{reset}}"
    cargo test --test integration_tests
    @echo "{{green}}Integration tests passed{{reset}}"

# Run tests matching a pattern
test-filter pattern: check-cargo
    @echo "{{yellow}}Running tests matching '{{pattern}}'...{{reset}}"
    cargo test {{pattern}}

# Run clippy (pedantic + nursery)
lint: check-cargo
    @echo "{{yellow}}Running clippy (pedantic + nursery)...{{reset}}"
    cargo clippy -- -W clippy::pedantic -W clippy::nursery
    @echo "{{green}}Lint complete{{reset}}"

# Run benchmarks
bench: check-cargo
    @echo "{{yellow}}Running benchmarks...{{reset}}"
    cargo bench

# Regenerate generated_docs/ quickly (no cargo clean, uses debug build)
# Uses maximum detail flags: blanket impls, source locations, full method docs
regen: build
    @echo "{{yellow}}Regenerating generated_docs/...{{reset}}"
    rm -rf generated_docs/
    ./target/debug/cargo-docs-md docs-md --dir target/doc/ -o generated_docs/ \
        --primary-crate cargo_docs_md \
        --include-blanket-impls \
        --source-locations \
        --full-method-docs
    @echo "{{green}}Documentation regenerated in generated_docs/{{reset}}"

# Generate only walkdir docs (useful for testing trait rendering)
walkdir: build
    @echo "{{yellow}}Generating walkdir docs...{{reset}}"
    rm -rf generated_docs/walkdir/
    ./target/debug/cargo-docs-md docs-md --path target/doc/walkdir.json -o generated_docs/walkdir --format nested
    @echo "{{green}}Walkdir docs generated in generated_docs/walkdir/{{reset}}"

# Show walkdir traits section (quick check)
walkdir-traits: walkdir
    @echo "{{cyan}}=== walkdir/index.md Traits Section ==={{reset}}"
    @grep -A30 "^## Traits" generated_docs/walkdir/index.md | head -35

# Verify test counts
test-count: check-cargo
    @echo "{{yellow}}Counting tests...{{reset}}"
    @echo "{{cyan}}Unit tests:{{reset}}"
    @cargo test --lib 2>&1 | grep -E "^test result:" || true
    @echo "{{cyan}}Integration tests:{{reset}}"
    @cargo test --test integration_tests 2>&1 | grep -E "^test result:" || true
