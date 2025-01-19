#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCode {
    None = 0x00, // No operation (NOP)

    Version = 0x17, // VERSION <major: u8> <minor: u8> <patch: u8>

    // Debugging
    Dump = 0x01, // Dump the stack
    Hi = 0x02,   // Print "Hi"

    // Functions
    Func = 0x03, // Define a function
    Call = 0x04, // Call a function

    // Constants
    PushConstString = 0x40, // Push a constant string onto the stack PushConstString <len: u32> <string: [u8; len]>
    PushConstInteger = 0x41, // Push a constant integer onto the stack PushConstInt <value: i32>
    PushConstFloat = 0x42,  // Push a constant float onto the stack PushConstFloat <value: f32>
    PushConstBoolean = 0x43, // Push a constant boolean onto the stack PushConstBoolean <value: bool>

    // Locals variables
    GetLocal = 0x09,     // Load a local variable onto the stack
    SetLocal = 0x0A,     // Store the top element of the stack in a local variable
    ReserveLocal = 0x18, // Reserve space for a local variable

    // Objects
    Allocate = 0x05, // Allocate a new object with the given number of fields on top of the stack
    GetField = 0x06, // Push the value of a field of an object in the top element of the stack
    SetField = 0x07, // Set the value of the top element of the stack into a field of an object on the second element of the stack

    // Stack manipulation
    Pop = 0x0B, // Pop the top element of the stack
    Dup = 0x0C, // Duplicate the top element of the stack

    // Arithmetic
    Add = 0x0D, // Add
    Sub = 0x0E, // Subtract
    Mul = 0x0F, // Multiply
    Div = 0x10, // Divide

    // Comparison
    Eq = 0x11, // Equal
    Ne = 0x12, // Not equal
    Lt = 0x13, // Less than
    Le = 0x14, // Less than or equal
    Gt = 0x15, // Greater than
    Ge = 0x16, // Greater than or equal

    // Modules
    Module = 0x1B, // Define a module

    // Dynamic Module
    LoadModule = 0x19,  // Load a dynamic module
    GetFunction = 0x1A, // Get a function from a dynamic module

    // Control flow
    Return = 0xFE,   // Return from the current function
    If = 0xFD,       // IF <block: [ByteCode]> END Execute a block of code conditionally
    Else = 0xFC, // IF <block: [ByteCode]> ELSE <block: [ByteCode]> END Execute a block of code conditionally
    Loop = 0xFB, // LOOP <block: [ByteCode]> END Execute a block of code in a loop until instructed to break
    Break = 0xFA, // BREAK Exit the current loop
    Continue = 0xF9, // CONTINUE Skip to the next iteration of the current loop
}

impl ByteCode {
    pub fn from_u8(value: u8) -> Option<ByteCode> {
        match value {
            0x00 => Some(ByteCode::None),
            0x17 => Some(ByteCode::Version),
            0x01 => Some(ByteCode::Dump),
            0x02 => Some(ByteCode::Hi),
            0x03 => Some(ByteCode::Func),
            0x04 => Some(ByteCode::Call),
            0x40 => Some(ByteCode::PushConstString),
            0x41 => Some(ByteCode::PushConstInteger),
            0x42 => Some(ByteCode::PushConstFloat),
            0x43 => Some(ByteCode::PushConstBoolean),
            0x09 => Some(ByteCode::GetLocal),
            0x0A => Some(ByteCode::SetLocal),
            0x18 => Some(ByteCode::ReserveLocal),
            0x05 => Some(ByteCode::Allocate),
            0x06 => Some(ByteCode::GetField),
            0x07 => Some(ByteCode::SetField),
            0x0B => Some(ByteCode::Pop),
            0x0C => Some(ByteCode::Dup),
            0x0D => Some(ByteCode::Add),
            0x0E => Some(ByteCode::Sub),
            0x0F => Some(ByteCode::Mul),
            0x10 => Some(ByteCode::Div),
            0x11 => Some(ByteCode::Eq),
            0x12 => Some(ByteCode::Ne),
            0x13 => Some(ByteCode::Lt),
            0x14 => Some(ByteCode::Le),
            0x15 => Some(ByteCode::Gt),
            0x16 => Some(ByteCode::Ge),
            0x1B => Some(ByteCode::Module),
            0x19 => Some(ByteCode::LoadModule),
            0x1A => Some(ByteCode::GetFunction),
            0xFE => Some(ByteCode::Return),
            0xFD => Some(ByteCode::If),
            0xFC => Some(ByteCode::Else),
            0xFB => Some(ByteCode::Loop),
            0xFA => Some(ByteCode::Break),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}
