use java_compiler::{compile, spike};
use java_compiler::tools::counting_allocator::counting_allocator;

mod io;

#[allow(dead_code)]
fn main_spike() {
    spike::compile("Simple2")
}


fn main() {
    let result = compile("samples/Simple.java");
    
    match result {
        Ok(_) => {
            println!("File written successfully")
        }
        Err(e) => {
            println!("There was an error compiling {:?}", e);
        }
    }

    println!("Allocations: {:}", counting_allocator());
}
