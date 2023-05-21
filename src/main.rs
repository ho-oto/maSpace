pub mod expr;
pub mod token;

use expr::parse;
use token::tokenize;

pub fn maspace(input: &str) -> Result<String, String> {
    parse(&tokenize(input).map_err(|x| format!("{:?}", x))?)
        .map(|x| x.to_string())
        .map_err(|x| format!("{:?}", x))
}

fn main() {
    println!(
        "{:#?}",
        tokenize(r"a + bᵃ⁺ᵇ⁼ᶜₕₒ/c <alpha>[<beta hat>^2] `(X)`   5_/ 123")
    );
    println!(
        "{}",
        parse(&tokenize(r"a + bᵃ⁺ᵇ⁼ᶜₕₒ/c <alpha>[<beta hat>^2] `(X)`   5_/ 123").unwrap()).unwrap()
    );
}
