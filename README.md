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
| $\#$ | `\#` | `#` | `##`, `"\#"L`
| $\text{a"b"c\#"}$ | `\text{a"b"c\#"}` || `#="a"b"c#""=#`
| $\mathbf{abc}$ | `\mathbf{abc}` | `bb"abc"` | `"abc"bb`, `#"abc"#bb`
| $(a)$ | `(a)` | `(a)`

## Lexer

- Tokenize
  - sep: `[ ]+`
  - frac: `/`
  - sub: `_`
  - sup: `^`
  - char: single Unicode character
    - alphabet: `[a-zA-Z]`
    - greek: `[ABΓΔEZHΘIKΛMNΞOΠPΣTΥΦXΨΩαβγδϵζηθικλμνξoπρστυϕχψωεϑϖϱςφϝ]`
    - styled: `[𝐀-𝐙𝐚-𝐳𝟎-𝟗𝐴-𝑍𝑎-𝑧𝑨-𝒁𝒂-𝒛𝒜-𝒵𝔄-ℨ𝔞-𝔷𝔸-ℤ𝖠-𝖹𝖺-𝗓𝟢-𝟫𝗔-𝗭𝗮-𝘇𝟬-𝟵𝘈-𝘡𝘢-𝘻𝙰-𝚉𝚊-𝚣𝟶-𝟿𝕜]`
    - misc: `[∂∇ℑℲℵℶℷℸ⅁ℏð]`
    - precomposed: `[ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖÙÚÛÜÝÞßàáâãäåçèéêëìíîïðñòóôöùúûüýþÿ]`
    - unicode_sub: `[₊₋₌₍₎₀₁₂₃₄₅₆₇₈₉ₐₑₕᵢⱼₖₗₘₙₒₚᵣₛₜᵤᵥₓᵦᵧᵨᵩᵪ]`
    - unicode_sup: `[⁺⁻⁼⁽⁾⁰¹²³⁴⁵⁶⁷⁸⁹ᵃᵇᶜᵈᵉᵍʰⁱʲᵏˡᵐⁿᵒᵖʳˢᵗᵘʷˣʸᶻᵛᵝᵞᵟᵠᵡ]`
    - arrow: `[←↑→↓↔↕↖↗↘↙↚↛↞↠↢↣↦↩↪↫↬↭↮↰↱↶↷↺↻↼↽↾↾↿⇀⇁⇂⇃⇄⇆⇇⇈⇉⇊⇋⇌⇍⇎⇏⇐⇑⇒⇓⇔⇕⇚⇛⇝⇠⇢⟵⟶⟷⟸⟹⟺⟼↽]`
    - rel: `[=<>:∈∋∝∼∽≂≃≅≈≊≍≎≏≐≑≒≓≖≗≜≡≤≥≦≧≫≬≳≷≺≻≼≽≾≿⊂⊃⊆⊇⊏⊐⊑⊒⊢⊣⊩⊪⊸⋈⋍⋐⋑⋔⋙⋛⋞⋟⌢⌣⩾⪆⪌⪕⪖⪯⪰⪷⪸⫅⫆≲⩽⪅≶⋚⪋⊥⊨⊶⊷≔≕⩴∉∌∤∦≁≆≠≨≩≮≯≰≱⊀⊁⊈⊉⊊⊋⊬⊭⊮⊯⋠⋡⋦⋧⋨⋩⋬⋭⪇⪈⪉⪊⪵⪶⪹⪺⫋⫌]`
    - op: `[+−∗⋅∘∙±×÷∓∔∧∨∩∪≀⊎⊓⊔⊕⊖⊗⊘⊙⊚⊛⊝◯∖]`
    - big_op: `[∫∬∭∮∏∐∑⋀⋁⋂⋃⨀⨁⨂⨄⨆∯∰]`
    - unicode_sqrt: `√`
  - combined Unicode character
    - `(alphabet|greek|styled|misc|arrow|rel|op|big_op)[\N{Combining Grave Accent}\N{Combining Acute Accent}\N{Combining Circumflex Accent}\N{Combining Tilde}\N{Combining Macron}\N{Combining Overline}\N{Combining Breve}\N{Combining Dot Above}\N{Combining Diaeresis}\N{Combining Hook Above}\N{Combining Ring Above}\N{Combining Double Acute Accent}\N{Combining Caron}\N{Combining Candrabindu}\N{Combining Turned Comma Above}\N{Combining Comma Above Right}\N{Combining Left Angle Above}\N{Combining Palatalized Hook Below}\N{Combining Retroflex Hook Below}\N{Combining Cedilla}\N{Combining Ogonek}\N{Combining Bridge Below}\N{Combining Tilde Below}\N{Combining Low Line}\N{Combining Long Stroke Overlay}\N{Combining Long Solidus Overlay}\N{Combining Left Right Arrow Below}]`
  - num: `[0-9]+(.[0-9]+)?`
  - symbol: `#[a-zA-Z]+(\.[a-zA-Z])*`
  - sharp: `##`
  - open: `#?[\({[]`
  - close `#?[\)}]]`
  - literal: `\"(?!\")\"[a-zA-Z]*` or `#(=*)\"(?!\"\1#)\"\1#[a-zA-Z]*`
