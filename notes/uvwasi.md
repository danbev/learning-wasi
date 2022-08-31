## uvwasi
Is a Wasi implementation that uses libuv. This is the Wasi implementation that
Node.js uses.

### Debugging tests
First build with debug symbols and also enable `UVWASI_DEBUG_LOG` enabled:
```console
$ cmake -DCMAKE_POSITION_INDEPENDENT_CODE=True -DBUILD_TESTING=ON -DUVWASI_DEBUG_LOG=On -DCMAKE_BUILD_TYPE=Debug ..
```
We can now debug a test with gdb, for example:
```console
$ gdb --args ./out/test-fd-readdir
```
