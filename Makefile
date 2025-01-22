
shahmeersgame.wasm: $(shell find src -type f)
	@forge build
	@cargo build \
		--release \
		--target wasm32-unknown-unknown
	@mv target/wasm32-unknown-unknown/release/shahmeersgame.wasm .
