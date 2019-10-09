(module
  (import "console" "log" (func $log (param i32)))
  (import "wasi_unstable" "args_get" 
        (func $get_args (param i32 i32) (result i32)))

  (memory (export "memory") 1)

  ;; utility function for incrementing a value
  (func $increment (param $value i32) (result i32)
    (i32.add 
      (get_local $value)
      (i32.const 1)
    )
  )

  (func $logArray
    (local $x i32)

    (set_local $x (i32.const 0))

    (block 
      (loop 

        (call $log
           ;; load a single unsigned byte from memory location $x
           (i32.load8_u (get_local $x))
        )

        (set_local $x (call $increment (get_local $x)))
        ;; break to a depth of 1 if x equals 50
        (br_if 1 (i32.eq (get_local $x) (i32.const 50)))
        ;; break to a depth of zero, continuing the loop
        (br 0)
      )
    )
  )

  (func $main (export "_start")
    call $logArray
  )
)
