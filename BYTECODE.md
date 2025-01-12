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

- `DUMP` (0x01): Dump the stack for debugging purposes.

- `FUNC` (0x02): Define a new function.
  - The next byte is the number of instructions in the function.
  - Next is the name of the function, which is a null-terminated string.
  - The remaining bytes are the instructions for the function.

