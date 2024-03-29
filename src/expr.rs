use super::token::Token;

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

impl std::error::Error for ParseError {}

pub fn parse(tokens: &[Token]) -> Result<Math, ParseError> {
    let order_max = tokens.iter().map(|x| x.order()).max().ok_or(ParseError {
        description: "Input tokens are empty".to_string(),
        unconsumed_tokens: tokens.to_owned(),
    })?;
    let (rest, math) = Math::parse(tokens, order_max, order_max)?;
    if !rest.is_empty() {
        let err_message = |x, y| {
            format!(
                "Double {}: use bracket to clarify a group \
                or change the number of spaces around \"{}\"",
                x, y
            )
        };
        return Err(ParseError {
            description: match rest {
                [Token::Frac(_), ..] => err_message("\"/\"", "/"),
                [Token::Root(_), ..] => err_message("\"_/\"", "_/"),
                [Token::Over(_), ..] => err_message("over", "^^"),
                [Token::Under(_), ..] => err_message("under", "__"),
                [Token::Sup(_), ..] => err_message("exponent", "^"),
                [Token::Sub(_), ..] => err_message("subscripts", "_"),
                _ => "Some tokens are unconsumed".to_string(),
            },
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
        if roots.is_empty() {
            write!(f, "{{}}")?;
            return Ok(());
        }
        for w in roots.windows(2) {
            if w[1].to_string().starts_with(|c| match c {
                'A'..='Z' | 'a'..='z' => true,
                _ => false,
            }) {
                write!(f, "{}", w[0])?;
            } else {
                write!(f, "{}", w[0].to_string().trim_end_matches(' '))?;
            }
        }
        if let Some(x) = roots.last() {
            write!(f, "{}", x)?;
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
            Self::Root { root, body } => write!(
                f,
                "\\sqrt[{}]{{{}}}",
                root.to_string().trim_end_matches(' '),
                body.to_string().trim_end_matches(' ')
            )?,
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
            Self::Frac { nume, denom } => write!(
                f,
                "\\frac{{{}}}{{{}}}",
                nume.to_string().trim_end_matches(' '),
                denom.to_string().trim_end_matches(' ')
            )?,
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
                under.to_string().trim_end_matches(' '),
                over.to_string().trim_end_matches(' '),
                body.to_string().trim_end_matches(' ')
            )?,
            Self {
                body,
                over: Some(over),
                under: None,
            } => write!(
                f,
                "\\overset{{{}}}{{{}}}",
                over.to_string().trim_end_matches(' '),
                body.to_string().trim_end_matches(' ')
            )?,
            Self {
                body,
                over: None,
                under: Some(under),
            } => write!(
                f,
                "\\underset{{{}}}{{{}}}",
                under.to_string().trim_end_matches(' '),
                body.to_string().trim_end_matches(' ')
            )?,
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
            } => {
                write!(f, "{}", body.to_string().trim_end_matches(' '))?;
                write!(f, "^{{{}}}", sup.to_string().trim_end_matches(' '))?;
                write!(f, "_{{{}}}", sub.to_string().trim_end_matches(' '))?;
            }
            Self {
                body,
                sup: Some(sup),
                sub: None,
            } => {
                write!(f, "{}", body.to_string().trim_end_matches(' '))?;
                write!(f, "^{{{}}}", sup.to_string().trim_end_matches(' '))?;
            }
            Self {
                body,
                sup: None,
                sub: Some(sub),
            } => {
                write!(f, "{}", body.to_string().trim_end_matches(' '))?;
                write!(f, "_{{{}}}", sub.to_string().trim_end_matches(' '))?;
            }
            Self {
                body,
                sup: None,
                sub: None,
            } => {
                write!(f, "{}", body)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Simple {
    UnaryExpr {
        operators: Vec<String>,
        body: Math,
    },
    UnarySymbol {
        operators: Vec<String>,
        symbol: String,
    },
    UnaryParened {
        operators: Vec<String>,
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
        let mut tokens = tokens;
        let mut operators = vec![];
        loop {
            tokens = match tokens {
                [] => {
                    return Err(ParseError {
                        description: "Failed to parse Simple: Missing symbol \
                        or expression enclosed in brackets"
                            .to_string(),
                        unconsumed_tokens: tokens.to_owned(),
                    })
                }
                [Token::Op(operator, ord), tokens @ ..] if *ord == order => {
                    operators.push(operator.to_owned());
                    tokens
                }
                _ => break,
            };
        }
        if order == 0 {
            match tokens {
                [Token::Symbol(symbol), tokens @ ..] => {
                    let symbol = symbol.to_owned();
                    Ok((tokens, Self::UnarySymbol { operators, symbol }))
                }
                [Token::Open(open), tokens @ ..] => {
                    let (tokens, body) = Math::parse(tokens, order_max, order_max)?;
                    match tokens {
                        [Token::Close(close), tokens @ ..] => {
                            let (open, close) = (open.to_owned(), close.to_owned());
                            Ok((
                                tokens,
                                Self::UnaryParened {
                                    operators,
                                    open,
                                    body,
                                    close,
                                },
                            ))
                        }
                        _ => Err(ParseError {
                            description: "Failed to parse Simple: No closing bracket".to_string(),
                            unconsumed_tokens: tokens.to_owned(),
                        }),
                    }
                }
                _ => Err(ParseError {
                    description: "Failed to parse Simple: Missing symbol \
                        or expression enclosed in brackets"
                        .to_string(),
                    unconsumed_tokens: tokens.to_owned(),
                }),
            }
        } else {
            let (tokens, body) = Math::parse(tokens, order - 1, order_max)?;
            Ok((tokens, Self::UnaryExpr { operators, body }))
        }
    }
}

impl Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operators = match self {
            Self::UnaryExpr { operators, .. } => operators,
            Self::UnaryParened { operators, .. } => operators,
            Self::UnarySymbol { operators, .. } => operators,
        };
        let fmt_op = |x: String| {
            if operators.is_empty() {
                return x;
            }
            let mut y = format!("{}", x.trim_end_matches(' '));
            for z in operators.iter().rev() {
                y = format!("{}{{{}}}", z, y);
            }
            y
        };
        match self {
            Self::UnaryExpr { body, .. } => write!(f, "{}", fmt_op(body.to_string()))?,
            Self::UnaryParened {
                open, body, close, ..
            } => {
                let (open, close) = match (open.as_str(), close.as_str()) {
                    ("", "") => (format!(""), format!("")),
                    ("", close) => (format!("\\left."), format!("\\right{}", close)),
                    (open, "") => (format!("\\left{}", open), format!("\\right.")),
                    (open, close) => (format!("\\left{}", open), format!("\\right{}", close)),
                };
                write!(f, "{}", open)?;
                write!(f, "{}", body.to_string().trim_end_matches(' '))?;
                write!(f, "{}", close)?;
            }
            Self::UnarySymbol { operators, symbol } if operators.is_empty() => {
                write!(f, "{}", symbol)?
            }
            Self::UnarySymbol { symbol, .. } => write!(f, "{}", fmt_op(symbol.to_string()))?,
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
            r"a_{b_{c}}".to_string()
        );
    }
}
