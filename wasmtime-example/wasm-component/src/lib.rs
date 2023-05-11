wit_bindgen::generate!("wasmcomponent");

struct Export;

impl Wasmcomponent for Export {
    fn something(s: String) -> String {
        format!("something was passed: {s}")
    }
}

export_wasmcomponent!(Export);
