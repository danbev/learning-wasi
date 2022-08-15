(module
  (; 
     This function adds two i32 values and pushes
     the result onto the stack
   ;)
  (func $add(param $a i32)(param $b i32) (result i32)
	local.get 0
	get_local $b ;; get_local was the old instruction, use the new local.get instead
	i32.add)
  (;
    Example of using prefix notation where the operation comes first.
  ;)
  (func $add2(param $a i32)(param $b i32) (result i32)
	(i32.add (local.get 0) (local.get 1))
  )
  (export "add" (func $add))
  (export "add2" (func $add2))
)
