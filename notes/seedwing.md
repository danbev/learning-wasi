## Seedwing Policy Engine WebAssembly Component Model


### Reason for Wasi
This is a question about the the reason for compiling Seedwing policy engine
using the wasm32-wasi target as opposed to wasm32-unknown-unknown.

If we try compiling using 
```console
$ cargo b --release -p seedwing-policy-engine --target=wasm32-unknown-unknown --no-default-features --features=""
```
We get the following error:
```console
error: the wasm32-unknown-unknown target is not supported by default, you may need to enable the "js" feature. For more information see: https://docs.rs/getrandom/#webassembly-support
   --> /home/danielbevenius/.cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.8/src/lib.rs:263:9
    |
263 | /         compile_error!("the wasm32-unknown-unknown target is not supported by \
264 | |                         default, you may need to enable the \"js\" feature. \
265 | |                         For more information see: \
266 | |                         https://docs.rs/getrandom/#webassembly-support");
    | |________________________________________________________________________^

error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> /home/danielbevenius/.cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.8/src/lib.rs:290:5
    |
290 |     imp::getrandom_inner(dest)
    |     ^^^ use of undeclared crate or module `imp`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `getrandom` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
make: *** [Makefile:8: wit-compile-wasm] Error 101
```

We can inspect the dependency tree using:
```console
$ cargo tree -p -p seedwing-policy-engine --target=wasm32-unknown-unknown > tree
```
And if we look we can see that getrandom is a dependency of rand, hyper, ecdsa,
oauth2, and uuid.

getrandom is an interface to the operating system's random number generator and
requires a implementation of this which could be a JavaScript impl or a Wasi
impl. See [getrandom webassembly support] for more details. In our case we want
to be able to run this in non-browser environments which is the reason for
choosing wasi.

[getrandom webassembly support]: https://docs.rs/getrandom/latest/getrandom/#webassembly-support

### Creating an executable component model module
This task is about taking a policy that a user has written and wrap it into
a webassembly component model that can be excuted by any wasm runtime that
supports webassembly component model.

So, what we currently have is a webassembly component for the policy engine and
we now want to take a policy file and have it executed, all contained in the
same .wasm component module.

Could we perhaps place the policy in a custom .wasm section, and have then
have function read from that section when it is executed?

Lets install `wasm-custom-section` to explore this a little:
```console
$ cargo install wasm-custom-section
```
And we can use this tool to list the custom sections:
```console
$ wasm-custom-section ../target/wasm32-wasi/release/seedwing_policy_engine.wasm list
Section `.debug_info` (479979 bytes)
Section `.debug_pubtypes` (306 bytes)
Section `.debug_ranges` (186080 bytes)
Section `.debug_abbrev` (4434 bytes)
Section `component-type:engine-world` (4975 bytes)
Section `.debug_line` (325971 bytes)
Section `.debug_str` (732736 bytes)
Section `.debug_pubnames` (292919 bytes)
Section `name` (360150 bytes)
Section `producers` (80 bytes)
Section `target_features` (41 bytes)
```

And we can also use it to add a custom section, in our case a policy file:
```console
$ wasm-custom-section ../target/wasm32-wasi/release/seedwing_policy_engine.wasm add "seedwing:policy" < policy.dog
```
The output will be in a file with a `.out` suffix:
```console
$ wasm-custom-section ../target/wasm32-wasi/release/seedwing_policy_engine.wasm.out list
Section `.debug_info` (479979 bytes)
Section `.debug_pubtypes` (306 bytes)
Section `.debug_ranges` (186080 bytes)
Section `.debug_abbrev` (4434 bytes)
Section `component-type:engine-world` (4975 bytes)
Section `.debug_line` (325971 bytes)
Section `.debug_str` (732736 bytes)
Section `.debug_pubnames` (292919 bytes)
Section `name` (360150 bytes)
Section `producers` (80 bytes)
Section `target_features` (41 bytes)
Section `seedwing:policy` (57 bytes)
```
And we can display/show the section using the following command:
```console
$ wasm-custom-section ../target/wasm32-wasi/release/seedwing_policy_engine.wasm.out show seedwing:policy
Section `seedwing:policy` (57 bytes):
Length: 57 (0x39) bytes
0000:   70 61 74 74  65 72 6e 20  64 6f 67 20  3d 20 7b 0a   pattern dog = {.
0010:   20 20 20 20  6e 61 6d 65  3a 20 73 74  72 69 6e 67       name: string
0020:   2c 0a 20 20  20 20 74 72  61 69 6e 65  64 3a 20 62   ,.    trained: b
0030:   6f 6f 6c 65  61 6e 0a 7d  0a                         oolean.}.
```

_work in progress_
