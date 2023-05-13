pub mod binop;
pub mod open_close;
pub mod symbol;
pub mod token;
pub mod unaryop;
pub mod unicode_subsup;
pub mod util;

use nom::branch::alt;
use nom::combinator::{eof, not};
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use nom::IResult;

use token::Token;

use binop::take_binop;
use open_close::{take_close, take_open};
use symbol::take_symbol;
use unaryop::take_op;
use unicode_subsup::{take_unicode_sub, take_unicode_sup};

fn tokenize(s: &str) -> IResult<&str, Vec<Token>> {
    terminated(
        many0(preceded(
            not(eof),
            alt((
                take_symbol,
                take_op,
                take_open,
                take_close,
                take_unicode_sub,
                take_unicode_sup,
                take_binop,
            )),
        )),
        eof,
    )(s)
}

struct Expr {}

fn parse(_: Vec<Token>) -> IResult<Vec<Token>, Expr> {
    todo!()
}

fn main() {
    println!(
        "{:#?}",
        tokenize(r"a + b/c <alpha>[<beta hat>^2] `(X)`   5_/ 123")
    );
    //parse(vec![]).unwrap();
}
