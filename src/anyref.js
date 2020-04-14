const fs = require('fs');
const buffer = fs.readFileSync('anyref.wasm');

WebAssembly.validate(buffer);

const importObject = {
  memory: new WebAssembly.Memory({initial: 10}),
  table: new WebAssembly.Table({element: 'anyref', initial: 1}),
  hello(name) {
    console.log('Hello: ', name);
    console.log(typeof name);
  }
};
const obj = {
  name: 'Fletch',
  creditcardnr: 123456
};

//importObject.table.set(0, 'Fletch');
importObject.table.set(0, obj);

WebAssembly.instantiate(buffer, {importObject}).then((results) => {
  results.instance.exports.main();
});
