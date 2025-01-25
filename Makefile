
.PHONY: clean

WASM_SRC := target/wasm32-unknown-unknown/release/shahmeersgame.wasm

shahmeersgame.wasm: $(shell find src -type f)
	@forge build
	@rm -f ${WASM_SRC} shahmeersgame.wasm
	@cargo build \
		--release \
		--target wasm32-unknown-unknown
	@mv ${WASM_SRC} .

clean:
	@rm -rf \
		mutants.out.old \
		mutants.out \
		target
