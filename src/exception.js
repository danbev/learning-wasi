const fs = require('fs');
const buffer = fs.readFileSync('exception.wasm');

WebAssembly.validate(buffer);

const importObject = {
  memory: new WebAssembly.Memory({initial: 10}),
};

WebAssembly.instantiate(buffer, {importObject}).then((results) => {
  try {
    results.instance.exports.something();
  } catch (e) {
    console.log('Caught exception from wasm:', e);
  }
});
