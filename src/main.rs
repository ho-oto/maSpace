use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, alphanumeric1, anychar, char, digit1, none_of, one_of},
    combinator::{map, map_res, opt, peek},
    error::ParseError,
    multi::fold_many0,
    multi::many0,
    sequence,
    sequence::{delimited, pair, tuple},
    IResult, Parser,
};
use std::{
    fmt::{format, Display},
    iter,
};
use unicode_normalization::UnicodeNormalization;

fn get_tex_from_char(c: char) -> Result<String, ()> {
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
        ' ' => return Err(()),
        '"' | '\'' | '`' => return Err(()),
        '(' | ')' | '[' | ']' | '{' | '}' => return Err(()),
        '#' | '/' | '^' | '_' | '@' => return Err(()),
        '0'..='9' => return Err(()),

        '$' | '%' | '&' => sym(c),
        '\\' => sym("backslash"),
        '~' => sym("sim"),

        '!' | '*' | '+' | ',' | '-' | '.' | ':' | ';' | '<' | '=' | '>' | '?' | '|' => raw(c),
        'A'..='Z' | 'a'..='z' => raw(c),

        // - special Unicode character
        '√' | '∛' | '∜' => return Err(()),
        '∕' => return Err(()),
        '⟨' | '⌈' | '⌊' | '⎰' | '⌜' | '⌞' | '⟦' => return Err(()),
        '⟩' | '⌉' | '⌋' | '⎱' | '⌝' | '⌟' | '⟧' => return Err(()),

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
        'ɸ' => sym("phi"), // Latin Phi -> Phi

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
            get_tex_from_char(nfkc(c)?)?
        }
        'ı' => cmb("text", 'ı'),
        'ȷ' => cmb("text", 'ȷ'),

        // - Symbols
        '§' => sym("S"),
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

fn expand_abbred_symbol(s: &str) -> String {
    match s {
        _ => s.to_string(),
    }
}

fn expand_abbred_op(s: &str) -> String {
    match s {
        _ => s.to_string(),
    }
}

fn expand_abbred_literal_suffix(s: &str) -> String {
    match s {
        _ => s.to_string(),
    }
}

fn escape_tex(s: &str) -> String {
    todo!()
}

fn get_unicode_accent(c: char) -> Result<String, ()> {
    Ok(match c {
        '\u{0300}' => "grave",
        '\u{0301}' => "acute",
        '\u{0302}' => "hat",
        '\u{0303}' => "tilde",
        '\u{0304}' => "bar",
        '\u{0305}' => "overbar",
        '\u{0306}' => "breve",
        '\u{0307}' => "dot",
        '\u{0308}' => "ddot",
        '\u{030A}' => "mathring",
        '\u{030C}' => "check",
        '\u{0332}' => "underline",
        '\u{0338}' => "not",
        '\u{034D}' => "underleftrightarrow",
        '\u{020D6}' => "overleftarrow",
        '\u{020D7}' => "vec",
        '\u{020DB}' => "dddot",
        '\u{020DC}' => "ddddot",
        '\u{020E1}' => "overleftrightarrow",
        '\u{020EE}' => "underleftarrow",
        '\u{020EF}' => "underrightarrow",
        _ => return Err(()),
    }
    .to_string())
}

fn get_sub(c: char) -> Result<char, ()> {
    match c {
        '₊' | '₋' | '₌' | '₍' | '₎' | '₀' | '₁' | '₂' | '₃' | '₄' | '₅' | '₆' | '₇' | '₈' | '₉'
        | 'ₐ' | 'ₑ' | 'ₕ' | 'ᵢ' | 'ⱼ' | 'ₖ' | 'ₗ' | 'ₘ' | 'ₙ' | 'ₒ' | 'ₚ' | 'ᵣ' | 'ₛ' | 'ₜ'
        | 'ᵤ' | 'ᵥ' | 'ₓ' | 'ᵦ' | 'ᵧ' | 'ᵨ' | 'ᵩ' | 'ᵪ' => {
            iter::once(c).nfkc().next().ok_or(())
        }
        _ => Err(()),
    }
}

fn get_sup(c: char) -> Result<char, ()> {
    match c {
        '⁺' | '⁻' | '⁼' | '⁽' | '⁾' | '⁰' | '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹'
        | 'ᴬ' | 'ᴮ' | 'ᴰ' | 'ᴱ' | 'ᴳ' | 'ᴴ' | 'ᴵ' | 'ᴶ' | 'ᴷ' | 'ᴸ' | 'ᴹ' | 'ᴺ' | 'ᴼ' | 'ᴾ'
        | 'ᴿ' | 'ᵀ' | 'ᵁ' | 'ⱽ' | 'ᵂ' | 'ᵃ' | 'ᵇ' | 'ᶜ' | 'ᵈ' | 'ᵉ' | 'ᵍ' | 'ʰ' | 'ⁱ' | 'ʲ'
        | 'ᵏ' | 'ˡ' | 'ᵐ' | 'ⁿ' | 'ᵒ' | 'ᵖ' | 'ʳ' | 'ˢ' | 'ᵗ' | 'ᵘ' | 'ᵛ' | 'ʷ' | 'ˣ' | 'ʸ'
        | 'ᶻ' | 'ᵝ' | 'ᵞ' | '\u{1D5F}' | 'ᶿ' | 'ᵠ' | 'ᵡ' => {
            iter::once(c).nfkc().next().ok_or(())
        }
        'ᵅ' => Ok('α'),
        'ᵋ' => Ok('ε'),
        'ᶥ' => Ok('ι'),
        'ᶲ' => Ok('ϕ'),
        'ꜛ' => Ok('↑'),
        'ꜜ' => Ok('↓'),
        'ꜝ' => Ok('!'),
        _ => Err(()),
    }
}

enum Token {
    Cat(usize),
    Sub(usize),
    Sup(usize),
    Over(usize),
    Under(usize),
    Frac(usize),
    Op(String, usize),
    Open(String),
    Close(String),
    Symbol(String),
    UnicodeSub(Box<Token>),
    UnicodeSup(Box<Token>),
}

fn num_space<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, usize, E> {
    fold_many0(char(' '), || 0, |x, _| x + 1)(s)
}

fn max_space_around<'a, R, F, E>(s: &'a str, parser: F) -> IResult<&'a str, usize, E>
where
    F: Parser<&'a str, R, E>,
    E: ParseError<&'a str>,
{
    let (s, (left, _, right)) = sequence::tuple((num_space, parser, num_space))(s)?;
    Ok((s, left.max(right)))
}

fn take_sub(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('_')).map(|(s, n)| (s, Token::Sub(n)))
}
fn take_under(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("__")).map(|(s, n)| (s, Token::Under(n)))
}
fn take_sup(s: &str) -> IResult<&str, Token> {
    max_space_around(s, char('^')).map(|(s, n)| (s, Token::Sup(n)))
}
fn take_over(s: &str) -> IResult<&str, Token> {
    max_space_around(s, tag("^^")).map(|(s, n)| (s, Token::Over(n)))
}
fn take_frac(s: &str) -> IResult<&str, Token> {
    max_space_around(s, one_of("/∕")).map(|(s, n)| (s, Token::Frac(n)))
}

fn take_cat(s: &str) -> IResult<&str, Token> {
    num_space(s).map(|(s, n)| (s, Token::Cat(n)))
}

fn take_symbol_unicode(s: &str) -> IResult<&str, Token> {
    let (s, (prefix, mut tex, unicode_props, ascii_props)) = tuple((
        opt(pair(char('#'), opt(char('!')))),
        map_res(anychar, get_tex_from_char),
        many0(map_res(anychar, get_unicode_accent)),
        many0(pair(char('.'), alphanumeric1)),
    ))(s)?;
    if let Some((_, Some(_))) = prefix {
        tex = format!(r"\not{{ {} }}", tex);
    };
    for prop in unicode_props {
        tex = format!(r"\{}{{ {} }}", prop, tex);
    }
    for (_, prop) in ascii_props {
        tex = format!(r"\{}{{ {} }}", prop, tex);
    }
    Ok((s, Token::Symbol(tex)))
}

fn take_symbol_ascii(s: &str) -> IResult<&str, Token> {
    let (s, (_, not, base, ascii_props)) = tuple((
        char('#'),
        opt(char('!')),
        alpha1,
        many0(pair(char('.'), alphanumeric1)),
    ))(s)?;
    let mut tex = format!(r"\{}", expand_abbred_symbol(base));
    if let Some(_) = not {
        tex = format!(r"\not{{ {} }}", tex);
    }
    for (_, prop) in ascii_props {
        tex = format!(r"\{}{{ {} }}", prop, tex);
    }
    Ok((s, Token::Symbol(tex)))
}

fn take_string_literal(s: &str) -> IResult<&str, Token> {
    let (s, l) = delimited(char('"'), many0(none_of(r#"""#)), char('"'))(s)?;
    let (s, suffix) = alpha0(s)?;
    let mut literal = format!("");
    for c in l {
        match c {
            '#' | '$' | '%' | '_' | '{' | '}' => {
                literal.push('\\');
                literal.push(c)
            }
            '~' => literal.push_str(r"\textasciitilde"),
            '^' => literal.push_str(r"\textasciicircum"),
            '\\' => literal.push_str(r"\backslash"),
            _ => literal.push(c),
        }
    }
    let suffix = expand_abbred_literal_suffix(suffix);
    Ok((s, Token::Symbol(format!(r"\{}{{ {} }}", suffix, literal))))
}

fn take_here_document(s: &str) -> IResult<&str, Token> {
    todo!()
}

fn take_number(s: &str) -> IResult<&str, Token> {
    let (s, (x, y)) = pair(digit1, opt(pair(char('.'), digit1)))(s)?;
    if let Some((_, y)) = y {
        Ok((s, Token::Symbol(format!("{}.{}", x, y))))
    } else {
        Ok((s, Token::Symbol(x.to_string())))
    }
}

fn take_symbol(s: &str) -> IResult<&str, Token> {
    alt((
        take_symbol_ascii,
        take_symbol_unicode,
        take_string_literal,
        take_here_document,
        take_number,
    ))(s)
}

fn take_op_unicode(s: &str) -> IResult<&str, Token> {
    let (s, (t, order)) = pair(one_of("√∛∜"), num_space)(s)?;
    Ok((
        s,
        match t {
            '√' => Token::Op(format!(r"\sqrt"), order),
            '∛' => Token::Op(format!(r"\sqrt[3]"), order),
            '∜' => Token::Op(format!(r"\sqrt[4]"), order),
            _ => unreachable!(),
        },
    ))
}

fn take_op_ascii(s: &str) -> IResult<&str, Token> {
    let (s, (_, base, ascii_props, order)) = tuple((
        char('@'),
        alpha1,
        many0(map(pair(char('.'), alphanumeric1), |(_, x)| x)),
        num_space,
    ))(s)?;
    let tex = format!(r"\{}[{}]", expand_abbred_op(base), ascii_props.join(","));
    Ok((s, Token::Op(tex, order)))
}

fn take_op(s: &str) -> IResult<&str, Token> {
    alt((take_op_ascii, take_op_unicode))(s)
}

fn tokenize(input: &str) -> Vec<Token> {
    todo!()
}

struct Expr {}

fn parse(input: Vec<Token>) -> IResult<Vec<Token>, Expr> {
    todo!()
}

fn main() {
    assert_eq!(get_tex_from_char('Γ').unwrap(), r"\Gamma");
}
