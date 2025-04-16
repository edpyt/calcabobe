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

#[derive(Clone)]
enum CalculatorState {
    FirstNumInput,
    SecondNumInput,
}

impl CalculatorOps {
    pub fn into_iter() -> core::array::IntoIter<CalculatorOps, 4> {
        [
            CalculatorOps::Plus,
            CalculatorOps::Minus,
            CalculatorOps::Mul,
            CalculatorOps::Div,
        ]
        .into_iter()
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
    let (a, set_a) = signal(0);
    let (b, set_b) = signal(0);
    let (state, set_state) = signal(CalculatorState::FirstNumInput);
    let (current_op, set_current_op) = signal(CalculatorOps::Plus);

    let numbers = 1..=9;

    view! {
        <div class="h-screen grid gap-2 content-center p-12">
            <div>
                <input
                    type="text"
                    class="input input-info w-full"
                    disabled
                    prop:value=move || {
                        match state.get() {
                            CalculatorState::FirstNumInput => a,
                            CalculatorState::SecondNumInput => b,
                        }
                    }
                />
            </div>
            <div class="grid grid-cols-4 gap-2">
                <div class="col-span-3 grid grid-cols-3 content-center gap-4">
                    {numbers
                        .into_iter()
                        .rev()
                        .map(|n| {
                            view! {
                                <button
                                    class="btn btn-soft"
                                    on:click=move |_| {
                                        match state.get() {
                                            CalculatorState::FirstNumInput => *set_a.write() += n,
                                            CalculatorState::SecondNumInput => *set_b.write() += n,
                                        }
                                    }
                                >
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
                                        set_state.set(CalculatorState::SecondNumInput)
                                    }
                                >
                                    {op.as_str()}
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
            <div class="grid grid-cols-2 gap-12">
                <button
                    class="btn btn-soft btn-success"
                    on:click=move |_| {
                        let b = b.get();
                        match current_op.get() {
                            CalculatorOps::Plus => *set_a.write() += b,
                            CalculatorOps::Mul => *set_a.write() *= b,
                            CalculatorOps::Minus => *set_a.write() -= b,
                            CalculatorOps::Div => *set_a.write() /= b,
                        };
                        set_b.set(0);
                        set_state.set(CalculatorState::FirstNumInput);
                    }
                >
                    =
                </button>

                <button
                    class="btn btn-soft btn-error"
                    on:click=move |_| {
                        *set_a.write() = 0;
                        set_state.set(CalculatorState::FirstNumInput);
                    }
                >
                    AC
                </button>
            </div>
        </div>
    }
}
