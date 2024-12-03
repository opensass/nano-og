use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::router::Route;
use crate::server::og::controller::get_ogs_for_user;
use crate::server::og::model::OG;
use crate::server::og::request::GetOGsForUserRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedOGsData {
    pub data: Vec<OG>,
    pub timestamp: i64,
}

pub const CACHE_KEY: &str = "ogs_cache";
pub const CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn OGsPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut ogs = use_signal(Vec::new);
    let mut displayed_ogs = use_signal(Vec::new);
    let mut loading = use_signal(|| true);
    let mut search_query = use_signal(String::new);

    let _ = use_resource(move || async move {
        let now = Utc::now().timestamp();

        if let Ok(cached_data) = LocalStorage::get::<CachedOGsData>(CACHE_KEY) {
            if now - cached_data.timestamp < CACHE_TIMEOUT {
                loading.set(false);
                ogs.set(cached_data.data.clone());
                displayed_ogs.set(cached_data.data);
                return;
            }
        }

        match get_ogs_for_user(GetOGsForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                let cached_data = CachedOGsData {
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CACHE_KEY, &cached_data);

                loading.set(false);
                ogs.set(response.data.clone());
                displayed_ogs.set(response.data);
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });

    let mut filter_ogs = move || {
        let query = search_query().to_lowercase();

        let filtered_ogs = ogs()
            .iter()
            .filter(|og| {
                let matches_query = if query.is_empty() {
                    true
                } else {
                    let title_matches = og.title.to_lowercase().contains(&query);
                    title_matches
                };

                matches_query
            })
            .cloned()
            .collect::<Vec<_>>();

        displayed_ogs.set(filtered_ogs);
    };

    rsx! {
        div {
            div {
                div {
                    class: "w-full md:w-1/3 pb-4 mb-4 md:mb-0 flex flex-col gap-8",

                    div {
                        h3 { class: "text-2xl font-bold mb-4", "Search" }
                        input {
                            class: format!(
                                "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                if dark_mode { "bg-gray-900" } else { "" },
                            ),
                            placeholder: "Search by title...",
                            value: "{search_query()}",
                            oninput: move |e| {
                                search_query.set(e.value());
                                filter_ogs();
                            },
                        }
                    }
                }
                h2 { class: "text-xl font-semibold mb-4", "All OGs" }
                if displayed_ogs().len() > 0 {
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                        for og in displayed_ogs() {
                            Link {
                                to: Route::ViewOG { id: og.id.to_string() },
                                class: format!(
                                    "p-4 shadow rounded-lg {}",
                                    if dark_mode { "bg-gray-700" } else { "bg-gray-100" }
                                ),
                                img {
                                    src: og.image_url,
                                    alt: "OG cover",
                                    class: "w-full h-48 object-cover rounded-md mb-4"
                                }
                                p {
                                    class: "text-sm text-gray-500 mb-2",
                                    "{og.created_at.format(\"%B %d, %Y\")}"
                                }
                                p {
                                    class: "mt-2 text-xl text-gray-100",
                                    "{og.title.chars().take(30).collect::<String>()}"
                                }
                            }
                        }
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
                            span { "Loading ogs..." }
                        } else {
                            span { "No ogs match your search filter." }
                        }
                    }
                }
            }
        }
    }
}
