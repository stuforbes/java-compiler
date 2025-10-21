mod io;
mod scanner;
mod spike;
pub mod ast;
pub mod test_support;
pub mod compiler;
pub mod java;

use crate::ast::to_ast;
use crate::ast::class::AstClass;

#[allow(clippy::needless_lifetimes)]
pub fn build_ast<'a>(source: &'a str) -> AstClass<'a> {
    let tokens = scanner::scan(source);
    to_ast(source, tokens)
}