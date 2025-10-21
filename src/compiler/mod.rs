mod class_file_builder;
mod method_builder;
mod resolved_class;
mod instruction_builder;
mod symbol;
mod java;
mod instruction;
mod result;

use crate::ast::class::AstClass;
use crate::compiler::class_file_builder::from;
use ristretto_classfile::{ClassFile, ConstantPool};
pub use crate::compiler::result::{CompileError, CompileResult, wrap};


pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();

    from(class, constant_pool)
}
