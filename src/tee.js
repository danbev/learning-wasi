'use strict';

const fs = require('fs');
const buffer = fs.readFileSync('out/tee.wasm');

WebAssembly.validate(buffer);

WebAssembly.instantiate(buffer, {console}).then((results) => {
  results.instance.exports.main();
});
