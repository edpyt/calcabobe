use leptos::{leptos_dom::logging::console_log, prelude::*};
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

#[derive(Clone, Copy)]
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
    let (a,set_a) = signal(0);
    let (b,set_b) = signal(0);

    let (current_op,set_current_op) = signal(CalculatorOps::Plus);
    let numbers = 1..=9;

    view! {
        <div class="h-screen grid gap-2 content-center p-12">
            <span>{move || current_op.get().as_str()}</span>
            <div>
                <input type="text" class="input input-info w-full" prop:value=a disabled />
            </div>
            <div class="grid grid-cols-4 gap-2">
                <div class="col-span-3 grid grid-cols-3 content-center gap-4">
                    {numbers
                        .into_iter()
                        .rev()
                        .map(|n| {
                            view! {
                                <button class="btn btn-soft" on:click=move |_| console_log("123")>
                                    {n}
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()}

                </div>
                <div class="grid gap-2">
                    {CalculatorOps::into_iter()
                        .map(|op| {
                            view! {
                                <button
                                    class="btn btn-soft"
                                    on:click=move |_| {
                                        set_current_op.set(op);
                                        console_log(current_op.get().as_str());
                                    }
                                >
                                    {op.as_str()}
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            <button class="btn btn-soft btn-success" on:click=move |_| set_a.set(2)>
                =
            </button>
        </div>
    }
}
