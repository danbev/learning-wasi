## Golang example of using wasmtime (wasmtime-go)

### Prerequisites
Install wasmtime-go:
```console
$ go get -u github.com/bytecodealliance/wasmtime-go/v8@v8.0.0

go: downloading github.com/bytecodealliance/wasmtime-go/v8 v8.0.0
go: added github.com/bytecodealliance/wasmtime-go/v8 v8.0.0
```

### Build
```console
$ make build
go install example.go
```

### Run
```console
$ make run 
go run example.go
wasmtime go example
Hello from Go!
```

