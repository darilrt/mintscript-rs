use core::panic;
use std::collections::HashMap;

use crate::{byte_reader::ByteReader, module::Module, ByteCode, Value};

pub struct VirtualMachine {
    stack: Vec<Value>,
    arg_stack: Vec<Value>,
    modules: HashMap<String, Module>,
    call_break: bool,
    call_continue: bool,
    call_return: bool,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            arg_stack: Vec::new(),
            modules: HashMap::new(),
            call_break: false,
            call_continue: false,
            call_return: false,
        }
    }

    pub fn load_module(&mut self, name: &str, module: Module) {
        self.modules.insert(name.to_string(), module);
    }

    pub fn execute(&mut self, code: &Vec<u8>) -> Value {
        let mut reader = ByteReader::new(&code);

        self.call_break = false;
        self.call_continue = false;
        self.call_return = false;

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
                ByteCode::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(a + b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a + b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Sub => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(a - b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a - b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Mul => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(a * b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a * b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Div => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(a / b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a / b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Eq => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a == b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a == b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Ne => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a != b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a != b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Lt => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a < b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a < b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Le => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a <= b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a <= b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Gt => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a > b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a > b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Ge => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(if a >= b { 1 } else { 0 }));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Integer(if a >= b { 1 } else { 0 }));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                ByteCode::Ret => {
                    self.call_return = true;
                    return self.stack.pop().unwrap();
                }
                ByteCode::If => {
                    let length = reader.read_u32().unwrap() as usize; // Read the length of the block
                    let block = reader.read_bytes(length).unwrap(); // Read the block

                    let condition = self.stack.pop().unwrap();

                    if let Value::Integer(1) = condition {
                        self.execute(&block);
                    } else {
                        reader.jump(length);
                    }
                }
                ByteCode::Else => {
                    let length = reader.read_u32().unwrap() as usize; // Read the length of the block
                    let block = reader.read_bytes(length).unwrap(); // Read the block

                    if let Value::Integer(0) = self.stack.pop().unwrap() {
                        self.execute(&block);
                    } else {
                        reader.jump(length);
                    }
                }
                ByteCode::Break => {
                    return Value::Null;
                }
                ByteCode::Continue => {
                    return Value::Null;
                }
                ByteCode::Loop => {
                    let length = reader.read_u32().unwrap() as usize; // Read the length of the block
                    let block = reader.read_bytes(length).unwrap(); // Read the block

                    loop {
                        let result = self.execute(&block);

                        if self.call_return {
                            self.call_return = false;
                            return result;
                        }

                        if self.call_continue {
                            self.call_continue = false;
                            continue;
                        }

                        if self.call_break {
                            self.call_break = false;
                            break;
                        }
                    }
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
                    let code = code.clone();
                    return self.execute(&code);
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
