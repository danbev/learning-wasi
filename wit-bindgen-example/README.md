### wit-bindgen example
This is a basic example of using [wit-bindgen]. More details can be found in
[wit-bindgen.md](../notes/wit-bindgen.md).

### Prerequisites

```console
$ rustup target add wasm32-unknown-unknown
```

```console
$ cargo install wasm-tools
```

### Building
First we need to compile our Rust code using the wasm32 target:
```console
$ make build-wasi
cargo build --target wasm32-wasi
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
```

Next, we need to generate a wasm component for the wasm that we built:
```console
$ make component-wasi 
wasm-tools component new ./target/wasm32-wasi/debug/wit_bindgen_example.wasm \
--adapt wasi_snapshot_preview1.wasm \
-o example-wasi-component.wasm
```

### Inspecting the wit
```console
$ make inspect-wit 
wasm-tools component wit example-component.wasm
default world example-component {
  export something: func(s: string)
}
```

###  Running the wasm component
Currently we have the following examples of running this wasm component:

#### JavaScript
The [JavaScript](./js) version can be run using:
```console
$ make js-bindings
$ make js-run
```

#### Python
The [Python](./python) version can be run using:
```console
$ make python-bindings
$ make python-run
```
#### Rust
The [Rust](./rust) version can be run using:
```console
$ make rust-bindings
$ make rust-run

[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
