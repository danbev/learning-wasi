bindings:
	python3 -m wasmtime.bindgen ../example-component.wasm --out-dir dist

run: 
	python3 example.py

clean:
	@${RM} -rf dist
