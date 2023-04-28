###


### Prerequisites

```console
$ rustup target add wasm32-wasi
```

```console
$ cargo install wasm-tools
```

### Building
First we need to compile our Rust code using the wasm32 target:
```console
$ make build
cargo build --target wasm32-unknown-unknown
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
```

Next, we need to generate a wasm component for the wasm that we built:
```console
$ make component 
wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm \
-o example-component.wasm
```

### Inspecting the wit
```console
$ make inspect-wit 
wasm-tools component wit example-component.wasm
default world example-component {
  export something: func(s: string)
}
```
