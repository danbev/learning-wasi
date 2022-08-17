'use strict';

const fs = require('fs');
const buffer = fs.readFileSync('out/add.wasm');

WebAssembly.validate(buffer);

WebAssembly.instantiate(buffer, {}).then((results) => {
  console.log("10 + 20 =", results.instance.exports.add(10, 20));
});
