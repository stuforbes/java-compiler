mod class_file_builder;
mod instruction;
mod method_builder;
mod resolved_class;
mod result;
mod state_stack;
mod resolver;
mod symbol_table;

use std::rc::Rc;
use crate::ast::class::{AstClass, AstParameter};
use crate::compiler::class_file_builder::from;
pub use crate::compiler::result::{wrap, CompileError, CompileResult, EmptyCompileResult};
use ristretto_classfile::{ClassFile, ConstantPool};
use crate::compiler::resolver::{ResolvedEntity, Resolver};
use crate::compiler::result::EMPTY_OK;
use crate::compiler::state_stack::StateStack;
use crate::compiler::symbol_table::{DataType, SymbolTable};
use crate::java::{new_class_loader, ClassLoader};

pub struct CompilationContext {
    resolver: Resolver,
    constant_pool: ConstantPool,
    class_loader: ClassLoader,
    this_class: Option<ObjectReference>,
    scoped_object: Option<ObjectReference>,
    stack: StateStack,
    symbol_table: SymbolTable
}

impl CompilationContext {

    pub fn register_this_class(&mut self, class_path: String, class_id: u16) {
        self.this_class = Some(ObjectReference { class_path, class_id });
    }

    pub fn this_class_id(&self) -> u16 {
        self.this_class
            .as_ref()
            .map(|o| o.class_id)
            .expect("This class has not been registered yet")
    }

    pub fn this_class_path(&self) -> String {
        self.this_class
            .as_ref()
            .map(|o| o.class_path.clone())
            .expect("This class has not been registered yet")
    }

    pub fn push_scoped_object(&mut self, class_path: String, class_id: u16) {
        self.scoped_object = Some(ObjectReference{ class_path, class_id })
    }

    pub fn clear_scoped_object(&mut self) {
        self.scoped_object = None;
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

    pub fn resolve(&mut self, name: &str) -> CompileResult<ResolvedEntity> {
        if let Some(scoped_object) = &self.scoped_object {
            self.resolver.resolve_scoped(name, scoped_object.class_id, scoped_object.class_path.as_str(), &mut self.class_loader)
        } else {
            self.resolver.resolve_unscoped(name, &self.stack, &mut self.class_loader)
        }
    }
}

pub fn compile(class: &AstClass) -> CompileResult<ClassFile> {
    let constant_pool = ConstantPool::default();
    let packages = new_class_loader();
    let mut compilation_context = CompilationContext {
        constant_pool,
        resolver: Resolver {},
        class_loader: packages,
        this_class: None,
        scoped_object: None,
        stack: StateStack::new(),
        symbol_table: SymbolTable::new(),
    };

    // 1st pass - build symbol table
    build_symbol_table(class, &mut compilation_context)?;

    from(class, &mut compilation_context)
}

fn build_symbol_table(class: &AstClass, compilation_context: &mut CompilationContext) -> EmptyCompileResult {

    for method in class.methods() {
        compilation_context.symbol_table.register_method(
            method.name().to_string(),
            map_data_type(method.return_type(), method.return_type_is_array(), &mut compilation_context.class_loader),
            map_parameters(method.parameters(), &mut compilation_context.class_loader)
        )
    }

    EMPTY_OK
}

fn map_parameters(parameters: &Vec<AstParameter>, class_loader: &mut ClassLoader) -> Vec<DataType> {
    parameters
        .iter()
        .map(|p| map_data_type(p.param_type(), p.is_array(), class_loader))
        .collect()
}

fn map_data_type(string: &str, is_array: bool, class_loader: &mut ClassLoader) -> DataType {
    if is_array {
        DataType::Array(Rc::new(map_data_type_inner(string, class_loader)))
    } else {
        map_data_type_inner(string, class_loader)
    }
}

fn map_data_type_inner(string: &str, class_loader: &mut ClassLoader) -> DataType {
    match string {
        "bool" => DataType::BooleanPrimitive,
        "byte" => DataType::BytePrimitive,
        "char" => DataType::CharPrimitive,
        "double"=> DataType::DoublePrimitive,
        "float"=> DataType::FloatPrimitive,
        "int"=> DataType::IntPrimitive,
        "long"=> DataType::LongPrimitive,
        "short"=> DataType::ShortPrimitive,
        "void"=> DataType::Void,
        _ => {
            match class_loader.load(string) {
                None => panic!("Could not load {:?}", string),
                Some(class) => DataType::Object(class.path().to_string())
            }
        },
    }
}

struct ObjectReference {
    // todo: can we use a reference instead?
    class_path: String,
    class_id: u16,
}