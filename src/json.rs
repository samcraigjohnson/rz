use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct JsonParseError;

impl error::Error for JsonParseError {
    fn description(&self) -> &str {
        "There was an error parsing json"
    }
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Shit!")
    }
}

pub struct Document {
    pub root: Value
}

impl Document {
    pub fn new() -> Document {
        Document {
            root: Value::Null
        }
    }

    pub fn print(&self) -> String {
        self.root.print(0)
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    String(String),
    Object(HashMap<String, Value>),
    Boolean(bool),
    Number(f64),
}

impl Value {
    pub fn new(val: &str) -> Self {
        let cleaned = val.trim();
        if let Ok(v) = cleaned.parse::<f64>() {
            return Value::Number(v);
        } else if let Ok(v) = cleaned.parse::<bool>() {
            return Value::Boolean(v);
        } else if cleaned == "null" {
            return Value::Null;
        } else {
            return Value::String(cleaned.trim_matches('"').to_string())
        }
    }

    fn print(&self, depth: usize) -> String {
        let out_spaces = "  ".repeat(depth);
        let in_spaces = "  ".repeat(depth + 1);
        let mut r = String::new();

        match *self {
            Value::String(ref s) => r += &format!("\"{}\"", s),
            // Value::Bool(ref b) => format!("{}", b),
            // Value::Integer(ref i) => format!("{}", i),
            Value::Object(ref o) => {
                r += &format!("{}{{\n", &out_spaces); // beginning of a new object
                for (k, v) in o.iter() {
                    r += &format!("{}\"{}\": {},\n", &in_spaces, k, v.print(depth + 1));
                }
                r += &format!("{}}}\n", &out_spaces);
            },
            Value::Boolean(ref b) => {
                r += &format!("{}", b);
            },
            Value::Number(ref n) => {
                r += &format!("{}", n);
            },
            Value::Null => r.push_str("null"),
        }
        r
    }

    pub fn add(&mut self, key: String, value: Value) -> Result<(), JsonParseError> {
        match *self {
            Value::Object(ref mut o) => {
                o.insert(key, value);
            },
            _ => return Err(JsonParseError)
        }
        Ok(())
    }
}
