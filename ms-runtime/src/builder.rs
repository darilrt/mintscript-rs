use std::collections::HashMap;

use crate::{
    byte_writer::ByteWriter,
    bytecode,
    version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH},
    ByteCode, Function,
};

pub struct ModuleBuilder {
    pub functions: HashMap<String, Function>, // Functions in the module
    current_function: String,                 // Name of the current function
    current_bytecode: Vec<u8>,                // Bytecode of the current function
    loop_stack: Vec<Vec<u8>>, // Stack of loops (used to store the bytecode of the loop)
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            functions: HashMap::new(),
            current_function: String::new(),
            current_bytecode: Vec::new(),
            loop_stack: Vec::new(),
        }
    }

    pub fn function(&mut self, name: &str) {
        self.current_function = name.to_string();
        self.current_bytecode = Vec::new();
    }

    #[inline]
    pub fn add(&mut self, bytecode: ByteCode) {
        ByteWriter::new(&mut self.current_bytecode).write_byte(bytecode as u8);
    }

    #[inline]
    pub fn add_u32(&mut self, value: u32) {
        ByteWriter::new(&mut self.current_bytecode).write_u32(value);
    }

    #[inline]
    pub fn add_i32(&mut self, value: i32) {
        ByteWriter::new(&mut self.current_bytecode).write_i32(value);
    }

    #[inline]
    pub fn add_f32(&mut self, value: f32) {
        ByteWriter::new(&mut self.current_bytecode).write_f32(value);
    }

    #[inline]
    pub fn add_string(&mut self, value: &str) {
        ByteWriter::new(&mut self.current_bytecode).write_string(value);
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

    pub fn none(&mut self) {
        self.add(ByteCode::None);
    }

    pub fn end_function(&mut self) {
        self.functions.insert(
            self.current_function.clone(),
            Function::ByteCode {
                name: self.current_function.clone(),
                code: self.current_bytecode.clone(),
            },
        );
    }

    pub fn loop_start(&mut self) {
        self.loop_stack.push(Vec::new());
    }

    // Copy the loop bytecode to the top loop on the stack
    // if there is no loop on the stack then add the bytecode to the current function
    pub fn end_loop(&mut self) {
        let loop_bytecode = self.loop_stack.pop();

        if let Some(loop_bytecode) = loop_bytecode {
            let mut bytecode: Vec<u8> = Vec::new();
            let mut writer = ByteWriter::new(&mut bytecode);

            // Loop header
            writer.write_byte(ByteCode::Loop as u8);
            writer.write_u32(loop_bytecode.len() as u32);

            // Loop bytecode
            writer.write_bytes(&loop_bytecode);

            // Check if there is a loop on the stack
            if let Some(mut loop_stack) = self.loop_stack.pop() {
                loop_stack.extend(bytecode);
                self.loop_stack.push(loop_stack);
            } else {
                self.current_bytecode.extend(bytecode);
            }
        } else {
            panic!("No loop to end");
        }
    }

    pub fn get_bytecode(&self) -> Vec<u8> {
        let mut bytecode = Vec::new();
        let mut writer = ByteWriter::new(&mut bytecode);

        // Module version
        writer.write_byte(VERSION_MAJOR as u8);
        writer.write_byte(VERSION_MINOR as u8);
        writer.write_byte(VERSION_PATCH as u8);

        // Add functions
        for (name, function) in &self.functions {
            match function {
                Function::ByteCode { name: _, code } => {
                    writer.write_byte(ByteCode::Func as u8);
                    writer.write_u32(code.len() as u32);
                    writer.write_string(&name);
                    writer.write_bytes(code);
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

#[cfg(test)]
mod tests {
    use crate::byte_reader::ByteReader;

    use super::*;

    #[test]
    fn test_module_builder_function() {
        let mut builder = ModuleBuilder::new();

        builder.function("test");
        builder.none();
        builder.end_function();

        let bytecode = builder.get_bytecode();

        let mut reader = ByteReader::new(&bytecode);

        // Check the version
        assert_eq!(reader.read_byte(), Some(VERSION_MAJOR as u8));
        assert_eq!(reader.read_byte(), Some(VERSION_MINOR as u8));
        assert_eq!(reader.read_byte(), Some(VERSION_PATCH as u8));

        // Check the function
        assert_eq!(reader.read_byte(), Some(ByteCode::Func as u8));
        assert_eq!(reader.read_u32(), Some(1));
        assert_eq!(reader.read_string(), Some("test".to_string()));

        // Check the bytecode
        assert_eq!(reader.read_byte(), Some(ByteCode::None as u8));
    }
}
