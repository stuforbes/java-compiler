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
use crate::java::class::JavaClass;

pub struct CompilationContext<'compiler> {
    constant_pool: ConstantPool,
    class_loader: ClassLoader,
    scoped_object: Option<ObjectReference<'compiler>>,
}

impl <'compiler> CompilationContext<'compiler> {
    pub fn push_scoped_object(&mut self, class: &'compiler JavaClass, class_id: u16) {
        self.scoped_object = Some(ObjectReference{ class, class_id })
    }
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = new_class_loader();
    let mut compilation_context = CompilationContext {
        constant_pool,
        class_loader: packages,
        scoped_object: None,
    };

    from(class, &mut compilation_context)
}

struct ObjectReference<'compiler> {
    class: &'compiler JavaClass,
    class_id: u16,
}