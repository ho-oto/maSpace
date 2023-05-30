pub mod expr;
pub mod token;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use anyhow::{Context, Result};

use expr::parse;
use token::tokenize;

pub fn maspace_to_tex(input: &str) -> Result<String> {
    let tokens = tokenize(input).context("tokenize failed")?;
    let result = parse(&tokens).context("parse failed")?.to_string();
    let result = result
        .split_ascii_whitespace()
        .into_iter()
        .collect::<Vec<_>>()
        .join(" ");
    Ok(result)
}

#[wasm_bindgen]
pub fn maspace_to_tex_wasm(input: &str) -> Result<String, String> {
    maspace_to_tex(input).map_err(|x| x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            maspace_to_tex(r"a + bᵃ⁺ᵇ⁼ᶜₕₒ/c <alpha>[<beta hat>^2] `(X)`   5_/ 1.23").unwrap(),
            "a + \\frac{ { b }^{ \\left. a + b = c \\right. }_{ \\left. h o \\right. } }\
            { c } \\alpha \\left. { \\hat{ \\beta } }^{ 2 } \\right. \\otimes \\sqrt[ 5 ]{ 1.23 }"
        );
        assert_eq!(
            maspace_to_tex(r"a _b_c  ^d ^e+f _g  /h").unwrap(),
            r"\frac{ { { a }_{ { b }_{ c } } }^{ { d }^{ e + f }_{ g } } }{ h }"
        );
        assert_eq!(
            maspace_to_tex(r"a _b_c^d ^[e+f _g/h]").unwrap(),
            r"{ a }^{ \left. { e + f }_{ \frac{ g }{ h } } \right. }_{ { b }^{ d }_{ c } }"
        );
        assert_eq!(
            maspace_to_tex(r"a  _b _c^d  ^e  +  f_g/h").unwrap(),
            r"{ a }^{ e }_{ { b }_{ { c }^{ d } } } + \frac{ { f }_{ g } }{ h }"
        );
    }
}
