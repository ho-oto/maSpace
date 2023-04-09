use std::iter::once;
use unicode_normalization::{is_nfkc_quick, IsNormalized, UnicodeNormalization};

fn unicode_char_to_tex(c: char) -> Option<String> {
    let shifted_char = |character: char, base: char, ascii_base: char| {
        char::from_u32(u32::from(character) - u32::from(base) + u32::from(ascii_base)).unwrap()
    };
    match c {
        // ASCII
        'A'..='Z'
        | 'a'..='z'
        | '!'
        | ','
        | ';'
        | '?'
        | '@'
        | '*'
        | '+'
        | '-'
        | ':'
        | '<'
        | '='
        | '>'
        | '|' => Some(c.to_string()),
        '$' | '%' | '&' => Some(format!(r"\{}", c)),
        '~' => Some(r"\sim".to_string()),
        // Styled alphabets
        // - Mathematical Bold
        '𝐀'..='𝐙' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝐀', 'A'))),
        '𝐚'..='𝐳' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝐚', 'a'))),
        '𝟎'..='𝟗' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝟎', '0'))),
        // - Mathematical Italic
        '𝐴'..='𝑍' => Some(format!(r"\mathit{{ {} }}", shifted_char(c, '𝐴', 'A'))),
        '𝑎'..='𝑧' => Some(format!(r"\mathit{{ {} }}", shifted_char(c, '𝑎', 'a'))),
        'ℎ' => Some(r"\mathit{ h }".to_string()),
        // - Mathematical Bold Italic
        '𝑨'..='𝒁' => Some(format!(r"\bm{{ {} }}", shifted_char(c, '𝑨', 'A'))),
        '𝒂'..='𝒛' => Some(format!(r"\bm{{ {} }}", shifted_char(c, '𝒂', 'a'))),
        // - Mathematical Script
        '𝒜'..='𝒵' => Some(format!(r"\mathscr{{ {} }}", shifted_char(c, '𝒜', 'A'))),
        '𝒶'..='𝓏' => Some(format!(r"\mathscr{{ {} }}", shifted_char(c, '𝒶', 'a'))),
        'ℬ' => Some(r"\mathscr{ B }".to_string()),
        'ℰ' => Some(r"\mathscr{ E }".to_string()),
        'ℱ' => Some(r"\mathscr{ F }".to_string()),
        'ℋ' => Some(r"\mathscr{ H }".to_string()),
        'ℐ' => Some(r"\mathscr{ I }".to_string()),
        'ℒ' => Some(r"\mathscr{ L }".to_string()),
        'ℳ' => Some(r"\mathscr{ M }".to_string()),
        'ℛ' => Some(r"\mathscr{ R }".to_string()),
        'ℯ' => Some(r"\mathscr{ e }".to_string()),
        'ℊ' => Some(r"\mathscr{ g }".to_string()),
        'ℴ' => Some(r"\mathscr{ o }".to_string()),
        // - Mathematical Bold Script
        '𝓐'..='𝓩' => Some(format!(
            r"\bm{{\mathscr{{ {} }}}}",
            shifted_char(c, '𝓐', 'A')
        )),
        '𝓪'..='𝔃' => Some(format!(
            r"\bm{{\mathscr{{ {} }}}}",
            shifted_char(c, '𝓪', 'a')
        )),
        // - Mathematical Fraktur
        '𝔄'..='𝔜' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝔄', 'A'))),
        '𝔞'..='𝔷' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝔞', 'a'))),
        'ℭ' => Some(r"\mathfrak{ C }".to_string()),
        'ℌ' => Some(r"\mathfrak{ H }".to_string()),
        'ℑ' => Some(r"\mathfrak{ I }".to_string()),
        'ℜ' => Some(r"\mathfrak{ R }".to_string()),
        'ℨ' => Some(r"\mathfrak{ Z }".to_string()),
        // - Mathematical Double-Struck
        '𝔸'..='𝕐' => Some(format!(r"\mathbb{{ {} }}", shifted_char(c, '𝔸', 'A'))),
        '𝕒'..='𝕫' => Some(format!(r"\mathbb{{ {} }}", shifted_char(c, '𝕒', 'a'))),
        '𝟘'..='𝟡' => Some(format!(r"\mathbb{{ {} }}", shifted_char(c, '𝟘', '0'))),
        'ℂ' => Some(r"\mathbb{ C }".to_string()),
        'ℍ' => Some(r"\mathbb{ H }".to_string()),
        'ℕ' => Some(r"\mathbb{ N }".to_string()),
        'ℙ' => Some(r"\mathbb{ P }".to_string()),
        'ℚ' => Some(r"\mathbb{ Q }".to_string()),
        'ℝ' => Some(r"\mathbb{ R }".to_string()),
        'ℤ' => Some(r"\mathbb{ Z }".to_string()),
        // - Mathematical Bold Fraktur => Mathematical Fraktur
        '𝕬'..='𝖅' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝕬', 'A'))),
        '𝖆'..='𝖟' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝖆', 'a'))),
        // - Mathematical Sans-Serif
        '𝖠'..='𝖹' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝖠', 'A'))),
        '𝖺'..='𝗓' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝖺', 'a'))),
        '𝟢'..='𝟫' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝟢', '0'))),
        // - Mathematical {Bold,Italic} Sans-Serif => Mathematical Sans-Serif
        '𝗔'..='𝗭' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝗔', 'A'))),
        '𝗮'..='𝘇' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝗮', 'a'))),
        '𝘈'..='𝘡' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘈', 'A'))),
        '𝘢'..='𝘻' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘢', 'a'))),
        '𝘼'..='𝙕' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘼', 'A'))),
        '𝙖'..='𝙯' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝙖', 'a'))),
        '𝟬'..='𝟵' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝟬', '0'))),
        // - Mathematical Monospace
        '𝙰'..='𝚉' => Some(format!(r"\mathtt{{ {} }}", shifted_char(c, '𝙰', 'A'))),
        '𝚊'..='𝚣' => Some(format!(r"\mathtt{{ {} }}", shifted_char(c, '𝚊', 'a'))),
        '𝟶'..='𝟿' => Some(format!(r"\mathtt{{ {} }}", shifted_char(c, '𝟶', '0'))),
        // Greek alphabets
        'Α' => Some(r"\Alpha".to_string()),
        'Β' => Some(r"\Beta".to_string()),
        'Γ' => Some(r"\Gamma".to_string()),
        'Δ' => Some(r"\Delta".to_string()),
        'Ε' => Some(r"\Epsilon".to_string()),
        'Ζ' => Some(r"\Zeta".to_string()),
        'Η' => Some(r"\Eta".to_string()),
        'Θ' | 'ϴ' => Some(r"\Theta".to_string()),
        'Ι' => Some(r"\Iota".to_string()),
        'Κ' => Some(r"\Kappa".to_string()),
        'Λ' => Some(r"\Lambda".to_string()),
        'Μ' => Some(r"\Mu".to_string()),
        'Ν' => Some(r"\Nu".to_string()),
        'Ξ' => Some(r"\Xi".to_string()),
        'Ο' => Some(r"\Omicron".to_string()),
        'Π' => Some(r"\Pi".to_string()),
        'Ρ' => Some(r"\Rho".to_string()),
        'Σ' => Some(r"\Sigma".to_string()),
        'Τ' => Some(r"\Tau".to_string()),
        'Υ' => Some(r"\Upsilon".to_string()),
        'Φ' => Some(r"\Phi".to_string()),
        'Χ' => Some(r"\Chi".to_string()),
        'Ψ' => Some(r"\Psi".to_string()),
        'Ω' => Some(r"\Omega".to_string()),
        'α' => Some(r"\alpha".to_string()),
        'β' => Some(r"\beta".to_string()),
        'γ' => Some(r"\gamma".to_string()),
        'δ' => Some(r"\delta".to_string()),
        'ε' => Some(r"\varepsilon".to_string()),
        'ζ' => Some(r"\zeta".to_string()),
        'η' => Some(r"\eta".to_string()),
        'θ' => Some(r"\theta".to_string()),
        'ι' => Some(r"\iota".to_string()),
        'κ' => Some(r"\kappa".to_string()),
        'λ' => Some(r"\lambda".to_string()),
        'μ' => Some(r"\mu".to_string()),
        'ν' => Some(r"\nu".to_string()),
        'ξ' => Some(r"\xi".to_string()),
        'ο' => Some(r"\omicron".to_string()),
        'π' => Some(r"\pi".to_string()),
        'ρ' => Some(r"\rho".to_string()),
        'ς' => Some(r"\varsigma".to_string()),
        'σ' => Some(r"\sigma".to_string()),
        'τ' => Some(r"\tau".to_string()),
        'υ' => Some(r"\upsilon".to_string()),
        'φ' => Some(r"\varphi".to_string()),
        'χ' => Some(r"\chi".to_string()),
        'ψ' => Some(r"\psi".to_string()),
        'ω' => Some(r"\omega".to_string()),
        'ϵ' => Some(r"\epsilon".to_string()),
        'ϑ' => Some(r"\vartheta".to_string()),
        'ϰ' => Some(r"\varkappa".to_string()),
        'ϕ' => Some(r"\phi".to_string()),
        'ϱ' => Some(r"\varrho".to_string()),
        'ϖ' => Some(r"\varpi".to_string()),
        'ϝ' => Some(r"\digamma".to_string()),
        // Styled
        '𝛢'..='𝛲' => unicode_char_to_tex(shifted_char(c, '𝛢', 'Α')),
        '𝛳' => Some(r"\Theta".to_string()),
        '𝛴'..='𝛺' => unicode_char_to_tex(shifted_char(c, '𝛴', 'Σ')),
        '𝛼'..='𝜔' => unicode_char_to_tex(shifted_char(c, '𝛼', 'α')),
        '𝜖' => Some(r"\epsilon".to_string()),
        '𝜗' => Some(r"\vartheta".to_string()),
        '𝜘' => Some(r"\varkappa".to_string()),
        '𝜙' => Some(r"\phi".to_string()),
        '𝜚' => Some(r"\varrho".to_string()),
        '𝜛' => Some(r"\varpi".to_string()),
        // - Bold
        '𝚨'..='𝚸' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝚨', 'Α'))?
        )),
        '𝜜'..='𝜬' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝜜', 'Α'))?
        )),
        '𝝖'..='𝝦' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝝖', 'Α'))?
        )),
        '𝞐'..='𝞠' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝞐', 'Α'))?
        )),
        '𝚹' | '𝜭' | '𝝧' | '𝞡' => Some(r"\bm{ \Theta }".to_string()),
        '𝚺'..='𝛀' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝚺', 'Σ'))?
        )),
        '𝜮'..='𝜴' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝜮', 'Σ'))?
        )),
        '𝝨'..='𝝮' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝝨', 'Σ'))?
        )),
        '𝞢'..='𝞨' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝞢', 'Σ'))?
        )),
        '𝛂'..='𝛚' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝛂', 'α'))?
        )),
        '𝜶'..='𝝎' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝜶', 'α'))?
        )),
        '𝝰'..='𝞈' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝝰', 'α'))?
        )),
        '𝞪'..='𝟂' => Some(format!(
            r"\bm{{ {} }}",
            unicode_char_to_tex(shifted_char(c, '𝞪', 'α'))?
        )),
        '𝛜' | '𝝐' | '𝞊' | '𝟄' => Some(r"\bm{ \epsilon }".to_string()),
        '𝛝' | '𝝑' | '𝞋' | '𝟅' => Some(r"\bm{ \vartheta }".to_string()),
        '𝛞' | '𝝒' | '𝞌' | '𝟆' => Some(r"\bm{ \varkappa }".to_string()),
        '𝛟' | '𝝓' | '𝞍' | '𝟇' => Some(r"\bm{ \phi }".to_string()),
        '𝛠' | '𝝔' | '𝞎' | '𝟈' => Some(r"\bm{ \varrho }".to_string()),
        '𝛡' | '𝝕' | '𝞏' | '𝟉' => Some(r"\bm{ \varpi }".to_string()),
        '𝟋' => Some(r"\bm{ \digamma }".to_string()),
        '𝛻' => Some(r"\nabla".to_string()),
        '𝜕' => Some(r"\partial".to_string()),
        '𝛁' | '𝜵' | '𝝯' | '𝞩' => Some(r"\bm{ \nabla }".to_string()),
        '𝛛' | '𝝏' | '𝞉' | '𝟃' => Some(r"\bm{ \partial }".to_string()),
        '𝚤' => Some(r"\imath".to_string()),
        'ı' => Some(r"\text{\i}".to_string()),
        '𝚥' => Some(r"\jmath".to_string()),
        'ȷ' => Some(r"\text{\j}".to_string()),
        _ => match is_nfkc_quick(once(c)) {
            IsNormalized::Yes => None,
            _ => unicode_char_to_tex(once(c).nfkc().next()?),
        },
    }
}

fn unicode_accent_to_tex() -> String {
    "aaa".to_string()
}

fn unicode_sub_to_ascii() -> String {
    "aaa".to_string()
}

fn unicode_sup_to_ascii() -> String {
    "aaa".to_string()
}
fn main() {
    assert_eq!(1, 1);
}
