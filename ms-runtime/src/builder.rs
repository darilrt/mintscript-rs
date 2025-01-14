use std::collections::HashMap;

use crate::{
    byte_writer::ByteWriter,
    version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH},
    ByteCode, Function,
};

pub enum Block {
    Loop(Vec<u8>),
    If(Vec<u8>),
    Else(Vec<u8>),
    Function(String, Vec<u8>),
}

pub struct ModuleBuilder {
    pub functions: HashMap<String, Function>, // Functions in the module
    nested_blocks: Vec<Block>,                // Stack of nested blocks
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            functions: HashMap::new(),
            nested_blocks: Vec::new(),
        }
    }

    pub fn fn_start(&mut self, name: &str) {
        self.nested_blocks
            .push(Block::Function(name.to_string(), Vec::new()));
    }

    pub fn loop_start(&mut self) {
        if self.nested_blocks.is_empty() {
            panic!("No block to add bytecode to");
        }
        self.nested_blocks.push(Block::Loop(Vec::new()));
    }

    pub fn if_start(&mut self) {
        if self.nested_blocks.is_empty() {
            panic!("No block to add bytecode to");
        }
        self.nested_blocks.push(Block::If(Vec::new()));
    }

    pub fn else_start(&mut self) {
        if self.nested_blocks.is_empty() {
            panic!("No block to add bytecode to");
        }
        self.nested_blocks.push(Block::Else(Vec::new()));
    }

    pub fn end(&mut self) {
        let block = self.nested_blocks.pop().unwrap();

        match block {
            Block::Function(name, bytecode) => {
                self.functions.insert(
                    name.clone(),
                    Function::ByteCode {
                        name,
                        code: bytecode,
                    },
                );
            }
            Block::Loop(bytecode) => {
                self.add(ByteCode::Loop);
                self.add_u32(bytecode.len() as u32);

                let mut writer = ByteWriter::new(self.get_current_bytecode());
                writer.write_bytes(&bytecode);
            }
            Block::If(bytecode) => {
                self.add(ByteCode::If);
                self.add_u32(bytecode.len() as u32);

                let mut writer = ByteWriter::new(self.get_current_bytecode());
                writer.write_bytes(&bytecode);
            }
            Block::Else(bytecode) => {
                self.add(ByteCode::Else);
                self.add_u32(bytecode.len() as u32);

                let mut writer = ByteWriter::new(self.get_current_bytecode());
                writer.write_bytes(&bytecode);
            }
        }
    }

    #[inline]
    pub fn get_current_block(&mut self) -> &mut Block {
        match self.nested_blocks.last_mut() {
            Some(block) => block,
            None => {
                panic!("No block to add bytecode to");
            }
        }
    }

    #[inline]
    pub fn get_current_bytecode(&mut self) -> &mut Vec<u8> {
        match self.get_current_block() {
            Block::Function(_, bytecode) => bytecode,
            Block::Loop(bytecode) => bytecode,
            Block::If(bytecode) => bytecode,
            Block::Else(bytecode) => bytecode,
        }
    }

    #[inline]
    pub fn add(&mut self, bytecode: ByteCode) {
        ByteWriter::new(self.get_current_bytecode()).write_byte(bytecode as u8);
    }

    #[inline]
    pub fn add_u32(&mut self, value: u32) {
        ByteWriter::new(self.get_current_bytecode()).write_u32(value);
    }

    #[inline]
    pub fn add_i32(&mut self, value: i32) {
        ByteWriter::new(self.get_current_bytecode()).write_i32(value);
    }

    #[inline]
    pub fn add_f32(&mut self, value: f32) {
        ByteWriter::new(self.get_current_bytecode()).write_f32(value);
    }

    #[inline]
    pub fn add_string(&mut self, value: &str) {
        ByteWriter::new(self.get_current_bytecode()).write_string(value);
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

        builder.fn_start("test");
        builder.none();
        builder.end();

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
