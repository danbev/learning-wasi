##

### Building
```console
$ npx wasm-pack build --target web
```

```console
$ npm run build
```

### Running
```console
$ npm run serve
```

### Internals
```console
$ cargo expand
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasm_bindgen::prelude::*;
#[allow(nonstandard_style)]
#[allow(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
///
fn alert(s: &str) {
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    unsafe fn __wbg_alert_129db1285a0bdcd5(
        s: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
    ) -> () {
        drop(s);
        {
            ::std::rt::begin_panic(
                "cannot call wasm-bindgen imported functions on \
                    non-wasm targets",
            )
        };
    }
    unsafe {
        let _ret = {
            let s = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(s);
            __wbg_alert_129db1285a0bdcd5(s)
        };
        ()
    }
}
#[allow(dead_code)]
pub fn greet(name: &str) {
    alert(
        &{
            let res = ::alloc::fmt::format(format_args!("Hello, {0}!", name));
            res
        },
    );
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_greet(
        arg0: <str as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
    ) -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0)
            };
            let arg0 = &*arg0;
            let _ret = greet(arg0);
            _ret
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
```
