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

type CalculatorNumber = i64;

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

    view! {
        <div class="h-screen grid gap-2 content-center p-12">
            <InputNumbers a b state />
            <div class="grid grid-cols-4 gap-2">
                <CalculatorButtons a set_a b set_b state />
                <CalculatorOperationsButtons set_current_op set_state />
            </div>
            <div class="grid grid-cols-2 gap-12">
                <CalculatorResultButtons set_a b set_b set_state current_op />
            </div>
        </div>
    }
}

#[component]
fn InputNumbers(
    a: ReadSignal<CalculatorNumber>,
    b: ReadSignal<CalculatorNumber>,
    state: ReadSignal<CalculatorState>,
) -> impl IntoView {
    view! {
        <div>
            <span>{a}</span>
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
    }
}

/// TODO: maybe use iterators?
/// ```
/// numbers.windows(3).map
/// ```
const CALCULATOR_NUMBERS: &[CalculatorNumber; 9] = &[
    7, 8, 9, //
    4, 5, 6, //
    1, 2, 3, //
];
#[component]
fn CalculatorButtons(
    a: ReadSignal<CalculatorNumber>,
    set_a: WriteSignal<CalculatorNumber>,
    b: ReadSignal<CalculatorNumber>,
    set_b: WriteSignal<CalculatorNumber>,
    state: ReadSignal<CalculatorState>,
) -> impl IntoView {
    view! {
        <div class="col-span-3 grid grid-cols-3 content-center gap-4">
            {CALCULATOR_NUMBERS
                .iter()
                .map(|n| {
                    view! {
                        <button
                            class="btn btn-soft"
                            on:click=move |_| {
                                let (numb, set_fn) = match state.get() {
                                    CalculatorState::FirstNumInput => (a.get(), set_a),
                                    CalculatorState::SecondNumInput => (b.get(), set_b),
                                };
                                let numb: CalculatorNumber = format!("{}{}", numb, *n)
                                    .parse()
                                    .expect("Can't concatenate numbers.");
                                set_fn.set(numb);
                            }
                        >
                            {*n}
                        </button>
                    }
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
fn CalculatorOperationsButtons(
    set_current_op: WriteSignal<CalculatorOps>,
    set_state: WriteSignal<CalculatorState>,
) -> impl IntoView {
    view! {
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
    }
}

#[component]
fn CalculatorResultButtons(
    set_a: WriteSignal<CalculatorNumber>,
    b: ReadSignal<CalculatorNumber>,
    set_b: WriteSignal<CalculatorNumber>,
    set_state: WriteSignal<CalculatorState>,
    current_op: ReadSignal<CalculatorOps>,
) -> impl IntoView {
    view! {
        <button
            class="btn btn-soft btn-success"
            on:click=move |_| {
                let b = b.get();
                let mut a = set_a.write();
                match current_op.get() {
                    CalculatorOps::Plus => *a += b,
                    CalculatorOps::Mul => *a *= b,
                    CalculatorOps::Minus => *a -= b,
                    CalculatorOps::Div => *a /= b,
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
    }
}

#[cfg(test)]
mod tests {
    use leptos::{mount::mount_to, prelude::*, view};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::Element;

    use super::{CalculatorButtons, CalculatorNumber, CalculatorState, InputNumbers};

    wasm_bindgen_test_configure!(run_in_browser);

    fn get_input_value_from_wrapper(wrapper: &Element) -> CalculatorNumber {
        wrapper
            .query_selector("div")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>()
            .query_selector("input")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value()
            .parse::<CalculatorNumber>()
            .unwrap()
    }

    #[wasm_bindgen_test]
    fn test_calculator_input_numbers() {
        let document = document();
        let (a, _) = signal(1);
        let (b, _) = signal(2);
        let (state, set_state) = signal(CalculatorState::FirstNumInput);

        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <InputNumbers a b state /> },
        );

        assert_eq!(
            get_input_value_from_wrapper(&test_wrapper),
            a.get_untracked()
        );

        set_state.set(CalculatorState::SecondNumInput);
        // FIXME: DRY
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <InputNumbers a b state /> },
        );

        assert_eq!(
            get_input_value_from_wrapper(&test_wrapper),
            b.get_untracked()
        );
    }

    #[wasm_bindgen_test]
    fn test_calculator_number_buttons() {
        let document = document();
        let (a, set_a) = signal(0);
        let (b, set_b) = signal(0);
        let (state, set_state) = signal(CalculatorState::FirstNumInput);
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <CalculatorButtons a set_a b set_b state /> },
        );
        let button_7 = test_wrapper // NOTE: 7 cause it first button in calculator numbers
            .query_selector("button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();

        button_7.click();

        assert_eq!(a.get_untracked(), 7);

        let button_8 = button_7
            .next_sibling()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();

        button_8.click();

        assert_eq!(a.get_untracked(), 78);

        set_state.set(CalculatorState::SecondNumInput);

        let button_7 = test_wrapper
            .query_selector("button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();

        button_7.click();

        assert_eq!(b.get_untracked(), 7);

        let buttons = test_wrapper.query_selector_all("button");
    }
}
