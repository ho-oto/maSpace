use super::token::Token;
use super::util::*;

use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::map, IResult};

pub fn take_binop(s: &str) -> IResult<&str, Token> {
    alt((
        take_over, take_under, take_root, take_sub, take_sup, take_frac, take_cat,
    ))(s)
}

fn take_sub(s: &str) -> IResult<&str, Token> {
    map(count_space_around(char('_')), |x| Token::Sub(x))(s)
}
fn take_under(s: &str) -> IResult<&str, Token> {
    map(count_space_around(tag("__")), |x| Token::Under(x))(s)
}
fn take_sup(s: &str) -> IResult<&str, Token> {
    map(count_space_around(char('^')), |x| Token::Sup(x))(s)
}
fn take_over(s: &str) -> IResult<&str, Token> {
    map(count_space_around(tag("^^")), |x| Token::Over(x))(s)
}
fn take_root(s: &str) -> IResult<&str, Token> {
    map(count_space_around(tag("_/")), |x| Token::Root(x))(s)
}
fn take_frac(s: &str) -> IResult<&str, Token> {
    map(count_space_around(char('/')), |x| Token::Frac(x))(s)
}
fn take_cat(s: &str) -> IResult<&str, Token> {
    map(num_space, |n| Token::Cat(n))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(take_binop("  _  123").unwrap(), ("123", Token::Sub(2)));
        assert_eq!(take_binop("  _ a").unwrap(), ("a", Token::Sub(2)));
        assert_eq!(take_binop("_  <a>").unwrap(), ("<a>", Token::Sub(2)));
        assert_eq!(take_binop("    _+").unwrap(), ("+", Token::Sub(4)));
        assert_eq!(take_binop("  _  123").unwrap(), ("123", Token::Sub(2)));
        assert_eq!(take_binop("_x").unwrap(), ("x", Token::Sub(0)));

        assert_eq!(take_binop("  ^  123").unwrap(), ("123", Token::Sup(2)));
        assert_eq!(take_binop("  ^ a").unwrap(), ("a", Token::Sup(2)));
        assert_eq!(take_binop("^  <a>").unwrap(), ("<a>", Token::Sup(2)));
        assert_eq!(take_binop("    ^+").unwrap(), ("+", Token::Sup(4)));
        assert_eq!(take_binop("  ^  123").unwrap(), ("123", Token::Sup(2)));
        assert_eq!(take_binop("^x").unwrap(), ("x", Token::Sup(0)));

        assert_eq!(take_binop("  __  123").unwrap(), ("123", Token::Under(2)));
        assert_eq!(take_binop("  __ a").unwrap(), ("a", Token::Under(2)));
        assert_eq!(take_binop("__  <a>").unwrap(), ("<a>", Token::Under(2)));
        assert_eq!(take_binop("    __+").unwrap(), ("+", Token::Under(4)));
        assert_eq!(take_binop("  __  123").unwrap(), ("123", Token::Under(2)));
        assert_eq!(take_binop("__x").unwrap(), ("x", Token::Under(0)));

        assert_eq!(take_binop("  ^^  123").unwrap(), ("123", Token::Over(2)));
        assert_eq!(take_binop("  ^^ a").unwrap(), ("a", Token::Over(2)));
        assert_eq!(take_binop("^^  <a>").unwrap(), ("<a>", Token::Over(2)));
        assert_eq!(take_binop("    ^^+").unwrap(), ("+", Token::Over(4)));
        assert_eq!(take_binop("  ^^  123").unwrap(), ("123", Token::Over(2)));
        assert_eq!(take_binop("^^x").unwrap(), ("x", Token::Over(0)));

        assert_eq!(take_binop("  /  123").unwrap(), ("123", Token::Frac(2)));
        assert_eq!(take_binop("  / a").unwrap(), ("a", Token::Frac(2)));
        assert_eq!(take_binop("/  <a>").unwrap(), ("<a>", Token::Frac(2)));
        assert_eq!(take_binop("    /+").unwrap(), ("+", Token::Frac(4)));
        assert_eq!(take_binop("  /  123").unwrap(), ("123", Token::Frac(2)));
        assert_eq!(take_binop("/x").unwrap(), ("x", Token::Frac(0)));

        assert_eq!(take_binop("  _/  123").unwrap(), ("123", Token::Root(2)));
        assert_eq!(take_binop("  _/ a").unwrap(), ("a", Token::Root(2)));
        assert_eq!(take_binop("_/  <a>").unwrap(), ("<a>", Token::Root(2)));
        assert_eq!(take_binop("    _/+").unwrap(), ("+", Token::Root(4)));
        assert_eq!(take_binop("  _/  123").unwrap(), ("123", Token::Root(2)));
        assert_eq!(take_binop("_/x").unwrap(), ("x", Token::Root(0)));

        assert_eq!(take_binop(" 123").unwrap(), ("123", Token::Cat(1)));
        assert_eq!(take_binop("  a").unwrap(), ("a", Token::Cat(2)));
        assert_eq!(take_binop("   <a>").unwrap(), ("<a>", Token::Cat(3)));
        assert_eq!(take_binop("+").unwrap(), ("+", Token::Cat(0)));
    }
}
