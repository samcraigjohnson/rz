use std::collections::HashMap;
use std::fmt;

pub struct Document {
    pub root: Object,
}

impl Document {
    pub fn new() -> Document {
        Document {
            root: Object::new(0)
        }
    }

    pub fn print(&self) {
        print!("{}\n", &self.root);
    }
}

pub struct Object {
    pub map: HashMap<String, Value>,
    pub depth: usize
}

impl Object {
    pub fn new(depth: usize) -> Object {
        Object {
            map: HashMap::new(),
            depth: depth
        }
    }

    pub fn add(&mut self, key: String, value: Value) {
        self.map.insert(key, value);
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out_spaces = "  ".repeat(self.depth);
        let in_spaces = "  ".repeat(self.depth + 1);

        write!(f, "{}{{\n", &out_spaces)?; // beginning of a new object
        for (k, v) in &self.map {
            write!(f, "{}\"{}\": {}", &in_spaces, k, v)?;
        }
        write!(f, "{}\n}}", &out_spaces)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Token {
    OpenBrace(char),
    ClosingBrace(char),
    Colon(char),
    Key(String),
    Value(String),
    Other(char),
}

pub enum Value {
    String(String),
    Bool(bool),
    Integer(i64),
    Object(Object),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::String(ref s) => write!(f, "\"{}\"", s),
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::Integer(ref i) => write!(f, "{}", i),
            Value::Object(ref o) => write!(f, "{}", o),
        }
    }
}
