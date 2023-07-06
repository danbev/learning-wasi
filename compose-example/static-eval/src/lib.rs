use std::env;
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

        let input = env::args().nth(1);
        if input.is_none() {
            return "Error: input to evaluate must be specified as an argument".to_string();
        }

        if let Ok(policy_name) = env::var("SEEDWING_POLICY_NAME") {
            println!("policy_name: {policy_name}");
            println!("input: {}", input.unwrap());
            return engine::eval(policy);
        } else {
            return "Error: Policy name must be set!".to_string();
        }
    }
}
export_static_evaluator!(Export);
