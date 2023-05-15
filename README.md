# maSpace

## sample

| Result | LaTeX | AsciiMath | maSpace |
|-|-|-|-|
| $\frac{a+b}{c}$ | `\frac{a+b}{c}` | `(a+b)/c` | `a+b /c` (`a+b␣/c`)
| $a+\frac{b}{c}$ | `a+\frac{b}{c}` | `a+b/c` | `a+b/c`
| $a_{b^c}$ | `a_{b^c}` | `a_(b^c)` | `a _b^c` (`a␣_b^c`)
| $a_b^c$ | `a_b^c` | `a_b^c` | `a_b^c`
| $\frac{a_{b_c}^{d^{e+f}_g}}{h}$ | `\frac{a_{b_c}^{d^{e+f}_g}}{h}` | `a_[b_c]^[d_g^[e+f]]/h` | `a _b_c  ^d ^e+f _g  /h` (`a␣_b_c␣␣^d␣^e+f␣_g␣␣/h`)
|||| `a _b_c ^d^[e+f]_g /h` (`a␣_b_c␣^d^[e+f]_g␣/h`)
| $a_{b_c^d}^{e+f_{\frac{g}{h}}}$ | `a_{b_c^d}^{e+f_{\frac{g}{h}}}` | `a_[b_c^d]^[e+f_[g/h]]` | `a _b_c^d ^[e+f _g/h]` (`a␣_b_c^d␣^[e+f␣_g/h]`)
|||| `a _b_c^d  ^e+f _g/h` (`a␣_b_c^d␣␣^e+f␣_g/h`)
| $a_{b_{c^d}}^e+\frac{f_g}{h}$ | `a_{b_{c^d}}^e+\frac{f_g}{h}` | `a_[b_[c^d]]^[e]+[f_g]/h` | `a  _b _c^d  ^e  +  f_g/h` (`a␣␣_b␣_c^d␣␣^e␣␣+␣␣f_g/h`)
|||| `a  _b _c^d  ^e + f_g/h` (`a␣␣_b␣_c^d␣␣^e␣+␣f_g/h`)
| $a$ | `a` | `a` | `a`
|||| `<a>`
| $\hat a$ | `\hat a` | `hat a` | `â`
|||| `<'hat>a`
|||| `<'hat><a>`
|||| `<a hat>`
| $\alpha'$ | `\alpha'` | `alpha'` | `α'`
|||| `<alpha>'`
| $\not\hat\alpha$ | `\not\hat\alpha` | `cancel hat alpha` | `<alpha hat not>`
|||| `<alpha hat!>`
|||| `<α hat !>`
|||| `<α̂!>`
|||| `<'not><'hat><alpha>`
|||| `α̸̂`
| $\infty$ | `\infry` | `infty` | `<infty>`
||| `oo` | `` `oo` ``
|||| `∞`
| $\dot\infty$ | `\dot\infty` | `dot infty` | `<infty dot>`
||| `dot oo` | ``<`oo` dot>``
|||| `<∞ dot>`
| $<$ | `<` | `<` | `` `<` ``
| $\not<$ | `\not<` | `cancel <` | ``<`<` not>``
|||| ``<!`<`>``
|||| `≮`
| $\sqrt{2}$ | `\sqrt{2}` | `sqrt 2` | `<'sqrt>2`
||| `sqrt[2]` | `<'sqrt>[2]`
|||| `√2`
|||| `` `_/`2 ``
| $\sqrt[3]{123}$ | `\sqrt[3]{123}` | `root 3 123` | `3 _/ 123`
| $\sqrt{3+4}$ | `\sqrt{3+4}` | `sqrt[3+4]` | `√ 3+4`
|||| `√[3+4]`
|||| `<'sqrt> 3+4`
|||| `<'sqrt>[3+4]`
| $\lVert a \rVert$ | `\lVert a \rVert` | `norm(a)` | `<'norm>a`
|||| `` `[\|\|` a `\|\|]` ``
| $\mathrm{abc}$ | `\mathrm{abc}` | `"abc"` | `<"abc" rm>`
|||| `"abc"`
|||| `<r#"abc">`
|||| `<r##"abc"## rm>`
| $\mathbf{ab\\\#"c}$ | ``\mathbf{ab#"c}`` || `<r##"ab"#c"## bf>`

## Lexer

1. NFD normalization
2. remove leading and trailing spaces
3. tokenize
4. insert virtual cat⁰ between connected symbols with no spaces
5. transform unicode_sub and unicode_sup to ASCII

## Grammer

```ebnf
mathⁱ = fracⁱ, (catⁱ, fracⁱ)*;
fracⁱ = rootⁱ, ['/'ⁱ, rootⁱ];
rootⁱ = interⁱ, ['_/'ⁱ, interⁱ];
interⁱ = simpⁱ, [overⁱ simpⁱ], [underⁱ simpⁱ], [supⁱ simpⁱ], [subⁱ simpⁱ];
simpⁱ = symbol | open, mathᵒᵒ, close | opⁱ, simpⁱ⁻¹ | mathⁱ⁻¹;
```
