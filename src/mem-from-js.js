const { readFileSync, openSync, closeSync } = require('fs');
const { join } = require('path');
const { readFile } = require('node:fs/promises');
const wasm_file = 'mem-from-js.wasm';

// Creates a new memory (think of this as an ArrayBuffer of size 1 page (64KB))
const memory = new WebAssembly.Memory({ initial: 1, maximum: 100 });

// Create a TypedArray for the memory buffer
const mem_view = new Uint8Array(memory.buffer);
mem_view.set(new TextEncoder().encode("bajja"));
console.log("memory: ", mem_view);

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
  // Here we are passing in the positions of the string that we want to pass
  // to the function, start=0, end=5.
  instance.exports.copy_string(0, 5);
  console.log(new Uint8Array(memory.buffer));

  // The wasm function will have copied/moved the bytes to these posistions:
  const bytes = new Uint8Array(memory.buffer, 5, 10);
  const str = new TextDecoder('utf-8').decode(bytes);
  console.log(`String from wasm function/memory: ${str}`);
})();
