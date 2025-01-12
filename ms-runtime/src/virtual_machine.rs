use core::panic;
use std::collections::HashMap;

use crate::{module::Module, module_reader::ByteReader, ByteCode, Value};

pub struct VirtualMachine {
    stack: Vec<Value>,
    arg_stack: Vec<Value>,
    modules: HashMap<String, Module>,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            arg_stack: Vec::new(),
            modules: HashMap::new(),
        }
    }

    pub fn load_module(&mut self, name: &str, module: Module) {
        self.modules.insert(name.to_string(), module);
    }

    pub fn execute(&mut self, code: Vec<u8>) -> Value {
        let mut reader = ByteReader::new(&code);

        while let Some(byte) = reader.read_byte() {
            let bytecode = ByteCode::from_u8(byte);

            if bytecode.is_none() {
                panic!("Invalid bytecode: 0x{:02X}", byte);
            }

            match bytecode.unwrap() {
                ByteCode::None => {}
                ByteCode::Hi => {
                    println!("Hi!");
                }
                ByteCode::Dump => {
                    println!("{:?}", self.stack);
                }
                ByteCode::Func => {
                    panic!("Function not expected");
                }
                ByteCode::Call => {
                    let module = reader.read_string().unwrap();
                    let name = reader.read_string().unwrap();

                    let args = self.arg_stack.clone();

                    let result = self.call(&module, &name, args);

                    self.stack.push(result);

                    self.arg_stack.clear();
                }
                ByteCode::PushConstString => {
                    let string = reader.read_string().unwrap();

                    self.stack.push(Value::String(string));
                }
                ByteCode::PushConstInt => {
                    let value = reader.read_i32().unwrap();

                    self.stack.push(Value::Integer(value));
                }
                ByteCode::PushConstFloat => {
                    let value = reader.read_f32().unwrap();

                    self.stack.push(Value::Float(value));
                }
                ByteCode::PushArg => {
                    let value = self.stack.pop().unwrap();

                    self.arg_stack.push(value);
                }
                ByteCode::PopArg => {
                    let value = self.arg_stack.pop().unwrap();

                    self.stack.push(value);
                }
                ByteCode::Pop => {
                    self.stack.pop();
                }
                ByteCode::Dup => {
                    let value = self.stack.last().unwrap().clone();

                    self.stack.push(value);
                }
            }
        }

        Value::Integer(0)
    }

    pub fn call(&mut self, module: &str, name: &str, args: Vec<Value>) -> Value {
        let module = self.modules.get(module).unwrap();

        if let Some(function) = module.get_function(name) {
            match function {
                crate::Function::ByteCode { name: _, code } => {
                    return self.execute(code.clone());
                }
                crate::Function::Native { name: _, function } => {
                    return function(args);
                }
            }
        } else {
            panic!("Function not found");
        }
    }
}
