use ristretto_classfile::{ClassFile, ConstantPool, Method, JAVA_21};
use crate::ast::class::{AstClass, AstMethod};
use crate::compiler::result::{wrap, CompileResult};

const DEFAULT_SUPER_CLASS: &str = "java/lang/Object";

pub fn from(class: &AstClass, mut constant_pool: ConstantPool) -> CompileResult<ClassFile> {

    let methods = map_methods(class.methods(), &mut constant_pool)?;

    let this_class = wrap(constant_pool.add_class(class.name()))?;
    let super_class = wrap(constant_pool.add_class(DEFAULT_SUPER_CLASS))?;

    let class_file = ClassFile {
        version: JAVA_21,
        constant_pool,
        this_class,
        super_class,
        methods,
        ..Default::default()
    };
    wrap(class_file.verify())?;

    Ok(class_file)
}

fn map_methods(ast_methods: &Vec<AstMethod>, constant_pool: &mut ConstantPool) -> CompileResult<Vec<Method>> {
    let mut methods: Vec<Method> = vec![];

    for ast_method in ast_methods {
            methods.push(crate::compiler::method_builder::from(&ast_method, constant_pool)?);
    }

    Ok(methods)
}