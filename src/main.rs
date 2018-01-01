use std::io::{self, Read};
//use std::fmt;

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json)
        .expect("unable to read from stdin");
    parse_json(json)
}

// What characters does json have
// " - deliniates a key or value
// { - starts an object
// } - ends an object
// [ - starts an array
// ] - ends an array
// : - ends a key/starts a value
// 45 - number

// enum JsonToken {
//     OpenBrace(char),
//     ClosingBrace(char),
//     ArrayStart(char),
//     ArraySeperator(char),
//     ArrayEnd(char),
//     Colon(char),
//     Key(String),
//     Value(JsonValue),
//     Other(char),
// }

// impl fmt::Display for JsonToken {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             JsonToken::OpenBrace(c) => write!(f, "{}\n", c.clone()),
//             JsonToken::ClosingBrace(c) => write!(f, "{}\n", c),
//             JsonToken::ArrayStart(c) => write!(f, "{}\n", c),
//             JsonToken::ArrayEnd(c) => write!(f, "{}\n", c),
//             JsonToken::ArraySeperator(c) => write!(f, "{} ", c),
//             JsonToken::Colon(c) => write!(f, "{} ", c),
//             JsonToken::Key(key) => write!(f, "\"{}\"", key),
//             JsonToken::Value(val) => write!(f, "{}", val),
//             JsonToken::Other(c) => write!(f, "{}", c),
//         }
//     }
// }

// enum JsonValue {
//     String(String),
//     Bool(bool),
//     Integer(i64),
// }


// impl fmt::Display for JsonValue {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             JsonValue::String(s) => write!(f, "\"{}\"", s),
//             JsonValue::Bool(b) => write!(f, "{}", b),
//             JsonValue::Integer(i) => write!(f, "{}", i),
//         }
//     }
// }

struct Parser {
    in_object: bool,
    in_key: bool,
    in_value: bool,
    in_array: bool,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            in_object: false,
            in_key: false,
            in_value: false,
            in_array: false,
        }
    }
}

// Tokenize input
fn parse_json(input: String) {

    for c in input.chars() {
        let token = read_character(c);
        token.show()
    }

    //println!("You wrote {}", input);
}

fn read_character(c: char) -> Token {
    Token { val: c }
}

struct Token {
    val: char
}

impl Token {
    fn show(&self) {
        match self.val {
            '{' => print!("{{\n"),
            '}' => print!("\n}}"),
            '[' => print!("[\n"),
            ']' => print!("\n]"),
            _ => print!("{}", self.val)
        }
    }
}
