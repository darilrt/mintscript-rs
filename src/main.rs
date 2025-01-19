use ms_runtime::{asm::assemble, Instruction};

// Print usage information
// Usage: ms <subcommand>
//   ms run <file> [options]
//   ms compile <file> [options]
fn usage() {
    println!("Usage: ms <subcommand>");
    println!("  ms run <file> [options]");
    println!("  ms compile <file> [options]");
}

struct Options {
    output: String,
    input: String,
}

impl Options {
    fn new() -> Options {
        Options {
            output: String::new(),
            input: String::new(),
        }
    }
}

// run subcommand
fn run(args: Vec<String>) {
    // Check if the user provided a file to run
    if args.len() == 0 {
        println!("Error: No input file");
        return;
    }

    let mut options = Options::new();

    // Parse options
    let mut it = args.iter();

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: ms run <file>");
                return;
            }
            _ => {
                if options.input.is_empty() {
                    options.input = arg.to_string();
                } else {
                    println!("Error: Invalid option '{}'", arg);
                    return;
                }
            }
        }
    }

    // Validate input file
    if !options.input.ends_with(".ms")
        && !options.input.ends_with(".msa")
        && !options.input.ends_with(".msb")
    {
        println!("Error: Unsupported file extension");
        return;
    }

    let code = if options.input.ends_with(".ms") {
        todo!()
    } else if options.input.ends_with(".msa") {
        assemble(&std::fs::read_to_string(&options.input).expect("Failed to read file"))
            .expect("Failed to assemble code")
    } else if options.input.ends_with(".msb") {
        let source = std::fs::read(&options.input).expect("Failed to read file");
        ms_runtime::Instruction::from_bytecode(&source).expect("Failed to load bytecode")
    } else {
        panic!("Unsupported file extension");
    };

    let mods = ms_runtime::load_modules(&code).expect("Failed to load modules");
    let mut vm = ms_runtime::VirtualMachine::new();

    for module in mods.0 {
        vm.add_module(module);
    }

    for module in mods.1 {
        vm.add_dynamic_module(module);
    }

    if !vm.has_function("main", "main") {
        println!("Error: Missing 'main' function in {}", options.input);
        return;
    }

    vm.call("main", "main", vec![]);
}

// compile subcommand
fn compile(args: Vec<String>) {
    // Check if the user provided a file to run
    if args.len() == 0 {
        println!("Error: No input file");
        return;
    }

    let mut options = Options::new();

    // Parse options
    let mut it = args.iter();

    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-o" => {
                if let Some(output) = it.next() {
                    options.output = output.to_string();
                } else {
                    println!("Error: Missing output file");
                    return;
                }
            }
            "-h" | "--help" => {
                println!("Usage: ms compile <file> [options]");
                return;
            }
            _ => {
                if options.input.is_empty() {
                    options.input = arg.to_string();
                } else {
                    println!("Error: Invalid option '{}'", arg);
                    return;
                }
            }
        }
    }

    if options.output.is_empty() {
        println!("Error: Missing output file");
        return;
    }

    let source = std::fs::read_to_string(&options.input).expect("Failed to read file");

    let code = if options.input.ends_with(".ms") {
        todo!()
    } else if options.input.ends_with(".msa") {
        assemble(&source).expect("Failed to assemble code")
    } else {
        panic!("Unsupported file extension");
    };

    let bytecode = Instruction::code_to_bytes(&code);

    std::fs::write(&options.output, &bytecode).expect("Failed to write file");
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if the user provided a file to run
    if args.len() < 2 {
        // usage();
        // run test file
        run(vec!["./examples/test.msa".to_string()]);
        return;
    }

    // Check if the user provided a valid subcommand
    match args[1].as_str() {
        "run" => {
            run(args[2..].to_vec());
        }
        "compile" => {
            compile(args[2..].to_vec());
        }
        _ => {
            usage();
            println!("Error: Invalid subcommand");
            return;
        }
    }
}
