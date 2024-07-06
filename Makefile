WASM_FLAGS=--no-typescript \
      --no-pack -t web \
      -d assets
build: src/lib.rs
	wasm-pack build --dev $(WASM_FLAGS)
release: src/lib.rs
	wasm-pack build $(WASM_FLAGS)

test: src/lib.rs
	wasm-pack test --node

run: compose.yaml
	docker compose up -d


clean: assets/enku.*
	rm -f assets/enku.*
	cargo clean
	docker compose down

