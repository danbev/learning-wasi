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
