## WebAssembly (Wasm)
Is a portable binary instruction format for a stack based virtual machine.

Just because it has Web in it's name does not limit Wasm to web/browser
environment, it can also be run with other runtimes.

### Spec
https://webassembly.github.io/spec/core/

### Stack based
The processors that I'm most familiar with are regsiter based ones, like the
CPU on my laptop, or the CPU on a microcontroller.
For example, we can add two integers using the `add` instruction:
```assembly
  add eax, ebx 
```

In a stack machine most of the instructions expect the operands, the values in
eax, and ebx above, to be sitting on the stack. This is a Last In First Out
(LIFO) stack.
```wasm
   push 2   
   push 3
   push add
```
Notice that we also push the instruction `add` and not just the arguments.
So these will then be popped off of the stack and the result of the operation
will be pushed onto the stack. The advantages of are that stack machines have a
small binary size and efficient instruction coding, plus ease of portability.
The JVM is an example of a stack machine and also .NET Common Languate Runtime.

### Data types
There are only 4:
```
i32
i64
f32
f64
```
Notice that there are no signed/unsigned types. Instead it is the operator, like
add which has different versions for signed/unsigned. For example, there is
`i32.add` for signed, and `i32.add_u` for unsigned.


### Control flow
```
if
else
loop
block
br
br_if
br_table
return
end
nop
```


### Memory
Wasm does not have a heap like we are used to. Instead wasm has a linear memory
which means memory is a contiguous block of bytes in the module. The actual
memory can be exported and made available to the outside, or it can be imported
from the host. It's like we have a variable that is a array of bytes which we
can store things into.


### WebAssembly Test (wat)

#### get_local
This retrieves a function scoped value and places it on the execution stack.
This was a little confusing to me as in the spec I could not find any
`get_local` and when looking at the generated instructions only `local.get` is
shown. So it seems to be equivalent to:
```
  local.get 1
```
It looks like this instruction was
[renamed](https://github.com/WebAssembly/wabt/commit/052d2864ec4cc45a3aca4bab1a833d1cc45e29d6)
(along with others) for consistency so `get_local` was the old name and the new
name is `local.get`.

