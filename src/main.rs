use crate::io::read_file;
use java_compiler::build_ast;
use java_compiler::compiler::compile;
use ristretto_classfile::ClassFile;
use std::fs;

mod io;

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

fn write(file_name: &str, class_file: ClassFile) -> ristretto_classfile::Result<()> {
    let mut buffer = Vec::new();
    class_file.to_bytes(&mut buffer)?;

    fs::write("{name}.class".replace("{name}", file_name), buffer)
        .map_err(ristretto_classfile::Error::from)
}
