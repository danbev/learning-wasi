use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() -> Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

    let module = Module::from_file(
        &engine,
        "../wasm-component/target/wasm32-wasi/debug/wasm_component.wasm",
    )?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;
    let (bindings, _) = Wasmcomponent::instantiate(&mut store, &component, &linker)?;

    let ret = bindings.call_something(&mut store, "bajja")?;
    println!("ret: {ret:?}");

    Ok(())
}
