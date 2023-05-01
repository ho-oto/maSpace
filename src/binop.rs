use super::token::Token;
use super::util::*;

use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of},
    IResult,
};

pub fn take_sub(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('_')).map(|(s, n)| (s, Token::Sub(n)))
}
pub fn take_under(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("__")).map(|(s, n)| (s, Token::Under(n)))
}
pub fn take_sup(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('^')).map(|(s, n)| (s, Token::Sup(n)))
}
pub fn take_over(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("^^")).map(|(s, n)| (s, Token::Over(n)))
}
pub fn take_frac(s: &str) -> IResult<&str, Token> {
    max_space_around(s, one_of("/∕")).map(|(s, n)| (s, Token::Frac(n)))
}
pub fn take_cat(s: &str) -> IResult<&str, Token> {
    num_space(s).map(|(s, n)| (s, Token::Cat(n)))
}
