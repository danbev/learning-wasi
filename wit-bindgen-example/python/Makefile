bindings:
	python3 ~/work/wasm/wasmtime-py/wasmtime/bindgen ../example-wasi-component.wasm --out-dir dist

run: 
	python3 example.py

clean:
	@${RM} -rf dist
