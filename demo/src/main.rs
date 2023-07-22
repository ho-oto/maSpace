use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use web_sys::HtmlTextAreaElement;

use maspace::maspace_to_tex;

#[wasm_bindgen(inline_js = r#"
import "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg-full.js"
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
                <textarea
                    style="width: 100%; box-sizing: border-box;"
                    aria-label="Input"
                    value={value.to_string()}
                    oninput={on_input}/>
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
                    match (&tex_code.data, &tex_code.error) {
                        (_, Some(error)) => {
                            html!{
                                <div style="color: red; white-space: pre-line;">
                                    {"error msg: "}{(*error).clone()}
                                </div>
                            }
                        },
                        (Some((tex, html)), None) => {
                            html!{
                                <>
                                    <div>{"generated LaTeX code: "}{(*tex).clone()}</div>
                                    {(*html).clone()}
                                </>
                            }
                        },
                        _ => html!{}
                    }
                }
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
