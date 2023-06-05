use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

use web_sys::HtmlTextAreaElement;

use maspace::maspace_to_tex;

#[wasm_bindgen(inline_js = r#"
import "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg-full.js"
export function tex2svg(tex_input) {
    return MathJax.tex2svgPromise(tex_input, {display: true}).then(function (node) {
        const adaptor = MathJax.startup.adaptor;
        return adaptor.outerHTML(node);
    });
}
"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn tex2svg(input: &str) -> Result<JsValue, JsValue>;
}

async fn maspace2svg(tex: &str) -> Result<(String, Html), String> {
    let tex = maspace_to_tex(tex).map_err(|x| format!("maspace error: {:?}", x))?;
    let html = tex2svg(&tex)
        .await
        .map(|x| Html::from_html_unchecked(AttrValue::from(x.as_string().unwrap_or_default())))
        .map_err(|x| x.as_string().unwrap_or_default())?;
    Ok((tex, html))
}

#[function_component]
fn App() -> Html {
    let value = use_state(|| String::from(""));
    let tex_code = {
        let value = value.clone();
        use_async(async move { maspace2svg(&value).await })
    };

    let on_input = {
        let value = value.clone();
        let tex_code = tex_code.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            value.set(input.value());
            tex_code.run();
        })
    };

    html! {
        <main>
            <div>
                <textarea rows="10" cols="100" value={value.to_string()} oninput={on_input} />
            </div>
            <div>
                {
                    if tex_code.loading {
                        html! { "Now rendering..." }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div>
                {
                    if let Some((tex, html)) = &tex_code.data {
                        html!{
                            <>
                                <div>{"generated LaTeX code: "}{(*tex).clone()}</div>
                                {(*html).clone()}
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
            <div>
                {
                    if let Some(error) = &tex_code.error {
                        html! { error }
                    } else {
                        html! {}
                    }
                }
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
