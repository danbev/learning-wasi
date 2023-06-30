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


### wasmtime-py print issue
I've made an attempt to update wasmtime-py to use the new WIT syntax and while
I can get it to work with the Seedwing Policy Engine, I'm not able to use
any println! calls in the Rust code. Doing so will produce the following error:
```console
$ make wit-python-run 
Seedwing Policy Engine version: 0.1.0
eval...
thread 'note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Traceback (most recent call last):
  File "/home/danielbevenius/work/security/seedwing/seedwing-policy/engine/wit-examples/python/engine.py", line 115, in <module>
    main()
  File "/home/danielbevenius/work/security/seedwing/seedwing-policy/engine/wit-examples/python/engine.py", line 88, in main
    result: EvaluationResult = engine.eval(store, policies, data, policy, policy_name, input)
                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/work/security/seedwing/seedwing-policy/engine/wit-examples/python/dist/__init__.py", line 449, in eval
    ret = self.lift_callee1(caller, result, len2, result15, len16, ptr18, len19, ptr20, len21, variant, variant86, variant87)
          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/danielbevenius/.local/lib/python3.11/site-packages/wasmtime/_func.py", line 91, in __call__
    with enter_wasm(store) as trap:
  File "/usr/lib64/python3.11/contextlib.py", line 144, in __exit__
    next(self.gen)
  File "/home/danielbevenius/.local/lib/python3.11/site-packages/wasmtime/_func.py", line 264, in enter_wasm
    raise trap_obj
wasmtime._trap.Trap: error while executing at wasm backtrace:
    0: 0x1bb8a7 - <unknown>!__rust_start_panic
    1: 0x1bb600 - <unknown>!rust_panic
    2: 0x1bb5c7 - <unknown>!std::panicking::rust_panic_with_hook::h1c67ce6bc4eb31b7
    3: 0x1ba677 - <unknown>!std::panicking::begin_panic_handler::{{closure}}::h749586aa4ef76f6f
    4: 0x1ba5a1 - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::h426b71926848cb31
    5: 0x1bac35 - <unknown>!rust_begin_unwind
    6: 0x1c1174 - <unknown>!core::panicking::panic_fmt::hf4ce15c1b219b988
    7: 0x1b99a3 - <unknown>!std::io::stdio::_print::hc2f2653d6b9a5348
    8: 0xb0dd6 - <unknown>!<seedwing_policy_engine::wit::Exports as seedwing_policy_engine::wit::exports::seedwing::policy::engine::Engine>::eval::h0b27621124f8242e
    9: 0xfd290 - <unknown>!seedwing_policy_engine::wit::exports::seedwing::policy::engine::call_eval::h918ff3577b37d690
   10: 0x100f1d - <unknown>!seedwing:policy/engine#eval

Caused by:
    wasm trap: wasm `unreachable` instruction executed
make[1]: *** [Makefile:6: run] Error 1
make: *** [Makefile:55: wit-python-run] Error 2
```
If we add a print statement in `_func.py` before line 91 we can see that the
address (I think of the function that is being called):
```
eval...
results: param.len: 4, params: (0, 0, 4, 0), [i32], func: <wasmtime._bindings.wasmtime_func object at 0x7fc757429760>
```
So we have a function that takes 4 arguments and returnes an i32. Since we are
using println! this could be `fd_write`:
```
(i32, i32, i32, i32) -> i32
``` 
This was a silly mistake on my part where I was not returning the bytes written
by the Streams implementation:
```python
class WasiStreams(streams.Streams):                                                 
    def drop_input_stream(self, this: streams.InputStream) -> None:                 
        return None                                                                 
                                                                                    
    def write(self, this: streams.OutputStream, buf: bytes) -> core_types.Result[int, streams.StreamError]:
        sys.stdout.buffer.write(buf)                                                
        return core_types.Ok(len(buf))                                              
                                                                                    
    def blocking_write(self, this: streams.OutputStream, buf: bytes) -> core_types.Result[int, streams.StreamError]:
        sys.stdout.buffer.write(buf)                                            
        return core_types.Ok(len(buf))                                              
                                                                                    
    def drop_output_stream(self, this: streams.OutputStream) -> None:               
        return None 
```
Initially I had just returned a hardcoded value to get a first example to work
and then forgot to update it :( 

