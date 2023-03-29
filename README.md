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
| $a$ | `a` | `a` | `a`, `:a`
| $\alpha$ | `\alpha` | `alpha` | `α`, `:alpha`
| $\sqrt{2}$ | `\sqrt{2}` | `sqrt 2`, `sqrt[2]` | `√2`, `:sqrt 2`, `:sqrt[2]`
| $\mathrm{abc}$ | `\mathrm{abc}` | `"abc"` | `"abc"`
| $\hat a$ | `\hat a` | `hat a` | `â`, `:hat a`, `$\hat a$`
| $\$$ | `\$` | `$` | `$`, `$$ \$ $$`
| $\text{a"b"c\#"}$ | `\text{a"b"c"}` || `##"a"b"c#""##`
| $\mathbf{abc}$ | `\mathbf{abc}` | `bb"abc"` | `"abc"bb`, `#"abc"#bb`
| $(a)$ | `(a)` | `(a)`

## note

- `a+b␣/c`は`[a+b]/c`だが`a␣+␣b␣␣/ c`は`a+[b/c]`

- `()`と`{}`はデフォルトで表示、`[]`は非表示にする

- `␣`をマイナスのインデントとみなす
