use super::token::Token;
use super::util::*;

use nom::{character::complete::one_of, sequence::pair, IResult};

pub fn take_op_unicode(s: &str) -> IResult<&str, Token> {
    let (s, (t, order)) = pair(one_of("√∛∜"), num_space)(s)?;
    Ok((
        s,
        match t {
            '√' => Token::Op(format!(r"\sqrt"), order),
            '∛' => Token::Op(format!(r"\sqrt[3]"), order),
            '∜' => Token::Op(format!(r"\sqrt[4]"), order),
            _ => unreachable!(),
        },
    ))
}
