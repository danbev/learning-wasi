use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

bindgen!(in "../wasm-component/wit");

struct MyState {
    name: String,
}

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = Component::from_file(&engine, "../wasm-component/example-wasm-component.wasm")?;

    let linker = Linker::new(&engine);
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
