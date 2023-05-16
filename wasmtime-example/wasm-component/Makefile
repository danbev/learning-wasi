build:
	cargo build --lib --target wasm32-unknown-unknown

build-wasi:
	cargo build --target wasm32-wasi

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wasm_component.wasm \
	-o example-wasm-component.wasm

wit-bindgen:
	wit-bindgen rust wit/wasmcomponent.wit


# The wasm32-wasi target supports wasi_snapshot_preview1 which is based on
# .witx files. This is not compatible with the component model. But we can
# create a component model from these by adapting the wasm module that
# wasm32-wasi generates. This is done using the --adapt option.
component-wasi-reactor:
	wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.reactor.wasm \
	-o example-wasi-component-reactor.wasm

# This is currently not used as there is not _start in the current example.
component-wasi-command:
	wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.command.wasm \
	-o example-wasi-component-command.wasm

inspect-wasm-wit:
	@wasm-tools component wit example-wasm-component.wasm

inspect-wasi-wit:
	@wasm-tools component wit example-wasi-component-reactor.wasm

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
