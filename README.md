# maSpace

[demo](https://ho-oto.github.io/maSpace/)

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
mathⁱ = rootⁱ, [' 'ⁱ, rootⁱ];
rootⁱ = fracⁱ, ['_/'ⁱ, fracⁱ];
fracⁱ = stackⁱ, ['/'ⁱ, stackⁱ];
stackⁱ = interⁱ, ['^^'ⁱ, interⁱ], ['__'ⁱ, interⁱ];
interⁱ = simpⁱ, ['^'ⁱ simpⁱ], ['_'ⁱ simpⁱ];
simpⁱ = [opⁱ,] mathⁱ⁻¹;
simp⁰ = [op⁰,] (symbol | open, mathᵒᵒ, close);
```

### example

```plain
a␣_b_c␣␣^d␣^e+f␣_g␣␣/h
"a" Sub(1) "b" Sub(0) "c" Sup(2) "d" Sup(1) "e" Cat(0) "+" Cat(0) "f" Sub(1) "g" Frac(2) "h"
-----------------------------------frac2----------------------------------------         "h"
---simp2-----------------        --------------simp2----------------------------
---math1-----------------        --------------math1----------------------------
"a"        ---simp1------        "d"        --------simp1------------        "g"
           ---math0------                   --------math0------------
           "b"        "c"                   "e"        "+"        "f"

a␣_b_c^d␣␣^e+f␣_g/h
"a" Sub(1) "b" Sub(0) "c" Sup(0) "d" Sup(2) "e" Cat(0) "+" Cat(0) "f" Sub(1) "g" Frac(0) "h"
-------------simp2------------------        --------------------simp2-----------------------
-------------math1------------------        --------------------math1-----------------------
"a"        --------simp1------------        ----------simp1----------        ----simp1------
           --------math0------------        ----------math0----------        ----math0------
           "b"        "c"        "d"        "e"        "+"        "f"        "g"         "h"

a␣_b_c^d␣^[e+f␣_g/h]
"a" Sub(1) "b" Sub(0) "c" Sup(0) "d" Sup(1) Open(".") "e" Cat(0) "+" Cat(0) "f" Sub(1) "g" Frac(0) "h" Close(".")
"a"        ---------simp1-----------        ---------------------------------simp1-------------------------------
           "b"        "c"        "d"        ---------------------------------math0-------------------------------
                                            ---------------------------------math-1------------------------------
                                                      -----------------------math1--------------------
                                                      ----------simp1----------        ----simp1------
                                                      "e"        "+"        "f"        "g"         "h"
```
