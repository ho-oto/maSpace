use super::token::Token;
use super::util::*;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, anychar, digit1},
    combinator::{map, map_res, opt},
    sequence::{delimited, pair, tuple},
    IResult,
};

pub fn take_op(s: &str) -> IResult<&str, Token> {
    map(
        pair(
            alt((
                take_op_unicode,
                take_root_in_angle_bracket,
                take_op_in_angle_bracket,
            )),
            num_space,
        ),
        |(x, y)| Token::Op(x, y),
    )(s)
}

fn take_op_unicode(s: &str) -> IResult<&str, String> {
    map_res(anychar, |c| match c {
        '√' => Ok(r"\sqrt".to_string()),
        '∛' => Ok(r"\sqrt[3]".to_string()),
        '∜' => Ok(r"\sqrt[4]".to_string()),
        _ => Err(()),
    })(s)
}

fn take_op_in_angle_bracket(s: &str) -> IResult<&str, String> {
    map(
        delimited(
            pair(tag("<'"), opt(is_a(" "))),
            alpha1,
            pair(opt(is_a(" ")), tag(">")),
        ),
        tex_of_maybe_abbreviated_op_name,
    )(s)
}

fn take_root_in_angle_bracket(s: &str) -> IResult<&str, String> {
    map(
        delimited(
            tuple((
                tag("<'"),
                opt(is_a(" ")),
                alt((tag("root"), tag("sqrt"))),
                opt(is_a(" ")),
            )),
            digit1,
            pair(opt(is_a(" ")), tag(">")),
        ),
        |x| format!(r"\root[{}]", x),
    )(s)
}

fn tex_of_maybe_abbreviated_op_name(s: &str) -> String {
    match s {
        _ => format!(r"\{}", s),
    }
}
