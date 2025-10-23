use crate::ast::class::{AstMethod, AstScope};
use ristretto_classfile::{ConstantPool, Method, MethodAccessFlags};
use ristretto_classfile::attributes::Attribute::Code;
use ristretto_classfile::attributes::Instruction;
use crate::compiler::instruction;
use crate::compiler::resolved_class::ResolvedClass;
use crate::compiler::result::{wrap, CompileResult};

pub fn from(
    ast_method: &AstMethod,
    constant_pool: &mut ConstantPool,
) -> CompileResult<Method> {
    let method_access_flags =
        append_scope_flag_from(ast_method.scope(),
            append_final_flag_from(ast_method.is_final(),
                append_static_flag_from(ast_method.is_static(),
                    MethodAccessFlags::empty()
                )
            )
        );

    let main_method_code = wrap(constant_pool.add_utf8("Code"))?;

    let instructions: Vec<Instruction> = build_instructions(ast_method, constant_pool)?;

    Ok(Method {
        access_flags: method_access_flags,
        name_index: wrap(constant_pool.add_utf8(ast_method.name()))?,
        descriptor_index: wrap(constant_pool.add_utf8(method_parameters_str(ast_method)))?,
        attributes: vec![Code {
            name_index: main_method_code,
            max_stack: 2, // need space for println parameters
            max_locals: 1, // args[]
            code: instructions,
            exception_table: vec![],
            attributes: vec![],
        }],
    })
}

fn build_instructions(method: &AstMethod, constant_pool: &mut ConstantPool) -> CompileResult<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = vec![];

    for statement in method.statements() {
        let statement_instructions = instruction::from(statement, constant_pool)?;
        for statement_instruction in statement_instructions {
            instructions.push(statement_instruction);
        }
    }

    Ok(instructions)
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
