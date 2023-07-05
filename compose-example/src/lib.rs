wit_bindgen::generate!("wasmcomponent");

use crate::exports::compose::example::engine::Engine;

struct Export;

impl Engine for Export {
    fn eval(policy: String) -> String {
        println!("In Engine::eval policy: {}", policy);
        format!("Result: Ok")
    }
}

export_wasmcomponent!(Export);
