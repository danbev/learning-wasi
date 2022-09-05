(module
  (import "console" "log" (func $log (param i32)))
  (export "main" (func $main))
  (func $main
    (local $something i32) ;; create a local variable named $something
    (i32.const 18) ;; load `10` onto the stack
    local.tee $something ;; set $something to 18 and keep 18 on the stack
    call $log
  )
  ;;(start $main)
)
