from dist import Root, RootImports
from wasmtime import Store

def main():
    store = Store()
    component = Root(store, RootImports(None, None, None, None, None, None, None, None))
    ret = component.something(store, "bajja")
    print(ret)

if __name__ == '__main__':
    main()
