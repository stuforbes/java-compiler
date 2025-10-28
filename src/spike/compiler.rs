use std::fs;
use ristretto_classfile::{ClassFile, ConstantPool, Method, MethodAccessFlags, JAVA_21};
use ristretto_classfile::attributes::Attribute::Code;
use ristretto_classfile::attributes::Instruction;

pub fn compile(class_name: &str) {
    let result = do_compile_and_verify(class_name)
        .and_then(|cf| write(class_name, cf));
    match result {
        Ok(_) => {
            println!("File written successfully")
        }
        Err(e) => {
            println!("There was an error compiling Foo {:?}", e);
        }
    }
}

fn do_compile_and_verify(file_name: &str) -> ristretto_classfile::Result<ClassFile>{
    let mut constant_pool = ConstantPool::default();

    // Class and superclass
    let this_class = constant_pool.add_class(file_name)?;
    let super_class = constant_pool.add_class("java/lang/Object")?;

    // Method name & descriptor
    let main_name_index = constant_pool.add_utf8("main")?;
    let main_descriptor_index = constant_pool.add_utf8("([Ljava/lang/String;)V")?;

    // For println
    let system_class = constant_pool.add_class("java/lang/System")?;
    let print_stream_class = constant_pool.add_class("java/io/PrintStream")?;
    let system_out = constant_pool.add_field_ref(
        system_class,
        "out",
        "Ljava/io/PrintStream;"
    )?;
    let println_method = constant_pool.add_method_ref(
        print_stream_class,
        "println",
        "(Ljava/lang/String;)V",
    )?;
    let hello_string_index = constant_pool.add_string("Hello World")?;
    let main_method_code = constant_pool.add_utf8("Code")?;

    // Code attribute for main
    let instructions = vec![
        Instruction::Getstatic(system_out).into(),  // System.out
        Instruction::Ldc_w(hello_string_index), // "Hello world"
        Instruction::Invokevirtual(println_method).into(), // println
        Instruction::Return.into(),
    ];
    println!("{:?}", instructions);
    let code_attr = Code {
        name_index: main_method_code,
        max_stack: 2, // need space for println parameters
        max_locals: 1, // args[]
        code: instructions,
        exception_table: vec![],
        attributes: vec![],
    };

    // Create a new method representing "public static void main(String[] args)"
    let method = Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: main_name_index,
        descriptor_index: main_descriptor_index,
        attributes: vec![code_attr],
    };

    let class_file = ClassFile {
        version: JAVA_21,
        constant_pool,
        this_class,
        super_class,
        methods: vec![method],
        ..Default::default()
    };
    class_file.verify()?;

    Ok(class_file)
}

fn write(file_name: &str, class_file: ClassFile) -> ristretto_classfile::Result<()> {
    let mut buffer = Vec::new();
    class_file.to_bytes(&mut buffer)?;

    fs::write("{name}.class".replace("{name}", file_name), buffer)
        .map_err(ristretto_classfile::Error::from)
}