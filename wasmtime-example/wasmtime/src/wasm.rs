bindgen!(in "../wasm-component/wit");

use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};

struct MyState {
    name: String,
}

impl WasmcomponentImports for MyState {
    fn name(&mut self) -> wasmtime::Result<String> {
        Ok(self.name.clone())
    }
}

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, "../wasm-component/example-wasm-component.wasm")?;

    let mut linker = Linker::new(&engine);
    Wasmcomponent::add_to_linker(&mut linker, |state: &mut MyState| state)?;
    let mut store = Store::new(
        &engine,
        MyState {
            name: "me".to_string(),
        },
    );
    let (bindings, _instance) = Wasmcomponent::instantiate(&mut store, &component, &linker)?;

    let ret = bindings.call_something(&mut store, "bajja")?;
    println!("ret: {ret:?}");
    Ok(())
}
