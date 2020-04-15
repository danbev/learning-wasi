(module
  (import "importObject" "memory" (memory $memory 1))
  (type (func (param i32)))
  (event $exception (type 0))

  (func $something (export "something") (result exnref)
    i32.const 1
    throw $exception
  )
)
