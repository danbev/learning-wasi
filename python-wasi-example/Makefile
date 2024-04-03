build:
	PYO3_NO_PYTHON=1 cargo b --lib --target wasm32-wasi --release

# This is needed to preopen the CPython standard libraries.
run:
	env RUST_BACKTRACE=1 wasmtime \
	--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
	--dir=./target/wasm32-wasi/wasi-deps/usr/local/lib/python3.12/site-packages::/usr/local/lib/python3.12/site-packages \
	--env PYTHONPATH=/usr/local/lib/python3.12/site-packages \
        target/wasm32-wasi/release/python_example.wasm

download-numpy-wasi:
	mkdir -p target/wasm32-wasi/wasi-deps/usr/local/lib/python3.12/site-packages
	cd target/wasm32-wasi/wasi-deps/usr/local/lib/python3.12/site-packages && \
	curl -OL https://github.com/dicej/wasi-wheels/releases/download/v0.0.1/numpy-wasi.tar.gz && \
	gunzip -dc numpy-wasi.tar.gz | tar xvf -

## Just using this as an example of how a python package can be used from
## a wasm module. But I've n not been able to get this to work with numpy.
emoji-dep: DEP=emoji
emoji-dep: dep

dep: 
	source venv/bin/activate && pip install ${DEP} \
	--target target/wasm32-wasi/wasi-deps/usr/local/lib/python3.12/site-packages \
	--only-binary :all: \
	--implementation py \
	--abi none \
	--platform any \
	--python-version "3.12"


download-sysroot:
	wget -P ./tmp https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sysroot-20.0.tar.gz

download-clangrt:
	wget -P ./tmp https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/libclang_rt.builtins-wasm32-wasi-20.0.tar.gz

download-python:
	wget -P ./tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/python%2F3.11.4%2B20230714-11be424/libpython-3.11.4-wasi-sdk-20.0.tar.gz
