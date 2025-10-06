use crate::ast::to_ast;
use crate::scanner::scan;
use crate::ast::class::Class;

mod io;
mod scanner;
mod spike;
mod ast;

#[allow(clippy::needless_lifetimes)]
pub fn build<'a>(source: &'a str) -> Class<'a> {
    let tokens = scan(source);
    to_ast(tokens)
}