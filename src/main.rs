pub mod expr;
pub mod token;

use expr::parse;
use token::tokenize;

pub fn maspace_to_tex(input: &str) -> Result<String, String> {
    parse(&tokenize(input).map_err(|x| format!("{:?}", x))?)
        .map(|x| x.to_string())
        .map_err(|x| format!("{:?}", x))
}

fn main() {
    println!(
        "{}",
        maspace_to_tex(r"a + bᵃ⁺ᵇ⁼ᶜₕₒ/c <alpha>[<beta hat>^2] `(X)`   5_/ 123").unwrap()
    );
    println!("{}", maspace_to_tex(r"a _b_c  ^d ^e+f _g  /h").unwrap());
    println!("{}", maspace_to_tex(r"a _b_c^d ^[e+f _g/h]").unwrap());
    println!("{}", maspace_to_tex(r"a  _b _c^d  ^e  +  f_g/h").unwrap());
}
