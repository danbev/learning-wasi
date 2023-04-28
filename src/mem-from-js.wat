(module
  (import "js" "mem" (memory 1 100))
  (func (export "copy_string") (param $start i32)(param $end i32)
    ;; store the bytes at the start posistion and the specified end position
    (i32.store (local.get $end)(i32.load(local.get $start)))
    (i32.store (i32.const 6)(i32.load(i32.const 1)))
    ;; clear the initial "string"
    (i32.store (i32.const 0)(i32.const 0))
    (i32.store (i32.const 1)(i32.const 0))
  )
)
