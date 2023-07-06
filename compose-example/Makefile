wasmtime_dir= /home/danielbevenius/work/wasm/wasmtime
wasi_preview_dir=${wasmtime_dir}/crates/wasi-preview1-component-adapter

# Create the core webassembly module
build-core-wasm:
	cargo b --target wasm32-wasi

component:
	wasm-tools component new ./target/wasm32-wasi/debug/compose_example.wasm \
	--adapt wasi_snapshot_preview1=wit-lib/wasi_preview1_component_adapter.command.wasm \
	-o target/example-component.wasm

build-core-static-eval-wasm:
	@cd static-eval && cargo b --target wasm32-wasi

component-static-eval:
	wasm-tools component new static-eval/target/wasm32-wasi/debug/static_eval.wasm \
	--adapt wasi_snapshot_preview1=wit-lib/wasi_preview1_component_adapter.command.wasm \
	-o target/static-eval-component.wasm

compose:
	wasm-tools compose -c config.yml \
	-o target/composed.wasm \
	target/static-eval-component.wasm

build-runner: 
	@cd runner && cargo b

run: 
	@cd runner && env SEEDWING_POLICY_NAME=dog cargo r

wit-bindgen-wasmcomponent:
	wit-bindgen rust wit/wasmcomponent.wit -w wasmcomponent

wit-bindgen-static-evaluator:
	wit-bindgen rust wit/wasmcomponent.wit -w static-evaluator

wasi_preview1_component_adapter_command:
	@cd ${wasi_preview_dir} && \
	cargo build -p wasi-preview1-component-adapter \
	--target wasm32-unknown-unknown --release \
	--features="command" --no-default-features
	@cp ${wasmtime_dir}/target/wasm32-wasi/release/wasi_snapshot_preview1.wasm wit-lib/wasi_preview1_component_adapter.command.wasm

clean:
	cargo clean

# The following targets are only for exploration of using a custom section
# but I don't think this will work for our current use case. 
add-policy:
	wasm-custom-section target/wasm32-wasi/debug/command_example.wasm add "seedwing:policy" < static-eval/policy.dog
	cp target/wasm32-wasi/debug/command_example.wasm.out target/wasm32-wasi/debug/command_example.wasm

list-sections:
	wasm-custom-section ./target/wasm32-wasi/debug/command_example.wasm list
