
(module
    (import "wasi_unstable" "args_get" 
        (func $get_args (param i32 i32) (result i32)))
    (import "wasi_unstable" "fd_write" 
        (func $print (param $fd i32) 
	             (param $iovec i32)
		     (param $len i32)
		     (param $written i32) (result i32)))
    (import "wasi_unstable" "proc_exit" 
        (func $exit (param i32)))

    (memory 1)
    (export "memory" (memory 0))

    (data (i32.const 100))
    (data (i32.const 104) "\n")

    (func $newline
	;; new line
        i32.const 32  ;; offset
        i32.const 104  ;; buf*
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 1  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	drop
    )

    (func $mymain (local i32 i32 i32)
        i32.const 0  ;; offset for argv pointer
        i32.const 4  ;; value 
        i32.store align=2
	;; so we have argv as the first slot in memory 0-4

        i32.const 4  ;; offset for argv pointer
        i32.const 0  ;; value 
        i32.store align=2
	;; this is the first pointer for argv 

        i32.const 64  ;; offset for argv_buf*
        i32.const 0   ;; value 
        i32.store align=2
	;; the second slot is pointer to a buffer to write the argument string data
	;; so this needs to be able to store all the data of the arguments, 
	;; argv will contain pointers into this area after args_get has been
	;; called.

        i32.const 0  ;; address 0 for argv
        i32.const 64 ;; address 64 for argv_buf
        call $get_args 
	drop

	;; set up __wasi_ciovec_t (const void buf* and size_t buf_len) for argv[0]
        i32.const 32  ;; offset
        i32.const 64  ;; buf*
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 12  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	drop

	call $newline

	;; set up __wasi_ciovec_t for argv[1]
        i32.const 32  ;; offset
        i32.const 77  ;; buf* 
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 3  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	drop

	call $newline

	;; set up __wasi_ciovec_t for argv[2]
        i32.const 32  ;; offset
        i32.const 81  ;; buf* 
        i32.store align=2
        i32.const 36  ;; offset
        i32.const 3  ;; buf_len
        i32.store offset=0 align=2

        i32.const 1  ;; 1 for stdout
        i32.const 32  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;; how many I/O vectors are passed in, just one in our case
        i32.const 100 ;; nwritten
        call $print
	drop

	call $newline

        i32.const 100  ;; offset
	i32.load offset=0 ;; length written
	call $exit
    )

    (func $main (export "_start")
        call $mymain
    )
)
