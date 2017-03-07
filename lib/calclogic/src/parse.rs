use std::slice;

use models::{Result, Error, Token, Value};

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Value> {
    let iter = tokens.iter();
    return parse_expr(&mut iter.clone());
}

fn parse_expr(iter: &mut slice::Iter<Token>) -> Result<Value> {
    let t = try!(iter.next().ok_or(Error::UnexpectedEOF));
    return match t {
        &Token::Num(n) => Ok(Value::Num(n)),
        &Token::Open => {
            let next_t = try!(iter.next().ok_or(Error::UnexpectedEOF));
            let operation = try!(match next_t {
                &Token::Operation(op) => Ok(op),
                _ => Err(Error::WrongToken("operator", *next_t)),
            });

            let v1 = try!(parse_expr(iter));
            let v2 = try!(parse_expr(iter));

            let exp_close = try!(iter.next().ok_or(Error::UnexpectedEOF));
            match exp_close {
                &Token::Close => Ok(Value::Operate(operation, Box::new(v1), Box::new(v2))),
                _ => Err(Error::WrongToken("close", *exp_close)),
            }

        }
        _ => Err(Error::WrongToken("num or open", *t)),
    };
}
