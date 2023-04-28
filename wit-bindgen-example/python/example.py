from dist import ExampleComponent
from wasmtime import Store

def main():
    store = Store()
    component = ExampleComponent(store)
    ret = component.something(store, "bajja")
    print(ret)

if __name__ == '__main__':
    main()
