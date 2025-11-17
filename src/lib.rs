mod io;
mod scanner;
pub mod spike;
pub mod ast;
pub mod test_support;
pub mod compiler;
pub mod java;
pub mod tools;

use std::fs;
use ristretto_classfile::ClassFile;
use crate::ast::to_ast;
use crate::ast::class::AstClass;
use crate::compiler::{wrap, CompileError, CompileResult};
use crate::io::read_file;

#[allow(clippy::needless_lifetimes)]
pub fn build_ast<'a>(source: &'a str) -> AstClass<'a> {
    let tokens = scanner::scan(source);
    to_ast(tokens)
}

pub fn compile(source_file_path: &str) -> CompileResult<()> {
    let source = read_file(source_file_path);
    let ast_class = build_ast(source.as_str());
    let name = ast_class.name();

    compiler::compile(&ast_class)
        .and_then(|cf| write(name, cf))
}

fn write(file_name: &str, class_file: ClassFile) -> CompileResult<()> {
    let mut buffer = Vec::new();
    // TODO: We shouldn't leak ristretto out of the compile module
    wrap(class_file.to_bytes(&mut buffer))?;

    fs::write("{name}.class".replace("{name}", file_name), buffer)
        .map_err(|e| CompileError::FileSystem(e))
}