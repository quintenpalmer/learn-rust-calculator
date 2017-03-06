use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IllegalToken(String),
    WrongToken(&'static str, Token),
    EvalNonNum,
    UnexpectedEOF,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IllegalToken(ref msg) => write!(f, "illegal token: {}", msg),
            Error::WrongToken(ref found, ref exp) => {
                write!(f, "found token: {:?} when expecting: {:?}", found, exp)
            }
            Error::EvalNonNum => write!(f, "eval did not boil down to a number"),
            Error::UnexpectedEOF => write!(f, "unexpected EOF"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Open,
    Close,
    Num(u8),
    Operation(Operation),
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Plus,
}


#[derive(Debug, Clone)]
pub enum Value {
    Num(u8),
    Operate(Operation, Box<Value>, Box<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Num(ref val) => write!(f, "{}", val),
            Value::Operate(_, _, _) => write!(f, "operation"),
        }
    }
}
