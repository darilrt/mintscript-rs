use std::vec;

use ms_runtime::*;
use version::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

fn modules_std() -> Module {
    let mut module = Module::new();

    module.add_native_function(
        "print",
        Box::new(|args| {
            let mut it = args.iter();

            while let Some(arg) = it.next() {
                match arg {
                    Value::Integer(value) => print!("{}", value),
                    Value::Float(value) => print!("{}", value),
                    Value::String(value) => print!("{}", value),
                    Value::Null => print!("null"),
                    Value::Boolean(value) => print!("{}", value),
                    Value::Object(value) => print!("{:?}", value),
                }

                if it.len() > 0 {
                    print!(" ");
                }
            }

            println!();
            Value::Null
        }),
    );

    module.add_native_function(
        "input",
        Box::new(|_| {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            Value::String(input.trim().to_string())
        }),
    );

    module
}

fn main() {
    let code = vec![
        Instruction::Version {
            major: VERSION_MAJOR,
            minor: VERSION_MINOR,
            patch: VERSION_PATCH,
        },
        Instruction::Func {
            name: "foo".to_string(),
            code: vec![
                Instruction::GetLocal { index: 0 },
                Instruction::Call {
                    module: "std".to_string(),
                    function: "print".to_string(),
                },
                Instruction::Pop, // Pop the return value
                Instruction::GetLocal { index: 1 },
                Instruction::Call {
                    module: "std".to_string(),
                    function: "print".to_string(),
                },
            ],
        },
        Instruction::Func {
            name: "main".to_string(),
            code: vec![
                Instruction::PushConstInteger { value: 1 },
                Instruction::PushConstInteger { value: 2 },
                Instruction::Call {
                    module: "main".to_string(),
                    function: "foo".to_string(),
                },
            ],
        },
    ];

    let code = Instruction::code_to_bytes(&code);

    let module = Module::try_from(code).expect("Failed to load module");

    let mut vm = VirtualMachine::new();

    vm.load_module("main", module);
    vm.load_module("std", modules_std());

    vm.call("main", "main", vec![]);
}
