use crate::io::read_file;
use crate::scanner::scan;

mod io;
mod scanner;
mod spike;

fn main() {
    let source = read_file("samples/simple.java");

    let tokens = scan(source.as_str());
    println!("Tokens: {:?}", tokens.len());

    for token in tokens {
        println!("Token: {:?}", token);
    }
}
