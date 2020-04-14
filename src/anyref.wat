(module
  (import "importObject" "memory" (memory $memory 1))
  (import "importObject" "table" (table $anyref_table 1 anyref))
  (import "importObject" "hello" (func $log (param anyref)))

  (func $main (export "main")
    (local $name anyref)
    (set_local $name
      (table.get $anyref_table
        (i32.const 0)
      )
    )
    (call $log
      (get_local $name)
    )
  )
)
