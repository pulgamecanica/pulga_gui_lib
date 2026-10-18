.PHONY: help build test fmt clippy clean doc coverage gallery wasm-test

.DEFAULT_GOAL := help

help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*##/ {printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## cargo build
	cargo build

test: ## cargo test
	cargo test

fmt: ## cargo fmt
	cargo fmt

clippy: ## cargo clippy
	cargo clippy

clean: ## cargo clean
	cargo clean

doc: ## cargo doc --open
	cargo doc --open

coverage: ## cargo tarpaulin --lib (widget logic only, no GL/window needed)
	cargo tarpaulin --lib --out Stdout --out Html

gallery: ## Open a window showing one of each widget (needs a display)
	cargo run --example gallery

wasm-test: ## Run widget tests in headless Firefox via wasm32 (needs `make -C ../42Run wasm-image` once)
	podman run --rm \
		-v $(CURDIR):$(CURDIR):Z \
		-v $(CURDIR)/../PulgaWinLib:$(CURDIR)/../PulgaWinLib:ro \
		-w $(CURDIR) \
		localhost/pulga-wasm-test wasm-pack test --headless --firefox -- --lib
