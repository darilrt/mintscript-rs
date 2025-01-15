// version.rs

pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 1;
pub const VERSION_PATCH: u8 = 0;

pub fn version() -> String {
    format!("{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH)
}
