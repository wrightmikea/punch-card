// Main App Component

use punch_card_core::ibm1130;
use punch_card_core::punch_card::{CardType, PunchCard as CorePunchCard};
use yew::prelude::*;

use super::{PunchCard, Tab, TabPanel, Tabs, TextInput};

#[function_component(App)]
pub fn app() -> Html {
    let text_value = use_state(String::new);
    let card = use_state(|| CorePunchCard::new(CardType::Text));
    let active_tab = use_state(|| "manual".to_string());

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

    let on_load_source_example = {
        let text_value = text_value.clone();
        Callback::from(move |_| {
            text_value.set("START DC   0             IBM 1130 EXAMPLE".to_string());
        })
    };

    let on_load_object_example = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |_| {
            text_value.set(String::new());
            let object_card = ibm1130::generate_example_object();
            card.set(object_card);
        })
    };

    let on_clear = {
        let text_value = text_value.clone();
        Callback::from(move |_| {
            text_value.set(String::new());
        })
    };

    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab_id: String| {
            active_tab.set(tab_id);
        })
    };

    let current_column = if text_value.len() < 80 {
        Some(text_value.len())
    } else {
        None
    };

    let tabs = vec![
        Tab {
            id: "manual".to_string(),
            label: "Manual Input".to_string(),
        },
        Tab {
            id: "examples".to_string(),
            label: "Examples".to_string(),
        },
        Tab {
            id: "load".to_string(),
            label: "Load".to_string(),
        },
        Tab {
            id: "about".to_string(),
            label: "About".to_string(),
        },
    ];

    html! {
        <div class="app">
            <header>
                <h1>{ "IBM 1130 Punch Card Simulator" }</h1>
            </header>
            <main>
                // Punch Card Display (First - most prominent)
                <div class="card-display">
                    <div class="card-info">
                        <span>{ format!("Column: {} / 80", text_value.len()) }</span>
                        <span>{ format!("Type: {}", if card.card_type() == CardType::Text { "Text" } else { "Binary" }) }</span>
                        <span>{ format!("Punched: {}", card.punched_count()) }</span>
                    </div>
                    <PunchCard
                        card={(*card).clone()}
                        current_column={current_column}
                    />
                </div>

                // Tabbed Interface
                <div class="control-panel">
                    <Tabs tabs={tabs} active_tab={(*active_tab).clone()} on_change={on_tab_change}>
                        // Tab A: Manual Input
                        <TabPanel id="manual" active_tab={(*active_tab).clone()}>
                            <h2>{ "Manual Input" }</h2>
                            <TextInput
                                value={(*text_value).clone()}
                                on_change={on_text_change}
                                max_length={80}
                            />
                            <div style="margin-top: 15px;">
                                <button onclick={on_clear}>{ "Clear Card" }</button>
                            </div>
                        </TabPanel>

                        // Tab B: Examples
                        <TabPanel id="examples" active_tab={(*active_tab).clone()}>
                            <h2>{ "Example Cards" }</h2>
                            <p>{ "Load example IBM 1130 punch cards:" }</p>
                            <div class="example-buttons">
                                <button onclick={on_load_source_example}>
                                    { "Assembler Source Card" }
                                </button>
                                <button onclick={on_load_object_example}>
                                    { "Object Deck Card (Binary)" }
                                </button>
                            </div>
                            <div style="margin-top: 20px;">
                                <h3>{ "About Examples" }</h3>
                                <p><strong>{ "Assembler Source:" }</strong>{ " IBM 1130 assembler instruction with label, opcode, and operands" }</p>
                                <p><strong>{ "Object Deck:" }</strong>{ " Binary compiled code with authentic 4:3 punch pattern" }</p>
                            </div>
                        </TabPanel>

                        // Tab C: Load
                        <TabPanel id="load" active_tab={(*active_tab).clone()}>
                            <h2>{ "Load from File" }</h2>
                            <p>{ "Upload an 80-byte binary file to load as a punch card:" }</p>
                            <div class="file-upload-container">
                                <label for="file-input">{ "Choose file (80 bytes):" }</label>
                                <input
                                    id="file-input"
                                    type="file"
                                    accept=".bin,.dat,.card"
                                    disabled=true
                                />
                                <p style="margin-top: 10px; color: #666; font-style: italic;">
                                    { "File upload functionality coming soon" }
                                </p>
                            </div>
                        </TabPanel>

                        // Tab D: About
                        <TabPanel id="about" active_tab={(*active_tab).clone()}>
                            <h2>{ "About This Simulator" }</h2>
                            <p>
                                { "This IBM 1130 Punch Card Simulator recreates the authentic experience of punching cards " }
                                { "using Hollerith encoding from the IBM 029 keypunch era." }
                            </p>
                            <h3>{ "Features" }</h3>
                            <ul>
                                <li>{ "Authentic Hollerith encoding (IBM 029 character set)" }</li>
                                <li>{ "80 columns × 12 rows per card" }</li>
                                <li>{ "Character printing at top (keypunch feature)" }</li>
                                <li>{ "Column highlighting for current position" }</li>
                                <li>{ "IBM 1130 assembler and object deck formats" }</li>
                            </ul>
                            <h3>{ "Technology" }</h3>
                            <ul>
                                <li>{ "Rust 2024 Edition with Yew framework" }</li>
                                <li>{ "WebAssembly (WASM) for performance" }</li>
                                <li>{ "SVG graphics for crisp rendering" }</li>
                                <li>{ "43 unit tests with 100% pass rate" }</li>
                            </ul>
                            <h3>{ "Source Code" }</h3>
                            <p>
                                <a href="https://github.com/wrightmikea/punch-card" target="_blank" rel="noopener noreferrer">
                                    { "View on GitHub" }
                                </a>
                                { " - MIT License" }
                            </p>
                            <p>
                                { "Built for educational purposes to preserve computing history." }
                            </p>
                        </TabPanel>
                    </Tabs>
                </div>
            </main>
        </div>
    }
}
