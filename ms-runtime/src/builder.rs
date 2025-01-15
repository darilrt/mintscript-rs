use std::collections::HashMap;

use crate::Function;

pub enum Block {
    Loop(Vec<u8>),
    If(Vec<u8>),
    Else(Vec<u8>),
    Function(String, Vec<u8>),
}

pub struct ModuleBuilder {
    pub functions: HashMap<String, Function>, // Functions in the module
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            functions: HashMap::new(),
        }
    }
}
