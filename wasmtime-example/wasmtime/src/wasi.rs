use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::wasi::command::add_to_linker;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../wasm-component/wit",
    world: "wasmcomponent",
    async: true,
});

struct CommandCtx {
    table: Table,
    wasi_ctx: WasiCtx,
}

impl WasiView for CommandCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi_ctx
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(
        &engine,
        "../wasm-component/example-wasi-component-reactor.wasm",
    )?;

    let mut table = Table::new();
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build(&mut table)?;
    let ctx = CommandCtx { table, wasi_ctx };

    let mut store = Store::new(&engine, ctx);
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;

    let (reactor, _instance) =
        Wasmcomponent::instantiate_async(&mut store, &component, &linker).await?;
    let string: String = reactor.call_something(&mut store, "from rust").await?;
    println!("{string:?}");
    Ok(())
}
