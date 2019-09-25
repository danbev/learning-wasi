LLVM_HOME=/usr/local/opt/llvm
LLVM_BIN=${LLVM_HOME}/bin

out/first.wasm: src/first.c | out
	${LLVM_BIN}/clang --target=wasm32-wasi --sysroot ./wasi-libc/sysroot -O2 -s -o out/first.wasm $<

out: 
	@mkdir $@

.PHONY: run
run:
	RUST_BACKTRACE=1 wasmtime/target/release/wasmtime out/first.wasm

.PHONY: fd_write
fd_write:
	@wasmtime/target/release/wasmtime src/$@.wat

.PHONY: args_sizes_get
args_sizes_get:
	@wasmtime/target/release/wasmtime src/$@.wat arg1 arg2 arg3
	@echo "status:$?"

.PHONY: clean

clean: 
	@rm -rf out
