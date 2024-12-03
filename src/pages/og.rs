#![allow(unused)]

use crate::components::dashboard::chat::ChatPanelPage;
use crate::components::dashboard::navbar::Navbar;
use crate::components::dashboard::ogs::create::CreateOGPanel;
use crate::components::dashboard::ogs::list::OGsPanel;
use crate::components::dashboard::ogs::read::ViewOGPanel;
use crate::components::dashboard::profile::EditProfilePanel;
use crate::components::dashboard::sidebar::Sidebar;
use crate::components::dashboard::sidebar::Tab;
use crate::server::auth::controller::about_me;
use crate::theme::Theme;
use crate::theme::THEME;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn ViewOG(id: String) -> Element {
    let active_tab = use_signal(|| Tab::ViewOG);
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut current_tab = rsx! { OGsPanel { user_token } };
    if id.is_empty() {
        current_tab = match active_tab() {
            Tab::OGs => rsx! { OGsPanel { user_token } },
            Tab::CreateOG => rsx! { CreateOGPanel { user_token } },
            Tab::ViewOG => rsx! { ViewOGPanel { og_id: id, user_token } },
            Tab::EditProfile => rsx! { EditProfilePanel {} },
            Tab::Chat => rsx! { ChatPanelPage { user_token, og_id: id} },
        };
    } else {
        current_tab = rsx! { ViewOGPanel { og_id: id, user_token } };
    }

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    rsx! {
        div { class: format!("min-h-screen flex {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
            Sidebar { navigate: true, active_tab: active_tab.clone() }

            div { class: "flex-1 p-4 md:p-8",
                Navbar { dark_mode }

                div { class: format!("p-4 shadow rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                    {current_tab}
                }
            }
        }
    }
}

#[component]
pub fn EditOG(id: String) -> Element {
    let active_tab = use_signal(|| Tab::ViewOG);
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut current_tab = rsx! { OGsPanel { user_token } };
    if id.is_empty() {
        current_tab = match active_tab() {
            Tab::OGs => rsx! { OGsPanel { user_token } },
            Tab::ViewOG => rsx! { ViewOGPanel { og_id: id , user_token } },
            Tab::EditProfile => rsx! { EditProfilePanel {} },
            Tab::Chat => rsx! { ChatPanelPage { user_token, og_id: id} },
            Tab::CreateOG => todo!(),
        };
    }

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    rsx! {
        div { class: format!("min-h-screen flex {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
            Sidebar { navigate: true, active_tab: active_tab.clone() }

            div { class: "flex-1 p-4 md:p-8",
                Navbar { dark_mode }

                div { class: format!("p-4 shadow rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                    {current_tab}
                }
            }
        }
    }
}
