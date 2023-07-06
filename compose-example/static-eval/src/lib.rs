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

        let Some(input) = std::env::args().nth(1) else {
            return "Error: input to evaluate must be specified as an argument".to_string();
        };

        let Ok(policy_name) = env::var("SEEDWING_POLICY_NAME") else {
            return "Error: Policy name must be set!".to_string();
        };
        println!("policy_name: {policy_name}");
        println!("input: {}", input);
        return engine::eval(policy);
    }
}
export_static_evaluator!(Export);
