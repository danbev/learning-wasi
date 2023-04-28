wit_bindgen::generate!("component");

struct Something;

impl Component for Something {
    fn something(s: String) -> String {
        format!("something was passed: {s}")
    }
}

export_component!(Something);
