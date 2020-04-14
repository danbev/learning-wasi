const fs = require('fs');
const buffer = fs.readFileSync('multivalue.wasm');

WebAssembly.validate(buffer);

const importObject = {
  memory: new WebAssembly.Memory({initial: 10}),
};

WebAssembly.instantiate(buffer, {importObject}).then((results) => {
  const values = results.instance.exports.multivalue(1);
  console.log('multivalue function returned: ', values);
});
