	.text
	.file	"add.c"
	.section	.text.add,"",@
	.hidden	add                     # -- Begin function add
	.globl	add
	.type	add,@function
add:                                    # @add
	.functype	add (i32, i32) -> (i32)
	.local  	i32, i32, i32, i32, i32, i32
# %bb.0:                                # %entry
	global.get	__stack_pointer
	local.set	2
	i32.const	16
	local.set	3
	local.get	2
	local.get	3
	i32.sub 
	local.set	4
	local.get	4
	local.get	0
	i32.store	12
	local.get	4
	local.get	1
	i32.store	8
	local.get	4
	i32.load	12
	local.set	5
	local.get	4
	i32.load	8
	local.set	6
	local.get	5
	local.get	6
	i32.add 
	local.set	7
	local.get	7
	return
	end_function
.Lfunc_end0:
	.size	add, .Lfunc_end0-add
                                        # -- End function
	.ident	"clang version 11.0.0 (https://github.com/llvm/llvm-project.git 89a66474b6c1e5843c3dbc96bde52e5a7076c6cc)"
	.globaltype	__stack_pointer, i32
	.section	.custom_section.producers,"",@
	.int8	1
	.int8	12
	.ascii	"processed-by"
	.int8	1
	.int8	5
	.ascii	"clang"
	.int8	90
	.ascii	"11.0.0 (https://github.com/llvm/llvm-project.git 89a66474b6c1e5843c3dbc96bde52e5a7076c6cc)"
	.section	.text.add,"",@
