use crate::io::read_file;
use java_compiler::{build_ast, spike};
use java_compiler::compiler::{compile, wrap, CompileError, CompileResult};
use ristretto_classfile::ClassFile;
use std::fs;

mod io;

#[allow(dead_code)]
fn main_spike() {
    spike::compile("Simple2")
}

fn main() {
    let source = read_file("samples/simple.java");
    let ast_class = build_ast(source.as_str());
    let name = ast_class.name();
    println!("{:?}", ast_class);

    let result = compile(&ast_class)
        .and_then(|cf| write(name, cf));

    match result {
        Ok(_) => {
            println!("File written successfully")
        }
        Err(e) => {
            println!("There was an error compiling Foo {:?}", e);
        }
    }
}

fn write(file_name: &str, class_file: ClassFile) -> CompileResult<()> {
    let mut buffer = Vec::new();
    // TODO: We shouldn't leak ristretto out of the compile module
    wrap(class_file.to_bytes(&mut buffer))?;

    fs::write("{name}.class".replace("{name}", file_name), buffer)
        .map_err(|e| CompileError::FileSystem(e))
}
