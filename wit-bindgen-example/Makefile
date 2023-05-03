build:
	cargo build --target wasm32-unknown-unknown

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm \
	-o example-component.wasm

inspect-wit:
	@wasm-tools component wit example-component.wasm

inspect-wasm-wat:
	@wasm-tools print target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm | rustfilt

inspect-component-wat:
	@wasm-tools print example-component.wasm | rustfilt

clean:
	cargo clean
	rm example-component.wasm
