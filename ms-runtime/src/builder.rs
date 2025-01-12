use std::collections::HashMap;

use crate::{
    version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH},
    ByteCode, Function,
};

pub struct ModuleBuilder {
    pub functions: HashMap<String, Function>,
    current_function: String,
    current_bytecode: Vec<u8>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            functions: HashMap::new(),
            current_function: String::new(),
            current_bytecode: Vec::new(),
        }
    }

    pub fn def(&mut self, name: &str) {
        self.current_function = name.to_string();
        self.current_bytecode = Vec::new();
    }

    pub fn add(&mut self, bytecode: ByteCode) {
        self.current_bytecode.push(bytecode as u8);
    }

    pub fn add_u32(&mut self, value: u32) {
        self.current_bytecode.extend(value.to_be_bytes().iter());
    }

    pub fn add_i32(&mut self, value: i32) {
        self.current_bytecode.extend(value.to_be_bytes().iter());
    }

    pub fn add_f32(&mut self, value: f32) {
        self.current_bytecode.extend(value.to_be_bytes().iter());
    }

    pub fn add_string(&mut self, value: &str) {
        self.add_u32(value.len() as u32);
        self.current_bytecode.extend(value.as_bytes());
    }

    // Call
    pub fn call(&mut self, module: &str, name: &str) {
        self.add(ByteCode::Call);
        self.add_string(module);
        self.add_string(name);
    }

    // PushConstString
    pub fn push_const_string(&mut self, value: &str) {
        self.add(ByteCode::PushConstString);
        self.add_string(value);
    }

    // PushConstInteger
    pub fn push_const_integer(&mut self, value: i32) {
        self.add(ByteCode::PushConstInt);
        self.add_i32(value);
    }

    // PushConstFloat
    pub fn push_const_float(&mut self, value: f32) {
        self.add(ByteCode::PushConstFloat);
        self.add_f32(value);
    }

    // PushArg
    pub fn push_arg(&mut self) {
        self.add(ByteCode::PushArg);
    }

    // PopArg
    pub fn pop_arg(&mut self) {
        self.add(ByteCode::PopArg);
    }

    // Pop
    pub fn pop(&mut self) {
        self.add(ByteCode::Pop);
    }

    // Dup
    pub fn dup(&mut self) {
        self.add(ByteCode::Dup);
    }

    pub fn end(&mut self) {
        self.functions.insert(
            self.current_function.clone(),
            Function::ByteCode {
                name: self.current_function.clone(),
                code: self.current_bytecode.clone(),
            },
        );
    }

    pub fn get_bytecode(&self) -> Vec<u8> {
        let mut bytecode = Vec::new();

        // Module version
        bytecode.push(VERSION_MAJOR as u8);
        bytecode.push(VERSION_MINOR as u8);
        bytecode.push(VERSION_PATCH as u8);

        fn write_u32(value: u32, buffer: &mut Vec<u8>) {
            let bytes = value.to_le_bytes();
            buffer.extend(bytes.iter().rev());
        }

        // Add functions
        for (name, function) in &self.functions {
            match function {
                Function::ByteCode { name: _, code } => {
                    bytecode.push(ByteCode::Func as u8);
                    write_u32(code.len() as u32, &mut bytecode);
                    write_u32(name.len() as u32, &mut bytecode);
                    bytecode.extend(name.as_bytes());
                    bytecode.extend(code.iter().map(|x| *x as u8));
                }
                Function::Native {
                    name: _,
                    function: _,
                } => {
                    panic!("Not implemented");
                }
            }
        }

        bytecode
    }
}
