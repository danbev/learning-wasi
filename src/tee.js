'use strict';

const fs = require('fs');
const buffer = fs.readFileSync('out/tee.wasm');

WebAssembly.validate(buffer);

WebAssembly.instantiate(buffer, {console}).then((results) => {
  const imports = WebAssembly.Module.imports(results.module);
  console.log('imports: ', imports);
  const exports = WebAssembly.Module.exports(results.module);
  console.log('exports: ', exports);
  results.instance.exports.main();
});
