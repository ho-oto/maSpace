pub mod symbol;
pub mod token;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, alphanumeric1, anychar, char, digit1, none_of, one_of},
    combinator::{map, map_res, not, opt, peek},
    error::ParseError,
    multi::{fold_many0, many0, many1},
    sequence::{delimited, pair, tuple},
    IResult, Parser,
};
use std::iter;
use token::Token;
use unicode_normalization::UnicodeNormalization;

fn get_tex_op_from_char(c: char) -> Result<String, ()> {
    Ok(match c {
        '√' => r"\sqrt",
        '∛' => r"\root[3]",
        '∜' => r"\root[4]",
        _ => return Err(()),
    }
    .to_string())
}

fn get_tex_open_from_char(c: char) -> Result<String, ()> {
    Ok(match c {
        '(' => "(",
        '[' => ".",
        '{' | '⟨' | '⌈' | '⌊' | '⎰' | '⌜' | '⌞' | '⟦' => return Err(()),
        _ => return Err(()),
    }
    .to_string())
}

fn expand_abbred_symbol(s: &str) -> String {
    if s.len() == 1 {
        return s.to_string();
    }
    match s {
        _ => s.to_string(),
    }
}

fn expand_abbred_op(s: &str) -> String {
    match s {
        _ => s.to_string(),
    }
}

fn expand_abbred_literal_suffix(s: &str) -> String {
    match s {
        _ => s.to_string(),
    }
}

fn get_sub(c: char) -> Result<char, ()> {
    match c {
        '₊' | '₋' | '₌' | '₍' | '₎' | '₀' | '₁' | '₂' | '₃' | '₄' | '₅' | '₆' | '₇' | '₈' | '₉'
        | 'ₐ' | 'ₑ' | 'ₕ' | 'ᵢ' | 'ⱼ' | 'ₖ' | 'ₗ' | 'ₘ' | 'ₙ' | 'ₒ' | 'ₚ' | 'ᵣ' | 'ₛ' | 'ₜ'
        | 'ᵤ' | 'ᵥ' | 'ₓ' | 'ᵦ' | 'ᵧ' | 'ᵨ' | 'ᵩ' | 'ᵪ' => {
            iter::once(c).nfkc().next().ok_or(())
        }
        _ => Err(()),
    }
}

fn get_sup(c: char) -> Result<char, ()> {
    match c {
        '⁺' | '⁻' | '⁼' | '⁽' | '⁾' | '⁰' | '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹'
        | 'ᴬ' | 'ᴮ' | 'ᴰ' | 'ᴱ' | 'ᴳ' | 'ᴴ' | 'ᴵ' | 'ᴶ' | 'ᴷ' | 'ᴸ' | 'ᴹ' | 'ᴺ' | 'ᴼ' | 'ᴾ'
        | 'ᴿ' | 'ᵀ' | 'ᵁ' | 'ⱽ' | 'ᵂ' | 'ᵃ' | 'ᵇ' | 'ᶜ' | 'ᵈ' | 'ᵉ' | 'ᵍ' | 'ʰ' | 'ⁱ' | 'ʲ'
        | 'ᵏ' | 'ˡ' | 'ᵐ' | 'ⁿ' | 'ᵒ' | 'ᵖ' | 'ʳ' | 'ˢ' | 'ᵗ' | 'ᵘ' | 'ᵛ' | 'ʷ' | 'ˣ' | 'ʸ'
        | 'ᶻ' | 'ᵝ' | 'ᵞ' | '\u{1D5F}' | 'ᶿ' | 'ᵠ' | 'ᵡ' => {
            iter::once(c).nfkc().next().ok_or(())
        }
        'ᵅ' => Ok('α'),
        'ᵋ' => Ok('ε'),
        'ᶥ' => Ok('ι'),
        'ᶲ' => Ok('ϕ'),
        'ꜛ' => Ok('↑'),
        'ꜜ' => Ok('↓'),
        'ꜝ' => Ok('!'),
        _ => Err(()),
    }
}

fn num_space<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, usize, E> {
    fold_many0(char(' '), || 0, |x, _| x + 1)(s)
}

fn max_space_around<'a, R, F, E>(s: &'a str, parser: F) -> IResult<&'a str, usize, E>
where
    F: Parser<&'a str, R, E>,
    E: ParseError<&'a str>,
{
    let (s, (left, _, right)) = tuple((num_space, parser, num_space))(s)?;
    Ok((s, left.max(right)))
}

fn take_sub(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('_')).map(|(s, n)| (s, Token::Sub(n)))
}
fn take_under(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("__")).map(|(s, n)| (s, Token::Under(n)))
}
fn take_sup(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('^')).map(|(s, n)| (s, Token::Sup(n)))
}
fn take_over(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("^^")).map(|(s, n)| (s, Token::Over(n)))
}
fn take_frac(s: &str) -> IResult<&str, Token> {
    max_space_around(s, one_of("/∕")).map(|(s, n)| (s, Token::Frac(n)))
}

fn take_cat(s: &str) -> IResult<&str, Token> {
    num_space(s).map(|(s, n)| (s, Token::Cat(n)))
}


fn symbol_alphabet(s: &str) -> IResult<&str, Token> {
    todo!()
}


fn take_op_unicode(s: &str) -> IResult<&str, Token> {
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

fn take_op_ascii(s: &str) -> IResult<&str, Token> {
    let (s, (_, base, ascii_props, order)) = tuple((
        char('@'),
        alpha1,
        many0(map(pair(char('.'), alphanumeric1), |(_, x)| x)),
        num_space,
    ))(s)?;
    let tex = format!(r"\{}[{}]", expand_abbred_op(base), ascii_props.join(","));
    Ok((s, Token::Op(tex, order)))
}

fn take_op(s: &str) -> IResult<&str, Token> {
    alt((take_op_ascii, take_op_unicode))(s)
}

fn tokenize(input: &str) -> Vec<Token> {
    todo!()
}

struct Expr {}

fn parse(input: Vec<Token>) -> IResult<Vec<Token>, Expr> {
    todo!()
}

fn main() {
    let hoge = "    / fdfd";
    let (_, b) = take_frac(hoge).unwrap();
    assert_eq!(if let Token::Frac(x) = b { x } else { 0 }, 4);
    let hoge = "#⊗̇";
    //let (_, b) = take_symbol_unicode(hoge).unwrap();
    assert_eq!(
        if let Token::Symbol(x) = b {
            x
        } else {
            panic!()
        },
        r"\dot{ \otimes }"
    );
}
