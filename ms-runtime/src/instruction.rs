use crate::{byte_reader::ByteReader, byte_writer::ByteWriter, ByteCode};

#[derive(Debug, Clone)]
pub enum Instruction {
    None,

    Version { major: u8, minor: u8, patch: u8 },

    // Debugging
    Dump,
    Hi,

    // Functions
    Func { name: String, code: Code },
    Call { module: String, function: String },

    // Constants
    PushConstString { value: String },
    PushConstInteger { value: i32 },
    PushConstFloat { value: f32 },
    PushConstBoolean { value: bool },

    // Locals variables
    GetLocal { index: u32 },
    SetLocal { index: u32 },
    ReserveLocal { size: u32 },

    // Stack manipulation
    Pop,
    Dup,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Control flow
    Return,
    If { if_block: Code, else_block: Code },
    Loop { block: Code },
    Break,
    Continue,
}

pub type Code = Vec<Instruction>;

impl<'a> Instruction {
    pub fn from_bytecode(bytecode: &'a Vec<u8>) -> Result<Code, String> {
        let mut code = Vec::new();
        let mut reader = ByteReader::new(bytecode);

        while let Some(byte) = reader.read_byte() {
            let Some(byte) = ByteCode::from_u8(byte) else {
                return Err(format!("Invalid instruction: 0x{:02X}", byte));
            };

            match byte {
                ByteCode::None => code.push(Instruction::None),
                ByteCode::Version => {
                    let Some(major) = reader.read_byte() else {
                        return Err("Expected major version".to_string());
                    };

                    let Some(minor) = reader.read_byte() else {
                        return Err("Expected minor version".to_string());
                    };

                    let Some(patch) = reader.read_byte() else {
                        return Err("Expected patch version".to_string());
                    };

                    code.push(Instruction::Version {
                        major: major,
                        minor: minor,
                        patch: patch,
                    });
                }
                ByteCode::Dump => code.push(Instruction::Dump),
                ByteCode::Hi => code.push(Instruction::Hi),
                ByteCode::Func => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected function code length".to_string());
                    };

                    let Some(name) = reader.read_string() else {
                        return Err("Expected function name".to_string());
                    };

                    let Some(fn_code) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected function code".to_string());
                    };

                    // This can be done in multy threads
                    code.push(Instruction::Func {
                        name: name,
                        code: Instruction::from_bytecode(&fn_code)?,
                    });
                }
                ByteCode::Call => {
                    let Some(module) = reader.read_string() else {
                        return Err("Expected module name".to_string());
                    };

                    let Some(function) = reader.read_string() else {
                        return Err("Expected function name".to_string());
                    };

                    code.push(Instruction::Call {
                        module: module,
                        function: function,
                    });
                }
                ByteCode::PushConstString => {
                    let Some(value) = reader.read_string() else {
                        return Err("Expected string value".to_string());
                    };

                    code.push(Instruction::PushConstString { value: value });
                }
                ByteCode::PushConstInteger => {
                    let Some(value) = reader.read_i32() else {
                        return Err("Expected integer value".to_string());
                    };

                    code.push(Instruction::PushConstInteger { value: value });
                }
                ByteCode::PushConstFloat => {
                    let Some(value) = reader.read_f32() else {
                        return Err("Expected float value".to_string());
                    };

                    code.push(Instruction::PushConstFloat { value: value });
                }
                ByteCode::PushConstBoolean => {
                    let Some(value) = reader.read_bool() else {
                        return Err("Expected boolean value".to_string());
                    };

                    code.push(Instruction::PushConstBoolean { value: value });
                }
                ByteCode::GetLocal => {
                    let Some(index) = reader.read_u32() else {
                        return Err("Expected local index".to_string());
                    };

                    code.push(Instruction::GetLocal { index: index });
                }
                ByteCode::SetLocal => {
                    let Some(index) = reader.read_u32() else {
                        return Err("Expected local index".to_string());
                    };

                    code.push(Instruction::SetLocal { index: index });
                }
                ByteCode::ReserveLocal => {
                    let Some(index) = reader.read_u32() else {
                        return Err("Expected local size".to_string());
                    };

                    code.push(Instruction::ReserveLocal { size: index });
                }
                ByteCode::Pop => code.push(Instruction::Pop),
                ByteCode::Dup => code.push(Instruction::Dup),
                ByteCode::Add => code.push(Instruction::Add),
                ByteCode::Sub => code.push(Instruction::Sub),
                ByteCode::Mul => code.push(Instruction::Mul),
                ByteCode::Div => code.push(Instruction::Div),
                ByteCode::Eq => code.push(Instruction::Eq),
                ByteCode::Ne => code.push(Instruction::Ne),
                ByteCode::Lt => code.push(Instruction::Lt),
                ByteCode::Le => code.push(Instruction::Le),
                ByteCode::Gt => code.push(Instruction::Gt),
                ByteCode::Ge => code.push(Instruction::Ge),
                ByteCode::Return => code.push(Instruction::Return),
                ByteCode::If => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected block code length".to_string());
                    };

                    let Some(block) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected block code".to_string());
                    };

                    let if_block = Instruction::from_bytecode(&block)?;
                    let mut else_block = Vec::new();

                    if let Some(byte) = reader.read_byte() {
                        if ByteCode::from_u8(byte) == Some(ByteCode::Else) {
                            let Some(lenght) = reader.read_u32() else {
                                return Err("Expected block code length".to_string());
                            };

                            let Some(block) = reader.read_bytes(lenght as usize) else {
                                return Err("Expected block code".to_string());
                            };

                            else_block = Instruction::from_bytecode(&block)?;
                        }
                    }

                    code.push(Instruction::If {
                        if_block: if_block,
                        else_block: else_block,
                    });
                }
                ByteCode::Else => {
                    panic!("Invalid instruction ELSE without IF");
                }
                ByteCode::Loop => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected block code length".to_string());
                    };

                    let Some(block) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected block code".to_string());
                    };

                    code.push(Instruction::Loop {
                        block: Instruction::from_bytecode(&block)?,
                    });
                }
                ByteCode::Break => code.push(Instruction::Break),
                ByteCode::Continue => code.push(Instruction::Continue),
            }
        }
        Ok(code)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut writer = ByteWriter::new(&mut bytes);

        match self {
            Instruction::None => writer.write_byte(ByteCode::None as u8),
            Instruction::Version {
                major,
                minor,
                patch,
            } => {
                writer.write_byte(ByteCode::Version as u8);
                writer.write_byte(*major);
                writer.write_byte(*minor);
                writer.write_byte(*patch);
            }
            Instruction::Dump => writer.write_byte(ByteCode::Dump as u8),
            Instruction::Hi => writer.write_byte(ByteCode::Hi as u8),
            Instruction::Func { name, code } => {
                writer.write_byte(ByteCode::Func as u8);

                let code_bytes = Instruction::code_to_bytes(code);

                writer.write_u32(code_bytes.len() as u32);
                writer.write_string(name);
                writer.write_bytes(&code_bytes);
            }
            Instruction::Call { module, function } => {
                writer.write_byte(ByteCode::Call as u8);
                writer.write_string(module);
                writer.write_string(function);
            }
            Instruction::PushConstString { value } => {
                writer.write_byte(ByteCode::PushConstString as u8);
                writer.write_string(value);
            }
            Instruction::PushConstInteger { value } => {
                writer.write_byte(ByteCode::PushConstInteger as u8);
                writer.write_i32(*value);
            }
            Instruction::PushConstFloat { value } => {
                writer.write_byte(ByteCode::PushConstFloat as u8);
                writer.write_f32(*value);
            }
            Instruction::PushConstBoolean { value } => {
                writer.write_byte(ByteCode::PushConstBoolean as u8);
                writer.write_bool(*value);
            }
            Instruction::GetLocal { index } => {
                writer.write_byte(ByteCode::GetLocal as u8);
                writer.write_u32(*index);
            }
            Instruction::SetLocal { index } => {
                writer.write_byte(ByteCode::SetLocal as u8);
                writer.write_u32(*index);
            }
            Instruction::ReserveLocal { size: index } => {
                writer.write_byte(ByteCode::ReserveLocal as u8);
                writer.write_u32(*index);
            }
            Instruction::Pop => writer.write_byte(ByteCode::Pop as u8),
            Instruction::Dup => writer.write_byte(ByteCode::Dup as u8),
            Instruction::Add => writer.write_byte(ByteCode::Add as u8),
            Instruction::Sub => writer.write_byte(ByteCode::Sub as u8),
            Instruction::Mul => writer.write_byte(ByteCode::Mul as u8),
            Instruction::Div => writer.write_byte(ByteCode::Div as u8),
            Instruction::Eq => writer.write_byte(ByteCode::Eq as u8),
            Instruction::Ne => writer.write_byte(ByteCode::Ne as u8),
            Instruction::Lt => writer.write_byte(ByteCode::Lt as u8),
            Instruction::Le => writer.write_byte(ByteCode::Le as u8),
            Instruction::Gt => writer.write_byte(ByteCode::Gt as u8),
            Instruction::Ge => writer.write_byte(ByteCode::Ge as u8),
            Instruction::Return => writer.write_byte(ByteCode::Return as u8),
            Instruction::If {
                if_block,
                else_block,
            } => {
                writer.write_byte(ByteCode::If as u8);

                let block_bytes = Instruction::code_to_bytes(if_block);

                writer.write_u32(block_bytes.len() as u32);
                writer.write_bytes(&block_bytes);

                if else_block.len() > 0 {
                    writer.write_byte(ByteCode::Else as u8);

                    let block_bytes = Instruction::code_to_bytes(else_block);

                    writer.write_u32(block_bytes.len() as u32);
                    writer.write_bytes(&block_bytes);
                }
            }
            Instruction::Loop { block } => {
                writer.write_byte(ByteCode::Loop as u8);

                let block_bytes = Instruction::code_to_bytes(block);

                writer.write_u32(block_bytes.len() as u32);
                writer.write_bytes(&block_bytes);
            }
            Instruction::Break => writer.write_byte(ByteCode::Break as u8),
            Instruction::Continue => writer.write_byte(ByteCode::Continue as u8),
        }

        bytes
    }

    // Convert a vector of instructions to a vector of bytes
    pub fn code_to_bytes(code: &Code) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut writer = ByteWriter::new(&mut bytes);

        for instruction in code.iter() {
            writer.write_bytes(&instruction.to_bytes());
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_to_bytes() {
        let code = vec![
            Instruction::PushConstInteger { value: 42 },
            Instruction::PushConstFloat { value: 3.14 },
            Instruction::Add,
            Instruction::Return,
        ];

        let bytes = Instruction::code_to_bytes(&code);

        let mut reader = ByteReader::new(&bytes);

        assert_eq!(
            reader.read_byte().unwrap(),
            ByteCode::PushConstInteger as u8
        );
        assert_eq!(reader.read_i32().unwrap(), 42);

        assert_eq!(reader.read_byte().unwrap(), ByteCode::PushConstFloat as u8);
        assert_eq!(reader.read_f32().unwrap(), 3.14);

        assert_eq!(reader.read_byte().unwrap(), ByteCode::Add as u8);

        assert_eq!(reader.read_byte().unwrap(), ByteCode::Return as u8);
    }

    #[test]
    fn instruction_from_bytes() {
        let bytes = vec![
            ByteCode::PushConstInteger as u8,
            0x00,
            0x00,
            0x00,
            42,
            ByteCode::PushConstFloat as u8,
            0x40,
            0x48,
            0xf5,
            0xc3,
            ByteCode::Add as u8,
            ByteCode::Return as u8,
        ];

        let code = Instruction::from_bytecode(&bytes).unwrap();

        assert_eq!(code.len(), 4);

        match &code[0] {
            Instruction::PushConstInteger { value } => assert_eq!(*value, 42),
            _ => panic!("Invalid instruction"),
        }

        match &code[1] {
            Instruction::PushConstFloat { value } => assert_eq!(*value, 3.14),
            _ => panic!("Invalid instruction"),
        }

        match &code[2] {
            Instruction::Add => {}
            _ => panic!("Invalid instruction"),
        }

        match &code[3] {
            Instruction::Return => {}
            _ => panic!("Invalid instruction"),
        }
    }
}
