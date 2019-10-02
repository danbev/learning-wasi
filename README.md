### Learning Web Assembly System Interface (WASI)
This project is a collection of notes and examples to help me understand WASI

### fd_write
The example [fd_write.wat](src/fd_write.wat) shows the usage of the
[fd_write](https://github.com/WebAssembly/WASI/blob/master/design/WASI-core.md#__wasi_fd_write) system call.
The input to fd_write are:
```
__wasi_fd_write(__wasi_fd_t fd, const __wasi_ciovec_t *iovs and size_t iovs_len
```
`__wasi_fd_t` is just defined as:
```c
typedef uint32_t uvwasi_fd_t;
```

And `__wasi_ciovec_t` as:
```c
typedef struct uvwasi_ciovec_s {
  const void* buf;
  size_t buf_len;
} uvwasi_ciovec_t;
```
So we can see that we have a pointer to a buffer and a length.

### args_sizes_get
The example [args_sizes_get.wat](src/args_sizes_get.wat) contains an example of calling 
[__wasi_args_sizes_get](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#__wasi_args_sizes_get).

This shown an important point that I totally missed when first looking at calling
it. Looking at the documentation we can see that this function outputs:
```
size_t argc            The number of arguments
size_t argv_buf_size   The size of the argument string data.
```
What I did not understand was that there are pointers that are passed into the
function. So we have to specify the memory locations that it should use to 
populate these values. 

The test can be run manually:
```console
$ ./wasmtime/target/release/wasmtime src/args_sizes_get.wat one two three four five six
$ echo $?
7
```
Just note that the name of the program also counts as an argument.

### args_get
For this example we need to set up an char** to store the information in.
We need to pass this into the args_get function and it will populate the.

The documentation for args_get states that the sizes of the buffers should
match that returned by __wasi_args_sizes_get(). In this case we are going to
hard code these sizes for simplicity.

The example [args_get.wat](src/args_get.wat) currently hard codes everything
and should be invoked with like this:
:
```console
$ ./wasmtime/target/release/wasmtime src/args_get.wat one two
args_get.wat
one
two
```
The reason for doing this is to demonstrate and make the example as simple as
possible so that I understand how memory management works.

Take the following C main function:
```
int main(int argc, char** argv) {
```
The standard library will set up argv for us:
```
 argv           char*
+-------+      +--------+      +--------+
|address| ---->|address | ---->|progname|
+-------+      +--------+      +--------+
```
The following is using a simple C example:
```console
$ lldb -- ptp one two three
(lldb) expr argv
(char **) $1 = 0x00007ffeefbfefd8
(lldb) memory read -f x -s 8 -c 8 0x00007ffeefbfefb8
0x7ffeefbfefb8: 0x00007ffeefbff288 0x00007ffeefbff2d3
0x7ffeefbfefc8: 0x00007ffeefbff2d7 0x00007ffeefbff2db

(lldb) expr *argv
(char *) $1 = 0x00007ffeefbff288 "/Users/danielbevenius/work/c++/learningc++11/src/fundamentals/pointers/ptp"
```

We can visualize this:
```

      char**                      char*
+------------------+       +------------------+       +--------+
|0x00007ffeefbfefb8| ----> |0x00007ffeefbff288| ----> |progname|
+------------------+       +------------------+       +--------+
                           +------------------+       +--------+
                           |0x00007ffeefbff2d3| ----> | "one"  |
                           +------------------+       +--------+
                           +------------------+       +--------+
                           |0x00007ffeefbff2d7| ----> | "two"  |
                           +------------------+       +--------+
                           +------------------+       +--------+
                           |0x00007ffeefbff2db| ----> | "three"|
                           +------------------+       +--------+
```
In our case with `get_args` we need to set this memory up manually. First, we
need argv** which is a pointer, size 4 if we are using 32 bit addressing.
```
argv:
    i32.const 0  ;; offset for argv pointer
    i32.const 0  ;; value 
    i32.store align=2
```
Remember the pointers should all come after each other in memory, so we should
be able to add able to dereference argv to get the first pointer, then add 4
to get to the second pointer.
So 
```        
+------------------+       +------------------+       +--------------+
|      4           | ----> |                  | ----> |"args_get.wat"|
+------------------+       +------------------+       +--------------+
0                  3       4                  8       64             76
                           +------------------+       +--------+
                           |                  | ----> | "one"  |
                           +------------------+       +--------+
                           9                 13       77      80
                           +------------------+       +--------+
                           |                  | ----> | "two"  |
                           +------------------+       +--------+
                           14                18       81      84
```

Just note that if the program name will not include the directory, only the
name of the executable:
```console
$ ./wasmtime/target/release/wasmtime out/first.wasm
args[0]=first.wasm
```
Just keep this in mind when inspecting memory as it took me a while to realise
that the directory was not expected.

```
Inputs:
char **argv
A pointer to a buffer to write the argument pointers.
char *argv_buf
A pointer to a buffer to write the argument string data.
```
```
Memory:
        8-bit 16-bit 32-bit         64-bit
         ↓    ↓       ↓               ↓
      [00][00][00][00][00][00][00][00][00][00][00][00]...
8-bit [ 0][ 1][ 2][ 3][ 4][ 5][ 6][ 7][ 8][ 9][10][11]...
16-bit[  0   ][  1   ][   2  ][   3  ][   4  ][  5   ]...
32-bit[      0       ][       1      ][      2       ]...
64-bit[              0               ][              1             ]...
```


### environ_sizes_get
The example [environ_sizes_get.wat](src/environ_sizes_get.wat) contains an 
example of calling [__wasi_environ_sizes_get](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#__wasi_environ_sizes_get).

```console
$ ./wasmtime/target/release/wasmtime --env="ONE=1" --env="TWO=2" src/environ_sizes_get.wat
$ echo $?
2
```

### environ_get
The example [environ_get.wat](src/environ_get.wat) contains an of calling environ_get. 
```
$ ./wasmtime/target/release/wasmtime --env="ONE=1" --env="TWO=2" src/environ_get.wat
```

### clock_res_get

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
$ RUSTFLAGS=-g CC="clang" CXX="clang++" cargo +nightly build --release
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
```
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


```console
c/cant_dotdot.c:1:10: fatal error: 'assert.h' file not found
#include <assert.h>
         ^~~~~~~~~~
1 error generated.

or 

/usr/include/sys/cdefs.h:784:2: error: Unsupported architecture
#error Unsupported architecture
```
Make sure you have specifed the correct --sysroot.




### Wasm modules
Sections in a module
```
* Types
* Import
* Function
* Tables
* Memory
* Global
* Export
* Start
* Element
* Code
* Data
```

#### Types section
Are function signatures that can be reused, for example for imports, functions
definitaions.

#### Table section
This maps values like JavaScript objects or OS file handles. Its a way to
allow the wasm module to access something outside its memory. For example, say
you have a function pointer which we want to call from our wasm module. If we 
had direct access to this memory pointer we might be able to learn about the 
memory layout and exploit it. For example, updating the pointer to something
different might execute some other piece of code.
A table is an array that lives outside of wasm's memory and the values stored
in the array are references to functions.

#### Element section
This section allows for the intialization the content or a table imported
or defined in the table section. What would this be like, would this be like
passing in an empty table entry and populating it with a function pointer to
a function in the wasm module. But would we not just export the function in 
that was (I'm probabably not understanding the usage here fully).

#### Memory section
Defines the optional memory of the module by defining its initial and max
size. This memory can be initialized by the data section.


#### Global section
This section contains any global (think static C variables).

### Export section
This are functions, tables, memory segements and global variables that are made
available to the host.


#### Stack
The Stack operations takes their operands from—and put their result onto—the
stack. There is now way to inspect the stack apart from using opcodes that 
push and pop values from the stack.

#### Local and globals
Locals and globals are like variables, they are named and can hold any of the
same basic types as the stack can hold (i32, et al.)

#### Memory
Memory is linear so all memory addresses used are expressed in terms of byte
offsets from the beginning of a memory segment

```
i32.load offset= alignment=
```
Now, operators can have immediate arguments and are considered part of the
instructions themselves.  The `alignment` is a hint of the alignment
```
0 = 8 bit
1 = 16 bit
2 = 32 bit
3 = 64 bit
```
```
        8-bit 16-bit 32-bit         64-bit
         ↓    ↓       ↓               ↓
      [00][00][00][00][00][00][00][00][00][00][00][00]...
8-bit [ 0][ 1][ 2][ 3][ 4][ 5][ 6][ 7][ 8][ 9][10][11]...
16-bit[  0   ][  1   ][   2  ][   3  ][   4  ][  5   ]...
32-bit[      0       ][       1      ][      2       ]...
64-bit[              0               ][              1             ]...
```
The second immediate for load is the address `offset`. The effective
address is the sum of the address operand and the offset.
```
                    (stack)          (immediate)
effective-address = address-operand + offset
```
The reason for the offset is when using dynamic memory where a compiler
may add a constant offset to all memory operations in order to relocate one
area of memory to another.

So when we want to store a value in memory we need to specify a address
operand
```
i32.const 0                  ;; address operand
i32.const 12                 ;; value to store
i32.store offset=0 align=2   ;; size_buf_len
```
The offset defaults to 𝟶, the alignment to the storage size of the
respective memory access, which is its natural alignment.

This would store the value `18` at address 0:
```
                    32-bit
                     ↓
      [12][00][00][00][00][00][00][00][00][00][00][00]...
      [       0      ][       1      ][      2       ]...
```
Notice that everything is stored in little endian.


#### Element
Elements are “handles” for opaque foreign values (like OS file handles.)


#### Labels
Unlike with other index spaces, indexing of labels is relative by nesting
depth, that is, label 0 refers to the innermost structured control instruction
enclosing the referring branch instruction, while increasing indices refer to
those farther out. 





### libuv Wasi (uvwasi)
```c
typedef struct uvwasi_s {
  struct uvwasi_fd_table_t fds;
  size_t argc;
  char** argv;
  char* argv_buf;
  size_t argv_buf_size;
  size_t envc;
  char** env;
  char* env_buf;
  size_t env_buf_size;
} uvwasi_t;
```
So we have a file descriptor table first followed by argc and argv. 
These are the arguments passed to the module. How are the passed?

In the uvwasi example these are configured programatically:
```c
  uvwasi_options_t init_options;
  ...
  init_options.argc = 3;
  init_options.argv = calloc(3, sizeof(char*));
  init_options.argv[0] = "--foo=bar";
  init_options.argv[1] = "-baz";
  init_options.argv[2] = "100";
  init_options.envp = (const char**) environ;
  init_options.preopenc = 1;
  init_options.preopens = calloc(1, sizeof(uvwasi_preopen_t));
  init_options.preopens[0].mapped_path = "/var";
  init_options.preopens[0].real_path = ".";

```
```c:
  r = uvwasi_init(uvw, &init_options);
```

### Inspecting the linked libraries
```console
$ otool -L  out/app
out/app:
	/usr/local/lib/libuv.1.dylib (compatibility version 2.0.0, current version 2.0.0)
	/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1252.250.1)
```
You can also use this environment variable:
```console
$ DYLD_PRINT_LIBRARIES=1 out/app
dyld: loaded: /Users/danielbevenius/work/wasm/uvwasi/out/app
dyld: loaded: /usr/local/lib/libuv.1.dylib
dyld: loaded: /usr/lib/libSystem.B.dylib
dyld: loaded: /usr/lib/system/libcache.dylib
dyld: loaded: /usr/lib/system/libcommonCrypto.dylib
dyld: loaded: /usr/lib/system/libcompiler_rt.dylib
dyld: loaded: /usr/lib/system/libcopyfile.dylib
dyld: loaded: /usr/lib/system/libcorecrypto.dylib
dyld: loaded: /usr/lib/system/libdispatch.dylib
dyld: loaded: /usr/lib/system/libdyld.dylib
dyld: loaded: /usr/lib/system/libkeymgr.dylib
dyld: loaded: /usr/lib/system/liblaunch.dylib
dyld: loaded: /usr/lib/system/libmacho.dylib
dyld: loaded: /usr/lib/system/libquarantine.dylib
dyld: loaded: /usr/lib/system/libremovefile.dylib
dyld: loaded: /usr/lib/system/libsystem_asl.dylib
dyld: loaded: /usr/lib/system/libsystem_blocks.dylib
dyld: loaded: /usr/lib/system/libsystem_c.dylib
dyld: loaded: /usr/lib/system/libsystem_configuration.dylib
dyld: loaded: /usr/lib/system/libsystem_coreservices.dylib
dyld: loaded: /usr/lib/system/libsystem_darwin.dylib
dyld: loaded: /usr/lib/system/libsystem_dnssd.dylib
dyld: loaded: /usr/lib/system/libsystem_info.dylib
dyld: loaded: /usr/lib/system/libsystem_m.dylib
dyld: loaded: /usr/lib/system/libsystem_malloc.dylib
dyld: loaded: /usr/lib/system/libsystem_networkextension.dylib
dyld: loaded: /usr/lib/system/libsystem_notify.dylib
dyld: loaded: /usr/lib/system/libsystem_sandbox.dylib
dyld: loaded: /usr/lib/system/libsystem_secinit.dylib
dyld: loaded: /usr/lib/system/libsystem_kernel.dylib
dyld: loaded: /usr/lib/system/libsystem_platform.dylib
dyld: loaded: /usr/lib/system/libsystem_pthread.dylib
dyld: loaded: /usr/lib/system/libsystem_symptoms.dylib
dyld: loaded: /usr/lib/system/libsystem_trace.dylib
dyld: loaded: /usr/lib/system/libunwind.dylib
dyld: loaded: /usr/lib/system/libxpc.dylib
dyld: loaded: /usr/lib/libobjc.A.dylib
dyld: loaded: /usr/lib/libc++abi.dylib
dyld: loaded: /usr/lib/libc++.1.dylib
uvwasi_fd_fdstat_get()
	stats.fs_rights_base = 6291603
uvwasi_fd_fdstat_get()
	stats.fs_rights_base = 6291603
```
In my case I don't want to use the system libuv but instead on that I've build
with debug symbols.


Show contents of archive:
```console
$ ar -t ~/work/nodejs/libuv/out/Debug/libuv.a
__.SYMDEF SORTED
fs-poll.o
idna.o
inet.o
threadpool.o
timer.o
uv-data-getter-setters.o
uv-common.o
version.o
async.o
core.o
dl.o
fs.o
getaddrinfo.o
getnameinfo.o
loop.o
loop-watcher.o
pipe.o
poll.o
process.o
signal.o
stream.o
tcp.o
thread.o
tty.o
udp.o
proctitle.o
darwin.o
fsevents.o
darwin-proctitle.o
bsd-ifaddrs.o
kqueue.o
```
See all the symbols:
```console
$ nm ~/work/nodejs/libuv/out/Debug/libuv.a
```

```console
gcc -o ./out/app out/obj/uvwasi.o out/obj/fd_table.o out/obj/uv_mapping.o app.c -g -L/Users/danielbevenius/work/nodejs/libuv/out/Debug/ -luv -Wall -Wsign-compare -I./include -luv
Undefined symbols for architecture x86_64:
  "_uv_gettimeofday", referenced from:
      _uvwasi_clock_time_get in uvwasi.o
ld: symbol(s) not found for architecture x86_64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```
```console
$ ar -t ~/work/nodejs/libuv/out/Debug/libuv.a  | grep _uv_gettimeof_day
```
Notice that this symbol does not exist.

```console
$ man dyld
```
Now run with `DYLD_LIBRARY_PATH`
```console
$ DYLD_LIBRARY_PATH=/Users/danielbevenius/work/nodejs/libuv-build/lib ./out/app
```
