LLVM_HOME=~/opt
LLVM_BIN=${LLVM_HOME}/bin
WASI_SYSROOT=${LLVM_HOME}/share/wasi-sysroot
WASMTIME=~/work/wasm/wasmtime/target/release/wasmtime
TRIPLE=wasm32-wasi

out/first.wasm: src/first.c | out
	${LLVM_BIN}/clang --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O2 -s -o out/first.wasm $<

out: 
	@mkdir $@

.PHONY: run
run:
	RUST_BACKTRACE=1 ${WASMTIME} out/first.wasm

.PHONY: fd_write
fd_write:
	@${WASMTIME} src/$@.wat

.PHONY: args_sizes_get
args_sizes_get:
	@${WASMSTIME} src/$@.wat arg1 arg2 arg3
	@echo "status:$?"

.PHONY: clean

clean: 
	@rm -rf out
