use std::str;
wit_bindgen::generate!({
   world: "static-evaluator",
   path: "../wit/wasmcomponent.wit",
});

struct Export;

use crate::compose::example::engine;

impl StaticEvaluator for Export {
    fn run() -> String {
        let bytes = include_bytes!("../policy.dog");
        let policy = str::from_utf8(bytes).unwrap();
        engine::eval(policy)
    }
}
export_static_evaluator!(Export);
