// Main App Component

use punch_card_core::ibm1130;
use punch_card_core::punch_card::{CardType, PunchCard as CorePunchCard};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use super::{PunchCard, Tab, TabPanel, Tabs, TextInput};

#[function_component(App)]
pub fn app() -> Html {
    let text_value = use_state(String::new);
    let card = use_state(|| CorePunchCard::new(CardType::Text));
    let active_tab = use_state(|| "manual".to_string());

    // Update card when text changes (only for Text cards, not Binary)
    {
        let text_value = text_value.clone();
        let card = card.clone();

        use_effect_with(text_value.clone(), move |text| {
            // Only update if current card is Text type (don't overwrite Binary cards)
            if card.card_type() == CardType::Text {
                let new_card = CorePunchCard::from_text(text);
                card.set(new_card);
            }
            || ()
        });
    }

    let on_text_change = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |new_text: String| {
            // When user types, ensure we're in text mode
            text_value.set(new_text.clone());
            // Force update to text card
            card.set(CorePunchCard::from_text(&new_text));
        })
    };

    let on_load_source_example = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |_| {
            // Load text example
            let example_text = "START DC   0             IBM 1130 EXAMPLE".to_string();
            text_value.set(example_text.clone());
            card.set(CorePunchCard::from_text(&example_text));
        })
    };

    let on_load_object_example = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |_| {
            // Load binary example - set card first, then clear text
            let object_card = ibm1130::generate_example_object();
            card.set(object_card);
            text_value.set(String::new());
        })
    };

    let on_clear = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |_| {
            // Clear both text_value and card state directly
            text_value.set(String::new());
            card.set(CorePunchCard::new(CardType::Text));
        })
    };

    let on_save = {
        let card = card.clone();
        Callback::from(move |_| {
            // Convert card to binary format (160 bytes, 2 per column, all 12 rows)
            let binary_data = card.to_binary();

            // Create a blob and download it
            if let Some(window) = web_sys::window()
                && let Some(document) = window.document()
            {
                // Create blob
                let array = js_sys::Uint8Array::from(&binary_data[..]);
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&array);

                if let Ok(blob) = web_sys::Blob::new_with_u8_array_sequence(&blob_parts)
                    && let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob)
                {
                    // Create download link
                    if let Ok(element) = document.create_element("a")
                        && let Ok(a) = element.dyn_into::<web_sys::HtmlAnchorElement>()
                    {
                        a.set_href(&url);
                        a.set_download("punchcard.bin");
                        a.click();
                        web_sys::Url::revoke_object_url(&url).ok();
                    }
                }
            }
        })
    };

    let on_file_change = {
        let text_value = text_value.clone();
        let card = card.clone();
        Callback::from(move |e: web_sys::Event| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = input
                && let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                let text_value = text_value.clone();
                let card = card.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let array_buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
                        .await
                        .ok();

                    if let Some(buffer) = array_buffer {
                        let array = js_sys::Uint8Array::new(&buffer);
                        let mut bytes = vec![0u8; array.length() as usize];
                        array.copy_to(&mut bytes);

                        if bytes.len() == 108 || bytes.len() == 80 {
                            // Load as binary format (108 bytes = IBM 1130 format, or 80 bytes = legacy)
                            // from_binary() handles both 108-byte and 80-byte formats
                            let new_card = CorePunchCard::from_binary(&bytes);
                            card.set(new_card);
                            text_value.set(String::new());
                        }
                    }
                });
            }
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
            label: "Save/Load".to_string(),
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
                            <TextInput
                                value={(*text_value).clone()}
                                on_change={on_text_change}
                                max_length={80}
                            />
                            <div style="margin-top: 15px;">
                                <button onclick={on_clear.clone()}>{ "Clear Card" }</button>
                            </div>
                        </TabPanel>

                        // Tab B: Examples
                        <TabPanel id="examples" active_tab={(*active_tab).clone()}>
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

                        // Tab C: Save/Load
                        <TabPanel id="load" active_tab={(*active_tab).clone()}>
                            <div style="display: flex; gap: 20px;">
                                // Save section (2/5 width = 40%)
                                <div style="flex: 0 0 40%; padding: 15px; border: 1px solid #ccc; border-radius: 5px; background: #f9f9f9;">
                                    <h3 style="margin-top: 0;">{ "Save Card" }</h3>
                                    <p style="font-size: 0.9em;">{ "Download the current punch card as a 108-byte binary file (IBM 1130 format: 72 columns × 12 rows, columns 73-80 not saved):" }</p>
                                    <button onclick={on_save}>{ "Download Card (.bin)" }</button>
                                </div>

                                // Load section (2/5 width = 40%)
                                <div style="flex: 0 0 40%; padding: 15px; border: 1px solid #ccc; border-radius: 5px; background: #f9f9f9;">
                                    <h3 style="margin-top: 0;">{ "Load Card" }</h3>
                                    <p style="font-size: 0.9em;">{ "Upload a binary file to load as a punch card (108 bytes IBM 1130 format, or legacy 80-byte format):" }</p>
                                    <div class="file-upload-container">
                                        <input
                                            type="file"
                                            accept=".bin,.dat,.card"
                                            onchange={on_file_change}
                                        />
                                    </div>
                                    <p style="margin-top: 10px; font-size: 0.85em; color: #666;">
                                        <strong>{ "Note:" }</strong>{ " Loaded binary cards will not display printed characters at the top of the card, only the punch hole patterns." }
                                    </p>
                                </div>

                                // Clear section (1/5 width = 20%)
                                <div style="flex: 0 0 20%; padding: 15px; border: 1px solid #ccc; border-radius: 5px; background: #f9f9f9;">
                                    <h3 style="margin-top: 0;">{ "Clear Card" }</h3>
                                    <p style="font-size: 0.9em;">{ "Reset the punch card to blank:" }</p>
                                    <button onclick={on_clear.clone()}>{ "Clear Card" }</button>
                                </div>
                            </div>
                        </TabPanel>

                        // Tab D: About
                        <TabPanel id="about" active_tab={(*active_tab).clone()}>
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
