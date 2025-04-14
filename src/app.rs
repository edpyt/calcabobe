use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! { <Calculator /> }
}

enum CalculatorOps {
    Plus,
    Minus,
    Mul,
    Div,
}

impl CalculatorOps {
    pub fn into_iter() -> core::array::IntoIter<CalculatorOps, 4> {
        [
            CalculatorOps::Plus,
            CalculatorOps::Minus,
            CalculatorOps::Mul,
            CalculatorOps::Div
        ].into_iter()
    }

    fn as_str(&self) -> &'static str {
        match self {
            CalculatorOps::Plus => "+",
            CalculatorOps::Minus => "-",
            CalculatorOps::Mul => "*",
            CalculatorOps::Div => "/",
        }
    }
}


#[component]
pub fn Calculator() -> impl IntoView {
    let (calculated_value,set_calculated_value) = signal(0);

    let mut current_op = CalculatorOps::Plus;


    let numbers = 1..=9;

    view! {
        <div class="h-screen grid gap-2 content-center p-12">
            <div>
                <input
                    type="text"
                    class="input input-info w-full"
                    prop:value=calculated_value
                    disabled
                />
            </div>
            <div class="grid grid-cols-4 gap-2">
                <div class="col-span-3 grid grid-cols-3 content-center gap-4">
                    {numbers
                        .into_iter()
                        .map(|n| view! { <button class="btn btn-soft">{n}</button> })
                        .collect::<Vec<_>>()}

                </div>
                <div class="grid gap-2">
                    {CalculatorOps::into_iter()
                        .map(|op| view! { <button class="btn btn-soft">{op.as_str()}</button> })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            <button class="btn btn-soft btn-success">=</button>
        </div>
    }
}
