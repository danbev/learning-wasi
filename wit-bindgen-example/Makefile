build:
	cargo build --target wasm32-unknown-unknown

component:
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm \
	-o example-component.wasm

inspect-wit:
	wasm-tools component wit example-component.wasm


clean:
	cargo clean
	rm example-component.wasm
