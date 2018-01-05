use std::io::{self, Read};
use std::collections::HashMap;
use std::fmt;

mod json;

struct Parser {
    state: ParserState,
    obj_stack: Vec<json::Value>,
    curr_char: usize,
    document: json::Document,
}

#[derive(Debug)]
enum ParserState {
    Start,
    ParsingKey { key: String },
    ParsingValue { key: String },
}

impl fmt::Display for ParserState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserState::Start => write!(f, "Start"),
            ParserState::ParsingKey { key: _ } => write!(f, "ParsingKey"),
            ParserState::ParsingValue { key: _ }=> write!(f, "ParsingValue"),
        }
    }
}

impl Parser {
    fn new() -> Parser {
        Parser {
            curr_char: 0,
            obj_stack: Vec::new(),
            state: ParserState::Start,
            document: json::Document::new(),
        }
    }
}

/// Gobble up the given string starting at start_loc and ending
/// when we reach the given list of possible character.
/// We then return the gobbled string and the ending location of the char in the string.
///
/// Return None if the end_char is not found
fn gobble_til(input: &str, start_loc: usize, end_chars: &[char]) -> Option<(String, usize)> {
    let mut gobbled = String::new();
    let mut location = start_loc;
    while let Some(curr_char) = input.chars().nth(location) {
        if end_chars.contains(&curr_char) {
            return Some((gobbled, location));
        } else {
            gobbled.push(curr_char);
            location += 1;
        }
    }
    None
}

// Tokenize input
// so far only handles double quoted strings as values
fn parse_json<'a>(input: String) -> Result<json::Document, json::JsonParseError> {
    let mut p = Parser::new();
    {
        // As we traverse through the document we need to keep track of which object we are currently in
        while p.curr_char < input.chars().count() {
            let c = input.chars().nth(p.curr_char).unwrap();

            println!("Current char: ({}, {}), State: {}", c, p.curr_char, p.state);

            p.state = match p.state {
                ParserState::ParsingKey { key } => {
                    match c {
                        '"' => ParserState::ParsingValue { key: key },
                        _ => {
                            let mut new_key = key;
                            new_key.push(c);
                            ParserState::ParsingKey { key: new_key }
                        },
                    }
                }
                ParserState::ParsingValue { key } => {
                    match c {
                        ':' => {
                            // Here is where we need to recurse if we hit a new json object
                            // If it is not a new object, then it is one of the other values and should
                            // just be added straight

                            let (val, new_location) = match gobble_til(&input, p.curr_char + 1, &[',', '}']) {
                                Some(v) => v,
                                None => return Err(json::JsonParseError),
                            };

                            // handle json parse error here
                            let curr_obj = p.obj_stack.last_mut().unwrap();
                            curr_obj.add(key, json::Value::new(&val)).unwrap();

                            p.curr_char = new_location;
                            println!("Curr char: {}, Char count: {}", p.curr_char, input.chars().count());
                            println!("Curr obj: {:?}", curr_obj);
                            p.state = ParserState::Start;
                            continue
                        }
                        _ => ParserState::ParsingValue { key }
                    }
                }
                ParserState::Start => {
                    match c {
                        '"' => {
                            ParserState::ParsingKey { key: String::new() }
                        }
                        '{' => {
                            let new_obj = json::Value::Object(HashMap::new());
                            p.obj_stack.push(new_obj);
                            ParserState::Start
                        }
                        '}' => {
                            match p.obj_stack.pop() {
                                Some(obj) => {
                                    println!("Changing the roooot!");
                                    p.document.root = obj;
                                },
                                None => return Err(json::JsonParseError)
                            }
                            ParserState::Start
                        },
                        _ => ParserState::Start
                    }
                }
            };
            p.curr_char += 1; // advance to next  character
        }
    }
    Ok(p.document)
}

fn main() {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json)
        .expect("unable to read from stdin");
    let d = parse_json(json).expect("unable to parse json!");
    print!("{}", d.print());
}
