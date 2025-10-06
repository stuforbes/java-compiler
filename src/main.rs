use crate::io::read_file;
use java_compiler::build;

mod io;

fn main() {
    let source = read_file("samples/simple.java");
    let class = build(source.as_str());
    println!("{:?}", class);
}
