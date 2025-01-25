mod options;

use std::time::Instant;

use ms_runtime::{asm::assemble, Instruction};
use options::Options;

// run subcommand
fn run(args: Vec<String>) {
    let total_time = Instant::now();

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
                println!("Usage: ms run <file> [options]");
                println!("Options:");
                println!("  -entry <function>  Entry point function (default: main.main)");
                println!("  -time              Print execution time");
                return;
            }
            "-entry" => {
                if let Some(entry) = it.next() {
                    options.entry = entry.to_string();
                } else {
                    println!("Error: Missing entry point");
                    return;
                }
            }
            "-time" => {
                options.time = true;
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

    let compile_time = Instant::now();

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

    let compile_time = compile_time.elapsed();
    let load_time = Instant::now();

    let mods = ms_runtime::load_modules(&code).expect("Failed to load modules");
    let mut vm = ms_runtime::VirtualMachine::new();

    for module in mods.0 {
        vm.add_module(module);
    }

    for module in mods.1 {
        vm.add_dynamic_module(module);
    }

    // Get the entry point function from the options.entry string (get the last part of the string)
    let parts: Vec<&str> = options.entry.split('.').collect();

    if parts.len() < 2 {
        println!("Error: Invalid entry point '{}'", options.entry);
        return;
    }

    let function = parts.last().unwrap();
    let module = parts[..parts.len() - 1].join(".");

    if !vm.has_function(&module, function) {
        println!("Error: Missing entry point '{}'", options.entry);
        return;
    }

    let load_time = load_time.elapsed();

    let execute_time = Instant::now();
    vm.call(&module, function, vec![]);
    let execute_time = execute_time.elapsed();

    if options.time {
        println!("Compile time: {:?}", compile_time);
        println!("Load time: {:?}", load_time);
        println!("Execute time: {:?}", execute_time);
        println!("Total time: {:?}", total_time.elapsed());
    }
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
        println!("Usage: ms <subcommand>");
        println!("Subcommands:");
        println!("  run <file> [options]");
        println!("  compile <file> [options]");
        // Debugging
        // run(vec!["./examples/test.msa".to_string()]);
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
            println!("Error: Invalid subcommand");
            return;
        }
    }
}
