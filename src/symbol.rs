use super::token::Token;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{alpha0, alphanumeric1, anychar, char, digit1, satisfy},
    combinator::{map, map_res, opt},
    multi::{count, many0, many1, many_m_n, many_till},
    sequence::{pair, tuple},
    IResult,
};
use std::{fmt::Display, iter};
use unicode_normalization::UnicodeNormalization;

fn take_symbol_from_single_char(s: &str) -> IResult<&str, String> {
    let (s, (mut tex, accents)) = pair(
        map_res(anychar, tex_of_char),
        many0(map_res(anychar, tex_of_unicode_accent)),
    )(s)?;
    for accent in accents {
        tex = format!("{}{{{}}}", accent, tex);
    }
    Ok((s, tex))
}

fn take_symbol_from_ascii_art(s: &str) -> IResult<&str, String> {
    let (s, (_, tex, _)) = tuple((
        char('.'),
        map_res(take_until("."), tex_of_ascii_art),
        char('.'),
    ))(s)?;
    Ok((s, tex))
}

fn take_symbol_in_angle_brackets(s: &str) -> IResult<&str, String> {
    //! `symbol_with_accent` is
    //!
    //! `<({char_with_unicode_accent}|{ascii_art}|{symbol_name})( +{accent_name})*>`
    //!
    //! where
    //! - `symbol_name` is `([A-Za-z]+)`
    //! - `ascii_art` is `(\.[^\.]+\.)`
    //! - `accent_name` is `!|[A-Za-z0-9]+`
    let (s, (_, _, mut tex, accents, _, _)) = tuple((
        char('<'),
        many0(char(' ')),
        alt((
            map(
                many_m_n(2, usize::MAX, satisfy(|x| x.is_alphabetic())),
                |x| tex_of_maybe_abbreviated_symbol_name(&x.into_iter().collect::<String>()),
            ),
            take_symbol_from_ascii_art,
            take_symbol_from_single_char,
        )),
        many0(pair(
            many1(char(' ')),
            map(
                alt((alphanumeric1, tag("!"))),
                tex_of_maybe_abbreviated_accent_name,
            ),
        )),
        many0(char(' ')),
        char('>'),
    ))(s)?;
    for (_, accent) in accents {
        tex = format!("{}{{{}}}", accent, tex);
    }
    Ok((s, tex))
}

pub fn take_symbol(s: &str) -> IResult<&str, Token> {
    //! `symbol` is
    //!
    //! `({char_with_unicode_accent}|{ascii_art}|{symbol_with_accent})'*`
    //!
    //! where
    //! - `ascii_art` is `(\.[^\.]+\.)`
    let (s, (t, u)) = pair(
        alt((
            take_symbol_in_angle_brackets,
            take_symbol_from_single_char,
            take_symbol_from_ascii_art,
        )),
        is_a("'"),
    )(s)?;
    Ok((s, Token::Symbol(t + u)))
}

fn take_string_literal_content(s: &str) -> IResult<&str, String> {
    let (s, (_, content, _)) = alt((
        tuple((char('"'), take_until("\""), char('"'))),
        tuple((char('`'), take_until("`"), char('`'))),
    ))(s)?;
    Ok((s, content.to_string()))
}

fn take_raw_string_literal_content(s: &str) -> IResult<&str, String> {
    let (s, (sharps, _)) = pair(many1(char('#')), char('"'))(s)?;
    let (s, (content, _)) = many_till(anychar, pair(char('"'), count(char('#'), sharps.len())))(s)?;
    Ok((s, content.into_iter().collect()))
}

fn escape_tex_string_math(s: &str) -> String {
    let mut rsl = "".to_string();
    for c in s.chars() {
        match c {
            '#' | '$' | '%' | '_' | '{' | '}' => {
                rsl.push('\\');
                rsl.push(c)
            }
            '~' => rsl.push_str(r"{\textasciitilde}"),
            '^' => rsl.push_str(r"{\textasciicircum}"),
            '\\' => rsl.push_str(r"{\backslash}"),
            _ => rsl.push(c),
        }
    }
    rsl
}

fn escape_tex_string_text(s: &str) -> String {
    s.chars()
        .into_iter()
        .map(|c| match c {
            '$' | '{' | '}' | '\\' => {
                format!(r"\{}", c)
            }
            _ => c.to_string(),
        })
        .collect()
}

pub fn take_string_literal(s: &str) -> IResult<&str, Token> {
    let (s, (t, u)) = alt((
        map(take_string_literal_content, |c| {
            (escape_tex_string_math(&c), r"\mathrm")
        }),
        map_res(
            tuple((
                char('<'),
                many0(char(' ')),
                alt((take_string_literal_content, take_raw_string_literal_content)),
                many0(char(' ')),
                alpha0,
                many0(char(' ')),
                char('>'),
            )),
            |(_, _, c, _, v, _, _)| match v {
                "" | "rm" | "mathrm" => Ok((escape_tex_string_math(&c), r"\mathrm")),
                "bf" | "mathbf" => Ok((escape_tex_string_math(&c), r"\mathbf")),
                "bb" | "mathbb" => Ok((escape_tex_string_math(&c), r"\mathbb")),
                "ca" | "mathcal" => Ok((escape_tex_string_math(&c), r"\mathcal")),
                "tt" | "mathtt" => Ok((escape_tex_string_math(&c), r"\mathtt")),
                "fr" | "mathfrak" => Ok((escape_tex_string_math(&c), r"\mathfrak")),
                "sf" | "mathsf" => Ok((escape_tex_string_math(&c), r"\mathsf")),
                "te" | "text" => Ok((escape_tex_string_text(&c), r"\text")),
                _ => Err(()),
            },
        ),
    ))(s)?;
    Ok((s, Token::Symbol(format!("{}{{{}}}", u, t))))
}

pub fn take_number(s: &str) -> IResult<&str, Token> {
    let (s, (x, y)) = pair(digit1, opt(pair(char('.'), digit1)))(s)?;
    if let Some((_, y)) = y {
        Ok((s, Token::Symbol(format!("{}.{}", x, y))))
    } else {
        Ok((s, Token::Symbol(x.to_string())))
    }
}

pub fn take_constant(s: &str) -> IResult<&str, Token> {
    alt((take_symbol, take_string_literal, take_number))(s)
}

fn tex_of_char(c: char) -> Result<String, ()> {
    let nfkc = |c: char| iter::once(c).nfkc().next().ok_or(());

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
        '!' | '*' | '+' | ',' | '-' | ':' | ';' | '=' | '?' | '@' | '|' => raw(c),
        'A'..='Z' | 'a'..='z' => raw(c),
        '#' | '$' | '%' | '&' => sym(c),
        '\\' => sym("backslash"),
        '~' => sym("sim"),
        // rest:
        //   ␠, ", ', (, ), ., /, 0-9, <, >, [, ], ^, _, `, {, }
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
        "!=" => r"\ne",
        "->" => r"\leftarrow",
        "<-" => r"\rightarrow",
        "=>" => r"\Rightarrow",
        "=->" => r"\Rightarrow",
        "-=>" => r"\Rightarrow",
        "<-=" => r"\Leftarrow",
        "<=-" => r"\Leftarrow",
        "<" => "<",
        "<=" => r"\le",
        ">" => ">",
        ">=" => r"\re",
        "||" => r"\|",
        "/" => "/",
        "x" => r"\times",
        "oo" => r"\infty",
        _ => return Err(()),
    }
    .to_string())
}

fn tex_of_maybe_abbreviated_symbol_name(s: &str) -> String {
    format!(
        r"\{}",
        match s {
            _ => s,
        }
    )
}

fn tex_of_maybe_abbreviated_accent_name(s: &str) -> String {
    format!(
        r"\{}",
        match s {
            "!" => "not",
            _ => s,
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_symbol() {
        assert_eq!(
            take_symbol_from_single_char("aΓ").unwrap(),
            (r"Γ", r"a".to_string())
        );
        assert_eq!(take_symbol_from_single_char("Γa").unwrap().1, r"\Gamma");
        assert_eq!(
            take_symbol_from_single_char("α̇").unwrap().1,
            r"\dot{\alpha}"
        );
        assert_eq!(take_symbol_in_angle_brackets("<a>").unwrap().1, "a");
        assert_eq!(
            take_symbol_in_angle_brackets("<a dot>").unwrap().1,
            r"\dot{a}"
        );
        assert_eq!(
            take_symbol_in_angle_brackets("<a dot !>").unwrap().1,
            r"\not{\dot{a}}"
        );
        assert_eq!(take_symbol_from_ascii_art(".oo.").unwrap().1, r"\infty");
        assert_eq!(
            take_symbol_in_angle_brackets("<α̇ tilde !>").unwrap().1,
            r"\not{\tilde{\dot{\alpha}}}"
        );
        assert_eq!(
            take_symbol_in_angle_brackets("<.oo. !>").unwrap().1,
            r"\not{\infty}"
        );
        assert_eq!(
            take_symbol_in_angle_brackets("<alpha>").unwrap().1,
            r"\alpha"
        );
        assert_eq!(
            take_symbol_in_angle_brackets("<alpha dot>").unwrap().1,
            r"\dot{\alpha}"
        );
    }

    #[test]
    fn test_take_literal() {
        assert_eq!(
            take_raw_string_literal_content(r###"##"aa"#Ba"##"###)
                .unwrap()
                .1,
            r##"aa"#Ba"##
        );
        assert_eq!(
            take_string_literal(r#""aaa""#).unwrap().1,
            Token::Symbol(r"\mathrm{aaa}".to_string())
        );
        assert_eq!(
            take_string_literal("`aa\"a`").unwrap().1,
            Token::Symbol(r#"\mathrm{aa"a}"#.to_string())
        );
        assert_eq!(
            take_string_literal(r#"<"aaa">"#).unwrap().1,
            Token::Symbol(r#"\mathrm{aaa}"#.to_string())
        );
        assert_eq!(
            take_string_literal(r#"< "aaa"  >"#).unwrap().1,
            Token::Symbol(r#"\mathrm{aaa}"#.to_string())
        );
        assert_eq!(
            take_string_literal(r####"< ##"aaa"## te  >"####).unwrap().1,
            Token::Symbol(r#"\text{aaa}"#.to_string())
        );
    }

    #[test]
    fn test_take_number() {
        assert_eq!(
            take_number("0.1234ABC").unwrap().1,
            Token::Symbol("0.1234".to_string())
        );
        assert_eq!(
            take_number("0A1B3C").unwrap().1,
            Token::Symbol("0".to_string())
        );
        assert_eq!(
            take_number("0 1 3").unwrap().1,
            Token::Symbol("0".to_string())
        );
    }
}
