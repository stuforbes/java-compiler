use crate::compiler::state_stack::StateStack;
use crate::compiler::{CompileError, CompileResult};
use crate::java::class::JavaClass;
use crate::java::ClassLoader;

pub enum ResolvedEntity {
    VariableOnStack(u8),
    StaticClass(String),
    StaticFieldReference {
        parent_id: u16,
        path: String,
        full_name: String,
        descriptor: String,
    },
}

pub struct Resolver;

impl Resolver {
    pub fn resolve_unscoped<'a>(&mut self, name: &str, stack: &StateStack, class_loader: &'a mut ClassLoader) -> CompileResult<ResolvedEntity> {
        if stack.contains(name) {
            return Ok(ResolvedEntity::VariableOnStack(stack.get(name)));
        }
        if let Some(class) = class_loader.load(name) {
            return Ok(ResolvedEntity::StaticClass(class.path().to_string()));
        }
        Err(CompileError::ResolutionFailure)
    }

    pub fn resolve_scoped(
        &mut self,
        name: &str,
        scoped_class_id: u16,
        scoped_class_path: &str,
        class_loader: &mut ClassLoader,
    ) -> CompileResult<ResolvedEntity> {
        let (path, full_name, descriptor) = lookup_field_on_class(name, scoped_class_path, class_loader)?;
        Ok(ResolvedEntity::StaticFieldReference {
            parent_id: scoped_class_id,
            path,
            full_name,
            descriptor,
        })
    }
}

fn lookup_field_on_class(field_name: &str, scoped_class_path: &str, class_loader: &mut ClassLoader) -> CompileResult<(String, String, String)> {
    let field_class_path = {
        let class = lookup_class(scoped_class_path, class_loader)?;
        let field = class.field_named(field_name).ok_or_else(|| CompileError::UnknownField {
            class: scoped_class_path.to_string(),
            field: field_name.to_string(),
        })?;
        field.class().to_string()
    };

    let field_class = lookup_class(field_class_path.as_str(), class_loader)?;

    Ok((field_class_path, field_class.full_name(), field_class.descriptor().to_string()))
}

fn lookup_class<'a>(class_name: &str, class_loader: &'a mut ClassLoader) -> CompileResult<&'a JavaClass> {
    class_loader
        .load(class_name)
        .ok_or_else(|| CompileError::UnknownClass(class_name.to_string()))
}
