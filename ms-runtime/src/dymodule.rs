use std::collections::HashMap;

use libloading::Library;

use crate::Value;

pub struct DyModule {
    pub name: String,
    pub lib: Library,
    pub fns: HashMap<String, Box<fn(Vec<Value>) -> Option<Value>>>,
}
