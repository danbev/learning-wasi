use wasi_common::preview1::{WasiPreview1Adapter, WasiPreview1View};
use wasi_common::{wasi, Table, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime::{
    component::{bindgen, Component as WasmComponent, Linker},
    Config, Engine as WasmtimeEngine, Store,
};

bindgen!({
    path: "../wit",
    world: "component",
    async: true,
    with: {
       "environment": wasi::environment,
       "streams": wasi::streams,
       "preopens": wasi::preopens,
       "filesystem": wasi::filesystem,
       "exit": wasi::exit,
    },
});

struct Preview1Ctx {
    table: Table,
    wasi_ctx: WasiCtx,
    adapter: WasiPreview1Adapter,
}

impl WasiView for Preview1Ctx {
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

impl WasiPreview1View for Preview1Ctx {
    fn adapter(&self) -> &WasiPreview1Adapter {
        &self.adapter
    }
    fn adapter_mut(&mut self) -> &mut WasiPreview1Adapter {
        &mut self.adapter
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = WasmtimeEngine::new(&config)?;

    let component = WasmComponent::from_file(&engine, "../example-wasi-component.wasm")?;

    let mut table = Table::new();
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build(&mut table)?;
    let adapter = WasiPreview1Adapter::new();
    let ctx = Preview1Ctx {
        table,
        wasi_ctx,
        adapter,
    };

    let mut store = Store::new(&engine, ctx);
    let mut linker = Linker::new(&engine);
    wasi::filesystem::add_to_linker(&mut linker, |x| x)?;
    wasi::streams::add_to_linker(&mut linker, |x| x)?;
    wasi::environment::add_to_linker(&mut linker, |x| x)?;
    wasi::preopens::add_to_linker(&mut linker, |x| x)?;
    wasi::exit::add_to_linker(&mut linker, |x| x)?;
    wasi::random::add_to_linker(&mut linker, |x| x)?;

    let (wit, _instance) = Component::instantiate_async(&mut store, &component, &linker).await?;

    let result = wit.call_something(&mut store, "bajja").await?;

    println!("result: {result}");

    Ok(())
}