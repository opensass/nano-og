use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::fields::number::NumberField;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::og::controller::{generate_detail_content, generate_og_outline};
use crate::server::og::request::{GenerateDetailContentRequest, GenerateOGRequest};
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::{Duration, Utc};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;

const CACHE_KEY: &str = "cached_og_data";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub image_url: String,
    pub author: String,
    pub locale: String,
    pub twitter_card: String,
    pub twitter_site: String,
}

#[derive(Deserialize, Serialize)]
struct CachedOGsData {
    data: Vec<Metadata>,
    timestamp: i64,
}

#[component]
pub fn CreateOGPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;

    let title = use_signal(|| "Default Title".to_string());
    let description = use_signal(|| "Default description for the OG metadata.".to_string());
    let site_name = use_signal(|| "Default Site Name".to_string());
    let image_url = use_signal(|| "https://via.placeholder.com/300".to_string());
    let author = use_signal(|| "Default Author".to_string());
    let locale = use_signal(|| "en_US".to_string());
    let twitter_card = use_signal(|| "summary_large_image".to_string());
    let twitter_site = use_signal(|| "@opensassorg".to_string());

    let mut title_valid = use_signal(|| true);
    let mut description_valid = use_signal(|| true);
    let mut site_name_valid = use_signal(|| true);
    let mut image_url_valid = use_signal(|| true);
    let mut author_valid = use_signal(|| true);
    let mut locale_valid = use_signal(|| true);
    let mut twitter_card_valid = use_signal(|| true);
    let mut twitter_site_valid = use_signal(|| true);

    let mut loading = use_signal(|| false);
    let mut generated_metadata = use_signal(|| None::<Metadata>);
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let validate_field = |value: &str| !value.trim().is_empty();

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        loading.set(true);

        title_valid.set(validate_field(&title()));
        description_valid.set(validate_field(&description()));
        site_name_valid.set(validate_field(&site_name()));
        image_url_valid.set(validate_field(&image_url()));
        author_valid.set(validate_field(&author()));
        locale_valid.set(validate_field(&locale()));
        twitter_card_valid.set(validate_field(&twitter_card()));
        twitter_site_valid.set(validate_field(&twitter_site()));

        if !title_valid()
            || !description_valid()
            || !site_name_valid()
            || !image_url_valid()
            || !author_valid()
            || !locale_valid()
            || !twitter_card_valid()
            || !twitter_site_valid()
        {
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Please ensure all fields are valid.".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            loading.set(false);
            return;
        }

        let request = GenerateOGRequest {
            title: title(),
            outline: description(),
            token: user_token(),
            subtitle: site_name(),
            model: "claude-3".to_string(),
            subtopics: 30,
            details: 5,
            language: locale(),
            max_length: 10,
        };

        spawn(async move {
            match generate_og_outline(request).await {
                Ok(response) => {
                    let new_metadata = Metadata {
                        title: title(),
                        description: description(),
                        site_name: site_name(),
                        image_url: image_url(),
                        author: author(),
                        locale: locale(),
                        twitter_card: twitter_card(),
                        twitter_site: twitter_site(),
                    };

                    generated_metadata.set(Some(new_metadata.clone()));

                    let mut cached_data =
                        LocalStorage::get::<CachedOGsData>(CACHE_KEY).unwrap_or(CachedOGsData {
                            data: Vec::new(),
                            timestamp: Utc::now().timestamp(),
                        });

                    cached_data.data.push(new_metadata);
                    let _ = LocalStorage::set(CACHE_KEY, &cached_data);

                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Info".into(),
                                "OG metadata generated successfully!".into(),
                                ToastType::Success,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                }
                Err(err) => {
                    let error_message = err
                        .to_string()
                        .split("error running server function:")
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .to_string();

                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Error".into(),
                                error_message,
                                ToastType::Error,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                }
            }
            loading.set(false);
        });
    };

    let copy_to_clipboard = move |_| {
        if let Some(window) = web_sys::window() {
            spawn(async move {
                let tags = generate_meta_tags(generated_metadata().unwrap_or_default());
                let promise = window.navigator().clipboard().write_text(&tags);
                let _ = JsFuture::from(promise).await;
            });
        }
    };

    rsx! {
        div {
            class: format!("flex flex-col lg:flex-row p-6 space-y-6 {}",
                if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }
            ),

            div {
                class: "flex-1 p-6 bg-gray-800 text-white rounded-lg space-y-6",
                h2 { class: "text-3xl font-semibold", "Create OG Metadata" },
                form {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                    onsubmit: handle_submit,

                    InputField { label: "Title", value: title, is_valid: title_valid, validate: validate_field, required: true },
                    InputField { label: "Description", value: description, is_valid: description_valid, validate: validate_field, required: true },
                    InputField { label: "Site Name", value: site_name, is_valid: site_name_valid, validate: validate_field, required: false },
                    InputField { label: "Image URL", value: image_url, is_valid: image_url_valid, validate: validate_field, required: false },
                    InputField { label: "Author", value: author, is_valid: author_valid, validate: validate_field, required: false },
                    InputField { label: "Locale", value: locale, is_valid: locale_valid, validate: validate_field, required: false },
                    InputField { label: "Twitter Card Type", value: twitter_card, is_valid: twitter_card_valid, validate: validate_field, required: false },
                    InputField { label: "Twitter Site", value: twitter_site, is_valid: twitter_site_valid, validate: validate_field, required: false },

                    div {
                        class: "col-span-2 flex justify-center",
                        button {
                            class: format!(
                                "flex items-center justify-center px-4 py-2 rounded bg-blue-500 text-white {}",
                                if loading() { "opacity-50 cursor-not-allowed" } else { "" }
                            ),
                            r#type: "submit",
                            disabled: loading(),
                            if loading() {
                                Spinner {
                                    aria_label: "Loading spinner".to_string(),
                                    size: SpinnerSize::Md,
                                    dark_mode,
                                }
                                span { "Generating..." }
                            } else {
                                span { "Generate" }
                            }
                        }
                    }
                }
            },

            div {
                class: "flex-1 space-y-6 p-6 bg-gray-700 rounded-lg h-full",
                h3 { class: "text-xl font-semibold text-white", "Preview" },
                div {
                    class: "w-full h-64 rounded-lg overflow-hidden relative",
                    div {
                        class: "relative bg-gradient-to-r from-purple-300 to-pink-300 p-6 rounded-lg shadow-md text-gray-900 w-full h-full",
                        div {
                            class: "absolute top-4 left-4 text-xl font-bold",
                            "{title()}"
                        },
                        div {
                            class: "absolute top-12 left-4 text-md font-medium",
                            "{description()}"
                        },
                        div {
                            class: "absolute bottom-4 left-4 text-sm italic",
                            "Author: {author()} | Site: {site_name()}"
                        },
                        img {
                            class: "absolute top-0 right-0 w-24 h-24 rounded-full shadow-lg",
                            src: "{image_url()}",
                            alt: "Preview Thumbnail"
                        }
                    }
                }
                div {
                    class: "p-4 bg-gray-800 rounded-md text-sm text-white",
                    h4 { class: "text-lg font-semibold mb-2", "Generated Meta Tags" },
                    pre {
                        class: "bg-gray-900 p-4 rounded overflow-x-auto",
                        "{generate_meta_tags(generated_metadata().unwrap_or_default())}"
                    },
                    button {
                        class: "mt-4 px-4 py-2 bg-indigo-500 text-white rounded-md",
                        onclick: copy_to_clipboard,
                        "Copy to Clipboard"
                    }
                }
            }
        }
    }
}

fn generate_meta_tags(metadata: Metadata) -> String {
    format!(
        "<title>{}</title>
<meta name=\"description\" content=\"{}\" />
<meta property=\"og:title\" content=\"{}\" />
<meta property=\"og:description\" content=\"{}\" />
<meta property=\"og:site_name\" content=\"{}\" />
<meta property=\"og:image\" content=\"{}\" />
<meta property=\"og:author\" content=\"{}\" />
<meta property=\"og:locale\" content=\"{}\" />
<meta name=\"twitter:card\" content=\"{}\" />
<meta name=\"twitter:site\" content=\"{}\" />",
        metadata.title,
        metadata.description,
        metadata.title,
        metadata.description,
        metadata.site_name,
        metadata.image_url,
        metadata.author,
        metadata.locale,
        metadata.twitter_card,
        metadata.twitter_site,
    )
}
