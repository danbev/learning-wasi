package main

import (
	"fmt"
	"github.com/bytecodealliance/wasmtime-go/v8"
)

func main() {
    fmt.Println("wasmtime go example")
    store := wasmtime.NewStore(wasmtime.NewEngine())

    wasm, err := wasmtime.Wat2Wasm(`
      (module
        (import "" "hello" (func $hello))
        (func (export "run")
          (call $hello))
      )
    `)
    check(err)

    module, err := wasmtime.NewModule(store.Engine, wasm)
    check(err)

    item := wasmtime.WrapFunc(store, func() {
        fmt.Println("Hello from Go!")
    })


    instance, err := wasmtime.NewInstance(store, module, []wasmtime.AsExtern{item})
    check(err)

    run := instance.GetFunc(store, "run")
    if run == nil {
        panic("not a function")
    }
    _, err = run.Call(store)
    check(err)
}

func check(e error) {
    if e != nil {
        panic(e)
    }
}
