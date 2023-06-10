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

#[derive(Debug, PartialEq, Eq, Clone)]
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
            | Self::Root(ord)
            | Self::Frac(ord)
            | Self::Op(_, ord) => *ord,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct TokenizeError {
    description: String,
    detail: Option<String>,
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.description)?;
        if let Some(detail) = &self.detail {
            writeln!(f, "{}", detail)?;
        }
        Ok(())
    }
}

impl std::error::Error for TokenizeError {}

pub fn tokenize(s: &str) -> Result<Vec<Token>, TokenizeError> {
    // normalize
    let s = s.nfd().to_string();
    let s = s.trim();
    // tokenize
    let (_, nom_outputs) = terminated(
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
    .map_err(|x| TokenizeError {
        description: "tokenize failed".to_string(),
        detail: Some(format!("{:?}", x)),
    })?;
    // remove unicode sub/sup
    let mut no_unicode_sub_sup = vec![];
    enum Mode {
        Sup,
        Sub,
        Normal,
    }
    let mut mode = Mode::Normal;
    for x in nom_outputs {
        match x {
            Token::UnicodeSub(y) => {
                match mode {
                    Mode::Normal => {
                        no_unicode_sub_sup.push(Token::Sub(0));
                        no_unicode_sub_sup.push(Token::Open(".".to_string()));
                    }
                    Mode::Sub => {}
                    Mode::Sup => {
                        no_unicode_sub_sup.push(Token::Close(".".to_string()));
                        no_unicode_sub_sup.push(Token::Sub(0));
                        no_unicode_sub_sup.push(Token::Open(".".to_string()));
                    }
                };
                no_unicode_sub_sup.push(*y);
                mode = Mode::Sub;
            }
            Token::UnicodeSup(y) => {
                match mode {
                    Mode::Normal => {
                        no_unicode_sub_sup.push(Token::Sup(0));
                        no_unicode_sub_sup.push(Token::Open(".".to_string()));
                    }
                    Mode::Sub => {
                        no_unicode_sub_sup.push(Token::Close(".".to_string()));
                        no_unicode_sub_sup.push(Token::Sup(0));
                        no_unicode_sub_sup.push(Token::Open(".".to_string()));
                    }
                    Mode::Sup => {}
                };
                no_unicode_sub_sup.push(*y);
                mode = Mode::Sup;
            }
            _ => {
                match mode {
                    Mode::Sub | Mode::Sup => no_unicode_sub_sup.push(Token::Close(".".to_string())),
                    Mode::Normal => {}
                }
                no_unicode_sub_sup.push(x);
                mode = Mode::Normal;
            }
        };
    }
    match mode {
        Mode::Sub | Mode::Sup => no_unicode_sub_sup.push(Token::Close(".".to_string())),
        Mode::Normal => {}
    }
    // insert Cat(0) between adjacent symbol
    let mut cat_inserted = vec![];
    let mut after_symbol = false;
    for x in no_unicode_sub_sup {
        match x {
            Token::Symbol(_) => {
                if after_symbol {
                    cat_inserted.push(Token::Cat(0));
                }
                cat_inserted.push(x);
                after_symbol = true;
            }
            Token::Open(_) => {
                if after_symbol {
                    cat_inserted.push(Token::Cat(0));
                }
                cat_inserted.push(x);
                after_symbol = false;
            }
            Token::Close(_) => {
                cat_inserted.push(x);
                after_symbol = true;
            }
            _ => {
                cat_inserted.push(x);
                after_symbol = false;
            }
        }
    }
    let max_order = cat_inserted
        .iter()
        .map(|x| x.order())
        .max()
        .unwrap_or_default();
    Ok(cat_inserted
        .into_iter()
        .map(|x| match x {
            Token::Cat(ord) if ord > 0 => Token::Cat(max_order),
            _ => x,
        })
        .collect::<Vec<_>>())
}
