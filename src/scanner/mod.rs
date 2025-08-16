mod token;
mod literal;
mod scanner;

pub use scanner::scan;
pub use token::{Token, TokenType};