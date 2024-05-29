WASM_FLAGS=--no-typescript \
      --no-pack -t web \
      -d assets
build: src/lib.rs
	$(MAKE) fix
	cargo fmt
	wasm-pack build --dev $(WASM_FLAGS)
release: src/lib.rs
	$(MAKE) fix
	cargo fmt
	wasm-pack build $(WASM_FLAGS)

fix:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo clippy --fix --allow-dirty --allow-staged -r

server: compose.yaml
	docker compose up -d


clean: assets/enku.*
	rm -f assets/enku.*
	cargo clean
	docker compose down

