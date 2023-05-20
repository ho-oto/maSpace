pub mod expr;
pub mod token;

use expr::*;
use token::*;

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
