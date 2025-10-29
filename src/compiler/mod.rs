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
use crate::java::ClassLoader;
use ristretto_classfile::{ClassFile, ConstantPool};

pub struct CompilationContext {
    constant_pool: ConstantPool,
    class_loader: ClassLoader,
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = ClassLoader::new();
    let mut compilation_context = CompilationContext {
        constant_pool,
        class_loader: packages,
    };

    from(class, &mut compilation_context)
}
