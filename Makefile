LLVM_HOME=/home/danielbevenius/work/wasm/wasi-sdk-20.0
LLVM_BIN=${LLVM_HOME}/bin
WASI_SYSROOT=${LLVM_HOME}/share/wasi-sysroot
WASMTIME=~/work/wasm/wasmtime/target/release/wasmtime
TRIPLE=wasm32-wasi

out/first.wasm: src/first.c | out
	${LLVM_BIN}/clang --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O2 -s -o out/first.wasm $<

out/firstcxx.wasm: src/first.cc | out
	${LLVM_BIN}/clang++ -v -std=c++11 --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O2 -s -o out/firstcxx.wasm $<

out/readdir.wasm: src/readdir.c | out
	${LLVM_BIN}/clang --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -g -O0 -s -o $@ $<

out/readdir.llvm: src/readdir.c | out
	${LLVM_BIN}/clang -S -emit-llvm --target=${TRIPLE} --sysroot ${WASI_SYSROOT} -O0 -o $@ $<

out/readdir: src/readdir.c | out
	${CC} -O0 -g -s -o $@ $<

out/readdir_s: src/readdir.c | out
	${CC} -O0 -g -s -static -Xlinker -Map=readdir_s.map -o $@ $<

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
	@${RM} -rf out
