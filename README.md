### Learning Web Assembly System Interface (WASI)
This project is a collection of notes and examples to help me understand WASI
and WASM. This is an API which provides access to operating system like
function calls to access files and the filesystem, Berkley sockets, clocks, and
random numbers. At the same time it has a capability-based security model for
this I/O (it extends WebAssembly's sandbox model).


### Install Wasmtime
```console
$ curl https://wasmtime.dev/install.sh -sSf | bash
$ source ~/.bashrc 
$ wasmtime --version
wasmtime-cli 0.40.0
```

### fd_write
The example [fd_write.wat](src/fd_write.wat) shows the usage of the
[fd_write](https://github.com/WebAssembly/WASI/blob/master/design/WASI-core.md#__wasi_fd_write) system call.
The input to fd_write is shown below:
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

### wasi api calls explained
This section is an attempt to answer a question I asked myself while working
on a uvwasi [issue](https://github.com/nodejs/uvwasi/pull/176). The fix in this
case was correcting a return value from uvwasi_fd_readdir. I read the docs and
figured out that something was calling this function, and depending on the value
returned would try to read more entries (or not if it was done). But what was
not clear to me done.

Lets take a look at compiling c program what uses readdir (man 3 readdir).
The program is very simple and prints out all the files (directory entries)
from the `src` directory:
```console
$ make out/readdir
$ ./out/readdir
readdir example
d_name: multivalue.wat
...
```
This is a dynmically linked executable:
```console
$ file out/readdir
out/readdir: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=e5ad67a7cb11843781c8b42bd281898dcef7a8b1, for GNU/Linux 3.2.0, stripped
```
And we can inspect the dynamic symbols using:
```console
$ nm -D out/readdir
                 U closedir@GLIBC_2.2.5
                 U fwrite@GLIBC_2.2.5
                 w __gmon_start__
                 U __libc_start_main@GLIBC_2.34
                 U opendir@GLIBC_2.2.5
                 U puts@GLIBC_2.2.5
                 U readdir@GLIBC_2.2.5
0000000000404060 B stderr@@GLIBC_2.2.5

$ ldd out/readdir
	linux-vdso.so.1 (0x00007ffc02fef000)
	libc.so.6 => /usr/lib64/libc.so.6 (0x00007fbfeb9ed000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fbfebc16000)
```
A wasm module is more like a statically linked executable:
```console
$ make out/readdir_s
cc -O0 -g -s -static -o out/readdir_s src/readdir.c

$ file out/readdir_s 
out/readdir_s: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, BuildID[sha1]=50e9b76eea75c197d1a417c3946b462108ead1b6, for GNU/Linux 3.2.0, stripped, too many notes (256)

$ ls -hgG out/readdir_s out/readdir
-rwxrwxr-x. 1  21K Sep  7 11:03 out/readdir
-rwxrwxr-x. 1 1.5M Sep  7 11:10 out/readdir_s
```
If we use objdump on readdir_s we won't see anything named `readdir`. 

Now, lets compile the same c program into wasm:
```console
$ make out/readdir.wasm
```
This target is using `wasi-sdk` to compile `readdir.c` to a wasm module.

So lets take a closer look at the generated module:
```console
$ wasm-objdump -x out/readdir.wasm

readdir.wasm:	file format wasm 0x1

Section Details:

Type[16]:
 ...
 - type[4] (i32, i32, i32, i64, i32) -> i32

Import[11]:
 - func[0] sig=2 <wasi_snapshot_preview1.args_get> <- wasi_snapshot_preview1.args_get
 - func[1] sig=2 <wasi_snapshot_preview1.args_sizes_get> <- wasi_snapshot_preview1.args_sizes_get
 - func[2] sig=3 <wasi_snapshot_preview1.fd_close> <- wasi_snapshot_preview1.fd_close
 - func[3] sig=2 <wasi_snapshot_preview1.fd_fdstat_get> <- wasi_snapshot_preview1.fd_fdstat_get
 - func[4] sig=2 <wasi_snapshot_preview1.fd_prestat_get> <- wasi_snapshot_preview1.fd_prestat_get
 - func[5] sig=0 <wasi_snapshot_preview1.fd_prestat_dir_name> <- wasi_snapshot_preview1.fd_prestat_dir_name
 - func[6] sig=4 <wasi_snapshot_preview1.fd_readdir> <- wasi_snapshot_preview1.fd_readdir
 - func[7] sig=5 <wasi_snapshot_preview1.fd_seek> <- wasi_snapshot_preview1.fd_seek
 - func[8] sig=6 <wasi_snapshot_preview1.fd_write> <- wasi_snapshot_preview1.fd_write
 - func[9] sig=7 <wasi_snapshot_preview1.path_open> <- wasi_snapshot_preview1.path_open
 - func[10] sig=8 <wasi_snapshot_preview1.proc_exit> <- wasi_snapshot_preview1.proc_exi
```
By looking at the above we can see that there are a number of imports that
need to provided by host environment, via the `importObject`. We can see that
function 6 which has signature 4 which is `wasi_snapshot_preview1.fd_readdir`
this is the uvwasi function.

In Node.js we can see that `fd_readdir` is mapped to the wasiImport object
and this function will call uvwasi_fd_readdir.
```console
WASI {
  wasiImport: WASI {
    args_get: [Function: bound args_get],
    ...
    fd_read: [Function: bound fd_read],
    fd_readdir: [Function: bound fd_readdir],
    ...
```
If we look at Imports in readdir.wasm we can find:

When we compile the c program into a wasm module we do so with a compile that
supports Wasi, in our case we are using clang from wasi-sdk which uses wasi-libc.
If we think about this it makes sense that we have different libc libraries for
different environments. The make target that compiles readdir.c into an
executable is using the GNU libc library, and the target that compile into a
wasm module will use wasi-libc.

So, the call to `readdir` which we know is declared in `dirent.h` but is not
included in our code but instead linked by the linker in the compilation
process.

So, lets start by finding out where `wasi_snapshot_preview1.fd_readdir` is
declared. It can be found in `libc-bottom-half/sources/__wasilibc_real.c`:
```c
int32_t __imported_wasi_snapshot_preview1_fd_readdir(int32_t arg0, int32_t arg1, int32_t arg2, int64_t arg3, int32_t arg4) __attribute__((
    __import_module__("wasi_snapshot_preview1"),
    __import_name__("fd_readdir")
));
```
This is specifying that this function will be defined as the symbol name
`fd_readdir` inside of the WebAssembly linking environment using
[__import-name__](https://clang.llvm.org/docs/AttributeReference.html#import-name).

Lets take another look at the wasm module:
```console
$ wasm-objdump -x out/readdir.wasm
Type[17]:
 - type[0] (i32, i32, i32) -> i32
 - type[1] (i32, i64, i32) -> i64
 - type[2] (i32, i32) -> i32
 - type[3] (i32) -> i32
 - type[4] (i32, i32, i32, i64, i32) -> i32
 ...
Import[11]:
 ...
 - func[6] sig=4 <wasi_snapshot_preview1.fd_readdir> <- wasi_snapshot_preview1.fd_readdir
```
So, the mistake I've been doing was that I was expecting to find a call to
`readdir` somewhere in the disassembled wasm but there is no guarantee that
the name of the function will be preserved so we have to figure out how this
works. This can be done using:
```console
$ wasm-objdump -d out/readdir.wasm > readdir.dis
```
If we search for where function 6 is called we find that happens in function 32:
```
003481 func[32]:
 003482: 20 00                      | local.get 0
 003484: 20 01                      | local.get 1
 003486: 20 02                      | local.get 2
 003488: 20 03                      | local.get 3
 00348a: 20 04                      | local.get 4
 00348c: 10 86 80 80 80 00          | call 6 <wasi_snapshot_preview1.fd_readdir>
 003492: 41 ff ff 03                | i32.const 65535
 003496: 71                         | i32.and
 003497: 0b                         | end
```
Function 32 uses the signature index 4 so it takes 5 parameters and returns
an i32. This sounds familiar, it looks like func 32 is
[fd_readdir](https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-fd_readdirfd-fd-buf-pointeru8-buf_len-size-cookie-dircookie---resultsize-errno)
```
fd_readdir(fd: fd, buf: Pointer<u8>, buf_len: size, cookie: dircookie) -> Result<size, errno>
```
In our case the Result<size> is actually one of the parameters passed in.

If we search for where function 32 is called (search for 'call 32') we find that
it gets called in functions, 25 and 46.
Lets start with function 25:
```
  (func (;25;) (type 3) (param i32) (result i32) ;; one parameter (DIR* dirp)
    (local i32 i32 i32 i32 i32 i32 i32 i64 i64)  ;; 9 local variables in this function
    local.get 0    ;; local 0 is the first parameter so this is DIR* dirp.
    i32.const 28   ;; this is placing 28 onto the stack
    i32.add        ;; this is adding 28 to the DIR pointer, which buffer_used (see struct _DIR) further down)
    local.set 1    ;; store this value in local variable 1
    local.get 0    ;; load local 0 again (it was popped off by i32.add)
    i32.load offset=20 ;; loads a 32-bit integer from memory, starting from the topmost value on the stack, which is DIR* dirp, and offset 20
                       ;; and if we again look at struct _DIR this looks like it will be the buffer_processed member of DIR.
    local.set 2        ;; The above value (buffer_processed is stored in local variable 2
                       ;; local 0 = DIR* dirp
                       ;; local 1 = buffer_used
                       ;; local 2 = buffer_processed
    loop (result i32)  ;; label = @, is is the for (;;) loop.
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1      ;; push buffer_used onto the stack
                  i32.load         ;; load a 32-bit integer starting from the topmost value on the stack
                  local.tee 3      ;; store this value in local variable 3 (buffer_used value), and also leave a copy on the stack
                  local.get 2      ;; push buffer_processed onto the stack
                  i32.sub          ;; buffer_used - buffer_processed
                  local.tee 4      ;; store the result in local variable 4 (buffer_left)
                  i32.const 23     ;; 23 is sizeof(__wasi__dirent_t) though I though this should be 21 bytes
                  i32.gt_u         ;; greater than unsigned checks if buffer_left < 23, if true 1 will be pushed onto the stack, 0 otherwise
                  br_if 0 (;@7;)   ;; if not greater than branch to label @7
                  local.get 3      ;; push local variable 3 (buffer_used value) onto the stack
                  local.get 0      ;; push local variable 0 (DIR* dirp)
                  i32.load offset=24 ;; load from memory starting at the topmost value on the stack, using offset 24 (buffer_size)
                  local.tee 4        ;; store the result in local variable 4 and also leave a copy on the stack
                  i32.ge_u           ;; if buffer_used < buffer_size
                  br_if 1 (;@6;)     ;; branch to end of block 1 if not true
                  i32.const 0        ;; load const 0 onto the stack (NULL)
                  return             ;; return topmost value (NULL);
                end
```
Function 25's signature can be found in index 3 which takes a single i32 and
returns an i32. I'm thinking that these i32 are pointers, which would match
up with the signature of `readdir`:
```c
struct dirent* readdir(DIR* dirp);
```
If we look at wasi-sdk it contains wasi-libc as a git submodule in
src/wasi-libc. There is a top-half of this libc implementation which is based
on [musl](https://musl.libc.org/)] and in
wasi-sdk/src/wasi-libc/libc-bottom-half/cloudlibc/src/libc/dirent/readdir.c we
can find the implementation of `readdir`:
```c
struct dirent *readdir(DIR *dirp) {
  for (;;) {
   if (buffer_left < sizeof(__wasi_dirent_t)) {
      // End-of-file.
      if (dirp->buffer_used < dirp->buffer_size)
        return NULL;
      goto read_entries;
   }
   ...

   read_entries:
    // Discard data currently stored in the input buffer.
    dirp->buffer_used = dirp->buffer_processed = dirp->buffer_size;

    // Load more directory entries and continue.
    __wasi_errno_t error = __wasi_fd_readdir(dirp->fd,
                                             (uint8_t *)dirp->buffer,
                                             dirp->buffer_size,
                                             dirp->cookie,
                                             &dirp->buffer_used);
    if (error != 0) {
      errno = error;
      return NULL;
    }
    dirp->buffer_processed = 0;
  }
}

#define DIRENT_DEFAULT_BUFFER_SIZE 4096

struct _DIR {
  // Directory file descriptor and cookie.
  int fd;                                   // 0 - 4
  __wasi_dircookie_t cookie;                // 4 ->12

  // Read buffer.
  char *buffer;                             // 12 ->20
  size_t buffer_processed;                  // 20 ->24
  size_t buffer_size;                       // 24 ->28
  size_t buffer_used;                       // 28

  // Object returned by readdir().
  struct dirent *dirent;
  size_t dirent_size;
};

typedef struct __wasi_dirent_t {
    __wasi_dircookie_t d_next;
    __wasi_inode_t d_ino;
    __wasi_dirnamlen_t d_namlen;
    __wasi_filetype_t d_type;
} __wasi_dirent_t;

typedef uint64_t __wasi_dircookie_t;    // size: 8 bytes
typedef uint64_t __wasi_inode_t;        // size: 8 bytes
typedef uint32_t __wasi_dirnamlen_t;    // size: 4 bytes
typedef uint8_t __wasi_filetype_t;      // size: 1 bytes
```

_work in progress_

libc-top-half/musl/arch/i386/bits/syscall.h.in:
```console
#define __NR_readdir             89
```
This file is used by libc-top-half/musl/Makefile.
```console
obj/include/bits/syscall.h: $(srcdir)/arch/$(ARCH)/bits/syscall.h.in
        cp $< $@
        sed -n -e s/__NR_/SYS_/p < $< >> $@
```

```c
#define DIRENT_DEFAULT_BUFFER_SIZE 4096

struct _DIR {
  // Directory file descriptor and cookie.
  int fd;                                   // 0 - 4
  __wasi_dircookie_t cookie;                // 4 ->12

  // Read buffer.
  char *buffer;                             // 12 ->20
  size_t buffer_processed;                  // 20 ->24
  size_t buffer_size;                       // 24 ->28
  size_t buffer_used;                       // 28

  // Object returned by readdir().
  struct dirent *dirent;
  size_t dirent_size;
};
```

```console
$ wasmtime --dir=. out/readdir.wasm
```


### args_sizes_get
The example [args_sizes_get.wat](src/args_sizes_get.wat) contains an example of calling 
[__wasi_args_sizes_get](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-api.md#__wasi_args_sizes_get).

This shows an important point that I totally missed when first looking at calling
it. Looking at the documentation we can see that this function outputs:
```
size_t argc            The number of arguments
size_t argv_buf_size   The size of the argument string data.
```
What I did not understand was that these are pointers that are passed into the
function. So we have to specify the memory locations that it should use to 
populate these values. 

The test can be run manually:
```console
$ wasmtime src/args_sizes_get.wat one two three four five six
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
$ wasmtime src/args_get.wat one two
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
$ wasmtime out/first.wasm
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
$ wasmtime --env="ONE=1" --env="TWO=2" src/environ_sizes_get.wat
$ echo $?
2
```

### environ_get
The example [environ_get.wat](src/environ_get.wat) contains an of calling environ_get. 
```
$ wasmtime --env="ONE=1" --env="TWO=2" src/environ_get.wat
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

Find the targets that 
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


### Target triples


#### Install Rust using rustup:
```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup install nightly-x86_64-apple-darwin
```
Add the `wasm32-wasi` target:
```console
$ rustup target add wasm32-wasi --toolchain nightly
```

Troubleshooting:
```console
--- stderr
error[E0463]: can't find crate for `std`
  |
  = note: the `wasm32-unknown-unknown` target may not be installed

error: aborting due to previous error
```
```console
$ rust target add wasm32-unknown-unknown --toolchain nightly
```

### wasmtime
Use the following command to build wasmtime:
```console
$ RUSTFLAGS=-g CC="clang" CXX="clang++" cargo +nightly build --release
```
You might need to update rust using:
```console
$ rustup update nightly
```
After this update add $WASMTIME_LOCAL_REPO/target/release/ to your PATH.

### Building the examples
After building clang with wasm/wasi support this project can be build using
make or cmake. The path to the directory where llvm and the wasi-sysroot can
be updated in the Makefile/CMakeList.txt. 

To compile src/first.c and then run using wasmtime using make:
```console
$ make
$ make run
```

To compile src/first.c using cmake:
```console
$ mkdir build
$ cmake ..
$ make
```

### Building wasi-sdk
```console
$ git clone https://github.com/CraneStation/wasi-sdk.git
$ git submodule init
$ git submodule update
```
I've also [downloaded](https://github.com/WebAssembly/wasi-sdk/releases) and
unpacked these locally as this is quicker than having to build llvm-project and
also it takes a look of disk space which is an issue for me.
This is what I specify in the Makefile as the `LLVM_HOME`.

### Building wasi-libc
```console
$ git clone https://github.com/CraneStation/wasi-libc.git
$ git submodule init
$ git submodule update
$ make WASM_CC=/usr/local/opt/llvm/bin/clang WASM_AR=/usr/local/opt/llvm/bin/llvm-ar WASM_NM=/usr/local/opt/llvm/bin/llvm-nm
$ make WASM_CC=/home/danielbevenius/opt/bin/clang WASM_AR=/home/danielbevenius/opt/bin/llvm-ar WASM_NM=/home/danielbevenius/opt/bin/llvm-nm
...
#
# The build succeeded! The generated sysroot is in /Users/danielbevenius/work/wasi/wasi-libc/sysroot.
#
```
Specifying a `--sysroot=/somedir` when building will make the compiler look for headers and
libraries in /somedir/include and /somedir/lib. So we will want to specify this
sysroot that was created above when compiling:
```console
$ clang --sysroot=/opt/wasi-sdk/ --target=wasm32-unknown-wasi -o module.wasm 
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
Is a libc implementation just like glibc. Pronounced muscle.
Musl uses less space compared to glibc and is written with security in mind.
So applications could be compiled against either glibc or musl. So how to
we compile a program against musl? 


### Building llvm with WebAssembly target
Clone [llvm-project](llvm-project).

```console
$ git clone https://github.com/llvm/llvm-project.git
$ cd llvm-project
$ mkdir build && cd build
```

Default build
```console
$ cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX=/home/dbeveniu/opt -DCMAKE_BUILD_TYPE=Debug -DLLVM_ENABLE_PROJECTS="clang;lld;clang-tools-extra;libcxx;libcxxabi;lldb" -DLLVM_TARGETS_TO_BUILD=WebAssembly -DLLVM_DEFAULT_TARGET_TRIPLE=wasm32-wasi -DDEFAULT_SYSROOT=/home/dbeveniu/opt/share/wasi-sysroot ../llvm
$ make -j8  
$ make install
```

The installation will be placed in /home/dbeveniu/opt. With this installed
we should be able to compile and specify the output target as wasm.

Generate llvm intermediate representation (IR):
```console
$ ~/opt/bin/clang --target=wasm32 -emit-llvm -c -S src/add.c
```

Compile the IR:
```console
$ ~/opt/bin/llc -march=wasm32 -filetype=obj add.ll
```
The above will produce an object file named `add.o`. This need to be linked
into a wasm module using `wasm-ld`:
```console
$ wasm-ld --no-entry --export-all -o add.wasm add.o
```
We can run this using node:
```console
$ node src/add.js
10 + 20 = 30
```

So the above allowed us to compile from c and output assembly. But that does
not allow us to use wasi. 

For this we need a wasi-libc.

### wasi-libc
This is a c library like glibc but with a limited number of functions available. 


```console
$ git clone https://github.com/CraneStation/wasi-libc.git
$ git submodule init
$ git submodule update
$ make WASM_CC=~/opt/bin/clang WASM_AR=~/opt/bin/llvm-ar WASM_NM=~/opt/bin/llvm-nm SYSROOT=~/opt/share/wasi-sysroot 
...
#
# The build succeeded! The generated sysroot is in /home/dbeveniu/opt/share/wasi-sysroot.
#
```
Specifying a `--sysroot=/somedir` when building will make the compiler look for headers and
libraries in /somedir/include and /somedir/lib. So we will want to specify this
sysroot that was created above when compiling:

```console
$ clang --sysroot=/opt/wasi-sdk/ --target=wasm32-unknown-wasi -o module.wasm 
```

The wasi header, the api.h file, is generated by tools/wasi-headers:
```console
cargo run -- WASI/phases/snapshot/witx/typenames.witx \
      WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx \
      --output ../../libc-bottom-half/headers/public/wasi/api.h
```

But we also need a compiler-rt:

### compiler-rt
builtins:
```
a simple library that provides an implementation of the low-level target-specific hooks required by code generation and other runtime components. For example, when compiling for a 32-bit target, converting a double to a 64-bit unsigned integer is compiling into a runtime call to the "__fixunsdfdi" function. The builtins library provides optimized implementations of this and other low-level routines, either in target-independent C form, or as a heavily-optimized assembly.
```
So lets say you have a division of a long long and on a 32 bit system there
might not be an instruction for this. In cases like this instead that instruction
will be replaced with an call to a function from the compiler-rt library.
TODO: Add an example of this.

```console
$ mkdir compiler-rt && cd compiler-rt
$ cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=RelWithDebInfo -DCMAKE_TOOLCHAIN_FILE=../../wasi-sdk.cmake -DCOMPILER_RT_BAREMETAL_BUILD=On -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_INCLUDE_TESTS=OFF -DCOMPILER_RT_HAS_FPIC_FLAG=OFF -DCOMPILER_RT_ENABLE_IOS=OFF -DCOMPILER_RT_DEFAULT_TARGET_ONLY=On -DWASI_SDK_PREFIX=/home/danielbevenius/opt -DCMAKE_C_FLAGS="-O1" -DLLVM_CONFIG_PATH=../bin/llvm-config -DCOMPILER_RT_OS_DIR=wasi -DCMAKE_INSTALL_PREFIX=/home/danielbevenius/opt/lib/clang/11.0.0/ -DCMAKE_VERBOSE_MAKEFILE:BOOL=ON ../../compiler-rt/lib/builtins

$ make -j8
$ make install
/usr/bin/cmake -P cmake_install.cmake
-- Install configuration: "RelWithDebInfo"
-- Installing: /home/dbeveniu/opt/lib/clang/11.0.0/lib/wasi/libclang_rt.builtins-wasm32.a
```

With this in place we should be able to compile a c source code into wasi:
```console
$ clang -### --target=wasm32-unknown-wasi --sysroot=/home/dbeveniu/opt/share/wasi-sysroot -nostdlib -Wl,--no-entry -Wl,--export-all -o add.wasm src/add.c
clang version 11.0.0 (https://github.com/llvm/llvm-project.git 89a66474b6c1e5843c3dbc96bde52e5a7076c6cc)
Target: wasm32-unknown-wasi
Thread model: posix
InstalledDir: /home/dbeveniu/opt/bin
 (in-process)
 "/home/dbeveniu/opt/bin/clang-11" "-cc1" "-triple" "wasm32-unknown-wasi" "-emit-obj" "-mrelax-all" "-disable-free" "-main-file-name" "add.c" "-mrelocation-model" "static" "-mthread-model" "posix" "-mframe-pointer=none" "-fno-rounding-math" "-masm-verbose" "-mconstructor-aliases" "-target-cpu" "generic" "-fvisibility" "hidden" "-dwarf-column-info" "-debugger-tuning=gdb" "-resource-dir" "/home/dbeveniu/opt/lib/clang/11.0.0" "-isysroot" "/home/dbeveniu/opt/share/wasi-sysroot" "-internal-isystem" "/home/dbeveniu/opt/lib/clang/11.0.0/include" "-internal-isystem" "/home/dbeveniu/opt/share/wasi-sysroot/include/wasm32-wasi" "-internal-isystem" "/home/dbeveniu/opt/share/wasi-sysroot/include" "-fdebug-compilation-dir" "/home/dbeveniu/work/wasm/learning-wasi" "-ferror-limit" "19" "-fmessage-length" "0" "-fgnuc-version=4.2.1" "-fobjc-runtime=gnustep" "-fno-common" "-fdiagnostics-show-option" "-fcolor-diagnostics" "-o" "/tmp/add-593917.o" "-x" "c" "src/add.c"
 "/home/dbeveniu/opt/bin/wasm-ld" "-L/home/dbeveniu/opt/share/wasi-sysroot/lib/wasm32-wasi" "--no-entry" "--export-all" "/tmp/add-593917.o" "-o" "add.wasm"
```
And we can try it out with the same node.js code as above:
```console
$ node src/add.js 
10 + 20 = 30
```

We can inspect `add.wasm` using wasm-objdump:
```console
$ wasm-objdump -x add.wasm 

add.wasm:	file format wasm 0x1

Section Details:

Type[2]:
 - type[0] () -> nil
 - type[1] (i32, i32) -> i32
Function[2]:
 - func[0] sig=0 <__wasm_call_ctors>
 - func[1] sig=1 <add>
Table[1]:
 - table[0] type=funcref initial=1 max=1
Memory[1]:
 - memory[0] pages: initial=2
Global[7]:
 - global[0] i32 mutable=1 - init i32=66560
 - global[1] i32 mutable=0 <__dso_handle> - init i32=1024
 - global[2] i32 mutable=0 <__data_end> - init i32=1024
 - global[3] i32 mutable=0 <__global_base> - init i32=1024
 - global[4] i32 mutable=0 <__heap_base> - init i32=66560
 - global[5] i32 mutable=0 <__memory_base> - init i32=0
 - global[6] i32 mutable=0 <__table_base> - init i32=1
Export[9]:
 - memory[0] -> "memory"
 - func[0] <__wasm_call_ctors> -> "__wasm_call_ctors"
 - func[1] <add> -> "add"
 - global[1] -> "__dso_handle"
 - global[2] -> "__data_end"
 - global[3] -> "__global_base"
 - global[4] -> "__heap_base"
 - global[5] -> "__memory_base"
 - global[6] -> "__table_base"
Code[2]:
 - func[0] size=2 <__wasm_call_ctors>
 - func[1] size=61 <add>
Custom:
 - name: "name"
 - func[0] <__wasm_call_ctors>
 - func[1] <add>
Custom:
 - name: "producers"
```
Notice that there are a number of functions and symboles that we did not 
add, infact we only have one function named `add`.
When is this added?  
If we just generate the IR from add.c we get:

Generate only the object file add.o:
```console
/home/danielbevenius/opt/bin/clang-11 "-cc1" "-triple" "wasm32-unknown-wasi" "-emit-obj" "-mrelax-all" "-disable-free" "-main-file-name" "add.c" "-mrelocation-model" "static" "-mthread-model" "posix" "-mframe-pointer=none" "-fno-rounding-math" "-masm-verbose" "-mconstructor-aliases" "-target-cpu" "generic" "-fvisibility" "hidden" "-dwarf-column-info" "-debugger-tuning=gdb" "-resource-dir" "/home/danielbevenius/opt/lib/clang/11.0.0" "-isysroot" "/home/danielbevenius/opt/share/wasi-sysroot" "-internal-isystem" "/home/danielbevenius/opt/lib/clang/11.0.0/include" "-internal-isystem" "/home/danielbevenius/opt/share/wasi-sysroot/include/wasm32-wasi" "-internal-isystem" "/home/danielbevenius/opt/share/wasi-sysroot/include" "-fdebug-compilation-dir" "/home/danielbevenius/work/wasm/learning-wasi" "-ferror-limit" "19" "-fmessage-length" "0" "-fgnuc-version=4.2.1" "-fobjc-runtime=gnustep" "-fno-common" "-fdiagnostics-show-option" "-fcolor-diagnostics" "-o" "add.o" "-x" "c" "src/add.c"
```
Then we can launch the the wasm linker and debug it:
```console
gdb --args /home/danielbevenius/opt/bin/wasm-ld "-L/home/dbeveniu/opt/share/wasi-sysroot/lib/wasm32-wasi" "--no-entry" "--export-all" "add.o" "-o" "add.wasm"
(gdb) b main
r
```
This will break in `lld/tools/lld/lld.cpp` which is a executable what contains
4 linkers. A ELF, COFF, WebAssembly, and a Mach-O linker. The name of the executable
run, in our case this is `wasm-ld`:
```console
(gdb) p argv[0]
$3 = 0x7fffffffd877 "/home/danielbevenius/opt/bin/wasm-ld"
```
Notice that `wasm-ls` is a lin to `lld`:
```console
lrwxrwxrwx. 1 danielbevenius danielbevenius 3 Feb 11 15:39 /home/danielbevenius/opt/bin/wasm-ld -> lld
```
This will be matched against the Flavor `Wasm` using the `getFlavor` function.
And this is what the switch case will match:
```c++
case Wasm:                                                                    
    return !wasm::link(args, canExitEarly(), llvm::outs(), llvm::errs()); 
```
`wasm::link` can be found in `lld/wasm/Driver.cpp`
```console
(gdb) b Driver.cpp:83
```
Which will call LinkerDriver().link(args) which can be found in the same
file `LinkerDriver::link`:
```c++
  ...
  createSyntheticSymbols();  
```
This is what will create the additional functions. For example, it will
create a synthetic function named `__wasm_call_ctor`.
There are others but I'm going to focus on this one.
So where will be a function added named `__wasm_call_ctor` and this will be
called by `_start`:
```c
extern void __wasm_call_ctors(void);                                            
                                                                                
void _start(void) {                                                             
    // The linker synthesizes this to call constructors.                        
    __wasm_call_ctors();    
    ...
}
```
Now, `_start` is define in wasi-libc but when we ran this 

In LinkDriver::link we find the following function call:
```c++
readConfigs(args);  
...
config->entry = getEntry(args);
```
In our case, we passed in --no-entry so this will be "":
```c++
 if (arg->getOption().getID() == OPT_no_entry)                                 
    return "";      
```

Note about llvm targets:
When you build llvm you specify the targets it should build. This will
generated a file named `build/include/llvm/Config/Targets.def`.
```console
LLVM_TARGET(WebAssembly)                                                        
```
The wasm Driver will call `initLLVM` which 
```c++
static void initLLVM() {
  InitializeAllTargets();
  ...
}

inline void InitializeAllTargets() {
  // FIXME: Remove this, clients should do it.
  InitializeAllTargetInfos();

inline void InitializeAllTargetInfos() {                                         
  #define LLVM_TARGET(TargetName) LLVMInitialize##TargetName##TargetInfo();          
  #include "llvm/Config/Targets.def"                                                 
}                                                                                
````
So this will be expanded by the preprocessor to:
```c++
LLVMInitializeWebAssemblyTargetInfo();          
```




When InitializeAllTargetInfos is called this file will be included




```console
$ clang -S -emit-llvm --target=wasm32-unknown-wasi --sysroot=/home/dbeveniu/opt/share/wasi-sysroot -nostdlib src/add.c
```
This will produce a file named `add.ll`:
```
$ cat add.ll 
; ModuleID = 'src/add.c'
source_filename = "src/add.c"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-wasi"

; Function Attrs: noinline nounwind optnone
define hidden i32 @add(i32 %x, i32 %y) #0 {
entry:
  %x.addr = alloca i32, align 4
  %y.addr = alloca i32, align 4
  store i32 %x, i32* %x.addr, align 4
  store i32 %y, i32* %y.addr, align 4
  %0 = load i32, i32* %x.addr, align 4
  %1 = load i32, i32* %y.addr, align 4
  %add = add nsw i32 %0, %1
  ret i32 %add
}

attributes #0 = { noinline nounwind optnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{!"clang version 11.0.0 (https://github.com/llvm/llvm-project.git 89a66474b6c1e5843c3dbc96bde52e5a7076c6cc)"}
```
This would then be passed to the linker `wasm-ld` which must be adding these 
additional symbols. It looks like these are being added by a function
named createSyntheticSymbols in `lld/wasm/Driver.cpp`:
```c++
  static WasmSignature nullSignature = {{}, {}};                                
  ...
  WasmSym::callCtors = symtab->addSyntheticFunction(                            
      "__wasm_call_ctors", WASM_SYMBOL_VISIBILITY_HIDDEN,                       
      make<SyntheticFunction>(nullSignature, "__wasm_call_ctors")); 
```
Alright, we can see that this is just an empty function in our module:
```console
$ wasm-objdump -d add.wasm 

add.wasm:	file format wasm 0x1

Code Disassembly:

0000d5 func[0] <__wasm_call_ctors>:
 0000d6: 0b                         | end
```
So where is this function called? And can we add something to it our selves?  
First, `__wasm_call_ctors` is called from `crt1.c`:
```c
extern void __wasm_call_ctors(void); 
...
void _start(void) {                                                             
  __wasm_call_ctors();   


``


So above we specify that our target should be `wasm32-unknown-wasi`. Is this
specifying the backend to be used?

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
signalhandlers/SignalHandlers.hpp:5:10: fatal error: 'setjmp.h' file not found
signalhandlers/SignalHandlers.hpp:5:10: fatal error: 'setjmp.h' file not found, err: true
thread 'main' panicked at 'Unable to generate bindings: ()', /home/danielbevenius/.cargo/git/checkouts/wasmtime-5c699c1a3ee5d368/b7d86af/wasmtime-runtime/build.rs:32:5
```
You can trouble shoot this by creating a directory and generating the cmake
build files:
```console
$ mkdir tmp && cd tmp
$ cmake "/home/danielbevenius/.cargo/git/checkouts/wasmtime-5c699c1a3ee5d368/b7d86af/wasmtime-runtime/signalhandlers" "-DCMAKE_INSTALL_PREFIX=/home/danielbevenius/work/openshift/faas-wasi-runtime-image/target/debug/build/wasmtime-runtime-a5693ecd59b8474f/out" "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64 -I/usr/include" "-DCMAKE_C_COMPILER=/usr/bin/cc" "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64 -I/usr/include" "-DCMAKE_CXX_COMPILER=/usr/bin/c++" "-DCMAKE_BUILD_TYPE=Debug"
```
Then we can run the command the failed:
```console
$ "cmake" "--build" "." "--target" "install" "--config" "Debug" "--"
```
This actually succeeded. 

Looking closer at the error message you can see that it is saying the it failed
when running rustc on wasm-runtime/build.rs. I've build rustc manually and it
is installed in a non-default location.
So this can be worked around by specifying:
```console
$ export BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include"
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
definitions.

#### Table section
This maps values like JavaScript objects or OS file handles. Its a way to
allow the wasm module to access something outside its memory. For example, say
you have a function pointer which you want to call from wasm module. If you 
had direct access to this memory pointer you might be able to learn about the 
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
These are functions, tables, memory segements and global variables that are made
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

### Wasmtime walkthrough
```console
$ lldb -- wasmtime --env="ONE=1" --env="TWO=2" src/fd_write.wat
(lldb) target create "wasmtime"
Current executable set to 'wasmtime' (x86_64).
(lldb) settings set -- target.run-args  "--env=ONE=1" "--env=TWO=2" "src/fd_write.wat"
```
Set a breakpoint in `$wasmtime_home/src/bin/wasmtime.rs`:
```console
(lldb) br s -f wasmtime.rs -l 203
```

```console
$ rustfmt --check somefile
$ echo $?
```
If there where any issues the exit value will be 1.


### Trace logging
Can be enabled by adding `RUST_LOG`:
```console
RUST_LOG=wasi_common=trace cargo test socket --features wasm_tests
```

### Cranelift
Compiler code generator backend.

cranelift-codegen takes as input the intermediate language of a functions, the
target (the arch like x86_64 for example), and compiler settings.
The output is machine code.


### Extended static checking (ESC)

### Enarx demo walk through:
```console
$ lldb -- target/debug/wasmtime-basic
(lldb) br s -f main.rs -l 31
(lldb) r
```
```rust
pub fn wasm_add_full() -> Result<ActionOutcome, ActionError> {
    let mut binary_file = File::open(concat!(env!("OUT_DIR"), "/add.wasm")).unwrap();
    let mut binary: Vec<u8> = Vec::new();
    binary_file.read_to_end(&mut binary).unwrap();
```
So this first section is reading `add.wasm` indo a Vector.
Next we have:
```rust
    // First, we need a Wasmtime context. To build one, we need to get an
    // Instruction Set Architectures (ISA) from `cranelift_native.
    let isa_builder = cranelift_native::builder().unwrap();
```
The is an Instruction Set Architecture builder that we are creating. If we
look in the `cranelift::native` lib.rs we find the `builder` function
This function will call the isa::lookup which can be found in
`~/work/wasm/cranelift/cranelift-codegen/src/isa/mod.rs`.

```rust
pub fn builder() -> Result<isa::Builder, &'static str> {
    let mut isa_builder = isa::lookup(Triple::host()).map_err(|err| match err {
        isa::LookupError::SupportDisabled => "support for architecture disabled at compile time",
        isa::LookupError::Unsupported => "unsupported architecture",
    })?;

    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        parse_x86_cpuid(&mut isa_builder)?;
    }

    Ok(isa_builder)
}
```
The call to Triple::host() will return the Triple which contains the host
Architecture, vendor, OS. TODO: show the host specific code for `host`.
The `host` function is found in `target-lexicon` crate
(~/.cargo/registry/src/github.com-1ecc6299db9ec823/target-lexicon-0.9.0/src/host.rs)
```rust
include!(concat!(env!("OUT_DIR"), "/host.rs"));
```
Notice that is is reading a file from the current projects output directory:
```console
$ find target/ -name 'host.rs'
target//debug/build/target-lexicon-6301a8d7cd05e389/out/host.rs
```rust
pub const HOST: Triple = Triple {
    architecture: Architecture::X86_64,
    vendor: Vendor::Apple,
    operating_system: OperatingSystem::Darwin,
    environment: Environment::Unknown,
    binary_format: BinaryFormat::Macho,
};
```

With this information `isa::lookup` will be called (~/work/wasm/cranelift/cranelift-codegen/src/isa/mod.rs):
```rust
pub fn lookup(triple: Triple) -> Result<Builder, LookupError> {
    match triple.architecture {
        Architecture::Riscv32 | Architecture::Riscv64 => isa_builder!(riscv, "riscv", triple),
        Architecture::I386 | Architecture::I586 | Architecture::I686 | Architecture::X86_64 => {
            isa_builder!(x86, "x86", triple)
        }
        Architecture::Arm { .. } => isa_builder!(arm32, "arm32", triple),
        Architecture::Aarch64 { .. } => isa_builder!(arm64, "arm64", triple),
        _ => Err(LookupError::Unsupported),
    }
}
```
Now, `isa_builder` is actually a macro:
```rust
macro_rules! isa_builder {
    ($name: ident, $feature: tt, $triple: ident) => {{
        #[cfg(feature = $feature)]
        {
            Ok($name::isa_builder($triple))
```
Notice that we are going go use the name `x86` in our case, which is a module
in the isa crate (~/work/wasm/cranelift/cranelift-codegen/src/isa/x86/mod.rs):
```rust
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    IsaBuilder {
        triple,
        setup: settings::builder(),
        constructor: isa_constructor,
    }
}
```
Lets take a closer look at IsaBuilder (~/work/wasm/cranelift/cranelift-codegen/src/isa/mod.rs).
```rust
pub struct Builder {
    triple: Triple,
    setup: settings::Builder,
    constructor: fn(Triple, settings::Flags, settings::Builder) -> Box<dyn TargetIsa>,
}
```

So back to the demo code we have `settings::builder()` but there is nothing
being set.
```rust
    let flag_builder = settings::builder();
    let isa = isa_builder.finish(settings::Flags::new(flag_builder));
```
Lets take a closer look at isa_builder.finish which will construct the target
isa:
```
impl Builder {
    /// Combine the ISA-specific settings with the provided ISA-independent settings and allocate a
    /// fully configured `TargetIsa` trait object.
    pub fn finish(self, shared_flags: settings::Flags) -> Box<dyn TargetIsa> {
        (self.constructor)(self.triple, shared_flags, self.setup)
    }
}
```
So at this point we have: 
```rust

    // Then, we use the ISA to build the context.
    let mut context = Context::with_isa(isa);
```
(~/work/wasm/wasmtime/crates/jit/src/lib.rs) we can find:
```rust
mod context;
```
So we can look in `~/work/wasm/wasmtime/crates/jit/src/context.rs` for the
`with_isa` function:
```rust
pub struct Context {
    namespace: Namespace,
    compiler: Box<Compiler>,
    global_exports: Rc<RefCell<HashMap<String, Option<wasmtime_runtime::Export>>>>,
    debug_info: bool,
    features: Features,
}
```

```rust
pub fn with_isa(isa: Box<TargetIsa>) -> Self {
   Self::new(Box::new(Compiler::new(isa)))
}
```
And the compiler crate:
```rust
pub struct Compiler {
    isa: Box<dyn TargetIsa>,
    code_memory: CodeMemory,
    trap_registration_guards: Vec<TrapRegistrationGuard>,
    trampoline_park: HashMap<*const VMFunctionBody, *const VMFunctionBody>,
    signatures: SignatureRegistry,
    strategy: CompilationStrategy,
    /// The `FunctionBuilderContext`, shared between trampline function compilations.
    fn_builder_ctx: FunctionBuilderContext,
}

    // Now, we instantiate the WASM module loaded into memory.
    let mut instance = context.instantiate_module(None, &binary).unwrap();

    // And, finally, invoke our function and print the results.
    // For this demo, all we're doing is adding 5 and 7 together.
    let args = [RuntimeValue::I32(5), RuntimeValue::I32(7)];
    context.invoke(&mut instance, "add", &args)
}
```

### Instruction Set Architectures (ISA)


### Threads
WebWorkers rely on message passing for communication, and in V8 each worker is
a separate Isolate and they don't share compiled code or JavaScript objects with
each other.
WASM threads can share the same wasm memory using an underlying SharedArrayBuffer.
Each wasm thread runs in a WebWorker but they share their memory which allows
them to work like a normal pthreads application would. 

This proposal provides the building blocks for writing a threading library and
does not provide the actual library itself.

Atomic operations have been added 

TODO: Add an example...

### Anyref
```console
$ wat2wasm --enable-reference-types src/anyref.wat -o anyref.wasm
```
```console
$ node --experimental-wasm-anyref src/anyref.js
```
This js example can be found in [anyref.js](./src/anyref.js) and the wat in
[anyref.wat](./src/anyref.wat)


### Multi-Value return
Currently a wasm function can only return zero or one value.

The following example demonstrates how a function that return multiple values
can be called:
```console
$ wat2wasm --enable-multi-value src/multivalue.wat -o multivalue.wasm
```
```console
$ node --experimental-wasm-mv src/multivalue.js
```
This js example can be found in [multivalue.js](./src/multivalue.js) and the wat
in [multvalue.wat](./src/multivalue.wat)

### Exception Handling
Some languages use exceptions, like C++ or C# and these can be polifilled but
this impacts performance. Also JavaScript has exceptions and so if a wasm function
calls a JS function which throws an error the wasm module will not be able to
handle the error.

```console
$ wat2wasm --enable-exceptions src/exception.wat -o exception.wasm
```
```console
$ node --experimental-wasm-anyref --experimental-wasm-eh src/exception.js
```
This js example can be found in [exception.js](./src/exception.js) and the wat
in [exception.wat](./src/exception.wat).
