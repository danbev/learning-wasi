## Wasmtime notes


### Engine
An Engine contains one field/member which is an Arc<EngineInner>, that is a
reference counted instance of EngineInner. So it can be cloned and that will
only increment the ref count.

So lets take a look at EngineInner.
```rust
struct EngineInner {
    config: Config,
    #[cfg(compiler)]
    compiler: Box<dyn wasmtime_environ::Compiler>,
    allocator: Box<dyn InstanceAllocator + Send + Sync>,
    profiler: Box<dyn ProfilingAgent>,
    signatures: SignatureRegistry,
    epoch: AtomicU64,
    unique_id_allocator: CompiledModuleIdAllocator,

    // One-time check of whether the compiler's settings, if present, are
    // compatible with the native host.
    compatible_with_native_host: OnceCell<Result<(), String>>,
}
```


### Component
This struct can be found in component/component.rs and the layout is simliar
to the Engine where we have an Arc<ComponentInner>. A Component represents
a compiled WebAssembly Component.
Calling Component::new or `Component::from_file`, or `Component::from_binary`
will compile the component either from a file on disk or bytes provided
directly to it. The bytes can also be of a `wat` if that feature is enabled,
either way both ways end up calling:
```rust
    pub fn from_binary(engine: &Engine, binary: &[u8]) -> Result<Component> {
        engine
            .check_compatible_with_native_host()
            .context("compilation settings are not compatible with the native host")?;

        let (mmap, artifacts) = Component::build_artifacts(engine, binary)?;
        let mut code_memory = CodeMemory::new(mmap)?;
        code_memory.publish()?;
        Component::from_parts(engine, Arc::new(code_memory), Some(artifacts))
    }
```

### Linker
```rust
pub struct Linker<T> {
    engine: Engine,
    strings: Strings,
    map: NameMap,
    allow_shadowing: bool,
    _marker: marker::PhantomData<fn() -> T>,
}
```


### wasmtime_wasi::add_to_linker
If we take a look at the following example:
```rust
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasm = wat::parse_str(
        r#"
        (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (param i32)))
        (memory (export "memory") 0)
        (func (export "_start")
            (call $__wasi_proc_exit (i32.const 123))
        )
        "#,
    )?;

    let module = Module::new(&engine, wasm)?;
    let mut store = Store::new(&engine, WasiCtxBuilder::new().build());
```
And focus on the `wasmtime_wasi::add_to_linker` function we can see the
following in `crates/wasi/src/lib.rs`:
```rust
pub fn add_to_linker<T, U>(
    linker: &mut Linker<T>,
    get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
) -> anyhow::Result<()>
    where U: Send
            + wasi_common::snapshots::preview_0::wasi_unstable::WasiUnstable
            + wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
        $($bounds)*
{
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
    snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
    Ok(())
}

pub mod snapshots {
    pub mod preview_1 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_1,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/snapshot/witx/wasi_snapshot_preview1.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
    pub mod preview_0 {
        wiggle::wasmtime_integration!({
            // The wiggle code to integrate with lives here:
            target: wasi_common::snapshots::preview_0,
            // This must be the same witx document as used above. This should be ensured by
            // the `WASI_ROOT` env variable, which is set in wasi-common's `build.rs`.
            witx: ["$WASI_ROOT/phases/old/snapshot_0/witx/wasi_unstable.witx"],
            errors: { errno => trappable Error },
            $async_mode: *
        });
    }
}
```
Now, what I'd like to understand is the:
```rust
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
    snapshots::preview_0::add_wasi_unstable_to_linker(linker, get_cx)?;
```
So first `snapshots` is a module which is defined further down, and it contains
two submodules, `preview_1` and `preview_0`. Lets focus on `preview_1` for now.

We can see that this is using a macro from `wiggle`. wiggle is able to generate
code from `.witx` files. I think this could be thought of as simliar to
generating Rust code/types from `.wit` types, where `.witx` be the predecessor
of wit (webassembly interface types). Like the comments say to understand how
this works we need to take a look in crates/wasi-common and its build.rs
```rust
fn main() {
    let cwd = std::env::current_dir().unwrap();
    let wasi = cwd.join("WASI");
    println!("cargo:wasi={}", wasi.display());
    println!("cargo:rustc-env=WASI_ROOT={}", wasi.display());
    println!("cargo:rerun-if-changed=build.rs");
}
```
And there is an empty `WASI` directory in wasi-common, unless you have run
`git submodule init && git submodule update`.
```console
 [submodule "WASI"]                                                              
           path = crates/wasi-common/WASI                                          
           url = https://github.com/WebAssembly/WASI 
```
After running those commands the directory crates/wasi-common/WASI will have
files in it.

Now we can take a look at the output of the build.rs when it is run:
```console
$ cat ../../target/debug/build/wasi-common-1afccc4f440c23eb/output
cargo:wasi=/home/danielbevenius/work/wasm/wasmtime/crates/wasi-common/WASI
cargo:rustc-env=WASI_ROOT=/home/danielbevenius/work/wasm/wasmtime/crates/wasi-common/WASI
cargo:rerun-if-changed=build.rs
```
So we can see that the environment variable `WASI_ROOT` has been set to point
to the WASI directory.

Back to the macro `wiggle::wasmtime_integration`. If we expand this macro:
```console
$ cd crates/wasi
$ cargo expand > expanded
```
And then take a look in `expanded` and look at the `preview_1` module we can
see what wiggle generated:
```rustc
    pub mod snapshots {
        pub mod preview_1 {

            pub fn add_wasi_snapshot_preview1_to_linker<T, U>(
                linker: &mut wiggle::wasmtime_crate::Linker<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wiggle::anyhow::Result<()>
            where
                U: wasi_common::snapshots::preview_1::wasi_snapshot_preview1::WasiSnapshotPreview1,
            {
                linker
                    .func_wrap(
                        "wasi_snapshot_preview1",
                        "args_get",
                        ...
```
So here we can see the implementation for `add_wasi_snapshot_preview1_to_linker`
which we saw a call to earlier:
```rust
    snapshots::preview_1::add_wasi_snapshot_preview1_to_linker(linker, get_cx)?;
```
And if we look at the rest of the expanded code we can see that all the export
[preview1 functions] are added in a similar way. And keep in mind that this is
from the crates/wasi module in wasmtime.

So if we have a wasm module with the following import:
```
(import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (type 4)))
```
We can see that after macro expansion crates/wasi/src/lib.rs will contain
the following call:
```rust
                linker                                                          
                    .func_wrap(                                                 
                       "wasi_snapshot_preview1",                               
                       "environ_get",                                          
                       move |                                                  
                           mut caller: wiggle::wasmtime_crate::Caller<'_, T>,  
                           arg0: i32,                                          
                           arg1: i32,                                          
                       | -> wiggle::anyhow::Result<i32> {                     
```
So after we call `wasmtime_wasi::add_to_linker` the function `environ_get`
function in the module `wasi_snapshot_preview1` will be available to the
wasm module to be executed/loaded.

```rust
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    ...
    let mut store = Store::new(&engine, WasiCtxBuilder::new().build());
```
Notice that the type of `s` in the closure passed to add_to_linker is determined
based on the type of the Store's type, which in this case is of type

__work in progress__

So if we set a break point in matching.rs:75 where this error originates from
and perhaps we can work backwards to understant this issue:
```console
$ rust-gdb --args target/debug/wasi
(gdb) br matching.rs:75
(gdb) r
(gdb) bt
#0  wasmtime::component::matching::TypeChecker::instance (self=0x7fffffffc588, expected=0x555556f70290, actual=...)
    at src/component/matching.rs:75
#1  0x00005555557fda6c in wasmtime::component::matching::TypeChecker::definition (self=0x7fffffffc588, 
    expected=0x555556f8b418, actual=...) at src/component/matching.rs:25
#2  0x0000555555713333 in wasmtime::component::linker::Linker<wasi_common::ctx::WasiCtx>::instantiate_pre<wasi_common::ctx::WasiCtx> (self=0x7fffffffcc58, component=0x7fffffffcc40)
    at /home/danielbevenius/work/wasm/wasmtime/crates/wasmtime/src/component/linker.rs:138
#3  0x0000555555712fc7 in wasmtime::component::linker::Linker<wasi_common::ctx::WasiCtx>::instantiate<wasi_common::ctx::WasiCtx, &mut wasmtime::store::Store<wasi_common::ctx::WasiCtx>> (self=0x7fffffffcc58, store=0x7fffffffcce0, 
    component=0x7fffffffcc40) at /home/danielbevenius/work/wasm/wasmtime/crates/wasmtime/src/component/linker.rs:197
#4  0x000055555570e643 in wasi::main () at src/wasi.rs:68
```


[preview1 functions]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md

### Resolve
This is a struct in wasm-tools/crates/wit-parser:
```rust
#[derive(Default, Clone)]                                                       
pub struct Resolve {                                                            
    pub worlds: Arena<World>,                                                   
    pub interfaces: Arena<Interface>,                                           
    pub types: Arena<TypeDef>,                                                  
    pub packages: Arena<Package>,                                               
    pub package_names: IndexMap<PackageName, PackageId>,                        
}
```

### Generated files
We most often start out with a core wasm module, and normally this module will
have component information generated for it. If we take wasmtime-py as an
example it has a crate named bindgen. And in rust/bindgen/src/lib.rs we can see
that it uses  wit_bindgen to generate code from a .wit file:
```rust
wit_bindgen::generate!("wasmtime-py" in "../bindgen.wit");
```
We compile this crate using the target wasm32-wasi or wasm32-unknown-unknown
and get a core wasm module, with a custom section:
```console
$ wasm-objdump -h target/wasm32-wasi/release/bindgen.wasm 

bindgen.wasm:	file format wasm 0x1

Sections:

     Type start=0x0000000b end=0x0000018d (size=0x00000182) count: 43
   Import start=0x00000190 end=0x0000024a (size=0x000000ba) count: 5
 Function start=0x0000024d end=0x00001160 (size=0x00000f13) count: 3857
    Table start=0x00001162 end=0x00001169 (size=0x00000007) count: 1
   Memory start=0x0000116b end=0x0000116e (size=0x00000003) count: 1
   Global start=0x00001170 end=0x00001179 (size=0x00000009) count: 1
   Export start=0x0000117b end=0x000011e7 (size=0x0000006c) count: 4
     Elem start=0x000011ea end=0x000017b9 (size=0x000005cf) count: 1
     Code start=0x000017be end=0x0023008a (size=0x0022e8cc) count: 3857
     Data start=0x0023008e end=0x0026073d (size=0x000306af) count: 2
   Custom start=0x00260740 end=0x00260896 (size=0x00000156) "component-type:wasmtime-py"
   Custom start=0x0026089a end=0x002bf4ad (size=0x0005ec13) "name"
   Custom start=0x002bf4af end=0x002bf511 (size=0x00000062) "producers"
   Custom start=0x002bf513 end=0x002bf54c (size=0x00000039) "target_features"
```
Notice that we have a `component-type:wasmtime-py` custom section.
We then use `wasm-tools component` to generate a webassembly component modules
module:
```console
$ wasm-tools component new target/wasm32-wasi/release/bindgen.wasm \
	--adapt wasi_snapshot_preview1=wasi_preview1_component_adapter.wasm \
		-o target/component.wasm
```
If we inspect the generated `target/component.wasm` we can see that this
component has multiple modules:
```console
$ wasm-tools objdump target/component.wasm 
  ...
  module                                 |      0x649 -   0x2bfa69 |   2880544 bytes | 1 count
    ------ start module 0 -------------
    types                                |      0x654 -      0x7d6 |       386 bytes | 43 count
    imports                              |      0x7d9 -      0x893 |       186 bytes | 5 count
    functions                            |      0x896 -     0x17a9 |      3859 bytes | 3857 count
    tables                               |     0x17ab -     0x17b2 |         7 bytes | 1 count
    memories                             |     0x17b4 -     0x17b7 |         3 bytes | 1 count
    globals                              |     0x17b9 -     0x17c2 |         9 bytes | 1 count
    exports                              |     0x17c4 -     0x1830 |       108 bytes | 4 count
    elements                             |     0x1833 -     0x1e02 |      1487 bytes | 1 count
    code                                 |     0x1e07 -   0x2306d3 |   2287820 bytes | 3857 count
    data                                 |   0x2306d7 -   0x260d86 |    198319 bytes | 2 count
    custom "name"                        |   0x260d8f -   0x2bf99d |    388110 bytes | 1 count
    custom "producers"                   |   0x2bf9aa -   0x2bfa2e |       132 bytes | 1 count
    custom "target_features"             |   0x2bfa40 -   0x2bfa69 |        41 bytes | 1 count
    ------ end module 0 -------------
  module                                 |   0x2bfa6d -   0x2c3de1 |     17268 bytes | 1 count
    ------ start module 1 -------------
    types                                |   0x2bfa77 -   0x2bfabd |        70 bytes | 13 count
    imports                              |   0x2bfac0 -   0x2bfd19 |       601 bytes | 17 count
    functions                            |   0x2bfd1b -   0x2bfd49 |        46 bytes | 45 count
    globals                              |   0x2bfd4b -   0x2bfd61 |        22 bytes | 4 count
    exports                              |   0x2bfd63 -   0x2bfdd6 |       115 bytes | 7 count
    code                                 |   0x2bfdd9 -   0x2c2b0a |     11569 bytes | 45 count
    custom "name"                        |   0x2c2b12 -   0x2c3d7d |      4715 bytes | 1 count
    custom "producers"                   |   0x2c3d89 -   0x2c3de1 |        88 bytes | 1 count
    ------ end module 1 -------------
  module                                 |   0x2c3de4 -   0x2c41ba |       982 bytes | 1 count
    ------ start module 2 -------------
    types                                |   0x2c3dee -   0x2c3e1c |        46 bytes | 8 count
    functions                            |   0x2c3e1e -   0x2c3e2c |        14 bytes | 13 count
    tables                               |   0x2c3e2e -   0x2c3e33 |         5 bytes | 1 count
    exports                              |   0x2c3e35 -   0x2c3e78 |        67 bytes | 14 count
    code                                 |   0x2c3e7b -   0x2c3f20 |       165 bytes | 13 count
    custom "producers"                   |   0x2c3f2c -   0x2c3f50 |        36 bytes | 1 count
    custom "name"                        |   0x2c3f58 -   0x2c41ba |       610 bytes | 1 count
    ------ end module 2 -------------
  module                                 |   0x2c41bd -   0x2c42ae |       241 bytes | 1 count
    ------ start module 3 -------------
    types                                |   0x2c41c7 -   0x2c41f5 |        46 bytes | 8 count
    imports                              |   0x2c41f7 -   0x2c424b |        84 bytes | 14 count
    elements                             |   0x2c424d -   0x2c4260 |        19 bytes | 1 count
    custom "producers"                   |   0x2c426c -   0x2c4290 |        36 bytes | 1 count
    custom "name"                        |   0x2c4297 -   0x2c42ae |        23 bytes | 1 count
    ------ end module 3 -------------
   ...
```
These modules will be read from the component and the written into the output
directory as the files:
```
bindgen.core0.wasm
bindgen.core1.wasm
bindgen.core2.wasm
bindgen.core3.wasm
```
These are first added to a map and later written to the output directory.

Next in the generate function the imports of a World are handled.
```rust
          let world = &resolve.worlds[id];                                        
~         for (name, import) in world.imports.clone().into_iter() {               
              match import {                                                      
                  WorldItem::Function(_) => unimplemented!(),                         
~                 WorldItem::Interface(id) => {                                       
+                     let interface = &resolve.interfaces[id];                        
+                     println!("import interface: {:?}", &interface.name);            
+                                                                                     
+                     self.import_interface(                                          
+                         &resolve,                                                   
+                         interface.name.as_ref().unwrap().as_str(),                  
+                         id,                                                         
+                         files,                                                      
+                     )                                                               
+                 }                                                                   
                  WorldItem::Type(_) => unimplemented!(),                             
              }                                                                       
          }
```
Lets take a closer look at `import_interface`.
For each interface:
```console
import_interface...name: streams
import_interface...name: filesystem
import_interface...name: random
import_interface...name: environment
import_interface...name: preopens
import_interface...name: exit
import_interface...name: stdin
import_interface...name: stdout
import_interface...name: stderr
```
For each of these we call the following function:
```rust
     fn import_interface(                                                            
          &mut self,                                                                  
          resolve: &Resolve,                                                          
          name: &str,                                                                 
          iface: InterfaceId,                                                         
          files: &mut Files,                                                          
      ) {                                                                             
          println!("import_interface...name: {name}");                                
          self.imported_interfaces.insert(iface, name.to_string());                   
          let mut gen = self.interface(resolve);                                      
          gen.interface = Some(iface);                                                
          gen.types(iface);
```
This will first insert an entry into the imported_interfaces map.
```rust
pub struct WasmtimePy { 
    ...
    imported_interfaces: HashMap<InterfaceId, String>,
    ...
}
```
Next, a InterfaceGenerator will be created by the self.interface(resolve)
function call. This struct looks like this:
```
  struct InterfaceGenerator<'a> {
      src: Source,
      gen: &'a mut WasmtimePy,
      resolve: &'a Resolve,
      interface: Option<InterfaceId>,
      at_root: bool,
  }
```
So we have a `Source` which is looks like this:
```rust
#[derive(Default)]                                                                 
pub struct Source {
    body: Body,
    imports: PyImports,
}

#[derive(Default)]
pub struct Body {
    contents: String,
    indent: usize,
}
```
The gen field is just a reference to the outer WasmtimePy instance. Resolve
is the resolved component.
Next, the code sets the interface. After that we have a call to
`gen.types(iface)` which we will go though which is in the bindgen.rs files
as `gen` is a reference to WasmtimePy:
```rust
      fn types(&mut self, interface: InterfaceId) {                               
          for (name, id) in self.resolve.interfaces[interface].types.iter() {     
              let id = *id;                                                       
              let ty = &self.resolve.types[id];                                   
              match &ty.kind {                                                    
                  TypeDefKind::Record(record) => self.type_record(id, name, record, &ty.docs),
                  TypeDefKind::Flags(flags) => self.type_flags(id, name, flags, &ty.docs),
                  TypeDefKind::Tuple(tuple) => self.type_tuple(id, name, tuple, &ty.docs),
                  TypeDefKind::Enum(enum_) => self.type_enum(id, name, enum_, &ty.docs),
                  TypeDefKind::Variant(variant) => self.type_variant(id, name, variant, &ty.docs),
                  TypeDefKind::Option(t) => self.type_option(id, name, t, &ty.docs),
                  TypeDefKind::Result(r) => self.type_result(id, name, r, &ty.docs),
                  TypeDefKind::Union(u) => self.type_union(id, name, u, &ty.docs),
                  TypeDefKind::List(t) => self.type_list(id, name, t, &ty.docs),  
                  TypeDefKind::Type(t) => self.type_alias(id, name, t, &ty.docs), 
                  TypeDefKind::Future(_) => todo!("generate for future"),         
                  TypeDefKind::Stream(_) => todo!("generate for stream"),         
+                 TypeDefKind::Resource(_) => todo!("generate for resource"),     
+                 TypeDefKind::Handle(_) => todo!("generate for handle"),         
                  TypeDefKind::Unknown => unreachable!(),                         
              }                                                                   
          }                                                                       
      }
```
`ty` is of type `wit_parser::TypeDef`.
```console
import_interface...name: streams
name: input-stream, ty: Type(U32)
```
So this is of type `Type(U32)` which  will call `self.type_alias`:
```rust
      fn type_alias(&mut self, _id: TypeId, name: &str, ty: &Type, docs: &Docs) { 
          self.src.comment(docs);                                                 
          self.src                                                                
              .push_str(&format!("{} = ", name.to_upper_camel_case()));           
          self.print_ty(ty, false);                                               
          self.src.push_str("\n");                                                
      }
```
This will create the following lines in
../wasmtime/bindgen/generated/imports/streams.py:
```
InputStream = int
```
So this python variable would be generated to reprent this wit type alias:
```wit
interface streams {
    ...
    type input-stream = u32
```
This will be done for all types in that imported interface (and for the others
later). 
After the types have been written to the Source a class is created for this
`Streams` import:
```rust
fn import_interface(                                                        
          &mut self,                                                              
          resolve: &Resolve,                                                      
          name: &str,                                                             
          iface: InterfaceId,                                                     
          files: &mut Files,                                                      
      ) {                                                                         
          self.imported_interfaces.insert(iface, name.to_string());               
          let mut gen = self.interface(resolve);                                  
          gen.interface = Some(iface);                                            
          gen.types(iface);                                                       
                                                                                  
          // Generate a "protocol" class which I'm led to believe is the rough    
          // equivalent of a Rust trait in Python for this imported interface.    
          // This will be referenced in the constructor for the main component.   
          let camel = name.to_upper_camel_case();                                 
          let snake = name.to_snake_case();                                       
          gen.src.pyimport("typing", "Protocol");                                 
          uwriteln!(gen.src, "class {camel}(Protocol):");                         
          gen.src.indent();                                                       
          for (_, func) in resolve.interfaces[iface].functions.iter() {           
              gen.src.pyimport("abc", "abstractmethod");                          
              gen.src.push_str("@abstractmethod\n");                              
              gen.print_sig(func, true);                                          
              gen.src.push_str(":\n");                                            
              gen.src.indent();                                                   
              gen.src.push_str("raise NotImplementedError\n");                    
              gen.src.dedent();                                                   
          }                                                                       
          if resolve.interfaces[iface].functions.is_empty() {                     
              gen.src.push_str("pass\n");                                         
          }                                                                       
          gen.src.dedent();                                                       
          gen.src.push_str("\n");                                                 
                                                                                  
          let src = gen.src.finish();                                             
          files.push(&format!("imports/{snake}.py"), src.as_bytes());             
          self.imports.push(name.to_string());                                    
      }                                
```
Notice that the functions are not implemented:
```
class Streams(Protocol):
    @abstractmethod
    def drop_input_stream(self, this: InputStream) -> None:
        raise NotImplementedError
    @abstractmethod
    def write(self, this: OutputStream, buf: bytes) -> Result[int, StreamError]:
        raise NotImplementedError
    @abstractmethod
    def blocking_write(self, this: OutputStream, buf: bytes) -> Result[int, StreamError]:
        raise NotImplementedError
    @abstractmethod
    def drop_output_stream(self, this: OutputStream) -> None:
        raise NotImplementedError
```
After that we do something simlilar for the exports.



### wasmtime-py bindings upgrade
While trying to update the wit syntax with the syntax I'm running into a few
issues which I'm not sure about the best/proper way to solve them.

One issue is that when we call generate:
```console
$ cargo run -p=bindgen --features=cli target/component.wasm ../wasmtime/bindgen/generated
```
This will first invoke rust/bindgen/src/bin/bootstrap.rs which in turn will
call `generate` in rust/bindgen/src/lib.rs. This function will decode the passed
in webassembly component model modules `target/component.wasm` which was created
in a previous step and specifing `wasi_preview1_component_adapter.wasm` which
I built to make sure that it also used the latest wit updates. Now he decode
function looks like this:
```rust
    let (resolve, id) = match wit_component::decode(binary) 
```
`wit-component::decode` can be round in the decoding module of wit-component:
(src/decoding.rs)
```rust
 pub use decoding::{decode, DecodedWasm};
```
In the versions prior to the wit updates decode would take a &str which was
the name of the component (I think) and this would also be part of the wasm
modules that are extracted from the component and placed in the generated
directory. So before we had:
```
bindgen.core0.wasm  bindgen.core2.wasm
bindgen.core1.wasm  bindgen.core3.wasm
```
Without any changes this will still be the case but, the code that referes to
these .wasm files will use root.core0.wasm. I'm been going back and forth about
if I should change this from root to bindgen or not. 


### Building 
To build the c-api in wasmtime the following command can be used:
```console
$ cargo build --release --manifest-path crates/c-api/Cargo.toml
```
On my linux system/platform the above command will generate the following
libraries:
```
target/release/libwasmtime.a
target/release/libwasmtime.so
```
Next, we can run a script that creates a release tar. The motivation for doing
this is that I wanted to try out an unreleased version with the wasmtime-py
project.
```console
$ ./ci/build-tarballs.sh linux
```
That script will generate two compressed tar files in dist:
```console
$ ls dist/
wasmtime-dev-linux-c-api.tar.xz  wasmtime-dev-linux.tar.xz
```
