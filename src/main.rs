use crate::io::read_file;
use java_compiler::build;

mod ast;
mod io;
mod scanner;
mod spike;

fn main() {
    let source = read_file("samples/simple.java");
    let class = build(source.as_str());
    println!("{:?}", class);
    // println!("Tokens: {:?}", tokens.len());
    //
    // for token in tokens {
    //     println!("Token: {:?}", token);
    // }
}
