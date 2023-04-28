## Python example
This example show how the same wasm component, which is build by the parent
project of this directory, can be run in by python.

### Prerequisites
```console
$ pip install wasmtime
Defaulting to user installation because normal site-packages is not writeable
Collecting wasmtime
  Downloading wasmtime-8.0.1-py3-none-manylinux1_x86_64.whl (6.8 MB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 6.8/6.8 MB 20.4 MB/s eta 0:00:00
Installing collected packages: wasmtime
Successfully installed wasmtime-8.0.1
```

### Building
```console
$ make bindings
python -m wasmtime.bindgen ../example-component.wasm --out-dir dist
Generating dist/__init__.py
Generating dist/example-component.core0.wasm
Generating dist/intrinsics.py
```

### Running
```console
$ make run
python example.py
something was passed: bajja
```
