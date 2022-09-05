LLVM_HOME=/home/danielbevenius/work/wasm/wasi-sdk/download/wasi-sdk-8.0
LLVM_BIN=${LLVM_HOME}/bin
WASI_SYSROOT=${LLVM_HOME}/share/wasi-sysroot
WASMTIME=~/work/wasm/wasmtime/target/release/wasmtime
TRIPLE=wasm32-wasi

out/first.wasm: src/first.c | out
	${LLVM_BIN}/clang --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O2 -s -o out/first.wasm $<

out/firstcxx.wasm: src/first.cc | out
	${LLVM_BIN}/clang++ -v -std=c++11 --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O2 -s -o out/firstcxx.wasm $<

out/%.wasm: src/%.wat | out
	wat2wasm -v -o $@ $< --debug-names

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
