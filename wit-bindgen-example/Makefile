wit-bindgen: wit/component.wit
	wit-bindgen rust --macro-export  wit/component.wit --out-dir src

build: wit-bindgen
	cargo build --target wasm32-unknown-unknown

build-wasi: wit-bindgen
	cargo build --target wasm32-wasi

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm \
	-o example-wasi-component.wasm

component-wasi:
	wasm-tools component new ./target/wasm32-wasi/debug/wit_bindgen_example.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.wasm \
	-o example-wasi-component.wasm

inspect-wit:
	@wasm-tools component wit example-component.wasm

inspect-wasm-wat:
	@wasm-tools print target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm | rustfilt

inspect-wasi-wat:
	@wasm2wat target/wasm32-wasi/debug/wit_bindgen_example.wasm | rustfilt

inspect-component-wat:
	@wasm-tools print example-component.wasm | rustfilt

objdump-module:
	@wasm-tools objdump ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm

objdump-wasi-module:
	@wasm-tools objdump ./target/wasm32-wasi/debug/wit_bindgen_example.wasm 

objdump-component:
	@wasm-tools objdump example-component.wasm

dump-component:
	@wasm-tools dump example-component.wasm

# JavaScript targets
js-bindings:
	cd js && npm run bindings

js-run:
	cd js && npm run example

# Python targets
python-bindings:
	cd python && make bindings

python-run:
	cd python && make run

# Rust targets
rust-bindings:
	cd rust && cargo b --release

rust-run:
	cd rust && cargo r --release --bin main

clean:
	cargo clean
	rm example-component.wasm
