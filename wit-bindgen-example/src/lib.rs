wit_bindgen::generate!("host");

// Define a custom type and implement the generated `Host` trait for it which
// represents implementing all the necesssary exported interfaces for this
// component.
struct MyHost;

impl Host for MyHost {
    fn something(s: String) -> String {
        format!("something was passed: {s}")
    }
}

export_host!(MyHost);
