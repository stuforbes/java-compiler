mod class_file_builder;
mod instruction;
mod method_builder;
mod resolved_class;
mod result;

use crate::ast::class::AstClass;
use crate::compiler::class_file_builder::from;
pub use crate::compiler::result::{wrap, CompileError, CompileResult};
use ristretto_classfile::{ClassFile, ConstantPool};
use crate::java::{new_class_loader, ClassLoader};

pub struct CompilationContext {
    constant_pool: ConstantPool,
    class_loader: ClassLoader,
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = new_class_loader();
    let mut compilation_context = CompilationContext {
        constant_pool,
        class_loader: packages,
    };

    from(class, &mut compilation_context)
}
