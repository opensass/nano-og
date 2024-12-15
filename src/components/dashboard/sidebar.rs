use crate::components::common::logo::Logo;
use crate::router::Route;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Tab {
    OGs,
    Chat,
    CreateOG,
    ViewOG,
    EditProfile,
}

#[component]
pub fn Sidebar(active_tab: Signal<Tab>, navigate: bool) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let navigator = use_navigator();

    let tab_style = |tab: Tab| -> String {
        if active_tab() == tab {
            format!(
                "w-full p-2 flex items-center space-x-2 rounded bg-blue-500 text-white {}",
                if dark_mode { "dark:bg-blue-600" } else { "" }
            )
        } else {
            format!(
                "w-full p-2 flex items-center space-x-2 rounded hover:bg-gray-100 {}",
                if dark_mode {
                    "dark:hover:bg-gray-700 text-gray-400"
                } else {
                    "text-gray-600"
                }
            )
        }
    };

    rsx! {
        div { class: format!("fixed bottom-0 w-full md:static md:w-64 p-4 space-y-4 md:min-h-screen flex md:flex-col items-center md:items-start {}",
                              if dark_mode { "bg-gray-900" } else { "bg-gray-200" }),
            Link {
                to: Route::Dashboard {},
                class: "hidden md:inline",
                Logo {}
            }

            div { class: tab_style(Tab::OGs),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::OGs);
                },
                i {
                    width: 100,
                    height: 100,
                    class: "fa-solid fa-folder-open",
                },
                span { class: "hidden md:inline", "OGs" }
            }

            div { class: tab_style(Tab::Chat),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::Chat);
                },
                i {
                    width: 100,
                    height: 100,
                    class: "fa-solid fa-message",
                },
                span { class: "hidden md:inline", "Chat" }
            }

            div { class: tab_style(Tab::CreateOG),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::CreateOG);
                },
                i {
                    width: 100,
                    height: 100,
                    class: "fa-solid fa-file-lines",
                },
                span { class: "hidden md:inline", "Generate" }
            }
            div { class: tab_style(Tab::ViewOG),
                onclick: move |_| active_tab.set(Tab::ViewOG),
                i {
                    width: 100,
                    height: 100,
                    class: "fa-solid fa-address-book",
                },
                span { class: "hidden md:inline", "View OG" }
            }
            div { class: tab_style(Tab::EditProfile),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::EditProfile);
                },
                i {
                    width: 100,
                    height: 100,
                    class: "fa-regular fa-pen-to-square",
                },
                span { class: "hidden md:inline", "Profile" }
            }
        }
    }
}
