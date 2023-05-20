pub mod binop;
pub mod open_close;
pub mod symbol;
pub mod unaryop;
pub mod unicode_subsup;
pub mod util;

use nom::branch::alt;
use nom::combinator::{eof, not};
use nom::multi::many0;
use nom::sequence::{preceded, terminated};
use unicode_normalization::UnicodeNormalization;

use binop::take_binop;
use open_close::{take_close, take_open};
use symbol::take_symbol;
use unaryop::take_op;
use unicode_subsup::{take_unicode_sub, take_unicode_sup};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Cat(usize),
    Sub(usize),
    Sup(usize),
    Over(usize),
    Under(usize),
    Root(usize),
    Frac(usize),
    Op(String, usize),
    Open(String),
    Close(String),
    Symbol(String),
    UnicodeSub(Box<Token>),
    UnicodeSup(Box<Token>),
}

impl Token {
    pub fn order(&self) -> usize {
        match self {
            Self::Cat(ord)
            | Self::Sub(ord)
            | Self::Sup(ord)
            | Self::Over(ord)
            | Self::Under(ord)
            | Self::Frac(ord)
            | Self::Op(_, ord) => *ord,
            _ => 0,
        }
    }
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, ()> {
    // normalize
    let s = s.nfd().to_string();
    let s = s.trim();
    // tokenize
    let (_, t) = terminated(
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
    )(&s)
    .map_err(|_| ())?;
    // remove unicode sub/sup
    let mut t2 = vec![];
    enum Mode {
        Sup,
        Sub,
        Normal,
    }
    let mut mode = Mode::Normal;
    for x in t {
        match x {
            Token::UnicodeSub(y) => {
                match mode {
                    Mode::Normal => {
                        t2.push(Token::Sub(0));
                        t2.push(Token::Open(".".to_string()));
                    }
                    Mode::Sub => {}
                    Mode::Sup => {
                        t2.push(Token::Close(".".to_string()));
                        t2.push(Token::Sub(0));
                        t2.push(Token::Open(".".to_string()));
                    }
                };
                t2.push(*y);
                mode = Mode::Sub;
            }
            Token::UnicodeSup(y) => {
                match mode {
                    Mode::Normal => {
                        t2.push(Token::Sup(0));
                        t2.push(Token::Open(".".to_string()));
                    }
                    Mode::Sub => {
                        t2.push(Token::Close(".".to_string()));
                        t2.push(Token::Sup(0));
                        t2.push(Token::Open(".".to_string()));
                    }
                    Mode::Sup => {}
                };
                t2.push(*y);
                mode = Mode::Sup;
            }
            _ => {
                match mode {
                    Mode::Sub | Mode::Sup => t2.push(Token::Close(".".to_string())),
                    Mode::Normal => {}
                }
                t2.push(x);
                mode = Mode::Normal;
            }
        };
    }
    match mode {
        Mode::Sub | Mode::Sup => t2.push(Token::Close(".".to_string())),
        Mode::Normal => {}
    }
    // insert Cat(0) between adjacent symbol
    let mut t3 = vec![];
    let mut after_symbol = false;
    for x in t2 {
        match x {
            Token::Symbol(_) => {
                if after_symbol {
                    t3.push(Token::Cat(0));
                }
                t3.push(x);
                after_symbol = true;
            }
            Token::Open(_) => {
                if after_symbol {
                    t3.push(Token::Cat(0));
                }
                t3.push(x);
                after_symbol = false;
            }
            Token::Close(_) => {
                t3.push(x);
                after_symbol = true;
            }
            _ => {
                t3.push(x);
                after_symbol = false;
            }
        }
    }
    Ok(t3)
}
