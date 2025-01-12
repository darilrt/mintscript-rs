use ms_runtime::*;

fn modules_std() -> Module {
    let mut module = Module::new();

    module.add_native_function(
        "print",
        Box::new(|args| {
            for arg in args {
                print!("{:?} ", arg);
            }
            println!();
            Value::Null
        }),
    );

    module
}

fn main() {
    let mut builder = ModuleBuilder::new();

    builder.def("main");
    builder.push_const_string("Hello, World!");
    builder.dup();
    builder.push_arg();
    builder.push_arg();
    builder.call("std", "print");
    builder.end();

    let code = builder.get_bytecode();

    let module = Module::from_source(&code).unwrap();

    let mut vm = VirtualMachine::new();

    vm.load_module("main", module);
    vm.load_module("std", modules_std());

    vm.call("main", "main", vec![Value::Integer(42)]);
}
