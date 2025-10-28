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

pub struct CompilationContext {
    constant_pool: ConstantPool,
    packages: Packages,
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = Packages::new();
    let mut compilation_context = CompilationContext {
        constant_pool,
        packages,
    };

    from(class, &mut compilation_context)
}
