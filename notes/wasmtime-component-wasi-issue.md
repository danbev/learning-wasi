## wasmtime wasi component issue.
I've been able to create a wasm component and wanted to use a component
generated from a base wasi module.

When doing this I run [wasi.rs] into this following issue:
```console
$ cargo r --bin wasi
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/wasi`
Error: import `streams` has the wrong type

Caused by:
    0: instance export `drop-input-stream` has the wrong type
    1: expected func found nothing
```

[wasm.rs]: https://github.com/danbev/learning-wasi/blob/master/wasmtime-example/wasmtime/src/wasm.rs
[wasi.rs]: https://github.com/danbev/learning-wasi/blob/master/wasmtime-example/wasmtime/src/wasi.rs
