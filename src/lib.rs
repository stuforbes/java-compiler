mod io;
mod scanner;
mod spike;
pub mod ast;
pub mod test_support;

use crate::ast::to_ast;
use crate::ast::class::Class;

#[allow(clippy::needless_lifetimes)]
pub fn build<'a>(source: &'a str) -> Class<'a> {
    let tokens = scanner::scan(source);
    to_ast(tokens)
}