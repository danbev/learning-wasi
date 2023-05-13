(module
    (import "wasi_snapshot_preview1" "fd_write" 
        (func $print (param $fd i32) 
	             (param $iovec i32)
		     (param $len i32)
		     (param $written i32) (result i32)))

    (memory 1)
    (export "memory" (memory 0))

    ;; 9 is the offset to write to
    (data (i32.const 9) "something\n")

    (func $main (export "_start")
        i32.const 0  ;; offset
        i32.const 9  ;; value start of the string
        i32.store    ;;

        i32.const 4                  ;; offset
        i32.const 11                 ;; value, the length of the string
        i32.store offset=0 align=2   ;; size_buf_len

        i32.const 1  ;; 1 for stdout
        i32.const 0  ;; 0 as we stored the beginning of __wasi_ciovec_t
        i32.const 1  ;;
        i32.const 20 ;; nwritten
        call $print
        drop ;; Discard the number of bytes written from the top the stack
    )
)
