use models::{Result, Error, Token, Operation};

pub fn tokenize(input: String) -> Result<Vec<Token>> {
    let mut vec: Vec<Token> = Vec::with_capacity(input.len());
    for c in input.chars() {
        if c == ' ' {
            continue;
        }
        let t = try!(match c {
            '(' => Ok(Token::Open),
            ')' => Ok(Token::Close),
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
            '+' => Ok(Token::Operation(Operation::Plus)),
            '-' => Ok(Token::Operation(Operation::Minus)),
            _ => Err(Error::IllegalToken(c.to_string())),
        });
        vec.push(t);
    }
    return Ok(vec);
}
