use std::hash::Hash;

use crate::{byte_reader::ByteReader, byte_writer::ByteWriter, sexpr::SExpr, ByteCode};

#[derive(Debug, Clone)]
pub enum Instruction {
    None,

    Version {
        major: u8,
        minor: u8,
        patch: u8,
    },

    // Debugging
    Dump,
    Hi,

    // Functions
    Fn {
        name: String,
        code: Code,
    },
    Call {
        module: String,
        function: String,
        param_count: u32,
    },

    // Constants
    PushConstString {
        value: String,
    },
    PushConstInteger {
        value: i32,
    },
    PushConstFloat {
        value: f32,
    },
    PushConstBoolean {
        value: bool,
    },

    // Locals variables
    GetLocal {
        index: u32,
    },
    SetLocal {
        index: u32,
    },
    ReserveLocal {
        size: u32,
    },

    // Objects
    Allocate {
        fields: u32,
    },
    GetField {
        index: u32,
    },
    SetField {
        index: u32,
    },

    // Stack manipulation
    Pop,
    Dup,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Modules
    Module {
        name: String,
        code: Code,
    },

    // Dynamic Module
    LoadModule {
        name: String,
        code: Code,
    },
    GetFunction {
        name: String,
        alias: Option<String>,
    },

    // Control flow
    Return,
    Then {
        then_block: Code,
        else_block: Code,
    },
    Loop {
        block: Code,
    },
    Break,
    Continue,
}

impl PartialEq<Self> for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Instruction::None, Instruction::None) => true,
            (
                Instruction::Version {
                    major: _a,
                    minor: _b,
                    patch: _c,
                },
                Instruction::Version {
                    major: _x,
                    minor: _y,
                    patch: _z,
                },
            ) => true,
            (Instruction::Dump, Instruction::Dump) => true,
            (Instruction::Hi, Instruction::Hi) => true,
            (Instruction::Fn { name: _a, code: _b }, Instruction::Fn { name: _x, code: _y }) => {
                true
            }
            (
                Instruction::Call {
                    module: _a,
                    function: _b,
                    param_count: _c,
                },
                Instruction::Call {
                    module: _x,
                    function: _y,
                    param_count: _z,
                },
            ) => true,
            (
                Instruction::PushConstString { value: _a },
                Instruction::PushConstString { value: _x },
            ) => true,
            (
                Instruction::PushConstInteger { value: _a },
                Instruction::PushConstInteger { value: _x },
            ) => true,
            (
                Instruction::PushConstFloat { value: _a },
                Instruction::PushConstFloat { value: _x },
            ) => true,
            (
                Instruction::PushConstBoolean { value: _a },
                Instruction::PushConstBoolean { value: _x },
            ) => true,
            (Instruction::GetLocal { index: _a }, Instruction::GetLocal { index: _x }) => true,
            (Instruction::SetLocal { index: _a }, Instruction::SetLocal { index: _x }) => true,
            (Instruction::ReserveLocal { size: _a }, Instruction::ReserveLocal { size: _x }) => {
                true
            }
            (Instruction::Allocate { fields: _a }, Instruction::Allocate { fields: _x }) => true,
            (Instruction::GetField { index: _a }, Instruction::GetField { index: _x }) => true,
            (Instruction::SetField { index: _a }, Instruction::SetField { index: _x }) => true,
            (Instruction::Pop, Instruction::Pop) => true,
            (Instruction::Dup, Instruction::Dup) => true,
            (Instruction::Add, Instruction::Add) => true,
            (Instruction::Sub, Instruction::Sub) => true,
            (Instruction::Mul, Instruction::Mul) => true,
            (Instruction::Div, Instruction::Div) => true,
            (Instruction::Inc, Instruction::Inc) => true,
            (Instruction::Dec, Instruction::Dec) => true,
            (Instruction::Eq, Instruction::Eq) => true,
            (Instruction::Ne, Instruction::Ne) => true,
            (Instruction::Lt, Instruction::Lt) => true,
            (Instruction::Le, Instruction::Le) => true,
            (Instruction::Gt, Instruction::Gt) => true,
            (Instruction::Ge, Instruction::Ge) => true,
            (
                Instruction::Module { name: _a, code: _b },
                Instruction::Module { name: _x, code: _y },
            ) => true,
            (
                Instruction::LoadModule { name: _a, code: _b },
                Instruction::LoadModule { name: _x, code: _y },
            ) => true,
            (
                Instruction::GetFunction {
                    name: _a,
                    alias: _b,
                },
                Instruction::GetFunction {
                    name: _x,
                    alias: _y,
                },
            ) => true,
            (Instruction::Return, Instruction::Return) => true,
            (
                Instruction::Then {
                    then_block: _a,
                    else_block: _b,
                },
                Instruction::Then {
                    then_block: _x,
                    else_block: _y,
                },
            ) => true,
            (Instruction::Loop { block: _a }, Instruction::Loop { block: _x }) => true,
            (Instruction::Break, Instruction::Break) => true,
            (Instruction::Continue, Instruction::Continue) => true,
            _ => false,
        }
    }
}

impl Eq for Instruction {}

impl Hash for Instruction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Instruction::None => 0.hash(state),
            Instruction::Version {
                major: _,
                minor: _,
                patch: _,
            } => 1.hash(state),
            Instruction::Dump => 2.hash(state),
            Instruction::Hi => 3.hash(state),
            Instruction::Fn { name: _, code: _ } => 4.hash(state),
            Instruction::Call {
                module: _,
                function: _,
                param_count: _,
            } => 5.hash(state),
            Instruction::PushConstString { value: _ } => 6.hash(state),
            Instruction::PushConstInteger { value: _ } => 7.hash(state),
            Instruction::PushConstFloat { value: _ } => 8.hash(state),
            Instruction::PushConstBoolean { value: _ } => 9.hash(state),
            Instruction::GetLocal { index: _ } => 10.hash(state),
            Instruction::SetLocal { index: _ } => 11.hash(state),
            Instruction::ReserveLocal { size: _ } => 12.hash(state),
            Instruction::Allocate { fields: _ } => 13.hash(state),
            Instruction::GetField { index: _ } => 14.hash(state),
            Instruction::SetField { index: _ } => 15.hash(state),
            Instruction::Pop => 16.hash(state),
            Instruction::Dup => 17.hash(state),
            Instruction::Add => 18.hash(state),
            Instruction::Sub => 19.hash(state),
            Instruction::Mul => 20.hash(state),
            Instruction::Div => 21.hash(state),
            Instruction::Inc => 22.hash(state),
            Instruction::Dec => 23.hash(state),
            Instruction::Eq => 24.hash(state),
            Instruction::Ne => 25.hash(state),
            Instruction::Lt => 26.hash(state),
            Instruction::Le => 27.hash(state),
            Instruction::Gt => 28.hash(state),
            Instruction::Ge => 29.hash(state),
            Instruction::Module { name: _, code: _ } => 30.hash(state),
            Instruction::LoadModule { name: _, code: _ } => 31.hash(state),
            Instruction::GetFunction { name: _, alias: _ } => 32.hash(state),
            Instruction::Return => 33.hash(state),
            Instruction::Then {
                then_block: _,
                else_block: _,
            } => 34.hash(state),
            Instruction::Loop { block: _ } => 35.hash(state),
            Instruction::Break => 36.hash(state),
            Instruction::Continue => 37.hash(state),
        }
    }
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
                    code.push(Instruction::Fn {
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

                    let Some(param_count) = reader.read_u32() else {
                        return Err("Expected parameter count".to_string());
                    };

                    code.push(Instruction::Call {
                        module,
                        function,
                        param_count,
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
                ByteCode::Allocate => {
                    let Some(fields) = reader.read_u32() else {
                        return Err("Expected number of fields".to_string());
                    };

                    code.push(Instruction::Allocate { fields: fields });
                }
                ByteCode::GetField => {
                    let Some(index) = reader.read_u32() else {
                        return Err("Expected field index".to_string());
                    };

                    code.push(Instruction::GetField { index: index });
                }
                ByteCode::SetField => {
                    let Some(index) = reader.read_u32() else {
                        return Err("Expected field index".to_string());
                    };

                    code.push(Instruction::SetField { index: index });
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
                ByteCode::Inc => code.push(Instruction::Inc),
                ByteCode::Dec => code.push(Instruction::Dec),
                ByteCode::Eq => code.push(Instruction::Eq),
                ByteCode::Ne => code.push(Instruction::Ne),
                ByteCode::Lt => code.push(Instruction::Lt),
                ByteCode::Le => code.push(Instruction::Le),
                ByteCode::Gt => code.push(Instruction::Gt),
                ByteCode::Ge => code.push(Instruction::Ge),
                ByteCode::Module => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected module code length".to_string());
                    };

                    let Some(name) = reader.read_string() else {
                        return Err("Expected module name".to_string());
                    };

                    let Some(module_code) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected module code".to_string());
                    };

                    code.push(Instruction::Module {
                        name: name,
                        code: Instruction::from_bytecode(&module_code)?,
                    });
                }
                ByteCode::LoadModule => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected module code length".to_string());
                    };

                    let Some(name) = reader.read_string() else {
                        return Err("Expected module name".to_string());
                    };

                    let Some(module_code) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected module code".to_string());
                    };

                    code.push(Instruction::LoadModule {
                        name: name,
                        code: Instruction::from_bytecode(&module_code)?,
                    });
                }
                ByteCode::GetFunction => {
                    let Some(name) = reader.read_string() else {
                        return Err("Expected function name".to_string());
                    };

                    reader.save_position();
                    if let Some(byte) = reader.read_byte() {
                        if ByteCode::from_u8(byte) == Some(ByteCode::Alias) {
                            let Some(alias) = reader.read_string() else {
                                return Err("Expected alias name".to_string());
                            };

                            code.push(Instruction::GetFunction {
                                name: name,
                                alias: Some(alias),
                            });
                            break;
                        }
                    }

                    reader.restore_position();
                    code.push(Instruction::GetFunction {
                        name: name,
                        alias: None,
                    });
                }
                ByteCode::Alias => {
                    return Err("Invalid instruction (as) outside of function".to_string());
                }
                ByteCode::Return => code.push(Instruction::Return),
                ByteCode::Then => {
                    let Some(lenght) = reader.read_u32() else {
                        return Err("Expected block code length".to_string());
                    };

                    let Some(block) = reader.read_bytes(lenght as usize) else {
                        return Err("Expected block code".to_string());
                    };

                    let then_block = Instruction::from_bytecode(&block)?;
                    let mut else_block = Vec::new();

                    reader.save_position();
                    if let Some(byte) = reader.read_byte() {
                        if ByteCode::from_u8(byte) == Some(ByteCode::Else) {
                            let Some(lenght) = reader.read_u32() else {
                                return Err("Expected block code length".to_string());
                            };

                            let Some(block) = reader.read_bytes(lenght as usize) else {
                                return Err("Expected block code".to_string());
                            };

                            else_block = Instruction::from_bytecode(&block)?;
                        } else {
                            reader.restore_position();
                        }
                    } else {
                        reader.restore_position();
                    }

                    code.push(Instruction::Then {
                        then_block,
                        else_block: else_block,
                    });
                }
                ByteCode::Else => {
                    return Err("Invalid instruction (else) outside of then block".to_string());
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
            Instruction::Fn { name, code } => {
                writer.write_byte(ByteCode::Func as u8);

                let code_bytes = Instruction::code_to_bytes(code);

                writer.write_u32(code_bytes.len() as u32);
                writer.write_string(name);
                writer.write_bytes(&code_bytes);
            }
            Instruction::Call {
                module,
                function,
                param_count,
            } => {
                writer.write_byte(ByteCode::Call as u8);
                writer.write_string(module);
                writer.write_string(function);
                writer.write_u32(*param_count);
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
            Instruction::Allocate { fields } => {
                writer.write_byte(ByteCode::Allocate as u8);
                writer.write_u32(*fields);
            }
            Instruction::GetField { index } => {
                writer.write_byte(ByteCode::GetField as u8);
                writer.write_u32(*index);
            }
            Instruction::SetField { index } => {
                writer.write_byte(ByteCode::SetField as u8);
                writer.write_u32(*index);
            }
            Instruction::Pop => writer.write_byte(ByteCode::Pop as u8),
            Instruction::Dup => writer.write_byte(ByteCode::Dup as u8),
            Instruction::Add => writer.write_byte(ByteCode::Add as u8),
            Instruction::Sub => writer.write_byte(ByteCode::Sub as u8),
            Instruction::Mul => writer.write_byte(ByteCode::Mul as u8),
            Instruction::Div => writer.write_byte(ByteCode::Div as u8),
            Instruction::Inc => writer.write_byte(ByteCode::Inc as u8),
            Instruction::Dec => writer.write_byte(ByteCode::Dec as u8),
            Instruction::Eq => writer.write_byte(ByteCode::Eq as u8),
            Instruction::Ne => writer.write_byte(ByteCode::Ne as u8),
            Instruction::Lt => writer.write_byte(ByteCode::Lt as u8),
            Instruction::Le => writer.write_byte(ByteCode::Le as u8),
            Instruction::Gt => writer.write_byte(ByteCode::Gt as u8),
            Instruction::Ge => writer.write_byte(ByteCode::Ge as u8),
            Instruction::Module { name, code } => {
                writer.write_byte(ByteCode::Module as u8);

                let code_bytes = Instruction::code_to_bytes(code);

                writer.write_u32(code_bytes.len() as u32);
                writer.write_string(name);
                writer.write_bytes(&code_bytes);
            }
            Instruction::LoadModule { name, code } => {
                writer.write_byte(ByteCode::LoadModule as u8);

                let code_bytes = Instruction::code_to_bytes(code);

                writer.write_u32(code_bytes.len() as u32);
                writer.write_string(name);
                writer.write_bytes(&code_bytes);
            }
            Instruction::GetFunction { name, alias } => {
                writer.write_byte(ByteCode::GetFunction as u8);
                writer.write_string(name);

                if let Some(alias) = alias {
                    writer.write_byte(ByteCode::Alias as u8);
                    writer.write_string(alias);
                }
            }
            Instruction::Return => writer.write_byte(ByteCode::Return as u8),
            Instruction::Then {
                then_block,
                else_block,
            } => {
                writer.write_byte(ByteCode::Then as u8);

                let block_bytes = Instruction::code_to_bytes(then_block);

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

    // Convert a S-expression to an instruction
    pub fn from_sexpr(sexpr: &SExpr) -> Result<Instruction, String> {
        match sexpr {
            SExpr::Atom(value) => Err(format!("Unexpected atom: {}", value)),
            SExpr::List(values) => {
                let mut it = values.iter();

                let name = match it.next() {
                    Some(SExpr::Atom(name)) => name,
                    _ => return Err("Expected function name".to_string()),
                };

                match name.as_str() {
                    "version" => {
                        let version = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected major version".to_string()),
                        };

                        let mut version = version.split(".");

                        let major = version.next().unwrap().parse::<u8>().unwrap();
                        let minor = version.next().unwrap().parse::<u8>().unwrap();
                        let patch = version.next().unwrap().parse::<u8>().unwrap();

                        Ok(Instruction::Version {
                            major: major,
                            minor: minor,
                            patch: patch,
                        })
                    }
                    "dump" => Ok(Instruction::Dump),
                    "hi" => Ok(Instruction::Hi),
                    "fn" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected function name".to_string()),
                        };

                        let mut code = Vec::new();

                        while let Some(value) = it.next() {
                            let instruction = Instruction::from_sexpr(value)?;
                            code.push(instruction);
                        }

                        Ok(Instruction::Fn {
                            name: name.to_string(),
                            code,
                        })
                    }
                    "call" => {
                        let module = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected module name".to_string()),
                        };

                        let function = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected function name".to_string()),
                        };

                        let param_count = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected parameter count".to_string()),
                        };

                        Ok(Instruction::Call {
                            module: module.to_string(),
                            function: function.to_string(),
                            param_count,
                        })
                    }
                    "str.const" => {
                        let value = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected string value".to_string()),
                        };

                        Ok(Instruction::PushConstString {
                            value: value.to_string(),
                        })
                    }
                    "i32.const" => {
                        let value = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<i32>().unwrap(),
                            _ => return Err("Expected integer value".to_string()),
                        };

                        Ok(Instruction::PushConstInteger { value })
                    }
                    "f32.const" => {
                        let value = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<f32>().unwrap(),
                            _ => return Err("Expected float value".to_string()),
                        };

                        Ok(Instruction::PushConstFloat { value })
                    }
                    "bool.const" => {
                        let value = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<bool>().unwrap(),
                            _ => return Err("Expected boolean value".to_string()),
                        };

                        Ok(Instruction::PushConstBoolean { value })
                    }
                    "local.get" => {
                        let index = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected local index".to_string()),
                        };

                        Ok(Instruction::GetLocal { index })
                    }
                    "local.set" => {
                        let index = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected local index".to_string()),
                        };

                        Ok(Instruction::SetLocal { index })
                    }
                    "local.reserve" => {
                        let size = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected local size".to_string()),
                        };

                        Ok(Instruction::ReserveLocal { size })
                    }
                    "alloc" => {
                        let fields = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected number of fields".to_string()),
                        };

                        Ok(Instruction::Allocate { fields })
                    }
                    "field.get" => {
                        let index = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected field index".to_string()),
                        };

                        Ok(Instruction::GetField { index })
                    }
                    "field.set" => {
                        let index = match it.next() {
                            Some(SExpr::Atom(value)) => value.parse::<u32>().unwrap(),
                            _ => return Err("Expected field index".to_string()),
                        };

                        Ok(Instruction::SetField { index })
                    }
                    "pop" => Ok(Instruction::Pop),
                    "dup" => Ok(Instruction::Dup),
                    "op.add" => Ok(Instruction::Add),
                    "op.sub" => Ok(Instruction::Sub),
                    "op.mul" => Ok(Instruction::Mul),
                    "op.div" => Ok(Instruction::Div),
                    "op.inc" => Ok(Instruction::Inc),
                    "op.dec" => Ok(Instruction::Dec),
                    "cmp.eq" => Ok(Instruction::Eq),
                    "cmp.ne" => Ok(Instruction::Ne),
                    "cmp.lt" => Ok(Instruction::Lt),
                    "cmp.le" => Ok(Instruction::Le),
                    "cmp.gt" => Ok(Instruction::Gt),
                    "cmp.ge" => Ok(Instruction::Ge),
                    "mod" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected module name".to_string()),
                        };

                        let mut module_code = Vec::new();

                        while let Some(value) = it.next() {
                            let instruction = Instruction::from_sexpr(value)?;
                            module_code.push(instruction);
                        }

                        Ok(Instruction::Module {
                            name: name.to_string(),
                            code: module_code,
                        })
                    }
                    "mod.load" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected module name".to_string()),
                        };

                        let mut module_code = Vec::new();

                        while let Some(value) = it.next() {
                            let instruction = Instruction::from_sexpr(value)?;
                            module_code.push(instruction);
                        }

                        Ok(Instruction::LoadModule {
                            name: name.to_string(),
                            code: module_code,
                        })
                    }
                    "fn.get" => {
                        let name = match it.next() {
                            Some(SExpr::Atom(value)) => value,
                            _ => return Err("Expected function name".to_string()),
                        };

                        let mut alias = None;

                        if let Some(SExpr::Atom(_)) = it.next() {
                            let alias_ = match it.next() {
                                Some(SExpr::Atom(value)) => value,
                                _ => return Err("Expected alias name".to_string()),
                            };

                            alias = Some(alias_.to_string());
                        }

                        Ok(Instruction::GetFunction {
                            name: name.to_string(),
                            alias,
                        })
                    }
                    "return" => Ok(Instruction::Return),
                    "then" => {
                        let mut then_block = Vec::new();
                        let mut else_block = Vec::new();

                        let mut has_else = false;

                        while let Some(value) = it.next() {
                            match value {
                                SExpr::Atom(value) => {
                                    if value == "else" {
                                        has_else = true;
                                        break;
                                    } else {
                                        return Err("Unexpected atom".to_string());
                                    }
                                }
                                SExpr::List(_) => {
                                    let instruction = Instruction::from_sexpr(value)?;
                                    then_block.push(instruction);
                                }
                            }
                        }

                        if has_else {
                            while let Some(value) = it.next() {
                                match value {
                                    SExpr::List(_) => {
                                        let instruction = Instruction::from_sexpr(value)?;
                                        else_block.push(instruction);
                                    }
                                    _ => return Err("Unexpected atom".to_string()),
                                }
                            }
                        }

                        Ok(Instruction::Then {
                            then_block,
                            else_block: else_block,
                        })
                    }
                    "loop" => {
                        let mut block = Vec::new();

                        while let Some(value) = it.next() {
                            match value {
                                SExpr::List(_) => {
                                    let instruction = Instruction::from_sexpr(value)?;
                                    block.push(instruction);
                                }
                                _ => return Err("Unexpected atom".to_string()),
                            }
                        }

                        Ok(Instruction::Loop { block: block })
                    }
                    "break" => Ok(Instruction::Break),
                    "continue" => Ok(Instruction::Continue),
                    _ => Err(format!("Unknown instruction: {}", name)),
                }
            }
        }
    }

    // Convert a vector of S-expressions to a vector of instructions
    pub fn from_sexprs(sexprs: &Vec<SExpr>) -> Result<Code, String> {
        let mut code = Vec::new();

        for sexpr in sexprs.iter() {
            let inst = Instruction::from_sexpr(sexpr)?;
            code.push(inst);
        }

        Ok(code)
    }
}
