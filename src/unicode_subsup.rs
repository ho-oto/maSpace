use super::open_close::*;
use super::symbol::*;
use super::token::*;

use std::iter::once;

use nom::{branch::alt, character::complete::anychar, combinator::map_res, IResult};
use unicode_normalization::UnicodeNormalization;

pub fn take_unicode_sub(s: &str) -> IResult<&str, Token> {
    map_res(anychar, |c| match c {
        '₊' | '₋' | '₌' | '₍' | '₎' | '₀' | '₁' | '₂' | '₃' | '₄' | '₅' | '₆' | '₇' | '₈' | '₉'
        | 'ₐ' | 'ₑ' | 'ₕ' | 'ᵢ' | 'ⱼ' | 'ₖ' | 'ₗ' | 'ₘ' | 'ₙ' | 'ₒ' | 'ₚ' | 'ᵣ' | 'ₛ' | 'ₜ'
        | 'ᵤ' | 'ᵥ' | 'ₓ' | 'ᵦ' | 'ᵧ' | 'ᵨ' | 'ᵩ' | 'ᵪ' => {
            Ok(Token::UnicodeSub(Box::new(char_to_token(c)?)))
        }
        _ => Err(()),
    })(s)
}

pub fn take_unicode_sup(s: &str) -> IResult<&str, Token> {
    fn sup(x: &str) -> Token {
        Token::UnicodeSup(Box::new(Token::Symbol(x.to_string())))
    }
    map_res(anychar, |c| match c {
        '⁺' | '⁻' | '⁼' | '⁽' | '⁾' | '⁰' | '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹'
        | 'ᴬ' | 'ᴮ' | 'ᴰ' | 'ᴱ' | 'ᴳ' | 'ᴴ' | 'ᴵ' | 'ᴶ' | 'ᴷ' | 'ᴸ' | 'ᴹ' | 'ᴺ' | 'ᴼ' | 'ᴾ'
        | 'ᴿ' | 'ᵀ' | 'ᵁ' | 'ⱽ' | 'ᵂ' | 'ᵃ' | 'ᵇ' | 'ᶜ' | 'ᵈ' | 'ᵉ' | 'ᵍ' | 'ʰ' | 'ⁱ' | 'ʲ'
        | 'ᵏ' | 'ˡ' | 'ᵐ' | 'ⁿ' | 'ᵒ' | 'ᵖ' | 'ʳ' | 'ˢ' | 'ᵗ' | 'ᵘ' | 'ᵛ' | 'ʷ' | 'ˣ' | 'ʸ'
        | 'ᶻ' | 'ᵝ' | 'ᵞ' | '\u{1D5F}' | 'ᶿ' | 'ᵠ' | 'ᵡ' => {
            Ok(Token::UnicodeSup(Box::new(char_to_token(c)?)))
        }
        'ᵅ' => Ok(sup(r"\alpha")),
        'ᵋ' => Ok(sup(r"\varepsilon")),
        'ᶥ' => Ok(sup(r"\iota")),
        'ᶲ' => Ok(sup(r"\phi")),
        'ꜛ' => Ok(sup(r"\uparrow")),
        'ꜜ' => Ok(sup(r"\downarrow")),
        'ꜝ' => Ok(sup(r"!")),
        _ => Err(()),
    })(s)
}

fn char_to_token(c: char) -> Result<Token, ()> {
    let s = once(c).nfkc().to_string();
    let s = alt((take_symbol, take_open, take_close))(&s);
    match s {
        Ok((_, x)) => Ok(x),
        Err(_) => Err(()),
    }
}
