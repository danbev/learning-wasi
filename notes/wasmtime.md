## Wasmtime notes


### Engine
An Engine contains one field/member which is an Arc<EngineInner>, that is a
reference counted instance of EngineInner. So it can be cloned and that will
only increment the ref count.

So lets take a look at EngineInner.
```rust
struct EngineInner {
    config: Config,
    #[cfg(compiler)]
    compiler: Box<dyn wasmtime_environ::Compiler>,
    allocator: Box<dyn InstanceAllocator + Send + Sync>,
    profiler: Box<dyn ProfilingAgent>,
    signatures: SignatureRegistry,
    epoch: AtomicU64,
    unique_id_allocator: CompiledModuleIdAllocator,

    // One-time check of whether the compiler's settings, if present, are
    // compatible with the native host.
    compatible_with_native_host: OnceCell<Result<(), String>>,
}
```


### Component
This struct can be found in component/component.rs and the layout is simliar
to the Engine where we have an Arc<ComponentInner>. A Component represents
a compiled WebAssembly Component.
Calling Component::new or `Component::from_file`, or `Component::from_binary`
will compile the component either from a file on disk or bytes provided
directly to it. The bytes can also be of a `wat` if that feature is enabled,
either way both ways end up calling:
```rust
    pub fn from_binary(engine: &Engine, binary: &[u8]) -> Result<Component> {
        engine
            .check_compatible_with_native_host()
            .context("compilation settings are not compatible with the native host")?;

        let (mmap, artifacts) = Component::build_artifacts(engine, binary)?;
        let mut code_memory = CodeMemory::new(mmap)?;
        code_memory.publish()?;
        Component::from_parts(engine, Arc::new(code_memory), Some(artifacts))
    }
```

### Linker
```rust
pub struct Linker<T> {
    engine: Engine,
    strings: Strings,
    map: NameMap,
    allow_shadowing: bool,
    _marker: marker::PhantomData<fn() -> T>,
}
```


### wasmtime_wasi::add_to_linker
If we take a look at the following example:
```rust
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasm = wat::parse_str(
        r#"
        (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))
        (memory (export "memory") 0)
        (func (export "_start")
            (call $__wasi_proc_exit (i32.const 123))
        )
        "#,
    )?;

    let module = Module::new(&engine, wasm)?;
    let mut store = Store::new(&engine, WasiCtxBuilder::new().build());
```
And focus on the `wasmtime_wasi::add_to_linker` function we can see the
following in `crates/wasi/src/lib.rs`:
```rust
pub fn add_to_linker<T, U>(
    linker: &mut Linker<T>,
    get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
) -> anyhow::Result<()>
    where U: Send
            + wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable
            + wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
        $($bounds)*
{
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
    snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
    Ok(())
}

pub mod snapshots {
    pub mod preview_1 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_1,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/snapshot/witx/wasi_snapshot_preview1.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
    pub mod preview_0 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_0,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/old/snapshot_0/witx/wasi_unstable.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
}
```
Now, what I'd like to understand is the:
```rust
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
    snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
```
So first `snapshots` is a module which is defined further down, and it contains
two submodules, `preview_1` and `preview_0`.
Lets focus on `preview_1` for now.
We can see that this is using a macro from `wiggle`. wiggle is able to generate
code from `.witx` files. I think this could be though of an simliar to
generating Rust code/types from `.wit` types, where `.witx` be the predecessor
of wit (webassembly interface types. Like the comments say to understant how
this works we need to take a look in crates/wasi-common and its build.rs
```rust
fn main() {
    let cwd = std::env::current_dir().unwrap();
    let wasi = cwd.join("WASI");
    println!("cargo:wasi={}", wasi.display());
    println!("cargo:rustc-env=WASI_ROOT={}", wasi.display());
    println!("cargo:rerun-if-changed=build.rs");
}
```
And there is an empty `WASI` directory in wasi-common, unless you have run
`git submodule init && git submodule update`.
```console
 [submodule "WASI"]                                                              
           path = crates/wasi-common/WASI                                          
           url = https://github.com/WebAssembly/WASI 
```
After running those commands the directory crates/wasi-common/WASI will have the
following files in it.
Now we can take a look at the output of the build.rs when it is run:
```console
$ cat ../../target/debug/build/wasi-common-1afccc4f440c23eb/output
cargo:wasi=/home/danielbevenius/work/wasm/wasmtime/crates/wasi-common/WASI
cargo:rustc-env=WASI_ROOT=/home/danielbevenius/work/wasm/wasmtime/crates/wasi-common/WASI
cargo:rerun-if-changed=build.rs
```
So we can see that the environment variable `WASI_ROOT` has been set to point
to the WASI directory.
Back to the macro `wiggle::wasmtime_integration`. If we expand this macro:
```console
$ cargo expand > expanded
```
And then take a look in `expanded` and look at the `preview_1` module we can
see what wiggle generated:
```rustc
    pub mod snapshots {
        pub mod preview_1 {

            pub fn add_wasi_snapshot_preview1_to_linker<T, U>(
                linker: &mut wiggle::wasmtime_crate::Linker<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wiggle::anyhow::Result<()>
            where
                U: wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
            {
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "args_get",
                        ...
```
So here we can see the implementation for `add_wasi_snapshot_preview1_to_linker`
which we saw a call to earlier:
```rust
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
```
And if we look at the rest of the expanded code we can see that all the
[preview1 functions] are added in a similar way.

[preview1 functions]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md

