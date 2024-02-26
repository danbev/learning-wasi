(module
  (import "host" "log" (func $log (param i32)))
  (import "host" "double" (func $double (param i32) (result i32)))
  (func (export "run")
    i32.const 0
    call $log
    i32.const 1
    call $log
    i32.const 2
    call $double
    call $log
  )
)
