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
        // Mathematical Bold
        '𝐀'..='𝐙' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝐀', 'A'))),
        '𝐚'..='𝐳' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝐚', 'a'))),
        '𝟎'..='𝟗' => Some(format!(r"\mathbf{{ {} }}", shifted_char(c, '𝟎', '0'))),
        // Mathematical Italic
        '𝐴'..='𝑍' => Some(format!(r"\mathit{{ {} }}", shifted_char(c, '𝐴', 'A'))),
        '𝑎'..='𝑧' => Some(format!(r"\mathit{{ {} }}", shifted_char(c, '𝑎', 'a'))),
        'ℎ' => Some(r"\mathit{ h }".to_string()),
        // Mathematical Bold Italic
        '𝑨'..='𝒁' => Some(format!(r"\bm{{ {} }}", shifted_char(c, '𝑨', 'A'))),
        '𝒂'..='𝒛' => Some(format!(r"\bm{{ {} }}", shifted_char(c, '𝒂', 'a'))),
        // Mathematical Script
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
        // Mathematical Bold Script
        '𝓐'..='𝓩' => Some(format!(
            r"\bm{{\mathscr{{ {} }}}}",
            shifted_char(c, '𝓐', 'A')
        )),
        '𝓪'..='𝔃' => Some(format!(
            r"\bm{{\mathscr{{ {} }}}}",
            shifted_char(c, '𝓪', 'a')
        )),
        // Mathematical Fraktur
        '𝔄'..='𝔜' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝔄', 'A'))),
        '𝔞'..='𝔷' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝔞', 'a'))),
        'ℭ' => Some(r"\mathfrak{ C }".to_string()),
        'ℌ' => Some(r"\mathfrak{ H }".to_string()),
        'ℑ' => Some(r"\mathfrak{ I }".to_string()),
        'ℜ' => Some(r"\mathfrak{ R }".to_string()),
        'ℨ' => Some(r"\mathfrak{ Z }".to_string()),
        // Mathematical Double-Struck
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
        // Mathematical Bold Fraktur => Mathematical Fraktur
        '𝕬'..='𝖅' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝕬', 'A'))),
        '𝖆'..='𝖟' => Some(format!(r"\mathfrak{{ {} }}", shifted_char(c, '𝖆', 'a'))),
        // Mathematical Sans-Serif
        '𝖠'..='𝖹' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝖠', 'A'))),
        '𝖺'..='𝗓' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝖺', 'a'))),
        // Mathematical {Bold,Italic} Sans-Serif => Mathematical Sans-Serif
        '𝗔'..='𝗭' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝗔', 'A'))),
        '𝗮'..='𝘇' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝗮', 'a'))),
        '𝘈'..='𝘡' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘈', 'A'))),
        '𝘢'..='𝘻' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘢', 'a'))),
        '𝘼'..='𝙕' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝘼', 'A'))),
        '𝙖'..='𝙯' => Some(format!(r"\mathsf{{ {} }}", shifted_char(c, '𝙖', 'a'))),
        // Mathematical Monospace
        _ => None,
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
