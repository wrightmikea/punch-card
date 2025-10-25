// Tabs Component

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub tabs: Vec<Tab>,
    pub active_tab: String,
    pub on_change: Callback<String>,
    pub children: Children,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let on_tab_click = |tab_id: String| {
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            on_change.emit(tab_id.clone());
        })
    };

    html! {
        <div class="tabs-container">
            <div class="tabs">
                {
                    props.tabs.iter().map(|tab| {
                        let is_active = tab.id == props.active_tab;
                        let class = if is_active { "tab-button active" } else { "tab-button" };

                        html! {
                            <button
                                class={class}
                                onclick={on_tab_click(tab.id.clone())}
                            >
                                { &tab.label }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="tab-contents">
                { props.children.clone() }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabPanelProps {
    pub id: String,
    pub active_tab: String,
    pub children: Children,
}

#[function_component(TabPanel)]
pub fn tab_panel(props: &TabPanelProps) -> Html {
    let is_active = props.id == props.active_tab;
    let class = if is_active {
        "tab-content active"
    } else {
        "tab-content"
    };

    html! {
        <div class={class}>
            { props.children.clone() }
        </div>
    }
}
