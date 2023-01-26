help: ## Display this message
	@grep -E '(^[a-zA-Z0-9_\-\.]+:.*?##.*$$)|(^##)' Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m## /[33m/'

all: help

build: ## Build
	cargo build

run: ## Run main binary
	cargo run

test: ## Run tests
	cargo test

format: ## Format code
	cargo fmt

check: ## Run code checks
	cargo check
	cargo fmt --check
