## WebAssembly Component Model compose example
This is an example of a component model module importing another and using
wasm-tool compose to combine/compose the two.

### Building
First we build the base component module into core wasm module
```console
$ make build-core-wasm 
cargo b --target wasm32-wasi
```
Next we make a webassembly component out of it:
```console
$ make component
wasm-tools component new ./target/wasm32-wasi/debug/compose_example.wasm \
--adapt wasi_snapshot_preview1=wit-lib/wasi_preview1_component_adapter.command.wasm \
-o target/example-component.wasm
```

Then we build the core wasm module for the component that will import the one
we just built:
```console
$ make build-core-static-eval-wasm
```
And we also have to turn it into a component:
```console
$ make component-static-eval 
wasm-tools component new static-eval/target/wasm32-wasi/debug/static_eval.wasm \
--adapt wasi_snapshot_preview1=wit-lib/wasi_preview1_component_adapter.command.wasm \
-o target/static-eval-component.wasm
```

We are now ready to compose these two:
```console
$ make compose 
wasm-tools compose -c config.yml \
-o target/composed.wasm \
target/static-eval-component.wasm
[2023-07-05T12:57:07Z WARN ] instance `wasi:io/streams` will be imported because a dependency named `wasi:io/streams` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:filesystem/filesystem` will be imported because a dependency named `wasi:filesystem/filesystem` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/environment` will be imported because a dependency named `wasi:cli-base/environment` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/preopens` will be imported because a dependency named `wasi:cli-base/preopens` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/exit` will be imported because a dependency named `wasi:cli-base/exit` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stdin` will be imported because a dependency named `wasi:cli-base/stdin` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stdout` will be imported because a dependency named `wasi:cli-base/stdout` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stderr` will be imported because a dependency named `wasi:cli-base/stderr` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:io/streams` will be imported because a dependency named `wasi:io/streams` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:filesystem/filesystem` will be imported because a dependency named `wasi:filesystem/filesystem` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/environment` will be imported because a dependency named `wasi:cli-base/environment` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/preopens` will be imported because a dependency named `wasi:cli-base/preopens` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/exit` will be imported because a dependency named `wasi:cli-base/exit` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stdin` will be imported because a dependency named `wasi:cli-base/stdin` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stdout` will be imported because a dependency named `wasi:cli-base/stdout` could not be found
[2023-07-05T12:57:07Z WARN ] instance `wasi:cli-base/stderr` will be imported because a dependency named `wasi:cli-base/stderr` could not be found
composed component `target/composed.wasm`
```
We are now ready to run the composed.wasm we just generated using:
```console
$ make run
   Compiling runner v0.1.0 (/home/danielbevenius/work/wasm/learning-wasi/compose-example/runner)
    Finished dev [unoptimized + debuginfo] target(s) in 7.98s
     Running `target/debug/main`
WebAssembly Component Runner
In Engine::eval policy: pattern dog = {
    name: string,
    trained: boolean
}

"Result: Ok"
```
