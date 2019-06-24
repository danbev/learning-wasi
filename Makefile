LLVM_HOME=/usr/local/opt/llvm/
LLVM_BIN=${LLVM_HOME}/bin

out/first.wasm: src/first.c | out
	${LLVM_BIN}/clang --target=wasm32-wasi --sysroot ./wasi-libc/sysroot -O2 -s -o out/first.wasm $<

out: 
	@mkdir $@

.PHONY: clean

clean: 
	@rm -rf out
