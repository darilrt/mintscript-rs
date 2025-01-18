use crate::{parser::Parser, Code, Instruction};

#[inline]
pub fn assemble(source: &str) -> Result<Code, String> {
    let source = std::fmt::format(format_args!(
        "(version {}.{}.{})\n{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        source
    ));
    Ok(Instruction::from_sexprs(
        &Parser::new(source.as_str()).parse()?,
    )?)
}
