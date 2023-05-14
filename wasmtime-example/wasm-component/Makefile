build:
	cargo build --lib --target wasm32-unknown-unknown

build-wasi:
	cargo build --target wasm32-wasi

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wasm_component.wasm \
	-o example-wasm-component.wasm

component-wasi-reactor:
	wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.reactor.wasm \
	-o example-wasi-component-reactor.wasm

component-wasi-command:
	wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.command.wasm \
	-o example-wasi-component-command.wasm

inspect-wit:
	@wasm-tools component wit example-wasm-component.wasm

inspect-wasm-wat:
	@wasm-tools print target/wasm32-unknown-unknown/debug/wasm_component.wasm | rustfilt

inspect-wasi-wat:
	@wasm2wat target/wasm32-wasi/debug/wasm_component.wasm | rustfilt

inspect-wasm-component-wat:
	@wasm-tools print example-wasm-component.wasm | rustfilt

inspect-wasi-component-wat:
	@wasm-tools print example-wasi-component.wasm | rustfilt

clean:
	cargo clean
	rm example-component.wasm
