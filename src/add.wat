(module
  (; 
     This function adds two i32 values and pushes
     the result onto the stack
   ;)
  (func $add(param $a i32)(param $b i32) (result i32)
	local.get 0
	get_local $b ;; get_local was the old instruction, use the new local.get instead
	i32.add)
  (export "add" (func $add))
)
