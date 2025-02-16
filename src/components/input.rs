use wasm_bindgen::prelude::*;
use web_sys::{Element, CustomEvent, KeyboardEvent};
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    type MathfieldElement;

    #[wasm_bindgen(constructor)]
    fn new() -> MathfieldElement;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &MathfieldElement, val: &str);

    #[wasm_bindgen(method, getter)]
    fn value(this: &MathfieldElement) -> String;
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    pub on_input: Callback<String>,
    pub on_calculate: Callback<()>,
}

#[function_component(CalculatorInput)]
pub fn calculator_input(props: &Props) -> Html {
    let mathfield_ref = use_node_ref();
    let on_input = props.on_input.clone();
    let on_calculate = props.on_calculate.clone();

    // Handle Enter key
    let on_keydown = {
        let on_calculate = on_calculate.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                on_calculate.emit(());
            }
        })
    };

    use_effect_with(
        (mathfield_ref.clone(), props.value.clone()),
        move |(mathfield_ref, value)| {
            if let Some(element) = mathfield_ref.cast::<Element>() {
                let mathfield = MathfieldElement::new();
                mathfield.set_value(value);
                
                let on_input = on_input.clone();
                let callback = Closure::wrap(Box::new(move |e: CustomEvent| {
                    let target: MathfieldElement = e.target().unwrap().unchecked_into();
                    on_input.emit(target.value());
                }) as Box<dyn FnMut(CustomEvent)>);
                
                element.add_event_listener_with_callback(
                    "input",
                    callback.as_ref().unchecked_ref(),
                ).unwrap();
                
                callback.forget();
            }
            || ()
        },
    );

    html! {
        <math-field
            ref={mathfield_ref}
            class="flex-1 p-4 bg-dark-bg rounded-lg border border-dark-accent/20 
                   focus:ring-2 focus:ring-dark-accent text-dark-text"
            virtual-keyboard-mode="onfocus"
            smart-mode="on"
            smart-fence="on"
            virtual-keyboards="numeric functions"
            onkeydown={on_keydown}
        />
    }
} 