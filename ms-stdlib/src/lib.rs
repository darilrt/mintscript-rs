use ms_runtime::*;

fn print(args: Vec<Value>) -> Option<Value> {
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
    None
}

fn input(_: Vec<Value>) -> Option<Value> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Some(Value::String(input.trim().to_string()))
}

pub fn get_module() -> Module {
    let mut module = Module::new();

    module.add_native_function("print", Box::new(print));
    module.add_native_function("input", Box::new(input));

    module
}
