## WebAssembly Component Model
The wasm spec provides a arch, platform, and languague indepedant format of
executable code. Such a module can import functions from the host and export
function to the host (and also memory and tables). This type of webassembly
module will be called a core webassembly module:
```
 Imports                    Exports
             +-------------+
   --------> | WebAssembly |------->
   --------> |    Core     |------->
   --------> |   Module    |------->
             +-------------+
```
So a webassembly module can import functions from the `host`, and it can export
functions. These functions can only work with four data types, `i32`, `i64`,
`f32`, and `f64`.

If we want to work with complex data types then these have to be
converted/transformed into these four data types (often using web assembly
memory). This is called binding and one example of this is wasm-bindgen which
can generate types in JavaScript and allow us to work with, passing/receiving
types in the host language, so we are no longer constrained to only the core
module data types.

What is not available it the ability to compose modules and if one module want
to communicate with another, they have to resort to host binding as it stands
today. If our functions use complex types, like strings, structs ("records"),
then we would have to generate bindings for the functions first, then import
both modules into a host languague, perhaps JavaScript and then call the first
function and then then other (something like that). The point is that there is
no way to get this kind of integration without having to jump through those
hoops.

What the WebAssembly Component Model provides the following on top of the core
WebAssembly spec/model and which "wraps" core wasm modules:
```
  Imports   +--------------------------------+  Exports
            | WebAssembly Component          |
            |  <types>                       |
   -------->|                                |---------->
   -------->|                                |---------->
   -------->| +---------------------------+  |---------->
            | | WebAssembly Core Module   |  |
            | +---------------------------+  |
            | +---------------------------+  |
            | | WebAssembly Core Module   |  |
            | +---------------------------+  |
            +--------------------------------+
```
We still imports and exports in the component, but these functions use types
defined in an Interface Definition Language (IDL) called WebAssembly Interface
Types (WIT) which describe the interfaces. The WebAssembly Core Modules are
still the same core modules as we had without the component module, so the
sources for these core modules could have been written in any language that can
compile to wasm.
The following is a simple example of a
[wit](../wit-bindgen-example/wit/component.wit):
```
default world component {
  export something: func(s: string) -> string
}
```
The `world` keyword defines a component named `component`. This is what a
guest language uses to figure out what functions are imported and exported. In this
case only a single function is exported named `something` and it takes a
`string` types and returns a `string` which are WIT types.

The wit file can be used by tools to generate bindings for the guest language
(the language that the wasm module is written in). For example, we can use the
command line tools `wit-bindgen rust` to generate a
[trait](../wit-bindgen-example/src/component.rs) for us to
[implement](../wit-bindgen-example/src/lib.rs):
```rust
pub trait Component {
  fn something(s: wit_bindgen::rt::string::String,) -> wit_bindgen::rt::string::String;
}
```
And what is actually exported is:
```rust
    #[doc(hidden)]
    #[export_name = "something"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_component_something(arg0: i32,arg1: i32,) -> i32 {
      call_something::<$t>(arg0,arg1,)
    }
```
And the `call_something` function will take care of converting from the core
wasm types, in this case two i32 which the first one represents a pointer into
the linear memory, and the second is the lenght:
```rust
pub unsafe fn call_something<T: Component>(arg0: i32,arg1: i32,) -> i32 {
  ...
  let len0 = arg1 as usize;
  let result1 = T::something(String::from_utf8_unchecked(Vec::from_raw_parts(arg0 as *mut _, len0, len0)));
}
```
And the same function will also take care of transforming the return value from
a string into two i32 to be returned.

With that file generated we can compile using the `wasm32-wasi` target to
produce a core webassembly module (.wasm file):
```console
$ make build-wasi
wit-bindgen rust --macro-export  wit/component.wit --out-dir src
Generating "src/component.rs"
cargo build --target wasm32-wasi
   Compiling wit-bindgen-example v0.1.0 (/home/danielbevenius/work/wasm/learning-wasi/wit-bindgen-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
```
The resulting .wasm file will have a custom section named
'component-type:component' added to it (which is done by component.rs):
```console
$ make objdump-wasi-module 
  types                                  |        0xa -       0x82 |       120 bytes | 17 count
  imports                                |       0x85 -      0x11b |       150 bytes | 4 count
  functions                              |      0x11e -      0x21c |       254 bytes | 252 count
  tables                                 |      0x21e -      0x223 |         5 bytes | 1 count
  memories                               |      0x225 -      0x228 |         3 bytes | 1 count
  globals                                |      0x22a -      0x233 |         9 bytes | 1 count
  exports                                |      0x235 -      0x270 |        59 bytes | 4 count
  elements                               |      0x272 -      0x2f1 |       127 bytes | 1 count
  code                                   |      0x2f5 -     0xfd81 |     64140 bytes | 252 count
  data                                   |     0xfd84 -    0x11005 |      4737 bytes | 2 count
  custom ".debug_info"                   |    0x11015 -    0x9016c |    520535 bytes | 1 count
  custom ".debug_pubtypes"               |    0x9017f -    0x939d1 |     14418 bytes | 1 count
  custom ".debug_loc"                    |    0x939de -    0x939fb |        29 bytes | 1 count
  custom ".debug_ranges"                 |    0x93a0d -    0xc27fd |    191984 bytes | 1 count
  custom "component-type:component"      |    0xc2819 -    0xc28ea |       209 bytes | 1 count
  custom ".debug_abbrev"                 |    0xc28fb -    0xc5514 |     11289 bytes | 1 count
  custom ".debug_line"                   |    0xc5524 -   0x116eeb |    334279 bytes | 1 count
  custom ".debug_str"                    |   0x116efa -   0x1dcc91 |    810391 bytes | 1 count
  custom ".debug_pubnames"               |   0x1dcca5 -   0x227f51 |    307884 bytes | 1 count
  custom "name"                          |   0x227f5a -   0x22c27d |     17187 bytes | 1 count
  custom "producers"                     |   0x22c289 -   0x22c2e1 |        88 bytes | 1 count
  custom "target_features"               |   0x22c2f3 -   0x22c31c |        41 bytes | 1 count
```
The next step is to take this core webassembly module and create a webassembly
component model module.
```console
$ make component-wasi 
wasm-tools component new ./target/wasm32-wasi/debug/wit_bindgen_example.wasm \
--adapt wasi_snapshot_preview1.wasm \
-o example-wasi-component.wasm
```
The reason for using `--adapt` is that the version of wasi used by the target
`wasm32-wasi` is based on `.witx` type definitions which are not compatible with
the omponent model which uses wit. This is like a polyfill from
wasi_snapshot_preview1 to wasi preview2.

We can then inspect the component:
```console
$ make objdump-component 
  module                                 |        0xc -   0x1ed87e |   2021490 bytes | 1 count
    ------ start module 0 -------------
    types                                |       0x16 -       0x8e |       120 bytes | 17 count
    functions                            |       0x91 -      0x16e |       221 bytes | 219 count
    tables                               |      0x170 -      0x175 |         5 bytes | 1 count
    memories                             |      0x177 -      0x17a |         3 bytes | 1 count
    globals                              |      0x17c -      0x195 |        25 bytes | 3 count
    exports                              |      0x197 -      0x1ed |        86 bytes | 6 count
    elements                             |      0x1ef -      0x23b |        76 bytes | 1 count
    code                                 |      0x23f -     0xb92f |     46832 bytes | 219 count
    data                                 |     0xb932 -     0xc28c |      2394 bytes | 1 count
    custom ".debug_info"                 |     0xc29c -    0x7a675 |    451545 bytes | 1 count
    custom ".debug_pubtypes"             |    0x7a688 -    0x7e15e |     15062 bytes | 1 count
    custom ".debug_loc"                  |    0x7e16b -    0x7e1a5 |        58 bytes | 1 count
    custom ".debug_ranges"               |    0x7e1b7 -    0xa8b2f |    174456 bytes | 1 count
    custom ".debug_abbrev"               |    0xa8b40 -    0xabb40 |     12288 bytes | 1 count
    custom ".debug_line"                 |    0xabb50 -    0xf4782 |    298034 bytes | 1 count
    custom ".debug_str"                  |    0xf4791 -   0x1a77b0 |    733215 bytes | 1 count
    custom ".debug_pubnames"             |   0x1a77c4 -   0x1e9b0d |    271177 bytes | 1 count
    custom "name"                        |   0x1e9b15 -   0x1ed7fb |     15590 bytes | 1 count
    custom "producers"                   |   0x1ed808 -   0x1ed87e |       118 bytes | 1 count
    ------ end module 0 -------------
  core instances                         |   0x1ed880 -   0x1ed884 |         4 bytes | 1 count
  component alias                        |   0x1ed886 -   0x1ed8a3 |        29 bytes | 2 count
  component types                        |   0x1ed8a5 -   0x1ed8ad |         8 bytes | 1 count
  component alias                        |   0x1ed8af -   0x1ed8d6 |        39 bytes | 2 count
  canonical functions                    |   0x1ed8d8 -   0x1ed8e5 |        13 bytes | 1 count
  custom "producers"                     |   0x1ed8f1 -   0x1ed914 |        35 bytes | 1 count
  component exports                      |   0x1ed916 -   0x1ed925 |        15 bytes | 1 count
```
Notice that the `module 0` is pretty much the same as the core module apart from
the custom section `component-type::component` not present. This is what we
meant above about the component model "wrapping" core modules.

Now, with or component module we can now use it with languages that provide a
wasm runtime that supports the webassembly component model.

Features of the Component Model are:
* Marshaling of types between modules in a standard way
Types in core webassembly modules get translated into the types specified by
the component model. This enables one core module to have imports from another
core module and the webassembly component runtime will be able to handle these
type conversions. What previsouly required host bindings like wasm-bindgen would
not be requied.

* linking modules
TODO:

### Component
A component is like an executable, think of ELF format which describes how
the executable is to be loaded into memory, linked etc.

So a component can be loaded and run and this is called an instance of the
component. Multple instance can be run an the same time and these do not share
any resources with each other. To be able to communicate with other component
instances interface types and the canonical abi is provided.

### Component/Module dynamic linking
One place where this is helpful, which was not obvious to me, is in cloud
environments where one might want to be able to dynamically link a module into
many others and thereby saving memory and storage. 

### Module
A module described a sort of library that can export/provide functions from
itself, and can import/consume function from others. Think of a shared library
which can export functions and can also expect symbols to be provided by
externa libraries.

A model that has been loaded into a component instance, is called a module
instance. A model instance shares the memory and resources of the component
instance it was loaded into. And modules can be loaded into multiple component
instance but they are completely separate.

### Interface Types
Interface types which allow for an language indepedent format for describing
the functions and types that a module imports/exports. It uses an Interface
Definition Language (IDL) to specify this information which can then be
processed by language specific tools to generate types in the corresponding
languages.

* bool
* s8, s16, s32, s64  signed ints
* u8, u16, u32, u64  unsigned ints
* float32, float64
* char
* record is like an JavaScript object with named fields, or a struct in Rust.
* variant is like a Rust enum. 
* enum is like a variant but can't have a object associated with it.
* union 
* bitflags
* list 
* tuple
* option
* result is like result in Rust.
* own

We can crate type aliases using the format:
```
type sometype = u32
```

### Canonical ABI
TODO:

### WASI and the Component Model
The next version of wasi, preview2, is based on the WIT IDL and the component
model. This is different from wasi preview1 and there are some incompatabilities
beteen these models. So if we have a wasi preview1 model it will need to be
adapted to preview2 which can be done using `--adapt` with `wasm-tools`:
```console
$ wasm-tools component new -v ../target/wasm32-wasi/debug/seedwing_policy_engine.wasm --adapt wasi_snapshot_preview1.wasm -o seedwing_policy-engine-component.wasm
```

### Resource types (handles)
To avoid having to copy of data types, for example if they are large or perhaps
if they contain recursive structures which is currently not supported by the
current component model version. These are similar to file descriptors in an 
operating system. So we can have a resource id which is passed to a component
and can also be passed back without the wasm runtime having to lower and lift
the types.
Again simliar to how file descriptors are handled in a file descriptor table and
indexed using integers (think of stdin (0), stdout(1), and stderr(2)), the
component model specifies a `handle table`.

```
(resource (rep i32) (dtor <funcidx>)?)
```

So we would define a 
```
resource some-resource {

   drop-fn: func()
}
```
The `drop-fn` functions will be called when the last handle to this resources
is dropped.

The type `own` is a handle type that declares/defines an opaque address of a
resource that will be destroyed when it is dropped.
The type `borrow` is a handle type that declares/defines an opaque address of a
resource that must be dropped before the current export call returns.

A type in a wit can be of type `resourcetype`.


### Recursive types support
At the time of this writing, before the MVP release of the component model,
support for recursive types is [not supported]. 

This means that modeling the wit types directly after the types in seedwing
policy engine will not work. 

For example, that this type:
```
record evaluation-result {
  input: runtime-value,
  ty: pattern,
  rationale: rationale,
  output: string,
  }
```
The field `ty` is of type `pattern`:
```
  record pattern {
    name: option<pattern-name>,
    metadata: pattern-meta,
    examples: list<example>,
    parameters: list<string>,
    inner: inner-pattern,
  }
```
This is still alright and not a recursive pattern. But if we look closer at
inner we will see the issue(s):
```
  variant inner-pattern {
    anything, 
    primordial(primordial-pattern),
    //bound(tuple<pattern, bindings>),
    //ref(tuple<syntactic-sugar, u32, list<pattern>),
    //deref(pattern),
    argument(string), 
    const(value-pattern),
    object(object-pattern),
    //expr(expr), // recursive pattern
    //%list(list<pattern>), // recursive pattern
    nothing,
  }
```
Notice that the commented out fields of tihs variant/enum are of the type
`pattern` which is also the type of inner-pattern.
When we write a pattern in Dogma (the name of the policy language in Seedwing)
it can look something like this:
```
pattern something = 10
```
This has a pattern-name which is 'something' and the inner-type here would be
primordial-pattern variant of integer. This would match any input that is the
number 10. 

```
pattern something = [1, 2]
```
This has a pattern-name which is 'something' and the inner-type here would be
list of patterns, which contains primordial-pattern vairants type integers.

This would match a input of  [1, 2]

[not suppported]: https://github.com/WebAssembly/component-model/issues/56#issuecomment-1557472099
