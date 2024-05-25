build: src/lib.rs
	$(MAKE) fix
	cargo fmt
	cargo dev-lib
	wasm-bindgen \
	target/wasm32-unknown-unknown/debug/enku.wasm \
	--target web --out-dir assets \
	--no-typescript --remove-name-section \
	--remove-producers-section
release: src/lib.rs
	$(MAKE) fix
	cargo fmt
	cargo lib
	wasm-bindgen \
	target/wasm32-unknown-unknown/release/enku.wasm \
	--target web --out-dir assets \
	--no-typescript --remove-name-section \
	--remove-producers-section

fix:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo clippy --fix --allow-dirty --allow-staged -r

server: compose.yaml
	docker compose up -d


clean: assets/enku.*
	rm -f assets/enku.*
	cargo clean
	docker compose down

