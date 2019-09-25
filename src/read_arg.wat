(module
    (import "wasi_unstable" "args_sizes_get" 
        (func $get_argc (param i32 i32) (result i32)))
    (import "wasi_unstable" "proc_exit" 
        (func $exit (param i32)))

    (memory 1)
    (export "memory" (memory 0))

    (func $mymain (local i32 i32 i32)
        get_local 0
        get_local 1
        call $get_argc 
	tee_local 0
	call $exit
    )

    (func $main (export "_start")
        call $mymain
    )
)
