use super::token::Token;
use super::util::*;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar},
    combinator::{map, map_res},
    sequence::{delimited, pair},
    IResult,
};

pub fn take_op_unicode(s: &str) -> IResult<&str, Token> {
    map(
        pair(map_res(anychar, tex_of_unicode_op), num_space),
        |(x, y)| Token::Op(x, y),
    )(s)
}

pub fn take_op_in_angle_bracket(s: &str) -> IResult<&str, Token> {
    map(
        pair(
            map_res(delimited(tag("<<"), alphanumeric1, tag(">>")), tex_of_ascii),
            num_space,
        ),
        |(x, y)| Token::Op(x, y),
    )(s)
}

fn tex_of_unicode_op(c: char) -> Result<String, ()> {
    Ok(match c {
        '√' => r"\sqrt",
        '∛' => r"\sqrt[3]",
        '∜' => r"\sqrt[4]",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_ascii(c: &str) -> Result<String, ()> {
    Ok(match c {
        "sqrt" | "root" => r"\sqrt",
        "root3" => r"\sqrt[3]",
        "root4" => r"\sqrt[4]",
        _ => return Err(()),
    }
    .to_string())
}
