use core::panic;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    instruction::{Code, Instruction},
    module::Module,
    Object, Value,
};

pub struct VirtualMachine {
    stack: Vec<Value>,
    modules: HashMap<String, Module>,
    local_vars: Vec<Vec<Value>>,
    call_break: bool,
    call_continue: bool,
    call_return: bool,
}

impl<'a> VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            modules: HashMap::new(),
            local_vars: Vec::new(),
            call_break: false,
            call_continue: false,
            call_return: false,
        }
    }

    pub fn load_module(&mut self, name: &str, module: Module) {
        self.modules.insert(name.to_string(), module);
    }

    pub fn execute(&mut self, code: &'a Code) {
        self.call_break = false;
        self.call_continue = false;
        self.call_return = false;

        for instruction in code.iter() {
            match instruction {
                Instruction::None => {}
                Instruction::Version {
                    major: _,
                    minor: _,
                    patch: _,
                } => {}
                Instruction::Dump => {
                    // Show the stack and locals
                    println!("Stack: {:?}", self.stack);
                    println!("Locals: {:?}", self.local_vars);
                }
                Instruction::Hi => {
                    println!("Hi!");
                }
                Instruction::Fn { name: _, code: _ } => {
                    panic!("Function call not allowed here");
                }
                Instruction::Call {
                    module,
                    function,
                    param_count,
                } => {
                    let args = self
                        .stack
                        .split_off(self.stack.len() - *param_count as usize);

                    self.call(module, function, args);
                }
                Instruction::PushConstString { value } => {
                    self.stack.push(Value::String(value.clone()));
                }
                Instruction::PushConstInteger { value } => {
                    self.stack.push(Value::Integer(*value));
                }
                Instruction::PushConstFloat { value } => {
                    self.stack.push(Value::Float(*value));
                }
                Instruction::PushConstBoolean { value } => {
                    self.stack.push(Value::Boolean(*value));
                }
                Instruction::GetLocal { index } => {
                    if let Some(value) = self.local_vars.last() {
                        if let Some(value) = value.get(*index as usize) {
                            self.stack.push(value.clone());
                        } else {
                            panic!("Local variable not found");
                        }
                    } else {
                        panic!("Local variable not found");
                    }
                }
                Instruction::SetLocal { index } => {
                    if let Some(value) = self.stack.pop() {
                        if let Some(locals) = self.local_vars.last_mut() {
                            locals[*index as usize] = value.clone();
                        } else {
                            panic!("Local variable not found");
                        }
                    } else {
                        panic!("Local variable not found");
                    }
                }
                Instruction::ReserveLocal { size } => {
                    if let Some(locals) = self.local_vars.last_mut() {
                        locals.resize(*size as usize, Value::Null);
                    } else {
                        panic!("Local variable not found");
                    }
                }
                Instruction::Allocate { fields } => {
                    let fields = vec![Value::Null; *fields as usize];
                    self.stack
                        .push(Value::Object(Arc::new(Mutex::new(Object::Values(fields)))));
                }
                Instruction::GetField { index } => {
                    let Some(object) = self.stack.pop() else {
                        panic!("No elements in the stack expected an object");
                    };

                    if let Value::Object(object) = object {
                        let lock = object.lock();
                        let object = lock.as_deref().unwrap();

                        if let Object::Values(fields) = object {
                            if let Some(value) = fields.get(*index as usize) {
                                self.stack.push(value.clone());
                            } else {
                                panic!("Field not found");
                            }
                        } else {
                            panic!("Expected an object with fields");
                        }
                    } else {
                        panic!("Expected an object");
                    }
                }
                Instruction::SetField { index } => {
                    let Some(value) = self.stack.pop() else {
                        panic!("No elements in the stack expected a value");
                    };
                    let Some(object) = self.stack.last() else {
                        panic!("No elements in the stack expected an object");
                    };

                    if let Value::Object(object) = object {
                        let mut lock = object.lock();
                        let object = lock.as_deref_mut().unwrap();

                        match object {
                            Object::Values(fields) => {
                                if let Some(_) = fields.get(*index as usize) {
                                    fields[*index as usize] = value.clone();
                                } else {
                                    panic!("Field not found");
                                }
                            }
                            _ => {
                                panic!("Expected an object with fields");
                            }
                        }
                    } else {
                        panic!("Expected an object");
                    }
                }
                Instruction::Pop => {
                    if let Some(_) = self.stack.pop() {
                    } else {
                        panic!("No elements in the stack");
                    }
                }
                Instruction::Dup => {
                    if let Some(arg) = self.stack.last() {
                        self.stack.push(arg.clone());
                    } else {
                        panic!("No elements in the stack");
                    }
                }
                Instruction::Add => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Integer(a + b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Float(a + b));
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::String(format!("{}{}", a, b)));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Sub => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

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
                Instruction::Mul => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

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
                Instruction::Div => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

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
                Instruction::Eq => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a == b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a == b));
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::Boolean(a == b));
                        }
                        (Value::Boolean(a), Value::Boolean(b)) => {
                            self.stack.push(Value::Boolean(a == b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Ne => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a != b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a != b));
                        }
                        (Value::String(a), Value::String(b)) => {
                            self.stack.push(Value::Boolean(a != b));
                        }
                        (Value::Boolean(a), Value::Boolean(b)) => {
                            self.stack.push(Value::Boolean(a != b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Lt => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a < b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a < b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Le => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a <= b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a <= b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Gt => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a > b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a > b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Ge => {
                    let Some(a) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };
                    let Some(b) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            self.stack.push(Value::Boolean(a >= b));
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            self.stack.push(Value::Boolean(a >= b));
                        }
                        _ => {
                            panic!("Invalid types");
                        }
                    }
                }
                Instruction::Return => {
                    self.call_return = true;
                    return;
                }
                Instruction::If {
                    if_block,
                    else_block,
                } => {
                    let Some(value) = self.stack.pop() else {
                        panic!("No elements in the stack");
                    };

                    if let Value::Boolean(value) = value {
                        let result = if value {
                            self.execute(if_block)
                        } else {
                            self.execute(else_block)
                        };

                        if self.call_return {
                            return result;
                        }
                    } else {
                        panic!("Invalid value");
                    }
                }
                Instruction::Loop { block } => loop {
                    let result = self.execute(block);

                    if self.call_break {
                        break;
                    }

                    if self.call_continue {
                        continue;
                    }

                    if self.call_return {
                        return result;
                    }
                },
                Instruction::Break => {
                    self.call_break = true;
                }
                Instruction::Continue => {
                    self.call_continue = true;
                }
            }
        }
    }

    pub fn call(&mut self, module: &str, name: &str, args: Vec<Value>) {
        let module = self.modules.get(module).unwrap();

        if let Some(function) = module.get_function(name) {
            match function {
                crate::Function::Code { name: _, code } => {
                    self.local_vars.push(args);
                    let code = code.clone();
                    self.execute(&code);
                    self.local_vars.pop();
                    return;
                }
                crate::Function::Native { name: _, function } => {
                    let res = function(args);

                    if let Some(res) = res {
                        self.stack.push(res);
                    }

                    return;
                }
            }
        } else {
            panic!("Function not found");
        }
    }

    pub fn has_function(&self, module: &str, name: &str) -> bool {
        if let Some(module) = self.modules.get(module) {
            if let Some(_) = module.get_function(name) {
                return true;
            }
        }

        false
    }
}
