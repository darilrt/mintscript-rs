use std::collections::HashMap;

use crate::module_reader::ByteReader;
use crate::{version::*, ByteCode, Function, Value};

pub struct Module {
    pub functions: HashMap<String, Box<Function>>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            functions: HashMap::new(),
        }
    }

    pub fn from_source(source: &Vec<u8>) -> Result<Module, String> {
        let mut module = Module::new();

        let mut reader = ByteReader::new(source);

        let version = reader.read_bytes(3);

        if version.is_none() {
            return Err("Invalid version".to_string());
        }

        if version
            != Some(vec![
                VERSION_MAJOR as u8,
                VERSION_MINOR as u8,
                VERSION_PATCH as u8,
            ])
        {
            return Err("Invalid version".to_string());
        }

        while let Some(byte) = reader.read_byte() {
            let bytecode = ByteCode::from_u8(byte);

            if bytecode.is_none() {
                return Err(format!("Invalid bytecode: 0x{:02X}", byte));
            }

            match bytecode.unwrap() {
                ByteCode::Func => {
                    // FUNC <length: u32> <name: string> <code: [ByteCode x length]>
                    // Read the length of the function name
                    let length = reader.read_u32();

                    if length.is_none() {
                        return Err("Invalid function length".to_string());
                    }

                    let length = length.unwrap() as usize;

                    // Read the function name
                    let name = reader.read_string();

                    if name.is_none() {
                        return Err("Invalid function name".to_string());
                    }

                    let name = name.unwrap();

                    // Read the function code and convert it to a vector of ByteCode
                    let code = reader.read_bytes(length);

                    if code.is_none() {
                        return Err("Invalid function length".to_string());
                    }

                    // Add the function to the module
                    module.add_function(&name, &code.unwrap());
                }
                _ => {
                    return Err(format!("Invalid bytecode: 0x{:02X}", byte));
                }
            }
        }

        Ok(module)
    }

    pub fn add_function(&mut self, name: &str, code: &Vec<u8>) {
        self.functions.insert(
            name.to_string(),
            Box::new(Function::ByteCode {
                name: name.to_string(),
                code: code.clone(),
            }),
        );
    }

    pub fn add_native_function(&mut self, name: &str, function: Box<dyn Fn(Vec<Value>) -> Value>) {
        self.functions.insert(
            name.to_string(),
            Box::new(Function::Native {
                name: name.to_string(),
                function,
            }),
        );
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name).map(|f| &**f)
    }

    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.get_mut(name).map(|f| &mut **f)
    }
}

#[cfg(test)]
mod tests {
    use crate::Value;

    use super::*;

    #[test]
    fn module_from_source() {
        let code = vec![
            VERSION_MAJOR as u8,
            VERSION_MINOR as u8,
            VERSION_PATCH as u8, // Version
            ByteCode::Func as u8,
            0x00,
            0x00,
            0x00,
            0x01, // Length of the function code
            0x00,
            0x00,
            0x00,
            0x04, // Length of the function name
            b't',
            b'e',
            b's',
            b't',                 // Function name
            ByteCode::Dump as u8, // Function code
        ];

        let module = Module::from_source(&code).unwrap();

        let function = module.get_function("test").unwrap();

        match function {
            Function::ByteCode { name: _, code } => {
                assert_eq!(code.len(), 1);
                assert_eq!(code[0], ByteCode::Dump as u8);
            }
            _ => {
                panic!("Invalid function type");
            }
        }
    }

    #[test]
    fn module_native_function() {
        let mut module = Module::new();

        module.add_native_function("test", Box::new(|_args| Value::Integer(42)));

        let function = module.get_function("test").unwrap();

        match function {
            Function::Native { name: _, function } => {
                let value = function(vec![]);

                match value {
                    Value::Integer(value) => {
                        assert_eq!(value, 42);
                    }
                    _ => {
                        panic!("Invalid return value");
                    }
                }
            }
            _ => {
                panic!("Invalid function type");
            }
        }
    }
}
