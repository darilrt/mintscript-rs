use std::{fmt::Debug, sync::Arc};

pub enum Object {
    Values(Vec<Box<Value>>),
    Native(Box<dyn NativeObject>),
}

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
    Object(Arc<Object>),
}

pub trait NativeObject {}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Values(values) => {
                write!(f, "Values[")?;
                for value in values {
                    write!(f, "{:?}, ", value)?;
                }
                write!(f, "]")
            }
            Object::Native(_) => write!(f, "Native"),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Object(arc) => {
                let obj = arc.as_ref();
                write!(f, "Object[{:?}]", obj)
            }
        }
    }
}
