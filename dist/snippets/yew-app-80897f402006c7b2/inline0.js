
import "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg-full.js"
export function tex2svg(tex_input) {
    return MathJax.tex2svgPromise(tex_input, {display: true}).then(function (node) {
        const adaptor = MathJax.startup.adaptor;
        return adaptor.outerHTML(node);
    });
}
