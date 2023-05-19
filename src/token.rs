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
