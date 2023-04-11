use std::{fmt::Display, iter::once};
use unicode_normalization::{is_nfkc_quick, IsNormalized, UnicodeNormalization};

fn unicode_char_to_tex(c: char) -> Option<String> {
    let shift = |character: char, base: char, ascii_base: char| {
        char::from_u32(u32::from(character) - u32::from(base) + u32::from(ascii_base)).unwrap()
    };

    fn raw(c: char) -> Option<String> {
        Some(c.to_string())
    }
    fn sym<T: Display>(s: T) -> Option<String> {
        Some(format!(r"\{}", s))
    }
    fn cmb<T: Display>(op: &str, arg: T) -> Option<String> {
        Some(format!(r"\{}{{ {} }}", op, arg))
    }
    fn cmb2<T: Display>(op1: &str, op2: &str, arg: T) -> Option<String> {
        Some(format!(r"\{}{{ \{}{{ {} }} }}", op1, op2, arg))
    }

    match c {
        // - ASCII
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
        'ϝ' | 'Ϝ' => sym("digamma"), // Digamma -> digamma
        'ϴ' => sym("Theta"),          // Theta Symbol -> Theta

        // - Mathematical Alphanumeric Symbols (1D400-1D7FF)
        //   - Alphabet
        //     * Mathematical Bold
        '𝐀'..='𝐙' => cmb("mathbf", shift(c, '𝐀', 'A')),
        '𝐚'..='𝐳' => cmb("mathbf", shift(c, '𝐚', 'a')),
        '𝟎'..='𝟗' => cmb("mathbf", shift(c, '𝟎', '0')),

        //     * Mathematical Italic
        '𝐴'..='𝑍' => cmb("mathit", shift(c, '𝐴', 'A')),
        '𝑎'..='𝑧' => cmb("mathit", shift(c, '𝑎', 'a')),
        'ℎ' => cmb("mathit", 'h'),

        //     * Mathematical Bold Italic
        '𝑨'..='𝒁' => cmb("bm", shift(c, '𝑨', 'A')),
        '𝒂'..='𝒛' => cmb("bm", shift(c, '𝒂', 'a')),

        //     * Mathematical Script
        '𝒜'..='𝒵' => cmb("mathscr", shift(c, '𝒜', 'A')),
        '𝒶'..='𝓏' => cmb("mathscr", shift(c, '𝒶', 'a')),
        'ℬ' => cmb("mathscr", 'B'),
        'ℰ' => cmb("mathscr", 'E'),
        'ℱ' => cmb("mathscr", 'F'),
        'ℋ' => cmb("mathscr", 'H'),
        'ℐ' => cmb("mathscr", 'I'),
        'ℒ' => cmb("mathscr", 'L'),
        'ℳ' => cmb("mathscr", 'M'),
        'ℛ' => cmb("mathscr", 'R'),
        'ℯ' => cmb("mathscr", 'e'),
        'ℊ' => cmb("mathscr", 'g'),
        'ℴ' => cmb("mathscr", 'o'),

        //     * Mathematical Bold Script
        '𝓐'..='𝓩' => cmb("mathbfscr", shift(c, '𝓐', 'A')),
        '𝓪'..='𝔃' => cmb("mathbfscr", shift(c, '𝓪', 'a')),

        //     * Mathematical Fraktur
        '𝔄'..='𝔜' => cmb("mathfrak", shift(c, '𝔄', 'A')),
        '𝔞'..='𝔷' => cmb("mathfrak", shift(c, '𝔞', 'a')),
        'ℭ' => cmb("mathfrak", 'C'),
        'ℌ' => cmb("mathfrak", 'H'),
        'ℑ' => cmb("mathfrak", 'I'),
        'ℜ' => cmb("mathfrak", 'R'),
        'ℨ' => cmb("mathfrak", 'Z'),

        //     * Mathematical Double-Struck
        '𝔸'..='𝕐' => cmb("mathbb", shift(c, '𝔸', 'A')),
        '𝕒'..='𝕫' => cmb("mathbb", shift(c, '𝕒', 'a')),
        '𝟘'..='𝟡' => cmb("mathbb", shift(c, '𝟘', '0')),
        'ℂ' => cmb("mathbb", 'C'),
        'ℍ' => cmb("mathbb", 'H'),
        'ℕ' => cmb("mathbb", 'N'),
        'ℙ' => cmb("mathbb", 'P'),
        'ℚ' => cmb("mathbb", 'Q'),
        'ℝ' => cmb("mathbb", 'R'),
        'ℤ' => cmb("mathbb", 'Z'),

        //     * Mathematical Bold Fraktur
        '𝕬'..='𝖅' => cmb("mathbffrak", shift(c, '𝕬', 'A')),
        '𝖆'..='𝖟' => cmb("mathbffrak", shift(c, '𝖆', 'a')),

        //     * Mathematical Sans-Serif
        '𝖠'..='𝖹' => cmb("mathsf", shift(c, '𝖠', 'A')),
        '𝖺'..='𝗓' => cmb("mathsf", shift(c, '𝖺', 'a')),
        '𝟢'..='𝟫' => cmb("mathsf", shift(c, '𝟢', '0')),

        '𝗔'..='𝗭' => cmb("mathbfsf", shift(c, '𝗔', 'A')),
        '𝗮'..='𝘇' => cmb("mathbfsf", shift(c, '𝗮', 'a')),
        '𝟬'..='𝟵' => cmb("mathbfsf", shift(c, '𝟬', '0')),

        '𝘈'..='𝘡' => cmb("mathsfit", shift(c, '𝘈', 'A')),
        '𝘢'..='𝘻' => cmb("mathsfit", shift(c, '𝘢', 'a')),

        '𝘼'..='𝙕' => cmb("mathbfsfit", shift(c, '𝘼', 'A')),
        '𝙖'..='𝙯' => cmb("mathbfsfit", shift(c, '𝙖', 'a')),

        //     * Mathematical Monospace
        '𝙰'..='𝚉' => cmb("mathtt", shift(c, '𝙰', 'A')),
        '𝚊'..='𝚣' => cmb("mathtt", shift(c, '𝚊', 'a')),
        '𝟶'..='𝟿' => cmb("mathtt", shift(c, '𝟶', '0')),

        //     * Dotless
        '𝚤' => sym("imath"),
        '𝚥' => sym("jmath"),

        //   - Greek alphabets
        //   ignore Bold/Italic style
        '𝛢'..='𝛲' => unicode_char_to_tex(shift(c, '𝛢', 'Α')), // it
        '𝚨'..='𝚸' => unicode_char_to_tex(shift(c, '𝚨', 'Α')), // bf
        '𝜜'..='𝜬' => unicode_char_to_tex(shift(c, '𝜜', 'Α')), // bfit
        '𝝖'..='𝝦' => unicode_char_to_tex(shift(c, '𝝖', 'Α')), // bfsf
        '𝞐'..='𝞠' => unicode_char_to_tex(shift(c, '𝞐', 'Α')), // bfsfit

        '𝛳' | '𝚹' | '𝜭' | '𝝧' | '𝞡' => sym("Theta"),

        '𝛴'..='𝛺' => unicode_char_to_tex(shift(c, '𝛴', 'Σ')),
        '𝚺'..='𝛀' => unicode_char_to_tex(shift(c, '𝚺', 'Σ')),
        '𝜮'..='𝜴' => unicode_char_to_tex(shift(c, '𝜮', 'Σ')),
        '𝝨'..='𝝮' => unicode_char_to_tex(shift(c, '𝝨', 'Σ')),
        '𝞢'..='𝞨' => unicode_char_to_tex(shift(c, '𝞢', 'Σ')),

        '𝛼'..='𝜔' => unicode_char_to_tex(shift(c, '𝛼', 'α')),
        '𝛂'..='𝛚' => unicode_char_to_tex(shift(c, '𝛂', 'α')),
        '𝜶'..='𝝎' => unicode_char_to_tex(shift(c, '𝜶', 'α')),
        '𝝰'..='𝞈' => unicode_char_to_tex(shift(c, '𝝰', 'α')),
        '𝞪'..='𝟂' => unicode_char_to_tex(shift(c, '𝞪', 'α')),

        '𝜖' | '𝛜' | '𝝐' | '𝞊' | '𝟄' => sym("epsilon"),
        '𝜗' | '𝛝' | '𝝑' | '𝞋' | '𝟅' => sym("vartheta"),
        '𝜘' | '𝛞' | '𝝒' | '𝞌' | '𝟆' => sym("varkappa"),
        '𝜙' | '𝛟' | '𝝓' | '𝞍' | '𝟇' => sym("phi"),
        '𝜚' | '𝛠' | '𝝔' | '𝞎' | '𝟈' => sym("varrho"),
        '𝜛' | '𝛡' | '𝝕' | '𝞏' | '𝟉' => sym("varpi"),
        '𝟋' | '𝟊' => sym("digamma"),

        '𝛻' | '𝛁' | '𝜵' | '𝝯' | '𝞩' => sym("nabla"),
        '𝜕' | '𝛛' | '𝝏' | '𝞉' | '𝟃' => sym("partial"),

        'ı' => cmb("text", 'ı'),
        'ȷ' => cmb("text", 'ȷ'),

        // - Mathematical Symbols

        //   ±×ð÷†‡…ħℏℑℲℵℶℷℸ⅁
        //   ←↑→↓↔↕↖↗↘↙↞↠↢↣↦↩↪↫↬↭↰↱↶↷↺↻↼↽↾↿⇀⇁⇂⇃⇄⇆⇇⇈⇉⇊⇋⇌⇐⇑⇒⇓⇔⇕⇚⇛⇝⇠⇢
        //   ∀∁∂∃∅∆∇∈∊∋∍∎∏∐∑−∓∔∕∖∗∘∙∝∞∟∠∡∢∣∥∧∨∩∪∫∬∭∮∯∰
        //   ∴∵∶∷∸∹∺∻∼∽≀≂≃≅≆≈≊≍≎≏≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≡
        //   ≤≥≦≧≨≩≪≫≬≲≳≶≷≺≻≼≽≾≿⊂⊃⊆⊇⊊⊋⊎⊏⊐⊑⊒⊓⊔
        //   ⊕⊖⊗⊘⊙⊚⊛⊝⊞⊟⊠⊡⊢⊣⊤⊥⊦⊧⊨⊩⊪⊫⊲⊳⊴⊵⊶⊷⊸⊺⊻⊼⊽
        //   ⋀⋁⋂⋃⋄⋅⋆⋇⋈⋉⋊⋋⋌⋍⋎⋏⋐⋑⋒⋓⋔⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋤⋥⋦⋧⋨⋩
        //   ⋮⋯⋰⋱⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⌢⌣◯⟵⟶⟷⟸⟹⟺⟼⨀⨁⨂⨄⨆⨿
        //   ⩴⩽⩾⪅⪆⪇⪈⪉⪊⪋⪌⪕⪖⪯⪰⪵⪶⪷⪸⪹⪺⫅⫆⫋⫌
        '±' => sym("pm"),
        '×' => sym("times"),
        // 'ð'
        '÷' => sym("div"),
        '†' => sym("dagger"),
        '‡' => sym("ddagger"),
        '…' => sym("ldots"),
        'ħ' => sym("hbar"),
        'ℏ' => sym("hslash"),
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
        // '⇠'
        // '⇢'
        '∀' => sym("forall"),
        '∁' => sym("complement"),
        '∂' => sym("partial"),
        '∃' => sym("exists"),
        '∅' => sym("emptyset"),
        // '∆' => sym("increment"), // \Delta ? \mathop{\Delta} ?
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
        '∝' => sym("propto"),
        '∞' => sym("infty"),
        // '∟'
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
        // '∯'
        // '∰'
        '∴' => sym("therefore"),
        '∵' => sym("because"),
        '∶' => raw(':'),
        '∷' => sym("dblcolon"),
        // '∸'
        '∹' => sym("eqcolon"),
        // '∺'
        // '∻'
        '∼' => sym("sim"),
        '∽' => sym("backsim"),
        '≀' => sym("wr"),
        '≂' => sym("eqsim"),
        '≃' => sym("simeq"),
        '≅' => sym("cong"),
        // '≆'
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
        // '≘'
        // '≙'
        // '≚'
        // '≛'
        '≜' => sym("triangleq"),
        // '≝'
        // '≞'
        // '≟'
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
        // '⊫'
        '⊲' => sym("vartriangleleft"),
        '⊳' => sym("vartriangleright"),
        '⊴' => sym("trianglelefteq"),
        '⊵' => sym("trianglerighteq"),
        // '⊶'
        // '⊷'
        '⊸' => sym("multimap"),
        '⊺' => sym("intercal"),
        '⊻' => sym("veebar"),
        '⊼' => sym("barwedge"),
        // '⊽'
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
        // '⋜'
        // '⋝'
        '⋞' => sym("curlyeqprec"),
        '⋟' => sym("curlyeqsucc"),
        // '⋤'
        // '⋥'
        '⋦' => sym("lnsim"),
        '⋧' => sym("gnsim"),
        '⋨' => sym("precnsim"),
        '⋩' => sym("succnsim"),
        '⋮' => sym("vdots"),
        '⋯' => sym("cdots"),
        // '⋰'
        '⋱' => sym("ddots"),
        // '⋲'
        // '⋳'
        // '⋴'
        // '⋵'
        // '⋶'
        // '⋷'
        // '⋸'
        // '⋹'
        // '⋺'
        // '⋻'
        // '⋼'
        // '⋽'
        // '⋾'
        // '⋿'
        '⌢' => sym("frown"),
        '⌣' => sym("smile"),
        '◯' => sym("bigcirc"),
        '⟵' => sym("longleftarrow"),
        '⟶' => sym("longrightarrow"),
        '⟷' => sym("longleftrightarrow"),
        '⟸' => sym("Longleftarrow"),
        '⟹' => sym("Longrightarrow"),
        '⟺' => sym("iff"),
        '⟼' => sym("longmapsto"),
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

        //'∸' => Some(r"\dot{ - }".to_string()),
        //'≆' => Some(r"\mathrel{ \widetilde{ \ne } }".to_string()),
        //'≘' => Some(r"\stackrel{ \frown }{ = }".to_string()),
        //'≙' => Some(r"\stackrel{ \wedge }{ = }".to_string()),
        //'≚' => Some(r"\stackrel{ \vee }{ = }".to_string()),
        //'≛' => Some(r"\stackrel{ \star }{ = }".to_string()),
        //'≝' => Some(r"\stackrel{ \mathrm{def} }{ = }".to_string()),
        //'≞' => Some(r"\stackrel{ \mathrm{m} }{ = }".to_string()),
        //'≟' => Some(r"\stackrel{ ? }{ = }".to_string()),
        //'⊽' => cmb("bar", sym("vee")?),
        //'⋵' => Some(r"\dot{ \in }".to_string()),
        //'⋶' | '⋷' => Some(r"\bar{ \in }".to_string()),
        //'⋸' => Some(r"\underline{ \in }".to_string()),
        //'⋽' | '⋾' => Some(r"\bar{ \ni }".to_string()),

        //   - unsupported
        'ð' | '⇠' | '⇢' | '∟' | '∺' | '∻' | '⊫' | '⋜' | '⋝' | '⋤' | '⋥' | '⋰' | '⋲' | '⋳' | '⋴'
        | '⋹' | '⋺' | '⋻' | '⋼' | '⋿' => raw(c),

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
