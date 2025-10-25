// Main App Component

use punch_card_core::punch_card::{CardType, PunchCard as CorePunchCard};
use yew::prelude::*;

use super::{PunchCard, TextInput};

#[function_component(App)]
pub fn app() -> Html {
    let text_value = use_state(String::new);
    let card = use_state(|| CorePunchCard::new(CardType::Text));

    // Update card when text changes
    {
        let text_value = text_value.clone();
        let card = card.clone();

        use_effect_with(text_value.clone(), move |text| {
            let new_card = CorePunchCard::from_text(text);
            card.set(new_card);
            || ()
        });
    }

    let on_text_change = {
        let text_value = text_value.clone();
        Callback::from(move |new_text: String| {
            text_value.set(new_text);
        })
    };

    let on_load_example = {
        let text_value = text_value.clone();
        Callback::from(move |_| {
            text_value.set("START DC   0             IBM 1130 EXAMPLE".to_string());
        })
    };

    let on_clear = {
        let text_value = text_value.clone();
        Callback::from(move |_| {
            text_value.set(String::new());
        })
    };

    let current_column = if text_value.len() < 80 {
        Some(text_value.len())
    } else {
        None
    };

    html! {
        <div class="app">
            <header>
                <h1>{ "IBM 1130 Punch Card Simulator" }</h1>
                <p>{ "Interactive punch card experience with Hollerith encoding" }</p>
            </header>
            <main>
                <div class="control-panel">
                    <h2>{ "Control Panel" }</h2>
                    <div class="example-buttons">
                        <button onclick={on_load_example}>{ "Load Example" }</button>
                        <button onclick={on_clear}>{ "Clear Card" }</button>
                    </div>
                </div>

                <div class="input-area">
                    <h2>{ "Input" }</h2>
                    <TextInput
                        value={(*text_value).clone()}
                        on_change={on_text_change}
                        max_length={80}
                    />
                </div>

                <div class="card-display">
                    <h2>{ "Punch Card" }</h2>
                    <div class="card-info">
                        <span>{ format!("Column: {} / 80", text_value.len()) }</span>
                        <span>{ format!("Type: Text") }</span>
                        <span>{ format!("Punched: {}", card.punched_count()) }</span>
                    </div>
                    <PunchCard
                        card={(*card).clone()}
                        current_column={current_column}
                    />
                </div>
            </main>
        </div>
    }
}
