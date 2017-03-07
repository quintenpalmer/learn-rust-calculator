extern crate calclogic;

mod errors;

use std::env;
use std::io::Read;
use std::fs::File;

use errors::{Result, Error};

fn main() {
    println!("time to calculate");
    match run_app() {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    };
}

fn get_expr_string() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(Error::ArgError);
    }
    let mut file = try!(File::open(args[1].as_str()).map_err(Error::IOError));
    let mut expr_string = String::new();
    try!(file.read_to_string(&mut expr_string).map_err(Error::IOError));
    return Ok(expr_string.trim().to_string());
}

fn run_app() -> Result<()> {
    let expr = try!(get_expr_string());

    match calclogic::calculate(expr.clone()) {
        Ok(evaled) => println!("{}", evaled),
        Err(err) => println!("{} could not be calculated ({})", expr, err),
    };

    return Ok(());
}
