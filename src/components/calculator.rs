use yew::prelude::*;
use crate::components::CalculatorInput;
use web_sys::MouseEvent;

#[function_component(Calculator)]
pub fn calculator() -> Html {
    let input_value = use_state(String::new);
    let result = use_state(String::new);

    let on_input = {
        let input_value = input_value.clone();
        let result = result.clone();
        Callback::from(move |value: String| {
            input_value.set(value.clone());
            result.set(format!("LaTeX: {}", value));
        })
    };

    let on_calculate = {
        let input_value = input_value.clone();
        Callback::from(move |_: MouseEvent| {
            log::info!("Calculating expression: {}", *input_value);
        })
    };

    let on_calculate_key = {
        let input_value = input_value.clone();
        Callback::from(move |_| {
            log::info!("Calculating expression (via Enter): {}", *input_value);
        })
    };

    html! {
        <div class="min-h-screen flex items-center p-4">
            <div class="w-full max-w-3xl mx-auto bg-dark-surface p-8 rounded-lg shadow-lg">
                <h1 class="text-2xl text-dark-text text-center mb-8">
                    {"RustyMath Calculator"}
                </h1>
                
                <div class="flex gap-4 mb-4">
                    <CalculatorInput 
                        value={(*input_value).clone()} 
                        on_input={on_input}
                        on_calculate={on_calculate_key}
                    />
                    <button 
                        onclick={on_calculate}
                        class="px-6 bg-dark-accent text-dark-text rounded-lg text-xl"
                    >
                        {"="}
                    </button>
                </div>
                
                <div class="bg-dark-bg p-4 rounded-lg text-dark-text font-mono">
                    {(*result).clone()}
                </div>
            </div>
        </div>
    }
} 