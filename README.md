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
| $a$ | `a` | `a` | `a`, `<a>`
| $\hat a$ | `\hat a` | `hat a` | `â`, `<<hat>>a`, `<<hat>><a>`, `<a.hat>`
| $\alpha$ | `\alpha` | `alpha` | `α`, `<alpha>`
| $\not\hat\alpha$ | `\not\hat\alpha` | `cancel hat alpha` | `<alpha hat not>`, `<alpha hat!>`, `<!alpha hat>`, `<!α hat>`, `<!α̂>`, `<<not>><<hat>><alpha>` `α̸̂`
| $\infty$ | `\infry` | `oo`, `infty` | `<infty>`, `.oo.`
| $\dot\infty$ | `\dot\infty` | `dot oo`, `dot infty` | `<infty dot>`, `<.oo. dot>`
| $<$ | `<` | `<` | `.<.`
| $\not<$ | `\not<` | `cancel <` | `<.<. not>`, `<!.<.>`, `≮`
| $\sqrt{2}$ | `\sqrt{2}` | `sqrt 2`, `sqrt[2]` | `√2`, `<<sqrt>>2`, `<<sqrt>>[2]`
| $\sqrt{3+4}$ | `\sqrt{3+4}` | `sqrt[3+4]` | `√ 3+4`, `√[3+4]`, `<<sqrt>> 3+4`, `<<sqrt>>[3+4]`
| $\mathrm{abc}$ | `\mathrm{abc}` | `"abc"` | `"abc"`, `` `[abc]` ``
| $\text{ab]`c}$ | ``\text{ab]`c}`` || `` `=[ ab]`c ]=` ``
| $\mathbf{abc}$ | `\mathbf{abc}` | `bb"abc"` | `"abc"b`
| $\lVert a \rVert$ | `\lVert a \rVert` | `norm(a)` | `<<‖> a <‖>>`, `<<.\|\|.> a <.\|\|.>>`

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
- racᵐᵃˣ⁽ⁱ⁾⁽ʲ⁾: `[ ]{i}/[ ]{j}`
- opⁱ: `(√|∛|∜|<\+[a-zA-Z]+( [a-zA-Z0-9]+)*>)[ ]{i}`
  - rootⁱ: `(√|∛|∜|<<sqrt>>|<<root[0-9]+>>)[ ]{i}`
  - typeⁱ: `(<<ord>>|<<op>>|<<bin>>|<<rel>>|<<open>>|<<close>>|<<punct>>)[ ]{i}`
- open:
  - visible: `.[. <<[> <<.<.> <<|> <<||> <<‖> ( { ⟨ ⌈ ⌊ ⎰ ⌜ ⌞ ⟦`
  - invisible: `[`
- close:
  - visible: `.]. <]>> <.>.>> <.>.>> <||>> <‖>> ) } ⟩ ⌉ ⌋ ⎱ ⌝ ⌟ ⟧`
  - invisible: `]`
- num: `[0-9]+(\.[0-9]+)?`
- literal: `\"(?!\")\"[a-zA-Z]*` or `#(=*)\"(?!\"\1#)\"\1#[a-zA-Z]*`
- symbol
  - `./.`
  - `.||.`
  - `.<-.`
  - `.->.`
  - `.<.`
  - `.<=.`
  - `.>.`
  - `.>=.`
- identifier: `(#|#!)?(char)(accent)*(\.[A-Za-z0-9]+)*`
  - char: single Unicode character
    - `! $ % & , ; ? @`
    - `* + - : < = > |`
    - `[a-zA-Z]`
    - `~`
    - `[𝐀-𝐙𝐚-𝐳𝟎-𝟗𝐴-𝑍𝑎-𝑧𝑨-𝒁𝒂-𝒛𝒜-𝒵𝔄-ℨ𝔞-𝔷𝔸-ℤ𝖠-𝖹𝖺-𝗓𝟢-𝟫𝗔-𝗭𝗮-𝘇𝟬-𝟵𝘈-𝘡𝘢-𝘻𝙰-𝚉𝚊-𝚣𝟶-𝟿𝕜]`
    - `± × ð ÷`
    - `Γ Δ Θ Λ Ξ Π Σ Υ Φ Ψ Ω α β γ δ ε ζ η θ ι κ λ μ ν ξ π ρ ς σ τ υ φ χ ψ ω ϑ ϕ ϖ ϝ ϱ ϵ`
    - `† ‡ … ħ ℏ ℑ Ⅎ ℵ ℶ ℷ ℸ ⅁`
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
    - grave: `Combining Grave Accent` '\u{0300}'
    - acute: `Combining Acute Accent` '\u{0301}'
    - hat: `Combining Circumflex Accent`
    - tilde: `Combining Tilde`
    - bar: `Combining Macron`
    - overbar: `Combining Overline`
    - breve: `Combining Breve`
    - dot: `Combining Dot Above`
    - ddot: `Combining Diaeresis`
    - mathring: `Combining Ring Above`
    - check: `Combining Caron`
    - underline: `Combining Low Line`
    - not: `Combining Long Solidus Overlay`
    - underleftrightarrow: `Combining Left Right Arrow Below`
  - `(#|#!)/(accent)*(\.[A-Za-z0-9]+)*`
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
const = num | literal | symbol;
```
