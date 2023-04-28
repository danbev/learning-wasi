### JavaScript example of using a wit-bindgen component

### Inspect the component
```console
$ npm run inspect-wit

> js@1.0.0 inspect-wit
> npx jco wit ../example-component.wasm

default world component {
  export something: func(s: string) -> string
}
```

### Generate the component bindings for JavaScript
```console
$ npm run bindings

> js@1.0.0 bindings
> npx jco transpile ../example-component.wasm -o dist


Transpiled JS Component Files:

 - dist/example-component.core.wasm  1.93 MiB
 - dist/example-component.d.ts       0.04 KiB
 - dist/example-component.js 
```


### Running the example
```console
$ npm run example

> js@1.0.0 example
> node index.mjs

something was passed: bajja
```
