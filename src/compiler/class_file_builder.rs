use crate::ast::class::{AstClass, AstMethod};
use crate::compiler::result::{wrap, CompileResult};
use crate::compiler::CompilationContext;
use ristretto_classfile::{ClassFile, Method, JAVA_21};

const DEFAULT_SUPER_CLASS: &str = "java/lang/Object";

pub fn from(
    class: &AstClass,
    mut compilation_context: CompilationContext,
) -> CompileResult<ClassFile> {
    let methods = map_methods(class.methods(), &mut compilation_context)?;

    let this_class = wrap(
        compilation_context
            .constant_pool
            .borrow_mut()
            .add_class(class.name()),
    )?;
    let super_class = wrap(
        compilation_context
            .constant_pool
            .borrow_mut()
            .add_class(DEFAULT_SUPER_CLASS),
    )?;

    let class_file = ClassFile {
        version: JAVA_21,
        constant_pool: compilation_context.constant_pool.borrow().to_owned(),
        this_class,
        super_class,
        methods,
        ..Default::default()
    };
    wrap(class_file.verify())?;

    Ok(class_file)
}

fn map_methods(
    ast_methods: &Vec<AstMethod>,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Method>> {
    let mut methods: Vec<Method> = vec![];

    for ast_method in ast_methods {
        methods.push(crate::compiler::method_builder::from(
            &ast_method,
            compilation_context,
        )?);
    }

    Ok(methods)
}
