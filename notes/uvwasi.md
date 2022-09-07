## uvwasi
Is a Wasi implementation that uses libuv. This is the Wasi implementation that
Node.js uses.


### preopens
These are file decsriptors passed into the program from the host environment.
If we take a look at a uvwasi test (`test_readdir`) we find:
```c
  uvwasi_options_init(&init_options);
  init_options.preopenc = 1;
  init_options.preopens = calloc(1, sizeof(uvwasi_preopen_t));
  init_options.preopens[0].mapped_path = "/var";
  init_options.preopens[0].real_path = TEST_PATH_READDIR;
```
And `uvwasi_preopen_t` struct is defined as:
```c
typedef struct uvwasi_preopen_s {
  const char* mapped_path;
  const char* real_path;
} uvwasi_preopen_t;
```
The struct `uvwasi_options` will later be passed to uvwasi_init, and there it
will iterate over all the items in the `preopens` array (error handling has
been removed in the code below):
```c
  for (i = 0; i < options->preopenc; ++i) {
    r = uv_fs_realpath(NULL,
                       &realpath_req,
                       options->preopens[i].real_path,
                       NULL);
    r = uv_fs_open(NULL, &open_req, realpath_req.ptr, 0, 0666, NULL);
    err = uvwasi_fd_table_insert_preopen(uvwasi,
                                         uvwasi->fds,
                                         open_req.result,
                                         options->preopens[i].mapped_path,
                                         realpath_req.ptr);
  }
```
Now, keep in mind that `options.preopens` only contain char pointers of how
files are mapped:
```console
(gdb) p init_options.preopens[0]
$9 = {mapped_path = 0x4320e3 "/var", real_path = 0x432080 "./out/tmp/test_readdir"}
```
In the loop above we are requesting the `readpath` for `real_path`:
```console
(gdb) p (char*)realpath_req.ptr
$18 = 0x4424e0 "/home/danielbevenius/work/nodejs/uvwasi/build/out/tmp/test_readdir"
```
Next a call to libuv `uv_fs_open` is performed with a request to open this file.
The result of `open` is a file descriptor but 
```console
(gdb) p open_req
$19 = {data = 0x350,
       type = UV_FS,
       reserved = {0x350, 0xffff00001f80, 0x0, 0x0, 0x4548530072696464, 0x2f6e69622f3d4c4c},
       fs_type = UV_FS_OPEN,
       loop = 0x0, 
       cb = 0x0,
       result = 3,
       ptr = 0x0, path = 0x4424e0 "/home/danielbevenius/work/nodejs/uvwasi/build/out/tmp/test_readdir",
       statbuf = { st_dev = 4992331448324809828,
                   st_mode = 3417785037641043020,
                   st_nlink = 1095216660480,
                   st_uid = 0,
                   st_gid = 0,
                   st_rdev = 18374686483949813760,
                   st_ino = 0,
                   st_size = 0,
                   st_blksize = 4992331448324809828,
                   st_blocks = 3417785037641043020,
                   st_flags = 1095216660480,
                   st_gen = 0,
                   st_atim = {tv_sec = 0,
                              tv_nsec = -72057589759737856},
                              st_mtim = {tv_sec = 3399988123389603631,
                                         tv_nsec = 3399988123389603631},
                                         st_ctim = {tv_sec = 0, tv_nsec = 0},
                              st_birthtim = {tv_sec = 0, tv_nsec = 0}
       },
       ...
```
```console
(gdb) p open_req.result
$20 = 3
```
Notice that `result` is `3` since we have stdin is 0, stdout is 1 and stderr is
2. This filedescriptor will be passed to `uvwasi_fd_table_insert_preopen`:
```c
    err = uvwasi_fd_table_insert_preopen(uvwasi,
                                         uvwasi->fds,
                                         open_req.result,
                                         options->preopens[i].mapped_path,
                                         realpath_req.ptr);
```
This will add this file descriptor to the files that this wasi instance can
access and will be entry 3 in the list of file descriptors.

### Debugging tests
First build with debug symbols and also enable `UVWASI_DEBUG_LOG` enabled:
```console
$ cmake -DCMAKE_POSITION_INDEPENDENT_CODE=True -DBUILD_TESTING=ON -DUVWASI_DEBUG_LOG=On -DCMAKE_BUILD_TYPE=Debug ..
```
We can now debug a test with gdb, for example:
```console
$ gdb --args ./out/test-fd-readdir
```
