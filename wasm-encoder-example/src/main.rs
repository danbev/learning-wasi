use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

fn main() {
    let mut module = Module::new();
    // Encode the type section.
    let mut types = TypeSection::new();
    let params = vec![ValType::I32, ValType::I32];
    let results = vec![ValType::I32];
    types.function(params, results);
    module.section(&types);

    let mut functions = FunctionSection::new();
    let type_index = 0;
    functions.function(type_index);
    module.section(&functions);

    // Encode the export section.
    let mut exports = ExportSection::new();
    exports.export("f", ExportKind::Func, 0);
    module.section(&exports);

    // Encode the code section.
    let mut codes = CodeSection::new();
    let locals = vec![];
    let mut f = Function::new(locals);
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(1));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    let wasm_bytes = module.finish();
    let created = wasmparser::validate(&wasm_bytes).is_ok();
    println!("Wasm module created: {}", created);
    // Save the Wasm bytes to a file.
    std::fs::write("add.wasm", &wasm_bytes).unwrap();
    println!("Saved wasm module in add.wasm");
}
