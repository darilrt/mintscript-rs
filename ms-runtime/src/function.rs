use crate::Value;

pub enum Function {
    ByteCode {
        name: String,
        code: Vec<u8>,
    },
    Native {
        name: String,
        function: Box<dyn Fn(Vec<Value>) -> Value>,
    },
}
