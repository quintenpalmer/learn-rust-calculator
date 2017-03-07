use models::{Result, Error, Value, Operation};

pub fn eval(v: Value) -> Result<Value> {
    return match v {
        Value::Num(n) => Ok(Value::Num(n)),
        Value::Operate(op, v1, v2) => {
            let eval1 = try!(eval(*v1));
            let eval2 = try!(eval(*v2));
            match (op, eval1, eval2) {
                (Operation::Plus, Value::Num(n1), Value::Num(n2)) => Ok(Value::Num(n1 + n2)),
                (Operation::Minus, Value::Num(n1), Value::Num(n2)) => Ok(Value::Num(n1 - n2)),
                (Operation::Mul, Value::Num(n1), Value::Num(n2)) => Ok(Value::Num(n1 * n2)),
                _ => Err(Error::EvalNonNum),
            }
        }
    };
}
