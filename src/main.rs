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
    Start,
    ParsingKey { key: String },
    ParsingValue { key: String },
}

impl Parser {
    fn new() -> Parser {
        Parser {
            curr_char: 0,
            state: ParserState::Start,
        }
    }
}

// Tokenize input
// so far only handles double quoted strings as values
fn parse_json<'a>(input: String) -> json::Document {
    let mut p = Parser::new();
    let mut d = json::Document::new();
    {
        let curr_obj = &mut d.root;
        while p.curr_char < input.chars().count() {
            let c = input.chars().nth(p.curr_char).unwrap();

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
                        ':' => { // Everything past the colon is a value
                            let mut new_val = String::new();
                            let mut location = p.curr_char + 1;

                            // TODO(samj) - special handling of nested objects here
                            while let Some(curr_char) = input.chars().nth(location) {
                                if curr_char == ',' || curr_char == '}' { break; } // Need to handle array here :D

                                new_val.push(curr_char);
                                location += 1;
                            }

                            // TODO(samj) - strip quotes and spaces
                            // TODO(samj) - try to convert to int, float, bool
                            let mut trimmed = new_val.trim();
                            if trimmed.chars().nth(0).unwrap() == '"' {
                                trimmed = trimmed.get(1..trimmed.chars().count() - 1).unwrap();
                            }

                            curr_obj.add(key, json::Value::String(trimmed.to_string()));
                            p.curr_char = location;
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
                        _ => ParserState::Start
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
