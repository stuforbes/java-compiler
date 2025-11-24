use std::io::Error;

pub type CompileResult<T> = Result<T, CompileError>;
pub type EmptyCompileResult = CompileResult<()>;

#[derive(Debug)]
pub enum CompileError {
    Ristretto(ristretto_classfile::Error),
    FileSystem(Error),
    UnknownClass(String),
    UnknownMethod { class: String, method: String },
    UnknownField { class: String, field: String },
}

pub fn wrap<T>(result: ristretto_classfile::Result<T>) -> CompileResult<T> {
    result.map_err(|e| CompileError::Ristretto(e))
}
