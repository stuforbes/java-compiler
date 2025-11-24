use crate::ast::class::{AstMethod, AstScope};
use ristretto_classfile::{Method, MethodAccessFlags};
use ristretto_classfile::attributes::Attribute::Code;
use ristretto_classfile::attributes::Instruction;
use crate::compiler::{instruction, CompilationContext};
use crate::compiler::resolved_class::ResolvedClass;
use crate::compiler::result::{wrap, CompileResult};

pub fn from(
    ast_method: &AstMethod,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Method> {
    let method_access_flags =
        append_scope_flag_from(ast_method.scope(),
            append_final_flag_from(ast_method.is_final(),
                append_static_flag_from(ast_method.is_static(),
                    MethodAccessFlags::empty()
                )
            )
        );

    let main_method_code = wrap(compilation_context.constant_pool.add_utf8("Code"))?;

    compilation_context.stack.new_layer();
    let mut instructions: Vec<Instruction> = vec![];
    build_instructions(ast_method, compilation_context, &mut instructions)?;

    // TODO: This will break when methods have inner stack layers
    let num_fields_in_stack = compilation_context.stack.count();

    compilation_context.stack.drop_layer();

    // TODO: multiple method arguments. Currently, only main() is supported, with `args` parameter
    let num_method_arguments: u16 = 1;

    Ok(Method {
        access_flags: method_access_flags,
        name_index: wrap(compilation_context.constant_pool.add_utf8(ast_method.name()))?,
        descriptor_index: wrap(compilation_context.constant_pool.add_utf8(method_parameters_str(ast_method)))?,
        attributes: vec![Code {
            name_index: main_method_code,
            max_stack: 2, // need space for println parameters
            max_locals: num_method_arguments + num_fields_in_stack,
            code: instructions,
            exception_table: vec![],
            attributes: vec![],
        }],
    })
}

fn build_instructions(method: &AstMethod, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> CompileResult<()> {
    for statement in method.statements() {
        instruction::from(statement, compilation_context, instructions)?;

        compilation_context.clear_scoped_object();
    }

    println!("{:?}", instructions);

    Ok(())
}

fn append_scope_flag_from(
    scope: AstScope,
    method_access_flags: MethodAccessFlags,
) -> MethodAccessFlags {
    match scope {
        AstScope::Public => method_access_flags | MethodAccessFlags::PUBLIC,
        AstScope::Protected => method_access_flags | MethodAccessFlags::PROTECTED,
        AstScope::Private => method_access_flags | MethodAccessFlags::PRIVATE,
        AstScope::Default => method_access_flags,
    }
}

fn append_static_flag_from(
    is_static: bool,
    method_access_flags: MethodAccessFlags,
) -> MethodAccessFlags {
    if is_static {
        method_access_flags | MethodAccessFlags::STATIC
    } else {
        method_access_flags
    }
}

fn append_final_flag_from(
    is_final: bool,
    method_access_flags: MethodAccessFlags,
) -> MethodAccessFlags {
    if is_final {
        method_access_flags | MethodAccessFlags::FINAL
    } else {
        method_access_flags
    }
}

fn method_parameters_str(ast_method: &AstMethod) -> String {
    let mut result = String::new();

    result.push('(');

    for parameter in ast_method.parameters() {
        parameter_str(parameter.param_type(), parameter.is_array(), &mut result);
    }

    result.push(')');
    parameter_str(ast_method.return_type(), false, &mut result);
    result
}

fn parameter_str(param_type: &str, is_array: bool, result: &mut String) {
    if is_array {
        result.push('[');
    }
    parameter_str_after_array_resolution(param_type, result);
}

fn parameter_str_after_array_resolution(param_type: &str, result: &mut String) {
    match param_type {
        "bool" => result.push('Z'),
        "byte" => result.push('B'),
        "char" => result.push('C'),
        "short" => result.push('S'),
        "int" => result.push('I'),
        "long" => result.push('J'),
        "float" => result.push('F'),
        "double" => result.push('D'),
        "void" => result.push('V'),
        s => handle_non_primitive(s, result),
    }
}

fn handle_non_primitive(s: &str, result: &mut String) {
    if let Some(class) = ResolvedClass::from_name(s) {
        result.push('L');
        result.push_str(class.fully_qualified());
        result.push(';');
    }
}
