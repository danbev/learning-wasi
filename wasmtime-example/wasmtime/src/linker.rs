use wasmtime::{Caller, Engine, Linker, Module, Store};

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    // A linker performs name based resolution of the imports.
    let mut linker = Linker::new(&engine);
    let module_name = "host";
    let func_name = "hello";
    linker.func_wrap(
        module_name,
        func_name,
        |mut caller: Caller<'_, u32>, param: i32| {
            println!("Got {param} from WebAssembly");
            println!("my host state is: {}", caller.data());
            let export = caller.get_export("hello");
            println!("{:?}", export);
        },
    )?;

    // Use the `linker` to instantiate the module, which will automatically
    // resolve the imports of the module using name-based resolution.
    let mut store = Store::new(&engine, 0);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<(), ()>(&mut store, "hello")?;
    hello.call(&mut store, ())?;

    Ok(())
}
