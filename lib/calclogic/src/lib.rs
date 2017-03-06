mod parse;
mod eval;
mod models;
mod token;

pub use models::{Result, Error, Value};

pub use token::tokenize;
pub use parse::parse_tokens;
pub use eval::eval;

pub fn calculate(input: String) -> Result<Value> {
    return eval(try!(parse_tokens(try!(tokenize(input)))));
}
