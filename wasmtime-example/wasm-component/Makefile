build:
	cargo build --lib --target wasm32-unknown-unknown

build-wasi:
	cargo build --target wasm32-wasi

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wasm_component.wasm \
	-o example-wasm-component.wasm

component-wasi:
	wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
	--adapt wasi_snapshot_preview1.wasm \
	-o example-wasi-component.wasm

inspect-wit:
	@wasm-tools component wit example-component.wasm

inspect-wasm-wat:
	@wasm-tools print target/wasm32-unknown-unknown/debug/wasm_component.wasm | rustfilt

inspect-wasi-wat:
	@wasm2wat target/wasm32-wasi/debug/wasm_component.wasm | rustfilt

inspect-component-wat:
	@wasm-tools print example-component.wasm | rustfilt

clean:
	cargo clean
	rm example-component.wasm
