use wasmparser::Payload::*;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::wasi::command::add_to_linker;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../wit/wasmcomponent.wit",
    world: "static-evaluator",
    async: true
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
use std::str;

#[tokio::main(flavor = "current_thread")]
async fn main() -> wasmtime::Result<()> {
    println!("WebAssembly Component Runner");
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let bytes = include_bytes!("../../target/composed.wasm");

    let parser = wasmparser::Parser::new(0);
    for payload in parser.parse_all(bytes) {
        match payload? {
            CustomSection(ref rs) => {
                if rs.name() == "seedwing:policy" {
                    println!(
                        "Section {}, data: {:?}",
                        rs.name(),
                        str::from_utf8(rs.data())
                    );
                }
            }
            _ => {}
        }
    }

    let component = Component::from_binary(&engine, bytes)?;
    let args: Vec<_> = std::env::args().collect();
    let vars: Vec<_> = std::env::vars().collect();

    let mut table = Table::new();
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .set_args(&args)
        .set_env(&vars)
        .build(&mut table)?;
    let ctx = CommandCtx { table, wasi_ctx };

    let mut store = Store::new(&engine, ctx);
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;

    let (reactor, _instance) =
        StaticEvaluator::instantiate_async(&mut store, &component, &linker).await?;
    let string: String = reactor.call_run(&mut store).await?;
    println!("{string:?}");
    Ok(())
}
