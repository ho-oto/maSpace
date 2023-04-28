pub enum Token {
    Cat(usize),
    Sub(usize),
    Sup(usize),
    Over(usize),
    Under(usize),
    Frac(usize),
    Op(String, usize),
    Open(String),
    Close(String),
    Symbol(String),
    UnicodeSub(Box<Token>),
    UnicodeSup(Box<Token>),
}
