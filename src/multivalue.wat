(module
  (import "importObject" "memory" (memory $memory 1))
  (func $multi (export "multivalue") (param i32) (result i32 i32 i32)
	i32.const 2
	i32.const 3
	i32.const 4
  )
)
