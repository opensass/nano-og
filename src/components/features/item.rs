use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    let dark_mode = *THEME.read();

    rsx! {
        div {
            class: format!(
                "flex flex-col items-center p-6 rounded-lg border transition-transform duration-300 shadow-lg {} {}",
                if dark_mode == Theme::Dark { "bg-gray-800 hover:bg-gray-700 border-gray-700" }
                else { "bg-white hover:bg-gray-100 border-gray-300" },
                "transform hover:-translate-y-2 hover:scale-105"
            ),
            div {
                class: "w-16 h-16 mb-4 flex items-center justify-center rounded-full bg-gradient-to-br from-gray-300 to-gray-400 p-4 shadow-lg transform transition-transform duration-300 hover:scale-110",
                {props.icon.clone()}
            }
            h3 {
                class: format!(
                    "text-lg font-semibold {}",
                    if dark_mode == Theme::Dark { "text-white" } else { "text-gray-800" }
                ),
                "{props.title}"
            }
            p {
                class: format!(
                    "text-sm text-center mt-2 {}",
                    if dark_mode == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                ),
                "{props.description}"
            }
        }
    }
}
