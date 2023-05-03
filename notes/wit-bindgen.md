## wit-bindgen
This documents contains notes about [wit-bindgen].

### Background
To understand what we mean by bindings and why they are useful we need to
remember that Wasm only has four data types, `i32`, `i64`, `f32`, and `f64`.
Wasm modules can be compiled from various languages and if our functions accepts
and/or return only those types then we are good to go. But more often than not
our functions will accepts and/or return complex types, like strings, so there
needs to be some kind of translation between those types and the types that
wasm can handle.

For example, if we want to write a function that takes a string, the bytes of
that string would need to be copied into Wasm memory and we would need to know
the start and end positions. With that information our function could take
two `i32` values and be able to perform operations on the bytes that make up our
string.

To make this a little more concete lets look at an example.
The [wat](../src/mem-from-js.wat) (wasm text format) looks like this:
```
(module
  (import "js" "mem" (memory 1 100))
  (func (export "copy_string") (param $start i32)(param $end i32)
    ;; store the bytes at the start posistion and the specified end position
    (i32.store (local.get $end)(i32.load(local.get $start)))
    (i32.store (i32.const 6)(i32.load(i32.const 1)))
    ;; clear the initial "string"
    (i32.store (i32.const 0)(i32.const 0))
    (i32.store (i32.const 1)(i32.const 0))
  )
)
```
This is declaring a function named `copy_string` which is takes two parameters
which of type `i32`. In this context they mean the start and end posistion of
a string in the wasm's linear memory.

So to call this function we need to first copy the bytes of a string into the
wasm memory which might then look something like this:
```
    Wasm memory

  start position
    ↓
    +-+-+-+-+-+-+-+-+-+-+-+-+
    |b|a|j|j|a| | | | | | | | 
    +-+-+-+-+-+-+-+-+-+-+-+-+
             ↑
           end position
```
We can compile the wat into a wasm module using the following command:
```console
$ make out/mem-from-js.wasm
```
And then run [mem-from-js.js](../src/mem-from-js.js) that uses this module by
first creating a new WebAssembly.Module and then writing a string as bytes into
this memory:
```js
  // Create a TypedArray for the memory buffer
  const mem_view = new Uint8Array(memory.buffer);
  mem_view.set(new TextEncoder().encode("bajja"));
```
We can then call the exported function `copy_string` and we specify the start
and end posisitions of our string:
```js
  instance.exports.copy_string(0, 5);
```

The example can be run using the following command:
```console
$ node src/mem-from-js.js 
input_string: "bajja", bytes[98,97,106,106,97]
memory before:  Uint8Array(65536) [
  98, 97, 106, 106, 97, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,  0, 0, 0, 0, 0, 0, 0, 0,
   0,  0,   0,   0,
  ... 65436 more items
]
memory after: Uint8Array(65536) [
  0, 0, 0, 0, 0, 98, 97, 106, 106, 97, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0, 0,  0,  0,   0,   0,  0, 0, 0,
  0, 0, 0, 0,
  ... 65436 more items
]
String from wasm function/memory: bajja
```
Hopefully this example shows that doing this kind of translation/setup/binding
is somewhat tedious which is where the various `*-bindgen` libraries and
utilities come into play. They take care of generating this stort of code, both
in the wasm module and in the callee of the module. 

The `wit` part of the name stands for Wasm Interface Type format which allows
for interfaces and types to be declared. These can then be used by tools to
generate bindings for different languages. If you think this sounds simlar to
wasm-bindgen you are correct and we can generate bindings for JavaScript using
wit tools (as we will see an example of shortly). But where wasm-bindgen is
specific to JavaScript this is not the case with wit-bindgen. Here the wit
component (not sure about the terminology yet) can be used by other languages
like python to get the same functionality. It can also be used by Rust which
might sound silly at first but this could be useful if you want thirdparty, or
user supplied code to be executed in an application written in Rust.

### wit-bindgen example
Lets start with really simple example which can be found in
[wit-bindgen-example](../wit-bindgen-example).

Now, lets start by looking at the function that we want to expose/export:
```rust
    fn something(s: String) -> String {
        format!("something was passed: {s}")
    }
```
So we can see that this function take a String and also returns a String.
The wit will look like this, in
[component.wit](../wit-bindgen-example/wit/component.wit):
```
default world component {
  export something: func(s: string) -> string
}
```
A Wasm `world` can be thought of executable that can be run by a wasm runtime
and describes what it imports and what it exports, with the types and
interfaces used/exposed.

With the `component.wit` file we can use a macro from wit-bindgen to generate
some boilerplate code in [lib.rs](../wit-bindgen-example/src/lib.rs):
```rust
wit_bindgen::generate!("component");
```
Now, the string "component" needs to match the file name `component.wit`, and
also needs to match the name of the `world` in that file.

For the following I've commented out everything else in that file to take a
closer look at what this macro does:
```console
$ cargo expand
```
I'll try to pick out the parts that look most interesting to our current
discussion.

First this will generate a trait named after the value world we defined:
```rust
pub trait Component {
    fn something(s: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String;
}
```
The type `wit_bindgen::rt::string::String` is from the
wit_bindgen/crates/guest-rust crate and it is a reexport of the [alloc] crate.
So the string in question is [String].

The following function will also be generated by the macro:
```rust
use wit_bindgen::rt::{alloc, vec::Vec, string::String};
#[doc(hidden)]
pub unsafe fn call_something<T: Component>(arg0: i32, arg1: i32) -> i32 {
    let len0 = arg1 as usize;
    let result1 = T::something({
        { String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap() }
    });
    let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr() as i32;
    let len3 = vec3.len() as i32;
    core::mem::forget(vec3);
    *((ptr2 + 4) as *mut i32) = len3;
    *((ptr2 + 0) as *mut i32) = ptr3;
    ptr2
}
```
This is what will actually call the function we wrote which take a String.
Notice the parameters to this function are two `i32` values. Here `arg0` is
really a pointer which is used with `Vec::from_raw_parts`:
```rust
pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
    unsafe { Self::from_raw_parts_in(ptr, length, capacity, Global) }
}
```
This function will also take care of the return value which notice is also
a `i32`. Notice that the return value is a pointer as well in reality which will
point to a memory location what will hold length of the string and the pointer
to the string.

TODO: explain _RetArea and post_return_something.

Next, with we can expand the rest of the files and we'll find:
```console
struct Something;
impl Component for Something {
    fn something(s: String) -> String {
        {
            let res = ::alloc::fmt::format(format_args!("something was passed: {0}", s));
            res
        }
    }
}
```
Which is just what we wrote with expanded macros, in this case the format!
macro.

After that we have this block:
```console
const _: () = {
    #[doc(hidden)]
    #[export_name = "something"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_component_something(arg0: i32, arg1: i32) -> i32 {
        call_something::<Something>(arg0, arg1)
    }
    #[doc(hidden)]
    #[export_name = "cabi_post_something"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_component_something(arg0: i32) {
        post_return_something::<Something>(arg0)
    }
};
```
I think this is called a free constant and notice that it contains a block and
is an expression. The functions in there will be exported but we could create
a function with the same name without a compilation error.
Also notice the `#[export_name = "something"`, so this is the function that will
be called later by an external host.

When build the wasm module like we would normally do, using:
```console
$ make build
cargo build --target wasm32-unknown-unknown
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
```
This will generate a core .wasm module for the crate which we can inspect using:
```console
$ make inspect-wasm-wat > wat
```
This will have the format that will looks like the .wat example that we showed
above. 

The next step is to create a wasm component of this which we do using the
`wasm-tools component` command and we specify our .wasm file as input:
```console
$ make component 
wasm-tools component new ./target/wasm32-unknown-unknown/debug/wit_bindgen_example.wasm \
-o example-component.wasm
```
This will generate a new wasm module named `example-component.wasm`.

We can inspect the wat of this using the following command:
```console
$ make inspect-component-wat > component.wat
```
If we inspect the .wat for the component module we will see that it is different
from the module (wit_bindgen_example.wasm).
__TODO__: Add details about the WebAssembly Component Model here or in a separate
document.

Now, lets take a look at running this example and we will use JavaScript as
the first language. The example can be found in
[js](../wit-bindgen-example/js/README.md)
```console
$ cd js
$ npm i
```
Then we can generate the bindings using `jco`:
```console
$ npm run bindings

> js@1.0.0 bindings
> npx jco transpile ../example-component.wasm -o dist
```
This will generate the following files in the dist directory:
```console
Transpiled JS Component Files:
 - dist/example-component.core.wasm  1.93 MiB
 - dist/example-component.d.ts       0.04 KiB
 - dist/example-component.js 
```

And we can run the example using:
```console
$ npm run example

> js@1.0.0 example
> node index.mjs

something was passed: bajja
```

We can also use the this with Python and an example can be found in
[python](../wit-bindgen-example/python/README.md).

__wip__


[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
[wit-format]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[alloc]: https://doc.rust-lang.org/alloc/index.html
[string]: https://doc.rust-lang.org/alloc/string/struct.String.html
[jco]: https://github.com/bytecodealliance/jco
