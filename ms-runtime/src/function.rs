use crate::{instruction::Code, Value};

pub enum Function {
    Code {
        name: String,
        code: Code,
    },
    Native {
        name: String,
        function: Box<dyn Fn(Vec<Value>) -> Option<Value>>,
    },
}
