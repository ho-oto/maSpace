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
    let result = parse(&tokens)
        .context("parse failed")?
        .to_string()
        .trim_end_matches(' ')
        .to_owned();
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
            r"a+\frac{b^{a+b=c}_{ho}}{c}\alpha\hat{\beta}^{2}\otimes\sqrt[5]{1.23}"
        );
        assert_eq!(
            maspace_to_tex(r"a _b_c  ^d ^e+f _g  /h").unwrap(),
            r"\frac{a_{b_{c}}^{d^{e+f}_{g}}}{h}"
        );
        assert_eq!(
            maspace_to_tex(r"a _b_c^d ^[e+f _g/h]").unwrap(),
            r"a^{e+f_{\frac{g}{h}}}_{b^{d}_{c}}"
        );
        assert_eq!(
            maspace_to_tex(r"a  _b _c^d  ^e  +  f_g/h").unwrap(),
            r"a^{e}_{b_{c^{d}}}+\frac{f_{g}}{h}"
        );
    }
}
