(module
  (import "js" "mem" (memory 1 100))
  (data (i32.const 1) "\02")    ;; write 2 to memory[1]
  (data (i32.const 2) "\04")    ;; write 3 to memory[2]
  (data (i32.const 3) "Fletch") ;; write string starting at memory[3]
)
