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
that string would need to be copied into the Wasm memory and we would need to
know the start and end positions. With that information our function could take
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

The `wit` part of the name stands for Wasm Interface Type format.

__wip__


[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
[wit-format]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
