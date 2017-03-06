use std::str::Chars;

use models::{Result, Error, Token, Operation};

pub fn tokenize(input: String) -> Result<Vec<Token>> {
    return parse_tokens(&mut input.clone().chars());
}

fn parse_tokens(bytes: &mut Chars) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    loop {
        match find_next(bytes) {
            Some(c) => {
                // println!("matching: {:?}", c);
                tokens.push(try!(match c {
                    '(' => Ok(Token::Open),
                    ')' => Ok(Token::Close),
                    '+' => Ok(Token::Operation(Operation::Plus)),
                    '0' => Ok(Token::Num(0)),
                    '1' => Ok(Token::Num(1)),
                    '2' => Ok(Token::Num(2)),
                    '3' => Ok(Token::Num(3)),
                    '4' => Ok(Token::Num(4)),
                    '5' => Ok(Token::Num(5)),
                    '6' => Ok(Token::Num(6)),
                    '7' => Ok(Token::Num(7)),
                    '8' => Ok(Token::Num(8)),
                    '9' => Ok(Token::Num(9)),
                    _ => Err(Error::IllegalToken(string_from_char(c))),
                }));
            }
            None => break,
        };
    }
    return Ok(tokens);
}

fn find_next(bytes: &mut Chars) -> Option<char> {
    return match bytes.next() {
        Some(' ') => find_next(bytes),
        Some(c) => Some(c),
        None => None,
    };
}

fn string_from_char(c: char) -> String {
    let mut s = String::new();
    s.push(c);
    return s;
}
