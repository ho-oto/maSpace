# maSpace

## sample

| Result | LaTeX | AsciiMath | maSpace |
|-|-|-|-|
| $\frac{a+b}{c}$ | `\frac{a+b}{c}` | `(a+b)/c` | `a+b␣/c`
| $a+\frac{b}{c}$ | `a+\frac{b}{c}` | `a+b/c` | `a+b/c`
| $a_{b^c}$ | `a_{b^c}` | `a_(b^c)` | `a␣_b^c`
| $a_b^c$ | `a_b^c` | `a_b^c` | `a_b^c`
| $\frac{a_{b_c}^{d^{e+f}_g}}{h}$ | `\frac{a_{b_c}^{d^{e+f}_g}}{h}` | `a_[b_c]^[d_g^[e+f]]/h` | `a␣_b_c␣␣^d␣^e+f␣_g␣␣/h`
|||| `a␣_b_c␣^d^[e+f]_g␣/h`
| $a_{b_c^d}^{e+f_{\frac{g}{h}}}$ | `a_{b_c^d}^{e+f_{\frac{g}{h}}}` | `a_[b_c^d]^[e+f_[g/h]]` | `a␣_b_c^d␣^[e+f␣_g/h]`
|||| `a␣_b_c^d␣␣^e+f␣_g/h`
| $a_{b_{c^d}}^e+\frac{f_g}{h}$ | `a_{b_{c^d}}^e+\frac{f_g}{h}` | `a_[b_[c^d]]^[e]+[f_g]/h` | `a␣␣_b␣_c^d␣␣^e␣␣+␣␣f_g/h`
|||| `a␣␣_b␣_c^d␣␣^e␣+␣f_g/h`
| $a$ | `a` | `a` | `a`, `#a`
| $\alpha$ | `\alpha` | `alpha` | `α`, `#alpha`
| $\sqrt{2}$ | `\sqrt{2}` | `sqrt 2`, `sqrt[2]` | `√2`, `#sqrt 2`, `#sqrt[2]`
| $\mathrm{abc}$ | `\mathrm{abc}` | `"abc"` | `"abc"`
| $\hat a$ | `\hat a` | `hat a` | `â`, `#hat a`, `"\hat a"L`, `a.hat`
| $\\\#$ | `\#` | `#` | `##`, `"\#"L`
| $\text{a"b"c\\\#"}$ | `\text{a"b"c\#"}` || `#="a"b"c#""=#`
| $\mathbf{abc}$ | `\mathbf{abc}` | `bb"abc"` | `"abc"bb`, `#"abc"#bb`

## Lexer

1. NFD normalization
2. remove leading and trailing spaces
3. tokenize
4. insert virtual cat⁰ between connected symbols with no spaces
5. transform unicode_sub and unicode_sup to ASCII

### Tokens

- catᵒᵒ: `[ ]+`
- subᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}_[ ]{j}`
- supᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}\^[ ]{j}`
- overᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}\^\^[ ]{j}`
- underᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}__[ ]{j}`
- fracᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}/[ ]{j}`
- opⁱ
  - rootⁱ: `(√|∛|∜|#sqrt|#root\.[1-9][0-9]*)[ ]{i}`
  - typeⁱ: `(#ord|#op|#bin|#rel|#open|#close|#punct)[ ]{i}`
- open:
  - visible: `#[ #< #<|< #<||< #<‖< ( { ⟨ ⌈ ⌊ ⎰ ⌜ ⌞ ⟦`
  - invisible: `[`
- close:
  - visible: `#] #> #>|> #>||> #>‖> ) } ⟩ ⌉ ⌋ ⎱ ⌝ ⌟ ⟧`
  - invisible: `]`
- num: `[0-9]+(\.[0-9]+)?`
- literal: `\"(?!\")\"[a-zA-Z]*` or `#(=*)\"(?!\"\1#)\"\1#[a-zA-Z]*` or `'(?!')'[a-zA-Z]*` or `#(=*)'(?!'\1#)'\1#[a-zA-Z]*`
- symbol
  - `//`
  - `#[a-zA-Z]+(\.[a-zA-Z0-9]+)*`
  - `#.||.`
  - `#.<-.`
  - `#.->.`
  - `#.<=.`
  - `#.>=.`
- identifier: `(#|#!)?(char)(accent)*(\.[A-Za-z0-9]+)*`
  - char: single Unicode character
    - ascii_symbols: `! $ % & ,  ; ? @ /`
    - alphabet: `[a-zA-Z]`
    - styled: `[𝐀-𝐙𝐚-𝐳𝟎-𝟗𝐴-𝑍𝑎-𝑧𝑨-𝒁𝒂-𝒛𝒜-𝒵𝔄-ℨ𝔞-𝔷𝔸-ℤ𝖠-𝖹𝖺-𝗓𝟢-𝟫𝗔-𝗭𝗮-𝘇𝟬-𝟵𝘈-𝘡𝘢-𝘻𝙰-𝚉𝚊-𝚣𝟶-𝟿𝕜]`
    - `* + - : < = > | ~`
    - `± × ð ÷`
    - `Γ Δ Θ Λ Ξ Π Σ Υ Φ Ψ Ω α β γ δ ε ζ η θ ι κ λ μ ν ξ π ρ ς σ τ υ φ χ ψ ω ϑ ϕ ϖ ϝ ϱ ϵ`
    - `† ‡ … ℏ ℑ Ⅎ ℵ ℶ ℷ ℸ ⅁`
    - `← ↑ → ↓ ↔ ↕ ↖ ↗ ↘ ↙ ↞ ↠ ↢ ↣ ↦ ↩ ↪ ↫ ↬ ↭ ↰ ↱ ↶ ↷ ↺ ↻ ↼ ↽ ↾ ↿ ⇀ ⇁ ⇂ ⇃ ⇄ ⇆ ⇇ ⇈ ⇉ ⇊ ⇋ ⇌ ⇐ ⇑ ⇒ ⇓ ⇔ ⇕ ⇚ ⇛ ⇝ ⇠ ⇢`
    - `∀ ∁ ∂ ∃ ∅ ∆ ∇ ∈ ∊ ∋ ∍ ∎ ∏ ∐ ∑ − ∓ ∔ ∕ ∖ ∗ ∘ ∙ ∝ ∞ ∟ ∠ ∡ ∢ ∣ ∥ ∧ ∨ ∩ ∪ ∫ ∬ ∭ ∮ ∯ ∰ ∴ ∵`
    - `∶ ∷ ∸ ∹ ∺ ∻ ∼ ∽ ≀ ≂ ≃ ≅ ≆ ≈ ≊ ≍ ≎ ≏ ≐ ≑ ≒ ≓ ≔ ≕ ≖ ≗ ≘ ≙ ≚ ≛ ≜ ≝ ≞ ≟ ≡ ≤ ≥ ≦ ≧ ≨ ≩ ≪ ≫ ≬ ≲ ≳ ≶ ≷ ≺ ≻ ≼ ≽ ≾ ≿ ⊂ ⊃ ⊆ ⊇ ⊊ ⊋ ⊎ ⊏ ⊐ ⊑ ⊒ ⊓ ⊔`
    - `⊕ ⊖ ⊗ ⊘ ⊙ ⊚ ⊛ ⊝ ⊞ ⊟ ⊠ ⊡ ⊢ ⊣ ⊤ ⊥ ⊦ ⊧ ⊨ ⊩ ⊪ ⊫ ⊲ ⊳ ⊴ ⊵ ⊶ ⊷ ⊸ ⊺ ⊻ ⊼ ⊽`
    - `⋀ ⋁ ⋂ ⋃ ⋄ ⋅ ⋆ ⋇ ⋈ ⋉ ⋊ ⋋ ⋌ ⋍ ⋎ ⋏ ⋐ ⋑ ⋒ ⋓ ⋔ ⋖ ⋗ ⋘ ⋙ ⋚ ⋛ ⋜ ⋝ ⋞ ⋟ ⋤ ⋥ ⋦ ⋧ ⋨ ⋩`
    - `⋮ ⋯ ⋰ ⋱`
    - `⋲ ⋳ ⋴ ⋵ ⋶ ⋷ ⋸ ⋹ ⋺ ⋻ ⋼ ⋽ ⋾ ⋿`
    - `⌢ ⌣ ◯ ⟵ ⟶ ⟷ ⟸ ⟹ ⟺ ⟼ ⨀`
    - `⨁ ⨂ ⨄ ⨆ ⨿ ⩴ ⩽ ⩾ ⪅ ⪆ ⪇ ⪈ ⪉ ⪊ ⪋ ⪌ ⪕ ⪖ ⪯ ⪰ ⪵ ⪶ ⪷ ⪸ ⪹ ⪺ ⫅ ⫆ ⫋ ⫌`
  - accent
    - `Combining Grave Accent`
    - `Combining Acute Accent`
    - `Combining Circumflex Accent`
    - `Combining Tilde`
    - `Combining Macron`
    - `Combining Overline`
    - `Combining Breve`
    - `Combining Dot Above`
    - `Combining Diaeresis`
    - `Combining Hook Above`
    - `Combining Ring Above`
    - `Combining Double Acute Accent`
    - `Combining Caron`
    - `Combining Candrabindu`
    - `Combining Turned Comma Above`
    - `Combining Comma Above Right`
    - `Combining Left Angle Above`
    - `Combining Palatalized Hook Below`
    - `Combining Retroflex Hook Below`
    - `Combining Cedilla`
    - `Combining Ogonek`
    - `Combining Bridge Below`
    - `Combining Tilde Below`
    - `Combining Low Line`
    - `Combining Long Stroke Overlay`
    - `Combining Long Solidus Overlay`
    - `Combining Left Right Arrow Below`
- unicode_sub: `₊₋₌₍₎₀₁₂₃₄₅₆₇₈₉ₐₑₕᵢⱼₖₗₘₙₒₚᵣₛₜᵤᵥₓᵦᵧᵨᵩᵪ`
- unicode_sup: `⁺⁻⁼⁽⁾⁰¹²³⁴⁵⁶⁷⁸⁹ᵃᵇᶜᵈᵉᵍʰⁱʲᵏˡᵐⁿᵒᵖʳˢᵗᵘʷˣʸᶻᵛᵝᵞᵟᵠᵡ`

## Grammer

```ebnf
maspace = mathᵒᵒ;
mathⁱ = exprⁱ, (catⁱ, exprⁱ)*;
exprⁱ = intermediateⁱ, [fracⁱ, intermediateⁱ];
intermediateⁱ = simpⁱ, [overⁱ simpⁱ], [underⁱ simpⁱ], [supⁱ simpⁱ], [subⁱ simpⁱ];
simpⁱ = const | parened | unary_exprⁱ | mathⁱ⁻¹;
unary_exprⁱ = opⁱ, simpⁱ⁻¹;
parened = open, maspace, close;
const = num | literal | symbol | identifier;
```
