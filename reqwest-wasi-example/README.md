## reqwest-wasi example
This is an example of using reqwest-wasi to explore how using reqwest-wasi in
a wasm module works.

Currently it looks like there is not TLS (Transport Layer Security) so it can
only access http endpoints and not https endpoints.

### Prerequisites
This example used wasmedge which can be installed using the following command:
```console
$ curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -v 0.11.1
```

### Building
```console
$ cargo b
```

### Running
Start a simple http server:
```console
$ node server.js
```

Now, we can run the example which will access the http server started in the
previous step:
```console
$ wasmedge target/wasm32-wasi/debug/reqwest-wasi-example.wasm
```

