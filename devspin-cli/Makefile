.PHONY: help build check test lint fmt clean run all

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build

check: ## Check the compiling
	cargo check

test: ## Run tests
	cargo test --all-features

lint: ## Run clippy linter
	cargo clippy -- -D warnings

fmt: ## Format code
	cargo fmt

clean: ## Clean build artifacts
	cargo clean

run: ## Run the CLI
	cargo run

doc: ## Generate documentation
	cargo doc --open

all: fmt check lint test build ## Run all checks
