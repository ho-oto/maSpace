use super::token::Token;

use std::fmt::Display;

pub enum Root {
    Root { root: Box<Frac>, body: Box<Frac> },
    Math { body: Box<Frac> },
    Symbol(String),
}

impl Root {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        let (tokens, frac_first) = Frac::parse(tokens, order, order_max)?;
        let Some((sep, tokens)) = tokens.split_first() else {
            return Ok((
                tokens,
                Self::Math { body: Box::new(frac_first) },
            ))
        };
        if *sep != Token::Root(order) {
            return Err(());
        }
        let (tokens, frac_second) = Frac::parse(tokens, order, order_max)?;
        Ok((
            tokens,
            Self::Root {
                root: Box::new(frac_first),
                body: Box::new(frac_second),
            },
        ))
    }
}

impl Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root { root, body } => write!(f, r"\root[{{{}}}]{{{}}}", root, body)?,
            Self::Math { body } => write!(f, "{}", body)?,
            Self::Symbol(s) => write!(f, "{}", s)?,
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
            return Ok((tokens, Self::Math { body: stack_first }))
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
            Self::Frac { nume, denom } => write!(f, r"\frac{{{}}}{{{}}}", nume, denom)?,
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
                return Ok((tokens, Self {body, over, under: None}));
            } else if *sep_first == Token::Under(order) {
                let under = Some(inter_first);
                return Ok((tokens, Self {body, over: None, under}));
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
                r"\underset{{{}}}{{\overset{{{}}}{{{}}}}}",
                under, over, body
            )?,
            Self {
                body,
                over: Some(over),
                under: None,
            } => write!(f, r"\overset{{{}}}{{{}}}", over, body)?,
            Self {
                body,
                over: None,
                under: Some(under),
            } => write!(f, r"\underset{{{}}}{{{}}}", under, body)?,
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
                return Ok((tokens, Self {body, sup, sub: None}));
            } else if *sep_first == Token::Sub(order) {
                let sub =Some(simple_first);
                return Ok((tokens, Self {body, sup: None, sub}));
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
            } => write!(f, r"{{{}}}^{{{}}}_{{{}}}", body, sup, sub)?,
            Self {
                body,
                sup: Some(sup),
                sub: None,
            } => write!(f, r"{{{}}}^{{{}}}", body, sup)?,
            Self {
                body,
                sup: None,
                sub: Some(sub),
            } => write!(f, r"{{{}}}_{{{}}}", body, sub)?,
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
        operator: String,
        body: Box<Root>,
    },
    Cat(Vec<Root>),
    Parened {
        open: String,
        body: Root,
        close: String,
    },
}

impl Simple {
    pub fn parse(tokens: &[Token], order: usize, order_max: usize) -> Result<(&[Token], Self), ()> {
        if tokens.is_empty() {
            return Err(());
        }
        if let Some((Token::Open(open), rest)) = tokens.split_first() {
            let (rest, body) = Root::parse(rest, order_max, order_max)?;
            if let Some((Token::Close(close), rest)) = rest.split_first() {
                return Ok((
                    rest,
                    Self::Parened {
                        open: open.to_string(),
                        body,
                        close: close.to_string(),
                    },
                ));
            } else {
                return Err(());
            }
        } else if let Some((Token::Op(operator, ord), rest)) = tokens.split_first() {
            if *ord != order {
                return Err(());
            }
            if order == 0 {
                todo!()
            } else {
                let (rest, body) = Root::parse(rest, order - 1, order_max)?;
                return Ok((
                    rest,
                    Self::UnaryExpr {
                        operator: operator.clone(),
                        body: Box::new(body),
                    },
                ));
            }
        }
        todo!()
    }
}

impl Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnaryExpr { operator, body } => write!(f, r"{}{{{}}}", operator, body)?,
            Self::Cat(v) => {
                for s in v {
                    write!(f, "{{{}}}", s)?
                }
            }
            Self::Parened { open, body, close } => {
                write!(f, r"\left{}{{{}}}\right{}", open, body, close)?
            }
        }
        Ok(())
    }
}
