pub mod binop;
pub mod open_close;
pub mod symbol;
pub mod token;
pub mod unaryop;
pub mod unicode_subsup;
pub mod util;

use nom::IResult;

//use symbol::take_constant;
use token::Token;

fn tokenize(_: &str) -> Vec<Token> {
    todo!()
}
struct Expr {}

fn parse(_: Vec<Token>) -> IResult<Vec<Token>, Expr> {
    todo!()
}

fn main() {
    tokenize("");
    parse(vec![]).unwrap();
}
