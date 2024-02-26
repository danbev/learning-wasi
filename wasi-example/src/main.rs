use std::error::Error;
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "hello.wat")?;
    // Second argument is the data to be stored in the store which is just `()`.
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let answer = instance
        .get_func(&mut store, "answer")
        .expect("`answer` was not an exported function");

    let answer = answer.typed::<(), i32>(&store)?;
    let result = answer.call(&mut store, ())?;
    println!("Answer: {:?}", result);
    Ok(())
}
