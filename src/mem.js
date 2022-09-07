const { readFileSync, openSync, closeSync } = require('fs');        
const { join } = require('path');                                               
const { readFile } = require('node:fs/promises');                                                                                
const wasm_file = 'mem.wasm';

// Creates a new memory (think ArrayBuffer of size 1 page (64KB)
//const memory = new WebAssembly.Memory({ initial: 1, maximum: 100, shared: true });
const memory = new WebAssembly.Memory({ initial: 1 });
const buffer = memory.buffer;
console.log(memory);

// Creata a TypedArray for the memory buffer
const a = new Uint32Array(memory.buffer);
// Write to the memory buffer
a[0] = 1;
console.log(buffer);
console.log(a);

const importObject = {
  js: {
    mem: memory
  }
};


(async () => {
  const wasm = await WebAssembly.compile(
    await readFile(join(__dirname, `../out/${wasm_file}`))
  );
  console.log(importObject);

  const instance = await WebAssembly.instantiate(wasm, importObject);
  console.log(instance.exports);
  console.log("wasm module should have added a 2 to index 1:");
  console.log(memory.buffer);
})();
