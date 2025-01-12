#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCode {
    None = 0x00, // No operation (NOP)

    // Debugging
    Dump = 0x01, // Dump the stack
    Hi = 0x02,   // Print "Hi"

    // Functions
    Func = 0x03, // Define a function
    Call = 0x04, // Call a function

    // Constants
    PushConstString = 0x06, // Push a constant string onto the stack PushConstString <len: u32> <string: [u8; len]>
    PushConstInt = 0x07,    // Push a constant integer onto the stack PushConstInt <value: i32>
    PushConstFloat = 0x08,  // Push a constant float onto the stack PushConstFloat <value: f32>

    // Function arguments
    PushArg = 0x09, // Push the top element of the stack onto the argument stack
    PopArg = 0x0A,  // Pop the top element of the argument stack and push it onto the stack

    // Stack manipulation
    Pop = 0x0B, // Pop the top element of the stack
    Dup = 0x0C, // Duplicate the top element of the stack
}

impl ByteCode {
    pub fn from_u8(value: u8) -> Option<ByteCode> {
        match value {
            0x00 => Some(ByteCode::None),
            0x01 => Some(ByteCode::Dump),
            0x02 => Some(ByteCode::Hi),
            0x03 => Some(ByteCode::Func),
            0x04 => Some(ByteCode::Call),
            0x06 => Some(ByteCode::PushConstString),
            0x07 => Some(ByteCode::PushConstInt),
            0x08 => Some(ByteCode::PushConstFloat),
            0x09 => Some(ByteCode::PushArg),
            0x0A => Some(ByteCode::PopArg),
            0x0B => Some(ByteCode::Pop),
            0x0C => Some(ByteCode::Dup),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
