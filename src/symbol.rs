use super::token::Token;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{alpha1, alphanumeric1, anychar, char, digit1, one_of, satisfy},
    combinator::{flat_map, map, map_res, opt},
    multi::{count, fold_many0, fold_many_m_n, many0, many1, many_till},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::{fmt::Display, iter::once};
use unicode_normalization::UnicodeNormalization;

pub fn take_constant(s: &str) -> IResult<&str, Token> {
    alt((take_symbol, take_string_literal, take_number))(s)
}

pub fn take_symbol(s: &str) -> IResult<&str, Token> {
    map(
        pair(
            alt((
                take_symbol_in_angle_brackets,
                take_symbol_from_single_char,
                take_symbol_from_ascii_art,
            )),
            opt(is_a("'")),
        ),
        |(tex, prime)| Token::Symbol(tex + prime.unwrap_or_default()),
    )(s)
}

pub fn take_string_literal(s: &str) -> IResult<&str, Token> {
    map(
        pair(
            alt((
                take_string_literal_plain,
                take_string_literal_in_angle_brackets,
            )),
            opt(is_a("'")),
        ),
        |(tex, prime)| Token::Symbol(tex + prime.unwrap_or_default()),
    )(s)
}

pub fn take_number(s: &str) -> IResult<&str, Token> {
    map(
        tuple((
            digit1,
            opt(map(preceded(char('.'), digit1), |x| format!(".{}", x))),
            opt(is_a("'")),
        )),
        |(integer, decimal, prime): (&str, Option<String>, Option<&str>)| {
            Token::Symbol(format!(
                "{}{}{}",
                integer,
                decimal.unwrap_or_default(),
                prime.unwrap_or_default()
            ))
        },
    )(s)
}

// symbol

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
        char('`'),
        map_res(take_until("`"), tex_of_ascii_art),
        char('`'),
    )(s)
}

fn take_symbol_in_angle_brackets(s: &str) -> IResult<&str, String> {
    flat_map(
        preceded(
            pair(char('<'), many0(char(' '))),
            alt((
                map_res(alpha1, |x: &str| {
                    (x.len() != 1)
                        .then_some(tex_of_maybe_abbreviated_symbol_name(x))
                        .ok_or(())
                }),
                take_symbol_from_ascii_art,
                take_symbol_from_single_char,
                map(one_of("^_"), |x| format!(r"\{}", x)),
                map(char('/'), |_| "/".to_string()),
            )),
        ),
        |tex| {
            terminated(
                fold_many0(
                    map(
                        alt((
                            preceded(many0(char(' ')), tag("!")),
                            preceded(many1(char(' ')), alphanumeric1),
                        )),
                        tex_of_maybe_abbreviated_accent_name,
                    ),
                    move || String::from(&tex),
                    |tex, accent| format!("{}{{{}}}", accent, tex),
                ),
                pair(many0(char(' ')), char('>')),
            )
        },
    )(s)
}

// literal

fn take_string_literal_plain(s: &str) -> IResult<&str, String> {
    map_res(take_string_literal_content, |c| {
        resolve_string_literal_accent(&c, vec![])
    })(s)
}

fn take_string_literal_in_angle_brackets(s: &str) -> IResult<&str, String> {
    flat_map(
        preceded(
            pair(char('<'), many0(char(' '))),
            pair(
                alt((take_string_literal_content, take_raw_string_literal_content)),
                opt(alpha1),
            ),
        ),
        |(content, accent)| {
            terminated(
                map_res(
                    fold_many0(
                        preceded(many1(char(' ')), alpha1),
                        move || accent.into_iter().collect::<Vec<_>>(),
                        |accents, a| accents.into_iter().chain(once(a)).collect(),
                    ),
                    move |accents| resolve_string_literal_accent(&content, accents),
                ),
                pair(many0(char(' ')), char('>')),
            )
        },
    )(s)
}

fn take_string_literal_content(s: &str) -> IResult<&str, String> {
    map(
        delimited(char('"'), take_until("\""), char('"')),
        String::from,
    )(s)
}

fn take_raw_string_literal_content(s: &str) -> IResult<&str, String> {
    flat_map(
        map(terminated(many1(char('#')), char('"')), |x| x.len()),
        |num| {
            map(
                many_till(anychar, pair(char('"'), count(char('#'), num))),
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
        'A'..='Z' | 'a'..='z' => raw(c),
        '#' | '$' | '%' | '&' => sym(c),
        '\\' => sym("backslash"),
        '~' => sym("sim"),
        // rest:
        //   ␠, ", ', (, ), /, 0-9, <, >, [, ], ^, _, `, {, }
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
        "+-" => r"\pm",
        "-+" => r"\mp",
        "-:-" => r"\div",
        "@" | "." => r"\cdot",
        "-" => r"\bullet",
        "o" => r"\circ",
        "x" => r"\times",
        "(x)" => r"\otimes",
        "(+)" => r"\oplus",
        "(.)" => r"\odot",

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

        "^" => r"\wedge",
        "V" | "v" => r"\vee",
        "n" => r"\cap",
        "U" | "u" => r"\cup",

        "<" => r"\lt",
        "<=" => r"\le",
        ">" => r"\gt",
        ">=" => r"\ge",
        "<<" => r"\ll",
        "<<<" => r"\lll",
        ">>" => r"\gg",
        ">>>" => r"\ggg",

        "_|_" => r"\bot",
        "T" => r"\top",
        "|-" => r"\vdash",
        "||-" => r"\Vdash",
        "|=" => r"\vDash",
        "-|" => r"\dashv",

        "||" => r"\|",

        "h-" => r"\hbar",
        "t" => r"\dagger",
        "A" => r"\forall",
        "E" => r"\exists",
        "oo" => r"\infty",

        "..." => r"\ldots",
        "---" => r"\cdots",
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

#[cfg(test)]
mod tests {
    use super::*;
    fn x<T: Display>(y: T) -> Token {
        Token::Symbol(y.to_string())
    }

    #[test]
    fn test_take_symbol() {
        assert_eq!(take_symbol("aΓ").unwrap(), (r"Γ", x(r"a")));
        assert_eq!(take_symbol("Γa").unwrap(), ("a", x(r"\Gamma")));
        assert_eq!(take_symbol("α̇bcd").unwrap(), ("bcd", x(r"\dot{\alpha}")));
        assert_eq!(take_symbol("<a>''").unwrap(), ("", x("a''")));
        assert_eq!(take_symbol("<a dot>'b").unwrap(), ("b", x(r"\dot{a}'")));
        assert_eq!(take_symbol("< a  dot  >").unwrap(), ("", x(r"\dot{a}")));
        assert_eq!(take_symbol("<  a dot>").unwrap(), ("", x(r"\dot{a}")));
        assert_eq!(take_symbol("<a dot !>").unwrap(), ("", x(r"\not{\dot{a}}")));
        assert_eq!(take_symbol("`oo`").unwrap(), ("", x(r"\infty")));
        assert_eq!(take_symbol("`oo`23").unwrap(), ("23", x(r"\infty")));
        assert_eq!(take_symbol("`oo`'23").unwrap(), ("23", x(r"\infty'")));
        assert_eq!(
            take_symbol("<α̇ tilde !>").unwrap(),
            ("", x(r"\not{\tilde{\dot{\alpha}}}"))
        );
        assert_eq!(take_symbol("<α̇!>").unwrap(), ("", x(r"\not{\dot{\alpha}}")));
        assert_eq!(
            take_symbol("<α̇! tilde>").unwrap(),
            ("", x(r"\tilde{\not{\dot{\alpha}}}"))
        );
        assert_eq!(
            take_symbol("< α̇ tilde ! >").unwrap(),
            ("", x(r"\not{\tilde{\dot{\alpha}}}"))
        );
        assert_eq!(
            take_symbol("<α̇  tilde !  >").unwrap(),
            ("", x(r"\not{\tilde{\dot{\alpha}}}"))
        );
        assert_eq!(take_symbol("<`oo` !>").unwrap(), ("", x(r"\not{\infty}")));
        assert_eq!(take_symbol("< `oo` !>").unwrap(), ("", x(r"\not{\infty}")));
        assert_eq!(take_symbol("<`<` !>").unwrap(), ("", x(r"\not{\lt}")));
        assert_eq!(take_symbol("<!!>").unwrap(), ("", x(r"\not{!}")));
        assert_eq!(
            take_symbol("<   `oo` !  >").unwrap(),
            ("", x(r"\not{\infty}"))
        );
        assert_eq!(take_symbol("<alpha>").unwrap(), ("", x(r"\alpha")));
        assert_eq!(
            take_symbol("<alpha dot>").unwrap(),
            ("", x(r"\dot{\alpha}"))
        );
    }

    #[test]
    fn test_take_literal() {
        assert_eq!(
            take_raw_string_literal_content(r###"##"aa"#Ba"##"###).unwrap(),
            ("", r##"aa"#Ba"##.to_string())
        );

        assert_eq!(
            take_string_literal(r#""aaa""#).unwrap(),
            ("", x(r"\mathrm{aaa}"))
        );
        assert_eq!(
            take_string_literal(r#"<"aaa">"#).unwrap(),
            ("", x(r#"\mathrm{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"< "aaa"  >"#).unwrap(),
            ("", x(r#"\mathrm{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"< "aaa"bb  >"#).unwrap(),
            ("", x(r#"\mathbb{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"< "aaa"  bf it>"#).unwrap(),
            ("", x(r#"\mathbfit{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"<"aaa"it    bf>"#).unwrap(),
            ("", x(r#"\mathbfit{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"<"aaa"it bf sf>"#).unwrap(),
            ("", x(r#"\mathbfsfit{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r#"<"aaa"it bf>"#).unwrap(),
            ("", x(r#"\mathbfit{aaa}"#))
        );
        assert_eq!(
            take_string_literal(r####"< ##"aaa"## te  >"####).unwrap(),
            ("", x(r#"\text{aaa}"#))
        );
    }

    #[test]
    fn test_take_number() {
        assert_eq!(take_number("0.1234ABC").unwrap(), ("ABC", x("0.1234")));
        assert_eq!(take_number("0A1B3C").unwrap(), ("A1B3C", x("0")));
        assert_eq!(take_number("0 1 3").unwrap(), (" 1 3", x("0")));
    }
}
