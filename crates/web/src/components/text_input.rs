// TextInput Component

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value: String,
    pub on_change: Callback<String>,
    pub max_length: usize,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let on_input = {
        let on_change = props.on_change.clone();
        let max_length = props.max_length;

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            // Limit to max_length and convert to uppercase
            let limited = value
                .chars()
                .take(max_length)
                .collect::<String>()
                .to_uppercase();

            on_change.emit(limited);
        })
    };

    html! {
        <div class="text-input-container">
            <label for="card-input">
                { "Enter text (max 80 characters):" }
            </label>
            <input
                id="card-input"
                type="text"
                value={props.value.clone()}
                oninput={on_input}
                maxlength={props.max_length.to_string()}
                placeholder="Type your text here..."
                autocomplete="off"
            />
            <div class="input-info">
                <span>{ format!("Characters: {} / {}", props.value.len(), props.max_length) }</span>
            </div>
        </div>
    }
}
