use super::token::Token;

use std::{fmt::Display, iter::once};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, digit1},
    combinator::{flat_map, map, map_res, opt},
    multi::{count, fold_many0, many0, many1, many_till},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use unicode_normalization::UnicodeNormalization;

pub fn take_symbol(s: &str) -> IResult<&str, Token> {
    map(
        pair(
            alt((
                take_symbol_from_single_char,
                take_symbol_from_ascii_art,
                take_symbol_in_angle_brackets,
                take_string_literal_in_angle_brackets,
                take_string_literal_plain,
            )),
            opt(is_a("'")),
        ),
        |(tex, prime)| Token::Symbol(tex + prime.unwrap_or_default()),
    )(s)
}

////

fn take_symbol_from_single_char(s: &str) -> IResult<&str, String> {
    flat_map(map_res(anychar, tex_of_char), |tex| {
        fold_many0(
            map_res(anychar, tex_of_unicode_accent),
            move || String::from(&tex),
            |tex, accent| format!("{}{{{}}}", accent, tex),
        )
    })(s)
}

fn take_symbol_from_ascii_art(s: &str) -> IResult<&str, String> {
    delimited(
        tag("`"),
        map_res(take_until("`"), tex_of_ascii_art),
        tag("`"),
    )(s)
}

fn take_symbol_in_angle_brackets(s: &str) -> IResult<&str, String> {
    fn take_symbol_from_alpha2(s: &str) -> IResult<&str, String> {
        map_res(alpha1, |x: &str| {
            (x.len() > 1)
                .then_some(tex_of_maybe_abbreviated_symbol_name(x))
                .ok_or(())
        })(s)
    }
    fn take_symbol_from_single_char_in_brackets(s: &str) -> IResult<&str, String> {
        flat_map(
            map_res(anychar, |c| match c {
                '^' => Ok(r"\^".to_string()),
                '_' => Ok(r"\_".to_string()),
                '/' => Ok("/".to_string()),
                _ => tex_of_char(c),
            }),
            |tex| {
                fold_many0(
                    map_res(anychar, tex_of_unicode_accent),
                    move || String::from(&tex),
                    |tex, accent| format!("{}{{{}}}", accent, tex),
                )
            },
        )(s)
    }
    fn take_number_in_brackets(s: &str) -> IResult<&str, String> {
        map(
            tuple((
                digit1,
                opt(map(preceded(tag("."), digit1), |x| format!(".{}", x))),
            )),
            |(integer, decimal): (&str, Option<String>)| {
                format!("{}{}", integer, decimal.unwrap_or_default(),)
            },
        )(s)
    }
    flat_map(
        preceded(
            pair(tag("<"), many0(tag(" "))),
            alt((
                take_symbol_from_alpha2,
                take_symbol_from_ascii_art,
                take_number_in_brackets,
                take_symbol_from_single_char_in_brackets,
            )),
        ),
        |tex| {
            terminated(
                fold_many0(
                    map(
                        alt((
                            preceded(many0(tag(" ")), tag("!")),
                            preceded(many1(tag(" ")), alphanumeric1),
                        )),
                        tex_of_maybe_abbreviated_accent_name,
                    ),
                    move || String::from(&tex),
                    |tex, accent| format!("{}{{{}}}", accent, tex),
                ),
                pair(many0(tag(" ")), tag(">")),
            )
        },
    )(s)
}

fn take_string_literal_plain(s: &str) -> IResult<&str, String> {
    map_res(take_string_literal_content, |c| {
        resolve_string_literal_accent(&c, vec![])
    })(s)
}

fn take_string_literal_in_angle_brackets(s: &str) -> IResult<&str, String> {
    flat_map(
        preceded(
            pair(tag("<"), many0(tag(" "))),
            pair(
                alt((take_string_literal_content, take_raw_string_literal_content)),
                opt(alpha1),
            ),
        ),
        |(content, accent)| {
            terminated(
                map_res(
                    fold_many0(
                        preceded(many1(tag(" ")), alpha1),
                        move || accent.into_iter().collect::<Vec<_>>(),
                        |accents, a| accents.into_iter().chain(once(a)).collect(),
                    ),
                    move |accents| resolve_string_literal_accent(&content, accents),
                ),
                pair(many0(tag(" ")), tag(">")),
            )
        },
    )(s)
}

////

fn tex_of_char(c: char) -> Result<String, ()> {
    fn nfkc(c: char) -> Result<char, ()> {
        once(c).nfkc().next().ok_or(())
    }
    fn raw(c: char) -> String {
        c.to_string()
    }
    fn sym<T: Display>(s: T) -> String {
        format!(r"\{}", s)
    }
    fn cmb<T: Display>(op: &str, arg: T) -> String {
        format!(r"\{}{{ {} }}", op, arg)
    }

    Ok(match c {
        // - ASCII
        '!' | '*' | '+' | ',' | '.' | '-' | ':' | ';' | '=' | '?' | '@' | '|' => raw(c),
        'A'..='Z' | 'a'..='z' | '0'..='9' => raw(c),
        '#' | '$' | '%' | '&' => sym(c),
        '\\' => sym("backslash"),
        '~' => sym("sim"),
        // rest:
        //   ␠, ", ', (, ), /, <, >, [, ], ^, _, `, {, }
        // - Greek alphabets
        //   * capital
        'Α' => sym("Alpha"),
        'Β' => sym("Beta"),
        'Γ' => sym("Gamma"),
        'Δ' => sym("Delta"),
        'Ε' => sym("Epsilon"),
        'Ζ' => sym("Zeta"),
        'Η' => sym("Eta"),
        'Θ' => sym("Theta"),
        'Ι' => sym("Iota"),
        'Κ' => sym("Kappa"),
        'Λ' => sym("Lambda"),
        'Μ' => sym("Mu"),
        'Ν' => sym("Nu"),
        'Ξ' => sym("Xi"),
        'Ο' => sym("Omicron"),
        'Π' => sym("Pi"),
        'Ρ' => sym("Rho"),
        // '\u3a2' is unassigned
        'Σ' => sym("Sigma"),
        'Τ' => sym("Tau"),
        'Υ' => sym("Upsilon"),
        'Φ' => sym("Phi"),
        'Χ' => sym("Chi"),
        'Ψ' => sym("Psi"),
        'Ω' => sym("Omega"),
        //   * small
        'α' => sym("alpha"),
        'β' => sym("beta"),
        'γ' => sym("gamma"),
        'δ' => sym("delta"),
        'ε' => sym("varepsilon"),
        'ζ' => sym("zeta"),
        'η' => sym("eta"),
        'θ' => sym("theta"),
        'ι' => sym("iota"),
        'κ' => sym("kappa"),
        'λ' => sym("lambda"),
        'μ' => sym("mu"),
        'ν' => sym("nu"),
        'ξ' => sym("xi"),
        'ο' => sym("omicron"),
        'π' => sym("pi"),
        'ρ' => sym("rho"),
        'ς' => sym("varsigma"),
        'σ' => sym("sigma"),
        'τ' => sym("tau"),
        'υ' => sym("upsilon"),
        'φ' => sym("varphi"),
        'χ' => sym("chi"),
        'ψ' => sym("psi"),
        'ω' => sym("omega"),
        //   * variants
        'ϵ' => sym("epsilon"),
        'ϑ' => sym("vartheta"),
        'ϰ' => sym("varkappa"),
        'ϕ' => sym("phi"),
        'ϱ' => sym("varrho"),
        'ϖ' => sym("varpi"),
        'ϝ' => sym("digamma"),
        'ϴ' => sym("varTheta"),
        'ɸ' => sym("phi"), // Latin phi -> phi
        // - Mathematical Alphanumeric Symbols (1D400-1D7FF)
        //   - Alphabet
        '𝐀'..='𝐙' | '𝐚'..='𝐳' | '𝟎'..='𝟗' => cmb("mathbf", nfkc(c)?),
        '𝐴'..='𝑍' | '𝑎'..='𝑧' | 'ℎ' => cmb("mathit", nfkc(c)?),
        '𝑨'..='𝒁' | '𝒂'..='𝒛' => cmb("mathbfit", nfkc(c)?),
        '𝒜'..='𝒵' | '𝒶'..='𝓏' => cmb("mathscr", nfkc(c)?),
        'ℬ' | 'ℰ' | 'ℱ' | 'ℋ' | 'ℐ' | 'ℒ' | 'ℳ' | 'ℛ' => cmb("mathscr", nfkc(c)?),
        'ℯ' | 'ℊ' | 'ℴ' => cmb("mathscr", nfkc(c)?),
        '𝓐'..='𝓩' | '𝓪'..='𝔃' => cmb("mathbfscr", nfkc(c)?),
        '𝔄'..='𝔜' | '𝔞'..='𝔷' => cmb("mathfrak", nfkc(c)?),
        'ℭ' | 'ℌ' | 'ℑ' | 'ℜ' | 'ℨ' => cmb("mathfrak", nfkc(c)?),
        '𝔸'..='𝕐' | '𝕒'..='𝕫' | '𝟘'..='𝟡' => cmb("mathbb", nfkc(c)?),
        'ℂ' | 'ℍ' | 'ℕ' | 'ℙ' | 'ℚ' | 'ℝ' | 'ℤ' => cmb("mathbb", nfkc(c)?),
        '𝕬'..='𝖅' | '𝖆'..='𝖟' => cmb("mathbffrak", nfkc(c)?),
        '𝖠'..='𝖹' | '𝖺'..='𝗓' | '𝟢'..='𝟫' => cmb("mathsf", nfkc(c)?),
        '𝗔'..='𝗭' | '𝗮'..='𝘇' | '𝟬'..='𝟵' => cmb("mathbfsf", nfkc(c)?),
        '𝘈'..='𝘡' | '𝘢'..='𝘻' => cmb("mathsfit", nfkc(c)?),
        '𝘼'..='𝙕' | '𝙖'..='𝙯' => cmb("mathbfsfit", nfkc(c)?),
        '𝙰'..='𝚉' | '𝚊'..='𝚣' | '𝟶'..='𝟿' => cmb("mathtt", nfkc(c)?),
        //     * Dotless
        '𝚤' => sym("imath"),
        '𝚥' => sym("jmath"),
        //   - Greek alphabets
        //   ignore Bold/Italic style
        '𝛢'..='𝜛' | '𝚨'..='𝛡' | '𝜜'..='𝝕' | '𝝖'..='𝞏' | '𝞐'..='𝟉' | '𝟋' => {
            tex_of_char(nfkc(c)?)?
        }
        'ı' => cmb("text", 'ı'),
        'ȷ' => cmb("text", 'ȷ'),
        // - Symbols
        // '§' => sym("S"),
        '¬' => sym("neg"),
        '®' => sym("circledR"),
        '±' => sym("pm"),
        '×' => sym("times"),
        'ð' => sym("eth"),
        '÷' => sym("div"),
        'ħ' => sym("hbar"),
        '϶' => sym("backepsilon"),
        '†' => sym("dagger"),
        '‡' => sym("ddagger"),
        '…' => sym("ldots"),
        'ℏ' => sym("hslash"),
        'ℓ' => sym("ell"),
        '℘' => sym("wp"),
        '℧' => sym("mho"),
        'Ⅎ' => sym("Finv"),
        'ℵ' => sym("aleph"),
        'ℶ' => sym("beth"),
        'ℷ' => sym("gimel"),
        'ℸ' => sym("daleth"),
        '⅁' => sym("Game"),
        '←' => sym("leftarrow"),
        '↑' => sym("uparrow"),
        '→' => sym("rightarrow"),
        '↓' => sym("downarrow"),
        '↔' => sym("leftrightarrow"),
        '↕' => sym("updownarrow"),
        '↖' => sym("nwarrow"),
        '↗' => sym("nearrow"),
        '↘' => sym("searrow"),
        '↙' => sym("swarrow"),
        '↞' => sym("twoheadleftarrow"),
        '↠' => sym("twoheadrightarrow"),
        '↢' => sym("leftarrowtail"),
        '↣' => sym("rightarrowtail"),
        '↦' => sym("mapsto"),
        '↩' => sym("hookleftarrow"),
        '↪' => sym("hookrightarrow"),
        '↫' => sym("looparrowleft"),
        '↬' => sym("looparrowright"),
        '↭' => sym("leftrightsquigarrow"),
        '↰' => sym("Lsh"),
        '↱' => sym("Rsh"),
        '↶' => sym("curvearrowleft"),
        '↷' => sym("curvearrowright"),
        '↺' => sym("circlearrowleft"),
        '↻' => sym("circlearrowright"),
        '↼' => sym("leftharpoonup"),
        '↽' => sym("leftharpoondown"),
        '↾' => sym("upharpoonright"),
        '↿' => sym("upharpoonleft"),
        '⇀' => sym("rightharpoonup"),
        '⇁' => sym("rightharpoondown"),
        '⇂' => sym("downharpoonright"),
        '⇃' => sym("downharpoonleft"),
        '⇄' => sym("rightleftarrows"),
        '⇆' => sym("leftrightarrows"),
        '⇇' => sym("leftleftarrows"),
        '⇈' => sym("upuparrows"),
        '⇉' => sym("rightrightarrows"),
        '⇊' => sym("downdownarrows"),
        '⇋' => sym("leftrightharpoons"),
        '⇌' => sym("rightleftharpoons"),
        '⇐' => sym("Leftarrow"),
        '⇑' => sym("Uparrow"),
        '⇒' => sym("Rightarrow"),
        '⇓' => sym("Downarrow"),
        '⇔' => sym("Leftrightarrow"),
        '⇕' => sym("Updownarrow"),
        '⇚' => sym("Lleftarrow"),
        '⇛' => sym("Rrightarrow"),
        '⇝' => sym("rightsquigarrow"),
        '⇠' => sym("dashleftarrow"),
        '⇢' => sym("dashrightarrow"),
        '∀' => sym("forall"),
        '∁' => sym("complement"),
        '∂' => sym("partial"),
        '∃' => sym("exists"),
        '∅' => sym("emptyset"),
        '∆' => sym("bigtriangleup"), // increment -> bigtriangleup
        '∇' => sym("nabla"),
        '∈' | '∊' => sym("in"),
        '∋' | '∍' => sym("ni"),
        '∎' => sym("blacksquare"),
        '∏' => sym("prod"),
        '∐' => sym("coprod"),
        '∑' => sym("sum"),
        '−' => raw('-'),
        '∓' => sym("mp"),
        '∔' => sym("dotplus"),
        '∖' => sym("setminus"),
        '∗' => sym("ast"),
        '∘' => sym("circ"),
        '∙' => sym("bullet"),
        '∝' => sym("propto"),
        '∞' => sym("infty"),
        '∠' => sym("angle"),
        '∡' => sym("measuredangle"),
        '∢' => sym("sphericalangle"),
        '∣' => sym("mid"),
        '∥' => sym("parallel"),
        '∧' => sym("wedge"),
        '∨' => sym("vee"),
        '∩' => sym("cap"),
        '∪' => sym("cup"),
        '∫' => sym("int"),
        '∬' => sym("iint"),
        '∭' => sym("iiint"),
        '∮' => sym("oint"),
        '∴' => sym("therefore"),
        '∵' => sym("because"),
        '∶' => raw(':'),
        '∷' => sym("dblcolon"),
        '∸' => cmb("dot", '-'),
        '∹' => sym("eqcolon"),
        '∼' => sym("sim"),
        '∽' => sym("backsim"),
        '≀' => sym("wr"),
        '≂' => sym("eqsim"),
        '≃' => sym("simeq"),
        '≅' => sym("cong"),
        '≈' => sym("approx"),
        '≊' => sym("approxeq"),
        '≍' => sym("asymp"),
        '≎' => sym("Bumpeq"),
        '≏' => sym("bumpeq"),
        '≐' => sym("doteq"),
        '≑' => sym("Doteq"),
        '≒' => sym("fallingdotseq"),
        '≓' => sym("risingdotseq"),
        '≔' => sym("coloneqq"),
        '≕' => sym("eqqcolon"),
        '≖' => sym("eqcirc"),
        '≗' => sym("circeq"),
        '≜' => sym("triangleq"),
        '≡' => sym("equiv"),
        '≤' => sym("leq"),
        '≥' => sym("geq"),
        '≦' => sym("leqq"),
        '≧' => sym("geqq"),
        '≨' => sym("lneqq"),
        '≩' => sym("gneqq"),
        '≪' => sym("ll"),
        '≫' => sym("gg"),
        '≬' => sym("between"),
        '≲' => sym("lesssim"),
        '≳' => sym("gtrsim"),
        '≶' => sym("lessgtr"),
        '≷' => sym("gtrless"),
        '≺' => sym("prec"),
        '≻' => sym("succ"),
        '≼' => sym("preccurlyeq"),
        '≽' => sym("succcurlyeq"),
        '≾' => sym("precsim"),
        '≿' => sym("succsim"),
        '⊂' => sym("subset"),
        '⊃' => sym("supset"),
        '⊆' => sym("subseteq"),
        '⊇' => sym("supseteq"),
        '⊊' => sym("subsetneq"),
        '⊋' => sym("supsetneq"),
        '⊎' => sym("uplus"),
        '⊏' => sym("sqsubset"),
        '⊐' => sym("sqsupset"),
        '⊑' => sym("sqsubseteq"),
        '⊒' => sym("sqsupseteq"),
        '⊓' => sym("sqcap"),
        '⊔' => sym("sqcup"),
        '⊕' => sym("oplus"),
        '⊖' => sym("ominus"),
        '⊗' => sym("otimes"),
        '⊘' => sym("oslash"),
        '⊙' => sym("odot"),
        '⊚' => sym("circledcirc"),
        '⊛' => sym("circledast"),
        '⊝' => sym("circleddash"),
        '⊞' => sym("boxplus"),
        '⊟' => sym("boxminus"),
        '⊠' => sym("boxtimes"),
        '⊡' => sym("boxdot"),
        '⊢' => sym("vdash"),
        '⊣' => sym("dashv"),
        '⊤' => sym("top"),
        '⊥' => sym("bot"),
        '⊦' => sym("vdash"),
        '⊧' => sym("models"),
        '⊨' => sym("vDash"),
        '⊩' => sym("Vdash"),
        '⊪' => sym("Vvdash"),
        '⊲' => sym("vartriangleleft"),
        '⊳' => sym("vartriangleright"),
        '⊴' => sym("trianglelefteq"),
        '⊵' => sym("trianglerighteq"),
        '⊸' => sym("multimap"),
        '⊺' => sym("intercal"),
        '⊻' => sym("veebar"),
        '⊼' => sym("barwedge"),
        '⋀' => sym("bigwedge"),
        '⋁' => sym("bigvee"),
        '⋂' => sym("bigcap"),
        '⋃' => sym("bigcup"),
        '⋄' => sym("diamond"),
        '⋅' => sym("cdot"),
        '⋆' => sym("star"),
        '⋇' => sym("divideontimes"),
        '⋈' => sym("bowtie"),
        '⋉' => sym("ltimes"),
        '⋊' => sym("rtimes"),
        '⋋' => sym("leftthreetimes"),
        '⋌' => sym("rightthreetimes"),
        '⋍' => sym("backsimeq"),
        '⋎' => sym("curlyvee"),
        '⋏' => sym("curlywedge"),
        '⋐' => sym("Subset"),
        '⋑' => sym("Supset"),
        '⋒' => sym("Cap"),
        '⋓' => sym("Cup"),
        '⋔' => sym("pitchfork"),
        '⋖' => sym("lessdot"),
        '⋗' => sym("gtrdot"),
        '⋘' => sym("lll"),
        '⋙' => sym("ggg"),
        '⋚' => sym("lesseqgtr"),
        '⋛' => sym("gtreqless"),
        '⋞' => sym("curlyeqprec"),
        '⋟' => sym("curlyeqsucc"),
        '⋦' => sym("lnsim"),
        '⋧' => sym("gnsim"),
        '⋨' => sym("precnsim"),
        '⋩' => sym("succnsim"),
        '⋮' => sym("vdots"),
        '⋯' => sym("cdots"),
        '⋱' => sym("ddots"),
        '⌢' => sym("frown"),
        '⌣' => sym("smile"),
        'Ⓢ' => sym("circledS"),
        '□' => sym("square"),
        '◯' => sym("bigcirc"),
        '★' => sym("bigstar"),
        '♠' => sym("spadesuit"),
        '♡' => sym("heartsuit"),
        '♢' => sym("diamondsuit"),
        '♣' => sym("clubsuit"),
        '♭' => sym("flat"),
        '♮' => sym("natural"),
        '♯' => sym("sharp"),
        '✓' => sym("checkmark"),
        '✠' => sym("maltese"),
        '⟵' => sym("longleftarrow"),
        '⟶' => sym("longrightarrow"),
        '⟷' => sym("longleftrightarrow"),
        '⟸' => sym("Longleftarrow"),
        '⟹' => sym("Longrightarrow"),
        '⟺' => sym("iff"),
        '⟼' => sym("longmapsto"),
        '⧫' => sym("blacklozenge"),
        '⨀' => sym("bigodot"),
        '⨁' => sym("bigoplus"),
        '⨂' => sym("bigotimes"),
        '⨄' => sym("biguplus"),
        '⨆' => sym("bigsqcup"),
        '⨿' => sym("amalg"),
        '⩴' => sym("Coloneqq"),
        '⩽' => sym("leqslant"),
        '⩾' => sym("geqslant"),
        '⪅' => sym("lessapprox"),
        '⪆' => sym("gtrapprox"),
        '⪇' => sym("lneq"),
        '⪈' => sym("gneq"),
        '⪉' => sym("lnapprox"),
        '⪊' => sym("gnapprox"),
        '⪋' => sym("lesseqqgtr"),
        '⪌' => sym("gtreqqless"),
        '⪕' => sym("eqslantless"),
        '⪖' => sym("eqslantgtr"),
        '⪯' => sym("preceq"),
        '⪰' => sym("succeq"),
        '⪵' => sym("precneqq"),
        '⪶' => sym("succneqq"),
        '⪷' => sym("precapprox"),
        '⪸' => sym("succapprox"),
        '⪹' => sym("precnapprox"),
        '⪺' => sym("succnapprox"),
        '⫅' => sym("subseteqq"),
        '⫆' => sym("supseteqq"),
        '⫋' => sym("subsetneqq"),
        '⫌' => sym("supsetneqq"),
        _ => return Err(()),
    })
}

fn tex_of_unicode_accent(c: char) -> Result<String, ()> {
    Ok(match c {
        '\u{0300}' => r"\grave",
        '\u{0301}' => r"\acute",
        '\u{0302}' => r"\hat",
        '\u{0303}' => r"\tilde",
        '\u{0304}' => r"\bar",
        '\u{0305}' => r"\overbar",
        '\u{0306}' => r"\breve",
        '\u{0307}' => r"\dot",
        '\u{0308}' => r"\ddot",
        '\u{030A}' => r"\mathring",
        '\u{030C}' => r"\check",
        '\u{0332}' => r"\underline",
        '\u{0338}' => r"\not",
        '\u{034D}' => r"\underleftrightarrow",
        '\u{020D6}' => r"\overleftarrow",
        '\u{020D7}' => r"\vec",
        '\u{020DB}' => r"\dddot",
        '\u{020DC}' => r"\ddddot",
        '\u{020E1}' => r"\overleftrightarrow",
        '\u{020EE}' => r"\underleftarrow",
        '\u{020EF}' => r"\underrightarrow",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_ascii_art(s: &str) -> Result<String, ()> {
    Ok(match s {
        // binop
        "+-" => r"\pm",
        "-+" => r"\mp",
        "-:-" => r"\div",
        "@" | "." => r"\cdot",
        "-" => r"\bullet",
        "o" | "O" => r"\circ",
        "x" | "X" => r"\times",
        "(x)" | "(X)" => r"\otimes",
        "(+)" => r"\oplus",
        "(.)" => r"\odot",
        "^" => r"\wedge",
        "V" | "v" => r"\vee",
        "n" => r"\cap",
        "U" | "u" => r"\cup",
        // rel
        "!=" => r"\ne",
        "-:" => r"\eqcolon",
        "-::" => r"\Eqcolon",
        "=:" => r"\eqqcolon",
        "=::" => r"\Eqqcolon",
        ":-" => r"\coloneq",
        "::-" => r"\Coloneq",
        ":=" => r"\coloneqq",
        "::=" => r"\Coloneqq",
        "-=" | "=-" => r"\equiv",
        "-~" => r"\eqsim",
        "~-" => r"\simeq",
        "~=" => r"\cong",
        "~~" => r"\approx",
        "~~-" => r"\approxeq",
        ":~" => r"\colonsim",
        "::~" => r"\Colonsim",
        "oc" => r"\propto",
        "<" => r"\lt",
        "<=" => r"\le",
        ">" => r"\gt",
        ">=" => r"\ge",
        "<<" => r"\ll",
        "<<<" => r"\lll",
        ">>" => r"\gg",
        ">>>" => r"\ggg",
        "|-" => r"\vdash",
        "||-" => r"\Vdash",
        "|=" => r"\vDash",
        "-|" => r"\dashv",
        // arrow
        "-->" => r"\leftarrow",
        "<--" => r"\rightarrow",
        "==>" => r"\Rightarrow",
        "<==" => r"\Leftarrow",
        "<<-" => r"\twoheadleftarrow",
        "->>" => r"\twoheadrightarrow",
        "<-<" => r"\leftarrowtail",
        ">->" => r"\rightarrowtail",
        "|->" => r"\mapsto",
        "<=>" => r"\Leftrightarrow",
        "<->" => r"\leftrightarrow",
        "~~>" => r"\rightsquigarrow",
        "<~>" => r"\leftrightsquigarrow",
        // symbol
        "_|_" => r"\bot",
        "T" => r"\top",
        "h-" => r"\hbar",
        "t" | "+" => r"\dagger",
        "A" => r"\forall",
        "E" => r"\exists",
        "oo" => r"\infty",
        "..." => r"\ldots",
        "---" => r"\cdots",
        "||" => r"\|",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_maybe_abbreviated_symbol_name(s: &str) -> String {
    match s {
        _ => format!(r"\{}", s),
    }
}

fn tex_of_maybe_abbreviated_accent_name(s: &str) -> String {
    match s {
        "!" => r"\not".to_string(),
        _ => format!(r"\{}", s),
    }
}

fn take_string_literal_content(s: &str) -> IResult<&str, String> {
    map(
        delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#)),
        String::from,
    )(s)
}

fn take_raw_string_literal_content(s: &str) -> IResult<&str, String> {
    flat_map(
        map(terminated(many1(tag("#")), tag(r#"""#)), |x| x.len()),
        |num| {
            map(
                many_till(anychar, pair(tag(r#"""#), count(tag("#"), num))),
                |(content, _)| content.into_iter().collect(),
            )
        },
    )(s)
}

fn escape_tex_string_math(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|c| match c {
            '#' | '$' | '%' | '_' | '{' | '}' => format!(r"\{}", c),
            '~' => r"{\textasciitilde}".to_string(),
            '^' => r"{\textasciicircum}".to_string(),
            '\\' => r"{\backslash}".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

fn escape_tex_string_text(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|c| match c {
            '$' | '{' | '}' | '\\' => format!(r"\{}", c),
            _ => c.to_string(),
        })
        .collect()
}

fn resolve_string_literal_accent(content: &str, accents: Vec<&str>) -> Result<String, ()> {
    let accents: Result<Vec<_>, _> = accents
        .into_iter()
        .map(|x| {
            Ok(match x {
                "bb" | "mathbb" => "bb",
                "b" | "bf" | "mathbf" => "bf",
                "c" | "cc" | "ca" | "cal" | "mathcal" => "cal",
                "f" | "fr" | "fra" | "frak" | "frk" | "mathfrak" => "frak",
                "i" | "it" | "mathit" => "it",
                "r" | "rm" | "mathrm" => "rm",
                "sc" | "scr" | "mathscr" => "scr",
                "sf" | "mathsf" => "sf",
                "tt" | "mathtt" => "tt",
                "bffr" | "frbf" | "bffrak" | "frakbf" | "mathbffrak" | "mathfrakbf" => "bffrak",
                "bfit" | "itbf" | "mathbfit" | "mathitbf" => "bfit",
                "bfsc" | "scbf" | "bfscr" | "scrbf" | "mathbfscr" | "mathscrbf" => "bfscr",
                "bfsf" | "sfbf" | "mathbfsf" | "mathsfbf" => "bfsf",
                "sfit" | "itsf" | "mathsfit" | "mathitsf" => "sfit",
                "bfsfit" | "bfitsf" | "sfbfit" | "sfitbf" | "itsfbf" | "itbfsf" | "mathbfsfit"
                | "mathbfitsf" | "mathsfbfit" | "mathsfitbf" | "mathitsfbf" | "mathitbfsf" => {
                    "bfsfit"
                }
                "t" | "te" | "text" => "text",
                _ => return Err(()),
            })
        })
        .collect();
    let mut accents = accents?;
    accents.sort();
    accents.dedup();
    let content = match accents[..] {
        ["text"] => escape_tex_string_text(&content),
        _ => escape_tex_string_math(&content),
    };
    let prefix = match accents[..] {
        ["bb"] => r"\mathbb",
        ["bf"] => r"\mathbf",
        ["bf", "cal"] => r"\mathbfcal",
        ["bf", "frak"] | ["bffrak"] => r"\mathbffrak",
        ["bf", "it"] | ["bfit"] => r"\mathbfit",
        ["bf", "scr"] | ["bfscr"] => r"\mathbfscr",
        ["bf", "it", "sf"] | ["bfit", "sf"] | ["bfsf", "it"] | ["bf", "sfit"] | ["bfsfit"] => {
            r"\mathbfsfit"
        }
        ["cal"] => r"\mathcal",
        ["frak"] => r"\mathfrak",
        ["it"] => r"\mathit",
        ["it", "sf"] | ["sfit"] => r"\mathsfit",
        ["rm"] | [] => r"\mathrm",
        ["scr"] => r"\mathscr",
        ["sf"] => r"\mathsf",
        ["tt"] => r"\mathtt",
        ["text"] => r"\text",
        _ => return Err(()),
    };
    Ok(format!("{}{{{}}}", prefix, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x(a: &str) -> (&str, Token) {
        take_symbol(a).unwrap()
    }
    fn y<T: Display>(y: T) -> Token {
        Token::Symbol(y.to_string())
    }

    #[test]
    fn test_take_symbol() {
        assert_eq!(x("123"), (r"23", y(r"1")));
        assert_eq!(x("1.23"), (r".23", y(r"1")));
        assert_eq!(x("1'.23"), (r".23", y(r"1'")));
        assert_eq!(x("aΓ"), (r"Γ", y(r"a")));
        assert_eq!(x("Γa"), ("a", y(r"\Gamma")));
        assert_eq!(x("α̇bcd"), ("bcd", y(r"\dot{\alpha}")));
        assert_eq!(x("<a>''"), ("", y("a''")));
        assert_eq!(x("<a dot>'b"), ("b", y(r"\dot{a}'")));
        assert_eq!(x("< a  dot  >"), ("", y(r"\dot{a}")));
        assert_eq!(x("<  a dot>"), ("", y(r"\dot{a}")));
        assert_eq!(x("<a dot !>"), ("", y(r"\not{\dot{a}}")));
        assert_eq!(x("`oo`"), ("", y(r"\infty")));
        assert_eq!(x("`oo`23"), ("23", y(r"\infty")));
        assert_eq!(x("`oo`'23"), ("23", y(r"\infty'")));
        assert_eq!(x("0.1234ABC"), (".1234ABC", y("0")));
        assert_eq!(x("0A1B3C"), ("A1B3C", y("0")));
        assert_eq!(x("0 1 3"), (" 1 3", y("0")));
        assert_eq!(x("<1.23 hat>"), ("", y(r"\hat{1.23}")));
        assert_eq!(x("<α̇ tilde !>"), ("", y(r"\not{\tilde{\dot{\alpha}}}")));
        assert_eq!(x("<α̇!>"), ("", y(r"\not{\dot{\alpha}}")));
        assert_eq!(x("<α̇! tilde>"), ("", y(r"\tilde{\not{\dot{\alpha}}}")));
        assert_eq!(x("< α̇ tilde ! >"), ("", y(r"\not{\tilde{\dot{\alpha}}}")));
        assert_eq!(x("<α̇  tilde !  >"), ("", y(r"\not{\tilde{\dot{\alpha}}}")));
        assert_eq!(x("<`oo` !>"), ("", y(r"\not{\infty}")));
        assert_eq!(x("< `oo` !>"), ("", y(r"\not{\infty}")));
        assert_eq!(x("<`<` !>"), ("", y(r"\not{\lt}")));
        assert_eq!(x("<!!>"), ("", y(r"\not{!}")));
        assert_eq!(x("<   `oo` !  >"), ("", y(r"\not{\infty}")));
        assert_eq!(x("<alpha>"), ("", y(r"\alpha")));
        assert_eq!(x("<alpha dot>"), ("", y(r"\dot{\alpha}")));
        assert_eq!(x(r#""aaa""#), ("", y(r"\mathrm{aaa}")));
        assert_eq!(x(r#"<"aaa">"#), ("", y(r#"\mathrm{aaa}"#)));
        assert_eq!(x(r#"< "aaa"  >"#), ("", y(r#"\mathrm{aaa}"#)));
        assert_eq!(x(r#"< "aaa"bb  >"#), ("", y(r#"\mathbb{aaa}"#)));
        assert_eq!(x(r#"< "aaa"  bf it>"#), ("", y(r#"\mathbfit{aaa}"#)));
        assert_eq!(x(r#"<"aaa"it    bf>"#), ("", y(r#"\mathbfit{aaa}"#)));
        assert_eq!(x(r#"<"aaa"it bf sf>"#), ("", y(r#"\mathbfsfit{aaa}"#)));
        assert_eq!(x(r#"<"aaa"it bf>"#), ("", y(r#"\mathbfit{aaa}"#)));
        assert_eq!(x(r####"< ##"aaa"## te  >"####), ("", y(r#"\text{aaa}"#)));
        assert_eq!(x(r####"< ##"aaa"## te  >"####), ("", y(r#"\text{aaa}"#)));
        assert_eq!(
            x(r####"< ##"aa"#a"## te  >"####),
            ("", y(r##"\text{aa"#a}"##))
        );
    }
}
