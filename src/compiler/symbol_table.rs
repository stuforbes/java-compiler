use std::collections::HashMap;
use std::rc::Rc;

pub struct SymbolTable {
    fields: HashMap<String, FieldSymbol>,
    // TODO: This is not right - it doesn't handle method overloading
    methods: HashMap<String, MethodSymbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn contains_field(&self, name: &String) -> bool {
        self.fields.contains_key(name)
    }

    pub fn register_field(&mut self, name: String, data_type: DataType) {
        if self.contains_field(&name) {
            panic!("Field {} already exists", name)
        }

        self.fields.insert(name, FieldSymbol { field_type: data_type });
    }

    pub fn contains_method(&mut self, name: &String) -> bool {
        self.methods.contains_key(name)
    }

    pub fn method_named(&self, name: &String) -> &MethodSymbol {
        self.methods.get(name)
            .expect(format!("Could not find method {}", name).as_str())
    }

    pub fn register_method(&mut self, name: String, return_type: DataType, parameters: Vec<DataType>) {
        if self.contains_method(&name) {
            panic!("Method {} already exists", name)
        }

        self.methods.insert(name, MethodSymbol { return_type, parameters });
    }
}

pub struct FieldSymbol {
    field_type: DataType,
}

pub struct MethodSymbol {
    return_type: DataType,
    parameters: Vec<DataType>,
}
impl MethodSymbol {
    pub fn descriptor(&self) -> String {
        let mut descriptor = String::new();
        descriptor.push('(');
        for parameter in &self.parameters {
            parameter.print_descriptor(&mut descriptor);
        }
        descriptor.push(')');
        self.return_type.print_descriptor(&mut descriptor);
        descriptor
    }
}

pub enum DataType {
    BooleanPrimitive,
    BytePrimitive,
    CharPrimitive,
    DoublePrimitive,
    FloatPrimitive,
    IntPrimitive,
    LongPrimitive,
    ShortPrimitive,
    Void,
    Object(String),
    Array(Rc<DataType>),
}
impl DataType {
    fn print_descriptor(&self, output: &mut String) {
        match self {
            DataType::BooleanPrimitive => output.push('Z'),
            DataType::BytePrimitive => output.push('B'),
            DataType::CharPrimitive => output.push('C'),
            DataType::DoublePrimitive => output.push('D'),
            DataType::FloatPrimitive => output.push('F'),
            DataType::IntPrimitive => output.push('I'),
            DataType::LongPrimitive => output.push('J'),
            DataType::ShortPrimitive => output.push('S'),
            DataType::Void => output.push('V'),
            DataType::Object(path) => {
                output.push('L');
                output.push_str(path.replace('.', "/").as_str());
                output.push(';');
            },
            DataType::Array(delegate) => {
                output.push('[');
                delegate.print_descriptor(output);
            }
        }
    }
}
