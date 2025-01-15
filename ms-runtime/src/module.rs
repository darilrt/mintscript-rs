use std::collections::HashMap;

use crate::instruction::{Code, Instruction};
use crate::{version::*, Function, Value};

pub struct Module {
    pub functions: HashMap<String, Box<Function>>,
}

impl TryFrom<Vec<u8>> for Module {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Module::try_from(Instruction::from_bytecode(&value).map_err(|e| e.to_string())?)
    }
}

impl TryFrom<Vec<Instruction>> for Module {
    type Error = String;

    fn try_from(value: Vec<Instruction>) -> Result<Self, Self::Error> {
        let mut module = Module::new();

        let Some(version) = value.get(0) else {
            return Err("Invalid version".to_string());
        };

        match version {
            Instruction::Version {
                major,
                minor,
                patch,
            } => {
                if *major != VERSION_MAJOR || *minor != VERSION_MINOR || *patch != VERSION_PATCH {
                    return Err("Invalid version".to_string());
                }
            }
            _ => {
                return Err("Invalid version".to_string());
            }
        }

        let value = &value[1..];

        for instruction in value.iter() {
            match instruction {
                Instruction::Func { name, code } => {
                    module.add_function(name.to_string(), code);
                }
                _ => {
                    panic!("Invalid instruction type");
                }
            }
        }

        Ok(module)
    }
}

impl Module {
    pub fn new() -> Module {
        Module {
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, code: &Code) {
        self.functions.insert(
            name.to_string(),
            Box::new(Function::Code {
                name: name,
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
    use crate::{ByteCode, Value};

    use super::*;

    #[test]
    fn module_from_source() {
        let code = vec![
            Instruction::Version {
                major: VERSION_MAJOR,
                minor: VERSION_MINOR,
                patch: VERSION_PATCH,
            },
            Instruction::Func {
                name: "test".to_string(),
                code: vec![],
            },
        ];

        let _module = Module::try_from(code).unwrap();
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
