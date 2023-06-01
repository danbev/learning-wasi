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
Reading that I think that the function is missing, that is not imported from
the host and not that it exists and the type if incorrect. So lets try to
figure out what is goind on here.

We can inspect the component.wasm
```console
$ cd wasmtime-example/wasm-component
$ make inspect-wasi-component-wat > wasi-component.wat
```
Opening `wasi-component.wat` we can see that there are number of imports which
are expected as core wasm module was compiled using wasm32-wasi.

There are also the following `streams` module imports:
```
    (import "exit" "exit" (func $wasi_snapshot_preview1::bindings::exit::exit::wit_import (;8;) (type 0)))

    (import "streams" "drop-input-stream"
      (func $wasi_snapshot_preview1::bindings::streams::drop_input_stream::wit_import (;9;) (type 0))
    )
    (import "streams" "drop-output-stream"
      (func $wasi_snapshot_preview1::bindings::streams::drop_output_stream::wit_import (;10;) (type 0))
    )
```
Where is this coming from?
If we inspect the core wasm module we can verify that there are no stream
imports:
```
  (import "wasi_snapshot_preview1" "fd_write" (func $wasi::lib_generated::wasi_snapshot_preview1::fd_write (type 8)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (type 4)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (type 4)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 1)))
```

And we created the wasm component using the following command:
```rust
$ wasm-tools component new ./target/wasm32-wasi/debug/wasm_component.wasm \
  --adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.reactor.wasm \
  -o example-wasi-component-reactor.wasm
```
Notice that we are specifying `--adapt` with `wasi_snapshot_preview1` name.

After that we will then run [wasmtime/src/wasi.rs](./wasmtime/src/wasi.rs)
which is what is generating the error we are seeing. So for some reason when
making the component, the import `drop-input-stream` was generated. This
function needs to be provided by the host, which is this case is us as we are
the ones that are writing wasi.rs. 

One thing to note is that there are other imports that are provided/available
without us having to specify them manually, like `preopens` but we don't provided
them. 

Looking at the core module wat we have the following imports:
```
  (import "wasi_snapshot_preview1" "fd_write" (func $wasi::lib_generated::wasi_snapshot_preview1::fd_write (typ      e 8)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (type 4))      )   
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_g      et (type 4)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (type 1)))
```

And if we take a look at the wasi_preview1_component_adapter.reactor.wasm wat
(which is the polyfill we specify with --adapt):
```
  (import "streams" "drop-input-stream" (func $wasi_preview1_component_adapter::bindings::streams::drop_input_s      tream::wit_import (type 0)))
```
We have the following imports:
```
name: streams
name: filesystem
name: environment
name: preopens
name: exit
```

We can inspect the wit for the component using:
```console
$ make inspect-wasi-wit 
interface streams {
  type input-stream = u32

  type output-stream = u32

  record stream-error {
  }

  drop-input-stream: func(this: input-stream)

  write: func(this: output-stream, buf: list<u8>) -> result<u64, stream-error>

  blocking-write: func(this: output-stream, buf: list<u8>) -> result<u64, stream-error>

  drop-output-stream: func(this: output-stream)
}

interface filesystem {
  use self.streams.{output-stream}

  type descriptor = u32

  type filesize = u64

  enum descriptor-type {
    unknown,
    block-device,
    character-device,
    directory,
    fifo,
    symbolic-link,
    regular-file,
    socket,
  }

  enum error-code {
    access,
    would-block,
    already,
    bad-descriptor,
    busy,
    deadlock,
    quota,
    exist,
    file-too-large,
    illegal-byte-sequence,
    in-progress,
    interrupted,
    invalid,
    io,
    is-directory,
    loop,
    too-many-links,
    message-size,
    name-too-long,
    no-device,
    no-entry,
    no-lock,
    insufficient-memory,
    insufficient-space,
    not-directory,
    not-empty,
    not-recoverable,
    unsupported,
    no-tty,
    no-such-device,
    overflow,
    not-permitted,
    pipe,
    read-only,
    invalid-seek,
    text-file-busy,
    cross-device,
  }

  write-via-stream: func(this: descriptor, offset: filesize) -> output-stream

  append-via-stream: func(this: descriptor) -> output-stream

  get-type: func(this: descriptor) -> result<descriptor-type, error-code>

  drop-descriptor: func(this: descriptor)
}

interface environment {
  get-environment: func() -> list<tuple<string, string>>
}

interface preopens {
  use self.streams.{input-stream, output-stream}
  use self.filesystem.{descriptor}

  record stdio-preopens {
    stdin: input-stream,
    stdout: output-stream,
    stderr: output-stream,
  }

  get-stdio: func() -> stdio-preopens

  get-directories: func() -> list<tuple<descriptor, string>>
}

interface exit {
  exit: func(status: result)
}

default world example-wasi-component-reactor {
  import streams: self.streams
  import filesystem: self.filesystem
  import environment: self.environment
  import preopens: self.preopens
  import exit: self.exit
  export something: func(s: string) -> string
}
```

If we start from the top we first have the `streams` interface:
```
interface streams {
  type input-stream = u32

  type output-stream = u32

  record stream-error {
  }

  drop-input-stream: func(this: input-stream)

  write: func(this: output-stream, buf: list<u8>) -> result<u64, stream-error>

  blocking-write: func(this: output-stream, buf: list<u8>) -> result<u64, stream-error>

  drop-output-stream: func(this: output-stream)
}
```
This looks simliar to [streams.wit] and next we have [filesystem].

Now, these are modules that the runtime is expects to be able to look up when
the component is instantiated. So we need to add them to the Linker so that
they can be resolved, for example:
```rust
    let mut store = Store::new(&engine, ctx);
    let mut linker = Linker::new(&engine);
    wasi::filesystem::add_to_linker(&mut linker, |x| x)?;
    wasi::streams::add_to_linker(&mut linker, |x| x)?;
    wasi::environment::add_to_linker(&mut linker, |x| x)?;
    wasi::preopens::add_to_linker(&mut linker, |x| x)?;
    wasi::exit::add_to_linker(&mut linker, |x| x)?;
```
For a complete example please take a look at [wasmtime-example].


### Issue 2
After a number of changes to wasmtime I had to refactor an example what we have
and I ran into the simlar issue once again: 
```console
$ cargo r
   Compiling rust v0.1.0 (/home/danielbevenius/work/security/seedwing/seedwing-policy/engine/rust)
warning: unused import: `wasi::command`
Error: import `wasi:filesystem/filesystem` has the wrong type

Caused by:
    0: instance export `write-via-stream` has the wrong type
    1: type mismatch with results
    2: expected `result` found `u32
```
After some troubleshooting I realized that the adapter .wasm had been updated
, actually it has not been moved out of the [preview-prototype2] repo into
the wasmtime repo. I built that .wasm and used it instead and then compiled
the core wasm module, and then created a component from it. I had also
previously had/been using some preview1 specific code to add the correct
functions to the linker which were no longer required. So the whole adapter
code was removed. See [main.rs] for details.

[main.r]: https://github.com/danbev/seedwing-policy/blob/cf77bfa96b388e0036e924c5ba2516363bff7f4a/engine/rust/src/main.rs

[preview-prototype2]: https://github.com/bytecodealliance/preview2-prototyping

[wasmtime-example]: wasmtime-example/wasmtime/src/wasi.rs

[filesystem]: https://github.com/webassembly/wasi-filesystem

[streams.wit]: https://github.com/WebAssembly/wasi-io/blob/main/wit/streams.wit
[wasi-io]: https://github.com/WebAssembly/wasi-io
[wasm.rs]: ../wasmtime-example/wasmtime/src/wasm.rs
[wasi.rs]: ../wasmtime-example/wasmtime/src/wasi.rs
