const { readFileSync, openSync, closeSync } = require('fs');        
const { join } = require('path');                                               
const { WASI } = require('wasi');                                               
const { readFile } = require('node:fs/promises');                                                                                

const modulePath = join(__dirname, '..', 'out', 'readdir.wasm');                       

const stdinFile = join(__dirname, 'stdin.txt');
const stdin = openSync(stdinFile, 'a');

const stdoutFile = join(__dirname, 'stdout.txt');
const stdout = openSync(stdoutFile, 'a');

const stderrFile = join(__dirname, 'stderr.txt');
const stderr = openSync(stderrFile, 'a');

const wasi = new WASI({
  stdin: stdin,
  stdout: stdout,
  stderr: stderr,
  returnOnExit: true,
    '/tmp': './src'
});
console.log(wasi);

const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
(async () => {
  const wasm = await WebAssembly.compile(
    await readFile(join(__dirname, '../out/readdir.wasm'))
  );
  const instance = await WebAssembly.instantiate(wasm, importObject);
  console.log(instance.exports);

  const r = wasi.start(instance);
  console.log('Return value from wasi.start: ', r);
  closeSync(stdin);
  closeSync(stdout);
  closeSync(stderr);

  const output = readFileSync(stdoutFile, 'utf8')
  console.log(`output: ${output}`);
})();
