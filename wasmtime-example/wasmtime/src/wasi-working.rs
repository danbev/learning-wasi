bindgen!(in "../wasm-component/wit");

use wasmtime::component::bindgen;
use wasmtime::{Caller, Config, Engine, Linker, Module, Store};
use wasmtime_wasi::I32Exit;
use wasmtime_wasi::WasiCtx;
use wasmtime_wasi::WasiCtxBuilder;
use wat;

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasm = wat::parse_str(
        r#"
        (import "wasi_snapshot_preview1" "proc_exit" (func $anyname_wasi_proc_exit (param i32)))
        ;;(import "streams" "drop-input-stream" (func $wasi_preview1_component_adapter::bindings::streams::drop_input_stream::wit_import (type 0)))
        (memory (export "memory") 0)
        (func (export "_start")
            (call $anyname_wasi_proc_exit (i32.const 18))
        )
        "#,
    )?;
    let mut store = Store::new(&engine, WasiCtxBuilder::new().build());

    let module = Module::new(&engine, wasm)?;
    let instance = linker.instantiate(&mut store, &module)?;

    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    let exit = start
        .call(&mut store, ())
        .unwrap_err()
        .downcast::<I32Exit>()?;
    println!("exit: {exit:?}");
    Ok(())
}
