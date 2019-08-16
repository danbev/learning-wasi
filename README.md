### Learning Web Assembly System Interface (WASI)

### Configuration
```console
$ xcode-select --install
$ brew install llvm
==> Caveats
To use the bundled libc++ please add the following LDFLAGS:
  LDFLAGS="-L/usr/local/opt/llvm/lib -Wl,-rpath,/usr/local/opt/llvm/lib"

llvm is keg-only, which means it was not symlinked into /usr/local,
because macOS already provides this software and installing another version in
parallel can cause all kinds of trouble.

If you need to have llvm first in your PATH run:
  echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.bash_profile

For compilers to find llvm you may need to set:
  export LDFLAGS="-L/usr/local/opt/llvm/lib"
  export CPPFLAGS="-I/usr/local/opt/llvm/include"
```

```console
$ /usr/local/opt/llvm/bin/llc --version
LLVM (http://llvm.org/):
  LLVM version 8.0.0
  Optimized build.
  Default target: x86_64-apple-darwin18.0.0
  Host CPU: haswell

  Registered Targets:
    ...
    wasm32     - WebAssembly 32-bit
    wasm64     - WebAssembly 64-bit
    ...
```
So this looks pretty good to me, we have `wasm32` and `wasm64`.

#### Install Rust using rustup:
```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup install nightly-x86_64-apple-darwin
```
Add the `wasm32-wasi` target:
```console
$ rustup target add wasm32-wasi --toolchain nightly
```

### wasmtime
Use the following command to build wasmtime:
```console
$ CC="clang" CXX="clang++" cargo +nightly build --release
```

### Building wasi-libc
```console
$ make WASM_CC=/usr/local/opt/llvm/bin/clang WASM_AR=/usr/local/opt/llvm/bin/llvm-ar WASM_NM=/usr/local/opt/llvm/bin/llvm-nm
...
#
# The build succeeded! The generated sysroot is in /Users/danielbevenius/work/wasi/wasi-libc/sysroot.
#

```
Download [libclang_rt.builtins-wasm32.a](https://github.com/jedisct1/libclang_rt.builtins-wasm32.a)
and copy it to the wasi lib:
```console
$ mkdir /usr/local/Cellar/llvm/8.0.0_1/lib/clang/8.0.0/lib/wasi
$ cp ~/Downloads/libclang_rt.builtins-wasm32.a /usr/local/Cellar/llvm/8.0.0_1/lib/clang/8.0.0/lib/wasi/
```

### Compile a c program
```console
$ make
```

### Run a WebAssembly binary with wasmtime:
```console
$ wasmtime/target/debug/wasmtime out/first.wasm

```

### Debugging wasmtime
First build without `--release` and then use rust-lldb:
```console
$ rust-lldb target/debug/wasmtime out/first.wasm
```

### Rustup 
Install beta channel:
```console
$ rustup install beta
```
Add wasi target to beta:
```console
$ rustup target add wasm32-wasi --toolchain beta
```
Build using beta:
```console
$ cargo +beta build --target=wasm32-wasi
```

### Wasmer
Install:
```console
$ curl https://get.wasmer.io -sSfL | sh
$ source /Users/danielbevenius/.wasmer/wasmer.sh
```
Run:
```console
$ wasmer run out/first.wasm
First wasi...
```

### WebAssembly (WASM)
The text format for wasm is of type S-expressions where the first label inside 
a parentheses tell what kind of node it is:
```wasm
(module (memory 1) (func))
```
The above has a root node named `module` and two child nodes, `memory` and `func`.
All code is grouped into functions:
```wasm
(func <signature> <locals> <body>)
```
The signature declares the functions parameters and its return type.
The locals are local variables to the function
The body is a list of instructions for the fuction.

```wasm
(module
  (func $add (param $first i32) (param $second i32) (result i32)
    get_local $first
    get_local $second
    (i32.add)
  )
  (export "add" (func $add))
)
```
Notice that a wasm "program" is simply named a module as the intention is to have 
it included and run by another program.
The body is stack based so `get_local` will push $first onto the stack. 
`i32.add` will take two values from the stack, add then and push the result onto
the stack.
Notice the `$add` in the function. This is much like the parameters that are 
index based but can be named to make the code clearer. So we could just as well
written:
```wasm
  (export "add" (func 0))
```
export is a function that makes the function available, using the name `add` in 
our case.

You can compile the above .wat file to wasm using [wabt](https://github.com/WebAssembly/wabt):
```console
$ out/clang/Debug/wat2wasm ~/work/nodejs/scripts/wasm-helloworld.wat -o helloworld.wasm
```
And the use the wasm in Node.js:
```javascript
const fs = require('fs');
const buffer = fs.readFileSync('helloworld.wasm');

const promise = WebAssembly.instantiate(buffer, {});
promise.then((result) => {
  const instance = result.instance;
  const module = result.module;
  console.log('instance.exports:', instance.exports);
  const addTwo = instance.exports.addTwo;
  console.log(addTwo(1, 2));
});
Lets take a closer look at the WebAssembly API.

`WebAssembly` is the how the api is exposed.
WebAssembly.instantiate:
compiles and instantiates wasm code and returns both an object with two
members `module` and `instance`.

To inspect the .wasm you can use wasm-objdump:
```console
$ wasm-objdump -x src/add.wasm

add.wasm:file format wasm 0x1

Section Details:

Type:
 - type[0] (i32, i32) -> i32
Function:
 - func[0] sig=0 <add>
 - func[1] sig=0 <addTwo>
Export:
 - func[0] <add> -> "add"
 - func[1] <addTwo> -> "addTwo"
```

### wasm c-api
The following repo, [wasm-c-api](https://github.com:rossberg/wasm-c-api) a C API to allow you
to use function defined in wasm from C/C++.

Make sure you configure V8 to have the following configuration options:
```console
$ gn args out.gn/x64.release/
is_debug = false
target_cpu = "x64"
is_component_build = false
v8_static_library = true
```

V8 is quite large and I looks like wasm-c-api expects v8 to be cloned in the
same directory. I just updated the Makefile to allow the V8 dir to be configured
to allow building using:
```console
$ make V8_DIR="/Users/danielbevenius/work/google/javascript" CFLAGS="-g"
```

`WebAssembly.Memory` is used to deal with more complex objects like strings. Is
just a large array of bytes which can grow. You can read/write using i32.load
and i32.store.

Memory is specified using WebAssembly.Memory{}:
```javascript
const memory = new WebAssembly.Memory({initial:10, maximum:100});
```
`10` and `100` are specified in pages which are fixed to 64KiB.
So here we are saying that we want an initial size of 640KiB.

### musl
A libc implementation. Pronounced muscle.

### Building llvm
```console
$ mkdir build
$ cd build
$ cmake -G Ninja -DCMAKE_INSTALL_PREFIX=/opt/llvm-dist/ -DCMAKE_BUILD_TYPE=Release -DLLVM_ENABLE_PROJECTS="clang;libcxx;libcxxabi" ../llvm
$ ninja
$ ninja dist
```

### Installing ninja
```console
$ git clone git://github.com/martine/ninja.git
$ configure.py --bootstrap
```
The ninja executable will be in the same directory.


### Troubleshooting
wasmtime compilation error:
```console
cargo:warning=In file included from signalhandlers/SignalHandlers.cpp:8:
cargo:warning=In file included from signalhandlers/SignalHandlers.hpp:5:
cargo:warning=/usr/local/opt/llvm/bin/../include/c++/v1/setjmp.h:35:15: fatal error: 'setjmp.h' file not found
cargo:warning=#include_next <setjmp.h>
cargo:warning=              ^~~~~~~~~~
cargo:warning=1 error generated.
exit code: 1

--- stderr
```
It seems that after upgrading to Mojove some headers were no longer in the `/include`
directory. These can be installed using the following command:
```console
open /Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg
```
