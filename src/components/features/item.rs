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
                "flex flex-col items-center p-6 rounded-lg border transition-transform duration-300 border-gray-300 {} {}",
                if dark_mode == Theme::Dark { "bg-gray-800 text-gray-200 hover:bg-gray-700 hover:shadow-md" }
                else { "bg-white text-gray-900 hover:bg-gray-100 hover:shadow-lg" },
                "transform hover:-translate-y-1"
            ),
            div {
                class: "w-16 h-16 mb-4 transform transition-transform duration-300 hover:scale-110",
                {props.icon}
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
                    "text-sm text-center {}",
                    if dark_mode == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                ),
                "{props.description}"
            }
        }
    }
}
