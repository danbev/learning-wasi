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

$ /usr/local/opt/llvm/bin/llc --version
LLVM (http://llvm.org/):
  LLVM version 8.0.0
  Optimized build.
  Default target: x86_64-apple-darwin18.0.0
  Host CPU: haswell

  Registered Targets:
    aarch64    - AArch64 (little endian)
    aarch64_be - AArch64 (big endian)
    amdgcn     - AMD GCN GPUs
    arm        - ARM
    arm64      - ARM64 (little endian)
    armeb      - ARM (big endian)
    bpf        - BPF (host endian)
    bpfeb      - BPF (big endian)
    bpfel      - BPF (little endian)
    hexagon    - Hexagon
    lanai      - Lanai
    mips       - MIPS (32-bit big endian)
    mips64     - MIPS (64-bit big endian)
    mips64el   - MIPS (64-bit little endian)
    mipsel     - MIPS (32-bit little endian)
    msp430     - MSP430 [experimental]
    nvptx      - NVIDIA PTX 32-bit
    nvptx64    - NVIDIA PTX 64-bit
    ppc32      - PowerPC 32
    ppc64      - PowerPC 64
    ppc64le    - PowerPC 64 LE
    r600       - AMD GPUs HD2XXX-HD6XXX
    sparc      - Sparc
    sparcel    - Sparc LE
    sparcv9    - Sparc V9
    systemz    - SystemZ
    thumb      - Thumb
    thumbeb    - Thumb (big endian)
    wasm32     - WebAssembly 32-bit
    wasm64     - WebAssembly 64-bit
    x86        - 32-bit X86: Pentium-Pro and above
    x86-64     - 64-bit X86: EM64T and AMD64
    xcore      - XCore
```

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup install nightly-x86_64-apple-darwin
$ rustup target add wasm32-wasi --toolchain nightly
$ cargo +nightly build --target wasm32-wasi
```

Issue when building wasmtime:
```console
$ cargo build --release
   Compiling backtrace-sys v0.1.28
   Compiling libloading v0.5.1
   Compiling wabt-sys v0.5.4
   Compiling synstructure v0.10.2
   Compiling scroll_derive v0.9.5
   Compiling structopt-derive v0.2.16
   Compiling memoffset v0.3.0
   Compiling raw-cpuid v6.1.0
error: failed to run custom build command for `libloading v0.5.1`
process didn't exit successfully: `/Users/danielbevenius/work/wasi/wasmtime/target/release/build/libloading-fbedac732d6452a3/build-script-build` (exit code: 1)
--- stdout
TARGET = Some("x86_64-apple-darwin")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-apple-darwin")
CC_x86_64-apple-darwin = None
CC_x86_64_apple_darwin = None
HOST_CC = None
CC = Some("ccache clang -Qunused-arguments")
CFLAGS_x86_64-apple-darwin = None
CFLAGS_x86_64_apple_darwin = None
HOST_CFLAGS = None
CFLAGS = None
CRATE_CC_NO_DEFAULTS = None
DEBUG = Some("false")
running: "ccache" "clang" "-Qunused-arguments" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-Wall" "-Wextra" "-o" "/Users/danielbevenius/work/wasi/wasmtime/target/release/build/libloading-790d6f32cec8c7fa/out/src/os/unix/global_static.o" "-c" "src/os/unix/global_static.c"
cargo:warning=src/os/unix/global_static.c:1:10: fatal error: 'pthread.h' file not found
cargo:warning=#include <pthread.h>
cargo:warning=         ^~~~~~~~~~~
cargo:warning=1 error generated.
exit code: 1

--- stderr


error occurred: Command "ccache" "clang" "-Qunused-arguments" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-apple-darwin" "-Wall" "-Wextra" "-o" "/Users/danielbevenius/work/wasi/wasmtime/target/release/build/libloading-790d6f32cec8c7fa/out/src/os/unix/global_static.o" "-c" "src/os/unix/global_static.c" with args "clang" did not execute successfully (status code exit code: 1).
```
Make sure you unset your `ccache` environment variables.


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
$ cp ~/Downloads/libclang_rt.builtins-wasm32.a /usr/local/Cellar/llvm/8.0.0_1/lib/clang/8.0.0/lib/wasi/
$ mkdir /usr/local/Cellar/llvm/8.0.0_1/lib/clang/8.0.0/lib/wasi
```

### Compile a c program
```console
$ . ./setenv.sh
$ clang --target=wasm32-unknown-wasi --sysroot /tmp/wasi-libc \
  -O2 -s -o example.wasm example.c
$ clang --target=wasm32-unknown-wasi --sysroot ../wasi-libc/sysroot -O2 -s -o first.wasm first.c
wasm-ld: error: cannot open /usr/local/Cellar/llvm/8.0.0_1/lib/clang/8.0.0/lib/wasi/libclang_rt.builtins-wasm32.a: No such file or directory
clang-8: error: lld command failed with exit code 1 (use -v to see invocation)
```
See above about how to fix this issue.


### Motivation
"V8 is completely sandboxed and does not offer a way to talk to host systems. 
Node.js is a way to open up V8 to allow it to take to the host system. Unfortunately, 
Node.js completely opens up the host system to the application in an uncontrolled and unmanaged way."

Unlike Docker which also provides fine grained sandboxing, WebAssembly operates 
at the application level not the OS userland level. This means WebAssembly 
programs can be started faster and consume less resources 
on both the host system and also when being transported over the wire.

A completely sandboxed and lightweight environment can allow for more tightly 
packing serverless applications on the same machine - allowing for serverless 
providers to lower costs. Additionally, startup times should be much lower (theoretically on the order of 1-2 ms).

Also, WebAssembly is meant to be completely language agnostic so in the future, 
you should be able to run whatever languages are capable of running in a 
WebAssembly environment, which could be every language.
