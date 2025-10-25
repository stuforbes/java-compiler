mod class_file_builder;
mod instruction;
mod instruction_builder;
mod method_builder;
mod resolved_class;
mod result;
mod symbol;

use crate::ast::class::AstClass;
use crate::compiler::class_file_builder::from;
pub use crate::compiler::result::{wrap, CompileError, CompileResult};
use crate::java::Packages;
use ristretto_classfile::{ClassFile, ConstantPool};
use std::cell::RefCell;

pub struct CompilationContext {
    constant_pool: RefCell<ConstantPool>,
    packages: RefCell<Packages>,
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = Packages::new();
    let compilation_context = CompilationContext {
        constant_pool: RefCell::new(constant_pool),
        packages: RefCell::new(packages),
    };

    from(class, compilation_context)
}
