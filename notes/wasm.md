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

So, in a .wat file we may have something like this:
```wat
	(i32.add
          (local.get 0)
          (i32.const 2)
        )
```
The order in how instructions get evaulated is that when the `i32.add`
expression is 'entered' it will see if there are any subexpressions and evaluate
them first.

The `local.get` instruction will retrieve the first argument and push that onto
the stack. The following argument will push a constant onto the stack. Since
there are no more subexpressions the `i32.add` expression will be evaulated and
the arguments it takes are now on the stack which can be popped off.
So the order would be like this:
```
  local.get 0   ;; stack [5]
  i32.const 2   ;; stack [2, 5]
  i32.add       ;; stack [7]
```

The following is an example where two local variables are used:
```console
$ wasm-objdump -d add.wasm

add.wasm:	file format wasm 0x1

Code Disassembly:

000022 func[0] <add>:
 000023: 20 00                      | local.get 0
 000025: 20 01                      | local.get 1
 000027: 6a                         | i32.add
 000028: 0b                         | end
```


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

An `if` statement requires that there is a i32 value on the top of the stack
(wasm does not have a boolean data type, instead i32 is used to represent
booleans). If that value is non-zero then it is treated as `true` and if  it is
0 or negative it will be `false`.


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

### wasm-objdump
```console
$ wasm-objdump -x out/add.wasm 

add.wasm:	file format wasm 0x1

Section Details:

Type[1]:
 - type[0] (i32, i32) -> i32          ;; type that excepts two i32 params and returns a i32
Function[2]:                          ;; functions defined
 - func[0] sig=0 <add>                ;; references type 0 (sig=0)
 - func[1] sig=0 <add2>
Export[2]:
 - func[0] <add> -> "add"
 - func[1] <add2> -> "add2"
Code[2]:
 - func[0] size=7 <add>
 - func[1] size=7 <add2>
```

### wasm2wat
```console
$ wasm2wat out/add.wasm 
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)
  (func (;1;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add)
  (export "add" (func 0))
  (export "add2" (func 1)))
```
Notice that this added a `type` and that the function names are now just using
indecies. A source map can be used link this back to the source code if
available.


### local.tee
This instruction can set a variable and also load the value onto the stack.
Example: [tee.wat](../src/tee.wat), [tee.js](../src/tee.js)
