ROOT := $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/../..)
ADMIN_DIR := crates/assets/js/admin
ASSETS_DIR := examples/admin-ui-refresh/assets
MANIFEST := examples/admin-ui-refresh/Cargo.toml

.PHONY: build frontend-assets check-assets wasm

build: wasm

frontend-assets:
	cd $(ROOT) && rm -rf $(ASSETS_DIR)/*
	cd $(ROOT) && TRAILBASE_ADMIN_BASE=/_/admin-refresh/ pnpm --dir $(ADMIN_DIR) exec vite build --outDir ../../../../$(ASSETS_DIR)

check-assets: frontend-assets
	grep -Fq '/_/admin-refresh/' $(ROOT)/$(ASSETS_DIR)/index.html
	! grep -Fq '/_/admin/assets' $(ROOT)/$(ASSETS_DIR)/index.html

wasm: check-assets
	cd $(ROOT) && cargo build --manifest-path $(MANIFEST) --release --target wasm32-wasip2
