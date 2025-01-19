use std::collections::HashMap;

use crate::instruction::{Code, Instruction};
use crate::Function;

pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Box<Function>>,
}

impl TryFrom<Instruction> for Module {
    type Error = String;

    fn try_from(value: Instruction) -> Result<Self, Self::Error> {
        let mut module;

        match value {
            Instruction::Module { name, code } => {
                module = Module::new(&name);

                for instruction in code.iter() {
                    match instruction {
                        Instruction::Fn { name, code } => {
                            module.add_function(name.to_string(), code);
                        }
                        _ => {
                            return Err("Invalid instruction type, expected (fn)".to_string());
                        }
                    }
                }
            }
            _ => {
                return Err("Invalid instruction type, expected (mod)".to_string());
            }
        }

        Ok(module)
    }
}

impl Module {
    pub fn new(name: &str) -> Module {
        Module {
            name: name.to_string(),
            functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, code: &Code) {
        self.functions.insert(
            name.to_string(),
            Box::new(Function {
                name: name,
                code: code.clone(),
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
