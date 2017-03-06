use models::{Result, Token, Value};

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Value> {
    return Ok(Value::Num(8));
}
