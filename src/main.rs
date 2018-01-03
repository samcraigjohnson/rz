use std::io::{self, Read};
mod json;

// What characters does json have
// " - deliniates a key or value
// { - starts an object
// } - ends an object
// [ - starts an array
// ] - ends an array
// : - ends a key/starts a value
// 45 - number


struct Parser {
    state: ParserState,
    curr_char: usize,
}

enum ParserState {
    StartOfObject,
    EndOfObject,
    ParsingKey,
    ParsingValue,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            curr_char: 0,
            state: ParserState::StartOfObject,
        }
    }
}

// Tokenize input
// so far only handles double quoted strings as values
fn parse_json(input: String) -> json::Document {
    let mut p = Parser::new();
    let mut d = json::Document::new();
    {
        let curr_obj = &mut d.root;
        while p.curr_char < input.chars().count() {
            let c = input.chars().nth(p.curr_char).unwrap();
            match c {
                '{' => { // new object created
                },
                '}' => { // object ended
                }
                ':' => {
                    p.in_key = false;
                    p.in_value = true;
                },
                '"' => {
                    if !p.in_key {

                    }

                    if p.in_key { // finished the key
                        p.in_key = false;
                    } else if p.in_value { // possible done with value
                        if p.value_has_started() { // make sure its not the start
                            p.in_value = false;
                            curr_obj.add(p.curr_key, json::Value::String(p.curr_value));
                            p.curr_value = String::new();
                            p.curr_key = String::new();
                        }
                    } else {
                        p.in_key = true;
                    }
                },
                _ => {
                    if p.in_key {
                        p.curr_key.push(c);
                    } else if p.in_value && (c != ' ' || p.value_has_started()) {
                        p.curr_value.push(c);
                    }
                }
            };

            p.curr_char += 1; // advance to next  character
        }
    }
    d
}

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json)
        .expect("unable to read from stdin");
    let d = parse_json(json);
    d.print();
}

// impl fmt::Display for json::Token {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             json::Token::OpenBrace(c) => write!(f, "{}\n", c.clone()),
//             json::Token::ClosingBrace(c) => write!(f, "{}\n", c),
//             json::Token::ArrayStart(c) => write!(f, "{}\n", c),
//             json::Token::ArrayEnd(c) => write!(f, "{}\n", c),
//             json::Token::ArraySeperator(c) => write!(f, "{} ", c),
//             json::Token::Colon(c) => write!(f, "{} ", c),
//             json::Token::Key(key) => write!(f, "\"{}\"", key),
//             json::Token::Value(val) => write!(f, "{}", val),
//             json::Token::Other(c) => write!(f, "{}", c),
//         }
//     }
// }


// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Value::String(s) => write!(f, "\"{}\"", s),
//             Value::Bool(b) => write!(f, "{}", b),
//             Value::Integer(i) => write!(f, "{}", i),
//         }
//     }
// }

// impl json::Token {
//     fn show(&self) {
//         match self.val {
//             '{' => print!("{{\n"),
//             '}' => print!("\n}}"),
//             '[' => print!("[\n"),
//             ']' => print!("\n]"),
//             _ => print!("{}", self.val)
//         }
//     }
// }
