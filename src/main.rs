use std::{fmt::Display, iter::once};
use unicode_normalization::UnicodeNormalization;

fn get_tex(c: char) -> Option<String> {
    let nfkc = |c: char| once(c).nfkc().next().unwrap();

    fn raw(c: char) -> Option<String> {
        Some(c.to_string())
    }
    fn sym<T: Display>(s: T) -> Option<String> {
        Some(format!(r"\{}", s))
    }
    fn cmb<T: Display>(op: &str, arg: T) -> Option<String> {
        Some(format!(r"\{}{{ {} }}", op, arg))
    }

    match c {
        // - ASCII
        ' ' => None,
        '!' => raw(c),
        '"' | '#' => None,
        '$' | '%' | '&' => sym(c),
        '\'' | '(' | ')' => None,
        '*' | '+' | ',' | '-' | '.' => raw(c),
        '/' => None,
        '0'..='9' => None,
        ':' | ';' | '<' | '=' | '>' | '?' | '@' => raw(c),
        'A'..='Z' => raw(c),
        '[' => None,
        '\\' => sym("backslash"),
        ']' | '^' | '_' | '`' => None,
        'a'..='z' => raw(c),
        '{' => None,
        '|' => raw(c),
        '}' => None,
        '~' => sym("sim"),

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
        '𝐀'..='𝐙' | '𝐚'..='𝐳' | '𝟎'..='𝟗' => cmb("mathbf", nfkc(c)),
        '𝐴'..='𝑍' | '𝑎'..='𝑧' | 'ℎ' => cmb("mathit", nfkc(c)),
        '𝑨'..='𝒁' | '𝒂'..='𝒛' => cmb("mathbfit", nfkc(c)),
        '𝒜'..='𝒵' | '𝒶'..='𝓏' | 'ℬ' | 'ℰ' | 'ℱ' | 'ℋ' | 'ℐ' | 'ℒ' | 'ℳ' | 'ℛ' | 'ℯ' | 'ℊ' | 'ℴ' => {
            cmb("mathscr", nfkc(c))
        }
        '𝓐'..='𝓩' | '𝓪'..='𝔃' => cmb("mathbfscr", nfkc(c)),
        '𝔄'..='𝔜' | '𝔞'..='𝔷' | 'ℭ' | 'ℌ' | 'ℑ' | 'ℜ' | 'ℨ' => {
            cmb("mathfrak", nfkc(c))
        }
        '𝔸'..='𝕐' | '𝕒'..='𝕫' | '𝟘'..='𝟡' | 'ℂ' | 'ℍ' | 'ℕ' | 'ℙ' | 'ℚ' | 'ℝ' | 'ℤ' => {
            cmb("mathbb", nfkc(c))
        }
        '𝕬'..='𝖅' | '𝖆'..='𝖟' => cmb("mathbffrak", nfkc(c)),
        '𝖠'..='𝖹' | '𝖺'..='𝗓' | '𝟢'..='𝟫' => cmb("mathsf", nfkc(c)),
        '𝗔'..='𝗭' | '𝗮'..='𝘇' | '𝟬'..='𝟵' => cmb("mathbfsf", nfkc(c)),
        '𝘈'..='𝘡' | '𝘢'..='𝘻' => cmb("mathsfit", nfkc(c)),
        '𝘼'..='𝙕' | '𝙖'..='𝙯' => cmb("mathbfsfit", nfkc(c)),
        '𝙰'..='𝚉' | '𝚊'..='𝚣' | '𝟶'..='𝟿' => cmb("mathtt", nfkc(c)),
        //     * Dotless
        '𝚤' => sym("imath"),
        '𝚥' => sym("jmath"),
        //   - Greek alphabets
        //   ignore Bold/Italic style
        '𝛢'..='𝜛' | '𝚨'..='𝛡' | '𝜜'..='𝝕' | '𝝖'..='𝞏' | '𝞐'..='𝟉' | '𝟋' => {
            get_tex(nfkc(c))
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
        '∕' => None,
        '∖' => sym("setminus"),
        '∗' => sym("ast"),
        '∘' => sym("circ"),
        '∙' => sym("bullet"),
        '√' | '∛' | '∜' => None,
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
        _ => None,
    }
}

fn unicode_accent_to_tex() -> String {
    "aaa".to_string()
}

fn get_sub(c: char) -> Option<char> {
    match c {
        '₊' | '₋' | '₌' | '₍' | '₎' | '₀' | '₁' | '₂' | '₃' | '₄' | '₅' | '₆' | '₇' | '₈' | '₉'
        | 'ₐ' | 'ₑ' | 'ₕ' | 'ᵢ' | 'ⱼ' | 'ₖ' | 'ₗ' | 'ₘ' | 'ₙ' | 'ₒ' | 'ₚ' | 'ᵣ' | 'ₛ' | 'ₜ'
        | 'ᵤ' | 'ᵥ' | 'ₓ' | 'ᵦ' | 'ᵧ' | 'ᵨ' | 'ᵩ' | 'ᵪ' => once(c).nfkc().next(),
        _ => None,
    }
}

fn get_sup(c: char) -> Option<char> {
    match c {
        '⁺' | '⁻' | '⁼' | '⁽' | '⁾' | '⁰' | '¹' | '²' | '³' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹'
        | 'ᴬ' | 'ᴮ' | 'ᴰ' | 'ᴱ' | 'ᴳ' | 'ᴴ' | 'ᴵ' | 'ᴶ' | 'ᴷ' | 'ᴸ' | 'ᴹ' | 'ᴺ' | 'ᴼ' | 'ᴾ'
        | 'ᴿ' | 'ᵀ' | 'ᵁ' | 'ⱽ' | 'ᵂ' | 'ᵃ' | 'ᵇ' | 'ᶜ' | 'ᵈ' | 'ᵉ' | 'ᵍ' | 'ʰ' | 'ⁱ' | 'ʲ'
        | 'ᵏ' | 'ˡ' | 'ᵐ' | 'ⁿ' | 'ᵒ' | 'ᵖ' | 'ʳ' | 'ˢ' | 'ᵗ' | 'ᵘ' | 'ᵛ' | 'ʷ' | 'ˣ' | 'ʸ'
        | 'ᶻ' | 'ᵝ' | 'ᵞ' | '\u{1D5F}' | 'ᶿ' | 'ᵠ' | 'ᵡ' => once(c).nfkc().next(),
        'ᵅ' => Some('α'),
        'ᵋ' => Some('ε'),
        'ᶥ' => Some('ι'),
        'ᶲ' => Some('ϕ'),
        'ꜛ' => Some('↑'),
        'ꜜ' => Some('↓'),
        'ꜝ' => Some('!'),
        _ => None,
    }
}
fn main() {
    assert_eq!(get_tex('Γ').unwrap(), r"\Gamma");
}
