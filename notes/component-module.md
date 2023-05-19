## WebAssembly Component Model
The wasm spec provides a arch, platform, and languague indepedant format of
executable code. Such a module can import functions from the host and export
function to the host. This type of webassembly module will be called a core
webassembly module:
```
 Imports                    Exports
             +-------------+
   --------> | WebAssembly |------->
   --------> |    Core     |------->
   --------> |   Module    |------->
             +-------------+

Data types: i32, i64, f32, f64
```
So a webassembly module can import functions from the `host`, and it can export
functions. These functions can only work with four data types, `i32`, `i64`,
`f32`, and `f64`. If we want to work with complex data types then these have to
be converted/transformed into these four data types (often using web assembly
memory). These is called bindings and one example of this is wasm-bindgen which
can generate types in a host language and allow us to work with,
passing/receiving types in the host language, so we are no longer constrained to
only the core module data types.

What is not available it the ability to compose modules and if one module want
to communicate with another it module they have to resort to host binding as it
stands today. For example, lets say that I have to wasm modules and want one to
call the other.

If the functions in question use complex types, like strings, structs
("records"), then one would have to generate bindings for the functions first,
then import both modules into a host languague, perhaps JavaScript and then call
the first function and then then other (something like that). The point is that
there is no way to get this kind of integration without having to jump through
those hoops.

What the WebAssembly Component Model provides the following on top of the core
WebAssembly spec/model.
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
We still imports and exports, but these functions use types defined in
an Interface Definition Language (IDL) called WebAssembly Interface Types (WIT)
which describe the interfaces. The WebAssembly Core Modules are still the same
core modules as we had without the component module, so the sources for these
core modules could have been written in any language that can compile to wasm.
The component module will then take care of translating between the types used
in the WIT and the types that the core modules work with.

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
