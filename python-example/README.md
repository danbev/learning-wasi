## Rust Python wasi example
This is an example of a wasi module that is written in Rust and evaluates and
Python script/snippet.

The goal is just to verify that it is possible to run Python code from Rust
in a wasi module.

### Building
```console
$ make build
```

### Running
```console
$ make run 
wasmtime \
--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
        target/wasm32-wasi/release/python_example.wasm
Going to eval the following Python code:  print('Printing from Python!')
Printing from Python!
Python code executed successfully!
```

### Wasi Python
There is a build.rs file that will download the python dependencies:
```rust
use wlr_libpy::bld_cfg::configure_static_libs;

fn main() {
    configure_static_libs().unwrap().emit_link_flags();
}
```
[configure_static_libs](https://github.com/vmware-labs/webassembly-language-runtimes/blob/5a40c308763309d227dd1dd85fe3e9397282c6d9/python/tools/wlr-libpy/src/bld_cfg.rs#L41).
This will download a wasi-sysroot, libclang_rt.builtins-wasm32-wasi.a, and

### Wasi sysroot
In traditional development for operating systems like Linux, Windows, or macOS,
the system root (often referred to as /usr) contains the libraries and tools
necessary for compiling and running applications. These include standard
libraries, header files, and other resources.
Similarly, a wasi-sysroot provides these components but tailored for WebAssembly
applications targeting WASI. 
* Standard C library implementations
* Header files for compiling code that uses WASI APIs

### libclang_rt
The compiler-rt project is a part of the LLVM project. It is a static library
that is part of the LLVM compiler infrastructure project. Specifically, it is a
runtime library for LLVM's Clang compiler that provides implementations of
low-level operations for the WebAssembly (wasm32) target. These low-level
operations include built-in functions that the compiler may use for various
purposes, such as arithmetic operations, bit manipulation, and other fundamental
operations that are not directly provided by the WebAssembly instruction set.


### Python Packages
A Python package can be distributed as a wheel which is basically an archive
of a build package, and there are also source distributions (sdist). The built
wheels can be dependent/compatible with a specific python version, OS, or
hardware arch.

When performing `pip install <packge>` pip will try to install a built wheel
and fall backe to a source distribution if a wheel is not available (at least
that is my understanding).

### Python packages in wasi
The example currently contains an example of using the `emoji` package. This
made availabe to the wasi runtime by specifying a directory mapping. While this
worked for the `emoji` package, it does not work for numpy which was my original
goal.

When trying to use numpy I get the following error:
```console
$ make run
env RUST_BACKTRACE=1 wasmtime \
--dir=./target/wasm32-wasi/wasi-deps/usr::/usr \
--dir=./target/numpy::/site-packages/numpy \
        target/wasm32-wasi/release/python_example.wasm
thread '<unnamed>' panicked at src/lib.rs:13:37:
called `Result::unwrap()` on an `Err` value: PyErr { type: <class 'ImportError'>, value: ImportError('\n\nIMPORTANT: PLEASE READ THIS FOR ADVICE ON HOW TO SOLVE THIS ISSUE!\n\nImporting the numpy C-extensions failed. This error can happen for\nmany reasons, often due to issues with your setup or how NumPy was\ninstalled.\n\nWe have compiled some common reasons and troubleshooting tips at:\n\n    https://numpy.org/devdocs/user/troubleshooting-importerror.html\n\nPlease note and check the following:\n\n  * The Python version is: Python3.12 from ""\n  * The NumPy version is: "1.26.0b1"\n\nand make sure that they are the versions you expect.\nPlease carefully study the documentation linked above for further help.\n\nOriginal error was: No module named \'numpy.core._multiarray_umath\'\n'), traceback: Some(<traceback object at 0x68c258>) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: failed to run main module `target/wasm32-wasi/release/python_example.wasm`

Caused by:
    0: failed to invoke command default
    1: error while executing at wasm backtrace:
           0: 0xe7d89b - <unknown>!__rust_start_panic
           1: 0xe7d6e5 - <unknown>!rust_panic
           2: 0xe7d618 - <unknown>!std::panicking::rust_panic_with_hook::h81406b1240dbef69
           3: 0xe7c994 - <unknown>!std::panicking::begin_panic_handler::{{closure}}::hebf6926319be97e0
           4: 0xe7c8bf - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::hd746ef736be15897
           5: 0xe7cfef - <unknown>!rust_begin_unwind
           6: 0xe9cd42 - <unknown>!core::panicking::panic_fmt::h66c1785b975136b0
           7: 0xe9e75e - <unknown>!core::result::unwrap_failed::h176e4651483acbd5
           8: 0x8d61 - <unknown>!_start
           9: 0xea139b - <unknown>!_start.command_export
       note: using the `WASMTIME_BACKTRACE_DETAILS=1` environment variable may show more debugging information
    2: wasm trap: wasm `unreachable` instruction executed
make: *** [Makefile:6: run] Error 134
```
First notice that original error is:
```console
Original error was: No module named numpy.core._multiarray_umath
```
This error originates from numpy/core/__init__.py:
```python
try:                                                                            
    from . import multiarray                                                    
except ImportError as exc:                                                      
    import sys                                                                  
    msg = """                                                                   
                                                                                
IMPORTANT: PLEASE READ THIS FOR ADVICE ON HOW TO SOLVE THIS ISSUE!              
                                                                                
Importing the numpy C-extensions failed. This error can happen for              
many reasons, often due to issues with your setup or how NumPy was              
installed.                                                                      
                                                                                
We have compiled some common reasons and troubleshooting tips at:               
                                                                                
    https://numpy.org/devdocs/user/troubleshooting-importerror.html             
                                                                                
Please note and check the following:                                            
                                                                                
  * The Python version is: Python%d.%d from "%s"                                
  * The NumPy version is: "%s"                                                  
                                                                                
and make sure that they are the versions you expect.                            
Please carefully study the documentation linked above for further help.         
                                                                                
Original error was: %s                                                          
""" % (sys.version_info[0], sys.version_info[1], sys.executable,                
        __version__, exc)                                                       
    raise ImportError(msg)
```
So the `from . import multiarray` is importing a submodule from the current
package which is _core.

After looking into this some more I found the following issue which is related
to the problem I am facing:
https://github.com/dicej/wasi-wheels/issues/4
