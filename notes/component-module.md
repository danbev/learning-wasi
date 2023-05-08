## WebAssembly Component Model
With wasm spec provides a arch, platform, and languague indepedent format of
executable code. Such a module can import functions from the host and export
function to the host. What is not available it the ability to compose modules
and if one module want to communicate with another it module they have to
resort to host binding as it stands today. For example, lets say that I have
to wasm modules and want one to call the other. If the functions in question
use complext types like strings, structs ("records"), the one would have to
generate bindings for the functions first, then import both modules into
a host languague, perhaps JavaScript and then call the first function and then
then other (something like that). The point is that there is no way to get this
kind of integration without having to jump through those hoops.

What the WebAssembly Component Model provide the following on top of the core
WebAssembly spec/model.

### Interface Types
Interface types which allow for an language indepedent format for describing
the functions and types that a module imports/exports.

TODO: specify the types


### Canonical ABI
TODO:

### Component/Module dynamic linking
One place where this is helpful, which was not obvious to me, is in cloud
environments where one might want to be able to dynamically link a module into
many others and thereby saving memory and storage. 

TODO:


### Component
A component is like an executable, think of ELF format which describes how
the executable is to be loaded into memory, linked etc.

So a component can be loaded and run and this is called an instance of the
component. Multple instance can be run an the same time and these do not share
any resources with each other. To be able to communicate with other component
instances interface types and the canonical abi is provided.

### Module
A module described a sort of library that can export/provide functions from
itself, and can import/consume function from others. Think of a shared library
which can export functions and can also expect symbols to be provided by
externa libraries.
A model that has been loaded into a component instance, is called a module
instance. A model instance shares the memory and resources of the component
instance it was loaded into. And modules can be loaded into multiple component
instance but they are completely separate.



