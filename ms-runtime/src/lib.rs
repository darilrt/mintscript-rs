pub mod asm;
mod builder;
mod byte_reader;
mod byte_writer;
mod bytecode;
pub mod dymodule;
mod function;
mod instruction;
mod module;
pub(crate) mod parser;
pub(crate) mod sexpr;
mod value;
mod virtual_machine;

use std::collections::HashMap;

pub use builder::*;
pub use bytecode::*;
pub use dymodule::*;
pub use function::*;
pub use instruction::*;
pub use module::*;
pub use value::*;
pub use virtual_machine::*;

pub fn load_modules(code: &Code) -> Result<(Vec<Module>, Vec<DyModule>), String> {
    // validate version

    let version = code.get(0).ok_or("Missing version")?;

    let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let version_patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    match version {
        Instruction::Version {
            major,
            minor,
            patch,
        } => {
            if *major != version_major || *minor != version_minor || *patch != version_patch {
                return Err("Invalid version".to_string());
            }
        }
        _ => {
            return Err("Invalid version".to_string());
        }
    }

    let code = &code[1..];

    // load modules
    let mut modules = vec![];
    let mut dy_modules = vec![];

    for instruction in code.iter() {
        match instruction {
            Instruction::Module { name: _, code: _ } => {
                modules.push(Module::try_from(instruction.clone()).map_err(|e| e.to_string())?);
            }
            Instruction::LoadModule { name, code } => {
                dy_modules.push(DyModule {
                    name: name.clone(),
                    lib: unsafe { libloading::Library::new(name).map_err(|e| e.to_string())? },
                    fns: HashMap::new(),
                });

                let dymodule = dy_modules.last_mut().unwrap();

                for instruction in code.iter() {
                    match instruction {
                        Instruction::GetFunction { name, alias } => {
                            let symbol = name.clone();
                            let func: libloading::Symbol<'_, fn(Vec<Value>) -> Option<Value>> = unsafe {
                                dymodule
                                    .lib
                                    .get(symbol.as_bytes())
                                    .map_err(|e| e.to_string())?
                            };

                            if let Some(alias) = alias {
                                dymodule.fns.insert(alias.clone(), Box::new(*func));
                            } else {
                                dymodule.fns.insert(name.clone(), Box::new(*func));
                            }
                        }
                        _ => {
                            return Err("Invalid instruction type, expected (fn.get)".to_string());
                        }
                    }
                }
            }
            _ => {
                return Err("Invalid instruction type, expected (mod) or (mod.load)".to_string());
            }
        }
    }

    Ok((modules, dy_modules))
}
