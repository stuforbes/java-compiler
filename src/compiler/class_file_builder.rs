use crate::ast::class::{AstClass, AstMethod};
use crate::compiler::result::{wrap, CompileResult};
use crate::compiler::{method_builder, CompilationContext};
use ristretto_classfile::{ClassFile, Method, JAVA_21};

const DEFAULT_SUPER_CLASS: &str = "java/lang/Object";

pub fn from(
    class: &AstClass,
    mut compilation_context: &mut CompilationContext,
) -> CompileResult<ClassFile> {

    let this_class = wrap(
        compilation_context
            .constant_pool
            .add_class(class.name()),
    )?;
    let super_class = wrap(
        compilation_context
            .constant_pool
            .add_class(DEFAULT_SUPER_CLASS),
    )?;

    compilation_context.register_this_class(class.name().to_string(), this_class);

    let methods = map_methods(class.methods(), &mut compilation_context)?;

    let class_file = ClassFile {
        version: JAVA_21,
        constant_pool: compilation_context.constant_pool.to_owned(),
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
        methods.push(method_builder::from(
            &ast_method,
            compilation_context,
        )?);
    }

    Ok(methods)
}
