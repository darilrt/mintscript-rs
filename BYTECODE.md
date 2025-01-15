### MintScript Bytecode Specification

MintScript bytecode is a binary format that represents a program in a compact form.

The bytecode is designed to be executed by the MintScript virtual machine.

The bytecode is a sequence of instructions, each of which is a single byte.

The bytecode is stored in a file with the extension `.msb`.

The bytecode file is a binary file that contains the bytecode for a single module, which is a single file in the MintScript language.

The bytecode file is organized as follows:

- The first 3 byte of the file is the version number of the bytecode format. The current version number is 0.1.0.
- The second byte of the file is the number of instructions in the bytecode.
- The remaining bytes of the file are the instructions themselves.
- The module is divided into functions, each of which is a sequence of instructions.

Each instruction is a single byte, which is an opcode that specifies the operation to be performed.

The following opcodes are defined:

- `NOP` (0x00): No operation.
- `VERSION` (0x17): VERSION <major: u8> <minor: u8> <patch: u8> Define the version of the bytecode format.
- `DUMP` (0x01): Dump the stack for debugging purposes.
- `HI` (0x02): Print "Hi" to the console.
- `FUNC` (0x03): FUNC <length: u32> <name: string> <code: [ByteCode x length]> Define a function.
- `CALL` (0x04): CALL <module: string> <name: string> Call a function.
- `STRPUSH` (0x05): STRPUSH <value: string> Push a string onto the stack.
- `INTPUSH` (0x06): INTPUSH <value: i32> Push an integer onto the stack.
- `FLOATPUSH` (0x07): FLTPUSH <value: f32> Push a float onto the stack.
- `LOCALGET` (0x08): LOCALGET <index: u32> Push the local variable at the given index onto the stack.
- `LOCALSET` (0x09): LOCALSET <index: u32> Pop the top element of the stack and store it in the local variable at the given index.
- `LOCARES` (0x18) LOCARES <index: u32> Reserve space for a local variable at the given index.
- `ALLOC` (0x19) ALLOC <size: u32> Allocate a object of the given amount of fields on the top of the stack.
- `FIELDGET` (0x1A) FIELDGET <index: u32> Push the value of the field at the given index of the object on the top of the stack.
- `FIELDSET` (0x1B) FIELDSET <index: u32> Pop the top element of the stack and store it in the field at the given index of the object on the top of the stack. 
- `POP` (0x0A): POP Pop the top element of the stack.
- `DUP` (0x0B): DUP Duplicate the top element of the stack.
- `ADD` (0x0C): ADD Pop two elements from the stack, add them, and push the result.
- `SUB` (0x0D): SUB Pop two elements from the stack, subtract them, and push the result.
- `MUL` (0x0E): MUL Pop two elements from the stack, multiply them, and push the result.
- `DIV` (0x0F): DIV Pop two elements from the stack, divide them, and push the result.
- `EQ` (0x10): EQ Pop two elements from the stack, compare them for equality, and push the result.
- `NE` (0x11): NE Pop two elements from the stack, compare them for inequality, and push the result.
- `LT` (0x12): LT Pop two elements from the stack, compare them for less than, and push the result.
- `LE` (0x13): LE Pop two elements from the stack, compare them for less than or equal, and push the result.
- `GT` (0x14): GT Pop two elements from the stack, compare them for greater than, and push the result.
- `GE` (0x15): GE Pop two elements from the stack, compare them for greater than or equal, and push the result.
- `RET` (0xFE): Return from the current function.
- `IF` (0xFD): IF <length: u32> <code: [ByteCode x length]> If the top element of the stack is true, execute the code block.
- `ELSE` (0xFC): is the continuation of an `IF` block. IF <length: u32> <code: [ByteCode x length]> ELSE <length: u32> <code: [ByteCode x length]>
- `LOOP` (0xFB): LOOP <length: u32> <code: [ByteCode x length]> Loop over the code block until the top element of the stack is false.
- `BREAK` (0xFA): BREAK Break out of the current loop.
- `CONTINUE` (0xF9): CONTINUE Continue to the next iteration of the current loop.