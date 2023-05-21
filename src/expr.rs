use super::token::Token;

use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    description: String,
    unconsumed_tokens: Vec<Token>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.description)?;
        writeln!(f, "unconsumed_tokens {:?}", self.unconsumed_tokens)?;
        Ok(())
    }
}

impl Error for ParseError {}

pub fn parse(tokens: &[Token]) -> Result<Math, ParseError> {
    let order_max = tokens.iter().map(|x| x.order()).max().ok_or(ParseError {
        description: "input tokens are empty".to_string(),
        unconsumed_tokens: tokens.to_owned(),
    })?;
    let (rest, math) = Math::parse(tokens, order_max, order_max)?;
    if !rest.is_empty() {
        return Err(ParseError {
            description: "some tokens are unconsumed".to_string(),
            unconsumed_tokens: rest.to_owned(),
        });
    }
    Ok(math)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Math(Vec<Root>);

impl Math {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let mut roots = vec![];
        let mut tokens = tokens;
        loop {
            let (rest, root) = Root::parse(tokens, order, order_max)?;
            roots.push(root);
            tokens = match rest {
                [Token::Cat(ord), tokens @ ..] if *ord == order => tokens,
                _ => return Ok((rest, Self(roots))),
            };
        }
    }
}

impl Display for Math {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(roots) = self;
        for root in roots {
            write!(f, "{}", root)?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Root {
    Root { root: Frac, body: Frac },
    Math { body: Frac },
}

impl Root {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let (tokens, frac_first) = Frac::parse(tokens, order, order_max)?;
        match tokens {
            [Token::Root(ord), tokens @ ..] if *ord == order => {
                let (tokens, frac_second) = Frac::parse(tokens, order, order_max)?;
                Ok((
                    tokens,
                    Self::Root {
                        root: frac_first,
                        body: frac_second,
                    },
                ))
            }
            _ => Ok((tokens, Self::Math { body: frac_first })),
        }
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root { root, body } => write!(f, "\\sqrt[{}]{{{}}}", root, body)?,
            Self::Math { body } => write!(f, "{}", body)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Frac {
    Frac { nume: Stack, denom: Stack },
    Math { body: Stack },
}

impl Frac {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let (tokens, stack_first) = Stack::parse(tokens, order, order_max)?;
        match tokens {
            [Token::Frac(ord), tokens @ ..] if *ord == order => {
                let (tokens, stack_second) = Stack::parse(tokens, order, order_max)?;
                Ok((
                    tokens,
                    Self::Frac {
                        nume: stack_first,
                        denom: stack_second,
                    },
                ))
            }
            _ => Ok((tokens, Self::Math { body: stack_first })),
        }
    }
}

impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Frac { nume, denom } => write!(f, "\\frac{{{}}}{{{}}}", nume, denom)?,
            Self::Math { body } => write!(f, "{}", body)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Stack {
    body: Inter,
    over: Option<Inter>,
    under: Option<Inter>,
}

impl Stack {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let (tokens, body) = Inter::parse(tokens, order, order_max)?;
        match tokens {
            [Token::Over(ord), tokens @ ..] if *ord == order => {
                let (tokens, over) = Inter::parse(tokens, order, order_max)?;
                let (over, under) = (Some(over), None);
                match tokens {
                    [Token::Under(ord), tokens @ ..] if *ord == order => {
                        let (tokens, under) = Inter::parse(tokens, order, order_max)?;
                        let under = Some(under);
                        Ok((tokens, Self { body, over, under }))
                    }
                    _ => Ok((tokens, Self { body, over, under })),
                }
            }
            [Token::Under(ord), tokens @ ..] if *ord == order => {
                let (tokens, under) = Inter::parse(tokens, order, order_max)?;
                let (over, under) = (None, Some(under));
                match tokens {
                    [Token::Over(ord), tokens @ ..] if *ord == order => {
                        let (tokens, over) = Inter::parse(tokens, order, order_max)?;
                        let over = Some(over);
                        Ok((tokens, Self { body, over, under }))
                    }
                    _ => Ok((tokens, Self { body, over, under })),
                }
            }
            _ => {
                let (over, under) = (None, None);
                Ok((tokens, Self { body, over, under }))
            }
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self {
                body,
                over: Some(over),
                under: Some(under),
            } => write!(
                f,
                "\\underset{{{}}}{{\\overset{{{}}}{{{}}}}}",
                under, over, body
            )?,
            Self {
                body,
                over: Some(over),
                under: None,
            } => write!(f, "\\overset{{{}}}{{{}}}", over, body)?,
            Self {
                body,
                over: None,
                under: Some(under),
            } => write!(f, "\\underset{{{}}}{{{}}}", under, body)?,
            Self {
                body,
                over: None,
                under: None,
            } => write!(f, "{}", body)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Inter {
    body: Simple,
    sup: Option<Simple>,
    sub: Option<Simple>,
}

impl Inter {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let (tokens, body) = Simple::parse(tokens, order, order_max)?;
        match tokens {
            [Token::Sup(ord), tokens @ ..] if *ord == order => {
                let (tokens, sup) = Simple::parse(tokens, order, order_max)?;
                let (sup, sub) = (Some(sup), None);
                match tokens {
                    [Token::Sub(ord), tokens @ ..] if *ord == order => {
                        let (tokens, sub) = Simple::parse(tokens, order, order_max)?;
                        let sub = Some(sub);
                        Ok((tokens, Self { body, sup, sub }))
                    }
                    _ => Ok((tokens, Self { body, sup, sub })),
                }
            }
            [Token::Sub(ord), tokens @ ..] if *ord == order => {
                let (tokens, sub) = Simple::parse(tokens, order, order_max)?;
                let (sup, sub) = (None, Some(sub));
                match tokens {
                    [Token::Sup(ord), tokens @ ..] if *ord == order => {
                        let (tokens, sup) = Simple::parse(tokens, order, order_max)?;
                        let sup = Some(sup);
                        Ok((tokens, Self { body, sup, sub }))
                    }
                    _ => Ok((tokens, Self { body, sup, sub })),
                }
            }
            _ => {
                let (sup, sub) = (None, None);
                Ok((tokens, Self { body, sup, sub }))
            }
        }
    }
}

impl Display for Inter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self {
                body,
                sup: Some(sup),
                sub: Some(sub),
            } => write!(f, "{{{}}}^{{{}}}_{{{}}}", body, sup, sub)?,
            Self {
                body,
                sup: Some(sup),
                sub: None,
            } => write!(f, "{{{}}}^{{{}}}", body, sup)?,
            Self {
                body,
                sup: None,
                sub: Some(sub),
            } => write!(f, "{{{}}}_{{{}}}", body, sub)?,
            Self {
                body,
                sup: None,
                sub: None,
            } => write!(f, "{}", body)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Simple {
    UnaryExpr {
        operator: Option<String>,
        body: Math,
    },
    UnarySymbol {
        operator: Option<String>,
        symbol: String,
    },
    UnaryParened {
        operator: Option<String>,
        open: String,
        body: Math,
        close: String,
    },
}

impl Simple {
    pub fn parse<'a>(
        tokens: &'a [Token],
        order: usize,
        order_max: usize,
    ) -> Result<(&'a [Token], Self), ParseError> {
        let (operator, tokens) = match tokens {
            [] => {
                return Err(ParseError {
                    description: "failed to parse Simple: tokens is empty".to_string(),
                    unconsumed_tokens: tokens.to_owned(),
                })
            }
            [Token::Op(operator, ord), tokens @ ..] if *ord == order => {
                (Some(operator.to_owned()), tokens)
            }
            _ => (None, tokens),
        };
        if order == 0 {
            match tokens {
                [Token::Symbol(symbol), tokens @ ..] => {
                    let symbol = symbol.to_owned();
                    Ok((tokens, Self::UnarySymbol { operator, symbol }))
                }
                [Token::Open(open), tokens @ ..] => {
                    let (tokens, body) = Math::parse(tokens, order_max, order_max)?;
                    match tokens {
                        [Token::Close(close), tokens @ ..] => {
                            let (open, close) = (open.to_owned(), close.to_owned());
                            Ok((
                                tokens,
                                Self::UnaryParened {
                                    operator,
                                    open,
                                    body,
                                    close,
                                },
                            ))
                        }
                        _ => Err(ParseError {
                            description: "failed to parse Simple".to_string(),
                            unconsumed_tokens: tokens.to_owned(),
                        }),
                    }
                }
                _ => Err(ParseError {
                    description: "failed to parse Simple".to_string(),
                    unconsumed_tokens: tokens.to_owned(),
                }),
            }
        } else {
            let (tokens, body) = Math::parse(tokens, order - 1, order_max)?;
            Ok((tokens, Self::UnaryExpr { operator, body }))
        }
    }
}

impl Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnaryExpr {
                operator: Some(operator),
                body,
            } => write!(f, "{}{{{}}}", operator, body)?,
            Self::UnaryExpr {
                operator: None,
                body,
            } => write!(f, "{}", body)?,
            Self::UnarySymbol {
                operator: Some(operator),
                symbol,
            } => write!(f, "{}{{{}}}", operator, symbol)?,
            Self::UnarySymbol {
                operator: None,
                symbol,
            } => write!(f, "{}", symbol)?,
            Self::UnaryParened {
                operator: Some(operator),
                open,
                body,
                close,
            } => write!(f, "{}{{\\left{}{}\\right{}}}", operator, open, body, close)?,
            Self::UnaryParened {
                operator: None,
                open,
                body,
                close,
            } => write!(f, "\\left{}{}\\right{}", open, body, close)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_math() {
        let x = [
            Token::Symbol("a".to_string()),
            Token::Sub(1),
            Token::Symbol("b".to_string()),
            Token::Sub(0),
            Token::Symbol("c".to_string()),
        ];
        assert_eq!(
            Math::parse(&x, 1, 1).unwrap().1.to_string(),
            r"{a}_{{b}_{c}}".to_string()
        );
    }
}
