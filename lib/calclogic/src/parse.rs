use std::slice;

use models::{Result, Error, Token, Operation, Value};

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Value> {
    return parse_expr(&mut tokens.iter().clone());
}

fn parse_expr(tokens: &mut slice::Iter<Token>) -> Result<Value> {
    return match try!(next_token(tokens)) {
        Token::Open => {
            let op = try!(parse_operation(tokens));
            let v1 = try!(parse_expr(tokens));
            let v2 = try!(parse_expr(tokens));
            try!(match try!(next_token(tokens)) {
                Token::Close => Ok(()),
                token => Err(Error::WrongToken("Close", token.clone())),
            });
            return Ok(Value::Operate(op, Box::new(v1), Box::new(v2)));
        }
        Token::Num(n) => Ok(Value::Num(n)),
        token => Err(Error::WrongToken("Open or Num", token.clone())),
    };
}

fn parse_operation(tokens: &mut slice::Iter<Token>) -> Result<Operation> {
    return match try!(next_token(tokens)) {
        Token::Operation(operation) => Ok(operation),
        token => Err(Error::WrongToken("Operation", token.clone())),
    };
}

fn next_token(tokens: &mut slice::Iter<Token>) -> Result<Token> {
    return tokens.next().map(|v| *v).ok_or(Error::UnexpectedEOF);
}
