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
    scoped_object: Option<ObjectReference>,
}

impl CompilationContext {
    pub fn push_scoped_object(&mut self, class_path: String, class_id: u16) {
        self.scoped_object = Some(ObjectReference{ class_path, class_id })
    }

    pub fn scoped_class_path(&self) -> Option<String> {
        self.scoped_object
            .as_ref()
            .map(|o| o.class_path.clone())
    }

    pub fn scoped_class_id(&self) -> Option<u16> {
        self.scoped_object
            .as_ref()
            .map(|o| o.class_id)
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

struct ObjectReference {
    // todo: can we use a reference instead?
    class_path: String,
    class_id: u16,
}