# Makefile for docs-md development

.PHONY: all docs clean build rustdoc generate quick test lint bench check-cargo check-nightly help

# Colors for output
CYAN := \033[36m
GREEN := \033[32m
YELLOW := \033[33m
RED := \033[31m
RESET := \033[0m

# Default target: regenerate docs
all: docs

# Show available targets
help:
	@echo "$(CYAN)docs-md development targets:$(RESET)"
	@echo "  $(GREEN)make$(RESET)        - Regenerate docs (clean + build + rustdoc + generate)"
	@echo "  $(GREEN)make quick$(RESET)  - Quick rebuild (skip cargo clean)"
	@echo "  $(GREEN)make build$(RESET)  - Build release binary"
	@echo "  $(GREEN)make test$(RESET)   - Run tests"
	@echo "  $(GREEN)make lint$(RESET)   - Run clippy"
	@echo "  $(GREEN)make bench$(RESET)  - Run benchmarks"
	@echo "  $(GREEN)make clean$(RESET)  - Remove docs/ and cargo clean"

# Check that cargo is available
check-cargo:
	@command -v cargo >/dev/null 2>&1 || { \
		echo "$(RED)Error: cargo is not installed$(RESET)"; \
		echo "Install Rust from https://rustup.rs"; \
		exit 1; \
	}

# Check that nightly toolchain is available
check-nightly: check-cargo
	@rustup toolchain list | grep -q nightly || { \
		echo "$(RED)Error: Rust nightly toolchain is not installed$(RESET)"; \
		echo ""; \
		echo "rustdoc JSON output requires the nightly toolchain."; \
		echo "Install it with:"; \
		echo ""; \
		echo "  $(CYAN)rustup toolchain install nightly$(RESET)"; \
		echo ""; \
		exit 1; \
	}

# Clean everything and regenerate documentation
docs: clean build rustdoc generate
	@echo "$(GREEN)Documentation generated successfully in docs/$(RESET)"

# Remove docs directory and run cargo clean
clean: check-cargo
	@echo "$(YELLOW)Cleaning...$(RESET)"
	@rm -rf docs/
	@cargo clean
	@echo "$(GREEN)Clean complete$(RESET)"

# Build the release binary
build: check-cargo
	@echo "$(YELLOW)Building release binary...$(RESET)"
	@cargo build --release
	@echo "$(GREEN)Build complete: target/release/cargo-docs-md$(RESET)"

# Generate rustdoc JSON with private items
rustdoc: check-nightly
	@echo "$(YELLOW)Generating rustdoc JSON (this may take a while)...$(RESET)"
	@RUSTDOCFLAGS='-Z unstable-options --output-format json --document-private-items' cargo +nightly doc
	@echo "$(GREEN)Rustdoc JSON generated in target/doc/$(RESET)"

# Generate markdown documentation (assumes binary is built)
generate:
	@if [ ! -f ./target/release/cargo-docs-md ]; then \
		echo "$(RED)Error: Binary not found at target/release/cargo-docs-md$(RESET)"; \
		echo "Run 'make build' first"; \
		exit 1; \
	fi
	@if [ ! -d ./target/doc ]; then \
		echo "$(RED)Error: Rustdoc output not found at target/doc/$(RESET)"; \
		echo "Run 'make rustdoc' first"; \
		exit 1; \
	fi
	@echo "$(YELLOW)Generating markdown documentation...$(RESET)"
	@./target/release/cargo-docs-md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate cargo_docs_md
	@echo "$(GREEN)Markdown docs generated in docs/$(RESET)"

# Quick regenerate (skip cargo clean, just rebuild docs)
quick: check-nightly
	@echo "$(YELLOW)Quick rebuild starting...$(RESET)"
	@rm -rf docs/
	@echo "$(YELLOW)Building release binary...$(RESET)"
	@cargo build --release
	@echo "$(YELLOW)Generating markdown documentation...$(RESET)"
	@./target/release/cargo-docs-md --dir target/doc/ -o docs/ --mdbook --search-index --primary-crate cargo_docs_md
	@echo "$(GREEN)Quick rebuild complete - docs in docs/$(RESET)"

# Run tests
test: check-cargo
	@echo "$(YELLOW)Running tests...$(RESET)"
	@cargo test
	@echo "$(GREEN)All tests passed$(RESET)"

# Run clippy
lint: check-cargo
	@echo "$(YELLOW)Running clippy...$(RESET)"
	@cargo clippy -- -W clippy::pedantic
	@echo "$(GREEN)Lint complete$(RESET)"

# Run benchmarks
bench: check-cargo
	@echo "$(YELLOW)Running benchmarks...$(RESET)"
	@cargo bench
