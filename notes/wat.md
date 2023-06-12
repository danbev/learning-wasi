### WebAssembly Text format (WAT or WAST)


### Binary format
```console
$ xxd add.wasm 
00000000: 0061 736d 0100 0000 010a 0260 0000 6002  .asm.......`..`.
00000010: 7f7f 017f 0303 0200 0104 0501 7001 0101  ............p...
00000020: 0503 0100 0206 2b07 7f01 4180 8804 0b7f  ......+...A.....
00000030: 0041 8008 0b7f 0041 8008 0b7f 0041 8008  .A.....A.....A..
00000040: 0b7f 0041 8088 040b 7f00 4100 0b7f 0041  ...A......A....A
00000050: 010b 077d 0906 6d65 6d6f 7279 0200 115f  ...}..memory..._
00000060: 5f77 6173 6d5f 6361 6c6c 5f63 746f 7273  _wasm_call_ctors
00000070: 0000 0361 6464 0001 0c5f 5f64 736f 5f68  ...add...__dso_h
00000080: 616e 646c 6503 010a 5f5f 6461 7461 5f65  andle...__data_e
00000090: 6e64 0302 0d5f 5f67 6c6f 6261 6c5f 6261  nd...__global_ba
000000a0: 7365 0303 0b5f 5f68 6561 705f 6261 7365  se...__heap_base
000000b0: 0304 0d5f 5f6d 656d 6f72 795f 6261 7365  ...__memory_base
000000c0: 0305 0c5f 5f74 6162 6c65 5f62 6173 6503  ...__table_base.
000000d0: 060a 4202 0200 0b3d 0106 7f23 8080 8080  ..B....=...#....
000000e0: 0021 0241 1021 0320 0220 036b 2104 2004  .!.A.!. . .k!. .
000000f0: 2000 3602 0c20 0420 0136 0208 2004 2802   .6.. . .6.. .(.
00000100: 0c21 0520 0428 0208 2106 2005 2006 6a21  .!. .(..!. . .j!
00000110: 0720 070f 0b00 2004 6e61 6d65 0119 0200  . .... .name....
00000120: 115f 5f77 6173 6d5f 6361 6c6c 5f63 746f  .__wasm_call_cto
00000130: 7273 0103 6164 6400 7a09 7072 6f64 7563  rs..add.z.produc
00000140: 6572 7301 0c70 726f 6365 7373 6564 2d62  ers..processed-b
00000150: 7901 0563 6c61 6e67 5a31 312e 302e 3020  y..clangZ11.0.0 
00000160: 2868 7474 7073 3a2f 2f67 6974 6875 622e  (https://github.
00000170: 636f 6d2f 6c6c 766d 2f6c 6c76 6d2d 7072  com/llvm/llvm-pr
00000180: 6f6a 6563 742e 6769 7420 3839 6136 3634  oject.git 89a664
00000190: 3734 6236 6331 6535 3834 3363 3364 6263  74b6c1e5843c3dbc
000001a0: 3936 6264 6535 3265 3561 3730 3736 6336  96bde52e5a7076c6
000001b0: 6363 29                                  cc)
```

Looking at the first row
```
Address:  
00000000: 0061 736d 0100 0000 010a 0260 0000 6002  .asm.......`..`.
```
The first 4 bytes is the [magic] number which 
00000000: 00 61 73 6d
          \0  a  s  m
```
So that becomes `\0asm`.

Following that we have the version
```
00000000: 0061 736d 0100 0000 010a 0260 0000 6002  .asm.......`..`.
                   [         ]
                       ↑ 
                    01 is the version
```
The magic and the version make out the preamble which is followed by a
sequence of sections.


[magic]: https://webassembly.github.io/spec/core/binary/modules.html#binary-module
