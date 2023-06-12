## WebAssembly Component Module text format
This document contains notes about compent model text format.

Lets take the following example in a file named demo.wit:
```wat
default world demo {
  export wit: self.wit
}

interface wit {
  use pkg.demo-types.{result-type}
  something: func() -> result-type
}
```
And the types are in an external file (demo-types.wit), since we are using `pkg`
above.
```wat
default interface demo-types {
  variant runtime-value {                                                          
    %string(string),                                                               
    nr
  }

  record result-type {
    value: runtime-value
  }

}
`demo`is a component and will be defined like this in wat:
```
  (component (;0;)
    (type (;0;) (variant (case "string" string) (case "nr")))
    (import "import-type-runtime-value" (type (;1;) (eq 0)))
    (type (;2;) (record (field "value" 1)))
    (import "import-type-result-type" (type (;3;) (eq 2)))
    (import "import-type-result-type0" (type (;4;) (eq 3)))
    (type (;5;) (func (result 4)))
    (import "import-func-something" (func (;0;) (type 5)))
    (export (;6;) "result-type" (type 3))
    (type (;7;) (func (result 6)))
    (export (;1;) "something" (func 0) (func (type 7)))
  )
```
Recall that a component is simlar to an executable file on disk which describes
how this component should be loaded and linked. Above we can see that this
component has a number of imports 

A component is loaded by creating an instance of the component and specifying
the requirements/imports it has.
```
  (instance (;6;) (instantiate 0
      (with "import-func-something" (func 12))
      (with "import-type-runtime-value" (type 12))
      (with "import-type-result-type" (type 13))
      (with "import-type-result-type0" (type 10))
    )
  )
```
The `instantiate` definition takes a core module id which is `0` above.
Notice that `import-func-something` is of `sort` `func` while the others are
of sort `type`.  `func 12` I think is referring to:
```
  (func (;12;) (type 11) (canon lift (core func 30) (memory 0) string-encoding=utf8 (post-return 31)))
```
We can take a look at `type 11`:
```
  (type (;11;) (func (result 10)))
```
Here in `result 10` `10` is actually an alias which was not obvious to me:
```
  (alias export 0 "result-type" (type (;10;)))
```
`export 0` is referring to the component instance 0 which I think is:
```
  (type (;0;)
    (instance
      (type (;0;) (variant (case "string" string) (case "nr")))
              ↑
              +-------------------------------+
                                              ↑
      (export (;1;) "runtime-value" (type (eq 0)))
                ↑
                +------------------------+
                                         ↑
      (type (;2;) (record (field "value" 1)))
              ↑
              +-----------------------------+
                                            ↑
      (export (;3;) "result-type" (type (eq 2)))
    )
  )
```
The following `canon lift` wraps a core function, in this case core func 30
```
  (func (;12;) (type 11) (canon lift (core func 30) (memory 0) string-encoding=utf8 (post-return 31)))
```
```
  (alias core export 2 "wit#something" (core func (;30;)))
```
Notice that this alias is not for a component but for a core module instance id
2:
```
  (core instance (;2;) (instantiate 0
      (with "wasi_snapshot_preview1" (instance 1))
    )
  )
```
And this says it will instantiate core module 0.
```
  (core module (;0;)
     ...
    (export "wit#something" (func $wit#something))
    ...
    (func $wit#something (;20;) (type 9) (result i32)
      (local i32)
      call $demo_module::wit::call_something
      local.set 0
      local.get 0
      return
    )
  )
```
So we are now at canon lift 
```
  (canon lift (func $wit#something (;20;) (type 9) (result i32))
    (memory 0) string-encoding=utf8 (post-return 31))
  )
```
This will wrap the core function to produce a component function. This wrapped
function can then be passed to other components.


```
  (core instance (;0;) (instantiate 2))
```
We can see that this instance is instantiated from core module 2
```
  (core module (;0;)
```

### Variant
A variant is represented by the following type:
```
(type (;0;) (variant (case "string" string) (case "nr")))
```
I'm still note sure about the semicolon syntax around the indices, like `;0;`.
We can see here that this variant has two types, named string and nr. Notice
that the first case is named `string` and is also of type string.
It looks like the  demo-types.wit is represented as the following type, which
is at top component level
```
(component
  (type (;0;)
    (instance
      (type (;0;) (variant (case "string" string) (case "nr")))
      (export (;1;) "runtime-value" (type (eq 0)))
      (type (;2;) (record (field "value" 1)))
      (export (;3;) "result-type" (type (eq 2)))
    )
  )
```
