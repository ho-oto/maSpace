use super::token::Token;

use std::fmt::Display;

pub struct Math(Vec<Root>);

impl Math {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        todo!()
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
pub enum Root {
    Root { root: Frac, body: Frac },
    Math { body: Frac },
}

impl Root {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (tokens, frac_first) = Frac::parse(tokens, order, order_max)?;
        let Some((sep, tokens)) = tokens.split_first() else {
            return Ok((
                tokens,
                Self::Math {
                    body: frac_first
                },
            ))
        };
        if *sep != Token::Root(order) {
            return Err(());
        }
        let (tokens, frac_second) = Frac::parse(tokens, order, order_max)?;
        Ok((
            tokens,
            Self::Root {
                root: frac_first,
                body: frac_second,
            },
        ))
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root { root, body } => write!(f, "\\root[{{{}}}]{{{}}}", root, body)?,
            Self::Math { body } => write!(f, "{}", body)?,
        }
        Ok(())
    }
}

pub enum Frac {
    Frac { nume: Stack, denom: Stack },
    Math { body: Stack },
}

impl Frac {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (tokens, stack_first) = Stack::parse(tokens, order, order_max)?;
        let Some((sep, tokens)) = tokens.split_first() else {
            return Ok((
                tokens,
                Self::Math {
                    body: stack_first
                }
            ))
        };
        if *sep != Token::Frac(order) {
            return Err(());
        }
        let (tokens, stack_second) = Stack::parse(tokens, order, order_max)?;
        Ok((
            tokens,
            Self::Frac {
                nume: stack_first,
                denom: stack_second,
            },
        ))
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

pub struct Stack {
    body: Inter,
    over: Option<Inter>,
    under: Option<Inter>,
}

impl Stack {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (tokens, body) = Inter::parse(tokens, order, order_max)?;
        let Some((sep_first, tokens)) = tokens.split_first() else {
            return Ok((tokens, Self { body, over: None, under: None }));
        };
        let (tokens, inter_first) = Inter::parse(tokens, order, order_max)?;
        let Some((sep_second, tokens)) = tokens.split_first() else {
            if *sep_first == Token::Over(order) {
                let over = Some(inter_first);
                return Ok((tokens, Self { body, over, under: None }));
            } else if *sep_first == Token::Under(order) {
                let under = Some(inter_first);
                return Ok((tokens, Self { body, over: None, under }));
            } else {
                return Err(());
            }
        };
        let (tokens, inter_second) = Inter::parse(tokens, order, order_max)?;
        if *sep_first == Token::Over(order) && *sep_second == Token::Under(order) {
            let over = Some(inter_first);
            let under = Some(inter_second);
            Ok((tokens, Self { body, over, under }))
        } else if *sep_first == Token::Under(order) && *sep_second == Token::Over(order) {
            let over = Some(inter_second);
            let under = Some(inter_first);
            Ok((tokens, Self { body, over, under }))
        } else {
            Err(())
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

pub struct Inter {
    body: Simple,
    sup: Option<Simple>,
    sub: Option<Simple>,
}

impl Inter {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (tokens, body) = Simple::parse(tokens, order, order_max)?;
        let Some((sep_first, tokens)) = tokens.split_first() else {
            return Ok((tokens, Self { body, sup: None, sub: None }));
        };
        let (tokens, simple_first) = Simple::parse(tokens, order, order_max)?;
        let Some((sep_second, tokens)) = tokens.split_first() else {
            if *sep_first == Token::Sup(order) {
                let sup = Some(simple_first);
                return Ok((tokens, Self { body, sup, sub: None }));
            } else if *sep_first == Token::Sub(order) {
                let sub = Some(simple_first);
                return Ok((tokens, Self { body, sup: None, sub }));
            } else {
                return Err(());
            }
        };
        let (tokens, simple_second) = Simple::parse(tokens, order, order_max)?;
        if *sep_first == Token::Sup(order) && *sep_second == Token::Sub(order) {
            let sup = Some(simple_first);
            let sub = Some(simple_second);
            Ok((tokens, Self { body, sup, sub }))
        } else if *sep_first == Token::Sub(order) && *sep_second == Token::Sup(order) {
            let sup = Some(simple_second);
            let sub = Some(simple_first);
            Ok((tokens, Self { body, sup, sub }))
        } else {
            Err(())
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
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (operator, tokens) = match tokens {
            [Token::Op(operator, ord), tokens @ ..] if *ord == order => {
                (Some(operator.to_owned()), tokens)
            }
            [] => return Err(()),
            _ => (None, tokens),
        };
        if order == 0 {
            match tokens {
                [Token::Symbol(symbol), tokens @ ..] => Ok((
                    tokens,
                    Self::UnarySymbol {
                        operator,
                        symbol: symbol.to_owned(),
                    },
                )),
                [Token::Open(open), tokens @ ..] => {
                    let (tokens, body) = Math::parse(tokens, order_max, order_max)?;
                    match tokens {
                        [Token::Close(close), tokens @ ..] => Ok((
                            tokens,
                            Self::UnaryParened {
                                operator,
                                open: open.to_owned(),
                                body,
                                close: close.to_owned(),
                            },
                        )),
                        _ => Err(()),
                    }
                }
                _ => Err(()),
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
            } => write!(f, "{{{}}}", body)?,
            Self::UnarySymbol {
                operator: Some(operator),
                symbol,
            } => write!(f, "{}{{{}}}", operator, symbol)?,
            Self::UnarySymbol {
                operator: None,
                symbol,
            } => write!(f, "{{{}}}", symbol)?,
            Self::UnaryParened {
                operator: Some(operator),
                open,
                body,
                close,
            } => write!(
                f,
                "{}{{\\left{}{{{}}}\\right{}}}",
                operator, open, body, close
            )?,
            Self::UnaryParened {
                operator: None,
                open,
                body,
                close,
            } => write!(f, "\\left{}{{{}}}\\right{}", open, body, close)?,
        }
        Ok(())
    }
}
