use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::server::og::controller::get_details_for_og;
use crate::server::og::model::Detail;
use crate::server::og::request::GetDetailContentRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedDetailData {
    pub og_id: String,
    pub data: Vec<Detail>,
    pub timestamp: i64,
}

pub const CHAPTERS_CACHE_KEY: &str = "details_cache";
pub const CHAPTERS_CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn ReadOGPanel(og_id: String) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut selected_detail = use_signal(|| None::<Detail>);
    let mut details = use_signal(Vec::<Detail>::new);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        let og_id_cloned = og_id.clone();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) = LocalStorage::get::<CachedDetailData>(CHAPTERS_CACHE_KEY) {
                if cached_data.og_id == og_id_cloned
                    && now - cached_data.timestamp < CHAPTERS_CACHE_TIMEOUT
                {
                    loading.set(false);
                    details.set(cached_data.data.clone());
                    if let Some(first_detail) = cached_data.data.first() {
                        selected_detail.set(Some(first_detail.clone()));
                    }
                    return;
                }
            }

            if let Ok(response) = get_details_for_og(GetDetailContentRequest {
                og_id: og_id_cloned.clone(),
                html: og_id_cloned.clone(),
            })
            .await
            {
                loading.set(false);
                details.set(response.data.clone());

                let cached_data = CachedDetailData {
                    og_id: og_id_cloned.clone(),
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CHAPTERS_CACHE_KEY, &cached_data);

                if let Some(first_detail) = response.data.first() {
                    selected_detail.set(Some(first_detail.clone()));
                }
            } else {
                loading.set(true);
            }
        });
    });

    let mut handle_detail_click = {
        let mut selected_detail = selected_detail.clone();
        move |detail: Detail| {
            selected_detail.set(Some(detail));
        }
    };

    rsx! {
        div {
            class: format!("flex h-full {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            div {
                class: "md:w-1/3 lg:w-1/4 sm:w-1/6 p-4 border-r border-blue-300",
                ul {
                    class: "space-y-4",
                    for (index, detail) in details().into_iter().enumerate() {
                        li {
                            class: format!("flex items-center p-3 rounded-lg cursor-pointer {}",
                                if detail.id == selected_detail().unwrap().id {
                                    "bg-gray-500 text-white font-semibold"
                                } else {
                                    "hover:bg-gray-200 dark:hover:bg-dark-800"
                                }),
                            onclick: move |_| handle_detail_click(detail.clone()),
                            div {
                                class: "w-8 h-8 flex items-center justify-center rounded-full border-2 border-blue-500 mr-4",
                                "{index + 1}"
                            },

                            div {
                                class: "flex-1 hidden sm:block",
                                h4 { class: "text-lg", "{detail.title}" }
                                p { class: "text-sm text-blue-500", "{detail.estimated_duration} minutes" }
                            }
                        }
                    }
                }
            }

            div {
                class: "flex-1 p-6 overflow-y-auto",
                if let Some(detail) = selected_detail() {
                    h2 { class: "text-2xl font-bold mb-4", "{detail.title}" }
                    p { class: "text-sm text-blue-500 mb-6", "{detail.estimated_duration} minutes" }
                    div {
                        class: "prose dark:prose-invert",
                        dangerous_inner_html: detail.html,
                    }
                } else {
                    p {
                        class: "flex items-center space-x-2 px-4 py-2 rounded",
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Loading og's details..." }
                        } else {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "No details found! Generating..." }
                        }
                    }
                }
            }
        }
    }
}
