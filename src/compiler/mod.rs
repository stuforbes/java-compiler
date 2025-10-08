mod class_file_builder;
mod method_builder;
mod resolved_class;

use crate::ast::class::AstClass;
use crate::compiler::class_file_builder::from;
use ristretto_classfile::{ClassFile, ConstantPool};


pub fn compile(class: &AstClass) -> ristretto_classfile::Result<ClassFile> {
    let constant_pool = ConstantPool::default();

    from(class, constant_pool)
}
