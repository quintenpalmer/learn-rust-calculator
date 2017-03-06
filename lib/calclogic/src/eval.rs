use models::{Result, Error, Operation, Value};

pub fn eval(v: Value) -> Result<Value> {
    return match v {
        Value::Num(n) => Ok(Value::Num(n)),
        Value::Operate(op, v1, v2) => {
            let r1 = try!(eval(*v1));
            let r2 = try!(eval(*v2));
            return match (op, r1, r2) {
                (Operation::Plus, Value::Num(rn1), Value::Num(rn2)) => Ok(Value::Num(rn1 + rn2)),
                (_, _, _) => Err(Error::EvalNonNum),
            };
        }
    };
}
