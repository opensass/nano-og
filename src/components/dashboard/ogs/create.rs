use crate::components::dashboard::fields::input::InputField;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::og::controller::store_og;
use crate::server::og::request::StoreOGRequest;
use crate::theme::Theme;
use chrono::{Duration, Utc};
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use nano_ai::client::NanoAI;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;

const CACHE_KEY: &str = "cached_og_data";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub site_name: String,
    pub image_url: String,
    pub brand_url: String,
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
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;

    let mut title = use_signal(|| "Open SASS".to_string());
    let mut description =
        use_signal(|| "Your Gateway to Secure Open-Source Rusty SaaS Solutions.".to_string());
    let site_name = use_signal(|| "opensass.org".to_string());
    let image_url = use_signal(|| "https://opensass.org/logo.webp".to_string());
    let author = use_signal(|| "Mahmoud".to_string());
    let locale = use_signal(|| "en_US".to_string());
    let twitter_card = use_signal(|| "summary_large_image".to_string());
    let twitter_site = use_signal(|| "@opensassorg".to_string());

    let mut from_color = use_signal(|| String::from("purple-300"));
    let mut to_color = use_signal(|| String::from("pink-300"));
    let mut img_drag = use_signal(|| None);
    let mut img_position = use_signal(|| (0., 0.));
    let mut author_drag = use_signal(|| None);
    let mut author_position = use_signal(|| (0., 0.));
    let mut title_drag = use_signal(|| None);
    let mut title_position = use_signal(|| (0., 0.));
    let mut description_drag = use_signal(|| None);
    let mut description_position = use_signal(|| (0., 0.));

    let mut title_valid = use_signal(|| true);
    let mut description_valid = use_signal(|| true);
    let mut site_name_valid = use_signal(|| true);
    let mut image_url_valid = use_signal(|| true);
    let mut author_valid = use_signal(|| true);
    let mut locale_valid = use_signal(|| true);
    let mut twitter_card_valid = use_signal(|| true);
    let mut twitter_site_valid = use_signal(|| true);
    let mut from_color_valid = use_signal(|| true);
    let mut to_color_valid = use_signal(|| true);

    let mut loading = use_signal(|| false);
    let mut generated_metadata = use_signal(|| None::<Metadata>);
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let validate_field = |value: &str| !value.trim().is_empty();

    // 9000 IQ hack to send the html tags as an image to an axum endpoint
    fn save_preview_as_image() {
        client! {
            document::eval(r#"
                const element = document.getElementById('preview-section');
                if (element) {
                    html2canvas(element, {
                        letterRendering: 1,
                        logging: true,
                        allowTaint: true,
                        useCORS: true,
                        })
                    .then((canvas) => {
                        const base64Image = canvas.toDataURL('image/png');
                        const imageData = base64Image.replace(/^data:image\/png;base64,/, '');
                        const payload = `req[image_url]=${encodeURIComponent(imageData)}`;

                        fetch('/api/upload_og', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/x-www-form-urlencoded'
                            },
                            body: payload
                        })
                        .then(response => {
                            dioxus.send(response);
                        })
                        .then(data => {
                            console.log('Image uploaded successfully:', data);
                        })
                        .catch(error => {
                            console.error('Error uploading image:', error);
                        });
                    }).catch((error) => {
                        console.error('Error generating image:', error);
                    });
                } else {
                    console.error('Preview section not found');
                }
            "#);
        }
    }

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

        save_preview_as_image();

        let request = StoreOGRequest {
            token: user_token(),
            title: title(),
            description: description(),
            site_name: site_name(),
            image_url: image_url(),
            brand_url: image_url(),
            author: author(),
            locale: locale(),
            twitter_card: twitter_card(),
            twitter_site: twitter_site(),
        };
        spawn(async move {
            match store_og(request).await {
                Ok(response) => {
                    let new_metadata = Metadata {
                        title: title(),
                        description: description(),
                        site_name: site_name(),
                        image_url: response.data.image_url,
                        brand_url: image_url(),
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
    let handle_generate_title = move |_| {
        spawn(async move {
            let ai_generated_title = generate_ai_title().await.unwrap_or_else(|err| {
                toasts_manager.set(
                    toasts_manager()
                        .add_toast(
                            "Error".into(),
                            err.to_string(),
                            ToastType::Error,
                            Some(Duration::seconds(5)),
                        )
                        .clone(),
                );
                title()
            });

            title.set(ai_generated_title);
        });
    };

    let handle_generate_description = move |_| {
        spawn(async move {
            let ai_generated_description = generate_ai_description().await.unwrap_or_else(|err| {
                toasts_manager.set(
                    toasts_manager()
                        .add_toast(
                            "Error".into(),
                            err.to_string(),
                            ToastType::Error,
                            Some(Duration::seconds(5)),
                        )
                        .clone(),
                );
                description()
            });

            description.set(ai_generated_description);
        });
    };
    fn download_preview_as_image() {
        client! {
            document::eval(r#"
                const element = document.getElementById('preview-section');
                if (element) {
                    html2canvas(element, {
                        letterRendering: 1,
                        logging: true,
                        allowTaint: true,
                        useCORS: true,
                        }).then((canvas) => {
                            const link = document.createElement('a');
                            link.download = 'og-preview.png';
                            link.href = canvas.toDataURL('image/png');
                            link.click();
                        }
                    );
                }
            "#);
        }
    }

    rsx! {
        div {
            class: format!("flex flex-col p-6 space-y-6 {}",
                if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }
            ),
            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

                div {
                    class: format!(
                        "p-6 rounded-lg border {}",
                        if dark_mode { "bg-gray-800 border-gray-700" } else { "bg-white border-gray-300" }
                    ),
                    h2 { class: "text-2xl font-semibold mb-4", "Create OG Metadata" },
                    form {
                        class: "flex flex-col space-y-4 w-full",
                        onsubmit: handle_submit,
                        div {
                            class: "items-center gap-x-2 w-full",
                            label {
                                class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }),
                                "Title"
                            }
                            div {
                                class: "flex gap-x-2",
                                input {
                                    class: format!(
                                        "flex-grow mt-1 block w-full p-2 border rounded-md shadow-sm {} {}",
                                        if dark_mode { "bg-gray-900" } else { "" },
                                        if title_valid() { "border-gray-300" } else { "border-red-500"
                                    }),
                                    value: "{title}",
                                    oninput: move |e: Event<FormData>| {
                                        let input_value = e.value().clone();
                                        title.set(input_value.clone());
                                        title_valid.set(validate_field(&input_value));
                                    },
                                    required: true
                                }
                                button {
                                    class: "px-4 py-2 h-[2rem] rounded bg-gray-500 text-white hover:bg-gray-600 transition flex items-center justify-end",
                                    r#type: "button",
                                    title: "AI Suggestion",
                                    onclick: handle_generate_title,
                                    disabled: loading(),
                                    "💡"
                                }
                            }
                            if !title_valid() {
                                p { class: "text-red-500 text-sm mt-1", "Invalid input" }
                            }
                        },
                        div {
                            class: "items-center gap-x-2 w-full",
                            label {
                                class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }),
                                "Description"
                            }
                            div {
                                class: "flex gap-x-2",
                                input {
                                    class: format!(
                                        "flex-grow mt-1 block w-full p-2 border rounded-md shadow-sm {} {}",
                                        if dark_mode { "bg-gray-900" } else { "" },
                                        if description_valid() { "border-gray-300" } else { "border-red-500"
                                    }),
                                    value: "{description}",
                                    oninput: move |e: Event<FormData>| {
                                        let input_value = e.value().clone();
                                        description.set(input_value.clone());
                                        description_valid.set(validate_field(&input_value));
                                    },
                                    required: true
                                }
                                button {
                                    class: "px-4 py-2 h-[2rem] rounded bg-gray-500 text-white hover:bg-gray-600 transition flex items-center justify-end",
                                    r#type: "button",
                                    title: "AI Suggestion",
                                    onclick: handle_generate_description,
                                    disabled: loading(),
                                    "💡"
                                }
                            }
                            if !description_valid() {
                                p { class: "text-red-500 text-sm mt-1", "Invalid input" }
                            }
                        },
                        InputField { label: "Site Name", value: site_name, is_valid: site_name_valid, validate: validate_field, required: false },
                        InputField { label: "Brand Image", value: image_url, is_valid: image_url_valid, validate: validate_field, required: false },
                        InputField { label: "Author", value: author, is_valid: author_valid, validate: validate_field, required: false },
                        div {
                            class: "mb-4 w-full",
                            label {
                                class: format!("text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }),
                                "Background Color"
                            }
                            div {
                                class: format!(
                                    "border rounded flex flex-col md:flex-row gap-4 mt-1 w-full p-2 {}",
                                    if dark_mode { "bg-gray-900" } else { "" }),
                                InputField {
                                    label: "From Color",
                                    value: from_color,
                                    is_valid: from_color_valid,
                                    validate: validate_field,
                                    required: true
                                },
                                InputField {
                                    label: "To Color",
                                    value: to_color,
                                    is_valid: to_color_valid,
                                    validate: validate_field,
                                    required: true
                                }
                            },
                        },
                        InputField { label: "Locale", value: locale, is_valid: locale_valid, validate: validate_field, required: false },
                        InputField { label: "Twitter Card Type", value: twitter_card, is_valid: twitter_card_valid, validate: validate_field, required: false },
                        InputField { label: "Twitter Site", value: twitter_site, is_valid: twitter_site_valid, validate: validate_field, required: false },

                        div {
                            class: "col-span-2 flex",
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
                    class: format!(
                        "p-6 rounded-lg border flex flex-col justify-between {}",
                        if dark_mode { "bg-gray-800 border-gray-700" } else { "bg-white border-gray-300" }
                    ),
                    h3 { class: "text-2xl font-semibold mb-4", "Preview" },

                    div {
                        id: "preview-section",
                        class: format!(
                            "relative bg-gradient-to-r from-{} to-{} p-4 rounded-lg shadow-md w-full h-full aspect-w-16 aspect-h-9",
                            from_color(),
                            to_color()
                        ),
                        div {
                            class: "absolute w-full",
                            draggable: true,
                            onmousedown: move |event| {
                                let (x, y) = (event.coordinates().client().x, event.coordinates().client().y);
                                title_drag.set(Some((x, y)));
                            },
                            onmousemove: move |event| {
                                if let Some((start_x, start_y)) = title_drag() {
                                    let delta_x = event.coordinates().client().x - start_x;
                                    let delta_y = event.coordinates().client().y - start_y;

                                    title_position.set((delta_x, delta_y));
                                }
                            },
                            onmouseup: move |_| {
                                title_drag.set(None);
                            },
                            style: if title_position().1 != 0. || title_position().1 != 0. {
                                    format!(
                                        "top: {}px; left: {}px;",
                                        title_position().1,
                                        title_position().0
                                    )
                                } else {
                                    "".to_string()
                                },
                            h1 {
                                class: "absolute top-4 left-4 text-4xl font-bold text-gray-900",
                                "{title()}"
                            },
                        },
                        div {
                            class: "absolute w-full top-1/3 left-4 text-xl text-gray-900",
                            draggable: true,
                            onmousedown: move |event| {
                                let (x, y) = (event.coordinates().client().x, event.coordinates().client().y);
                                description_drag.set(Some((x, y)));
                            },
                            onmousemove: move |event| {
                                if let Some((start_x, start_y)) = description_drag() {
                                    let delta_x = event.coordinates().client().x - start_x;
                                    let delta_y = event.coordinates().client().y - start_y;

                                    description_position.set((delta_x, delta_y));
                                }
                            },
                            onmouseup: move |_| {
                                description_drag.set(None);
                            },
                            style: if description_position().1 != 0. || description_position().1 != 0. {
                                    format!(
                                        "top: {}px; left: {}px;",
                                        description_position().1,
                                        description_position().0
                                    )
                                } else {
                                    "".to_string()
                                },
                            div {
                                class: "absolute top-1/3 left-4 text-xl text-gray-900",
                                "{description()}"
                            },
                        }
                        h3 {
                            class: "absolute bottom-4 left-4 text-sm text-gray-900 italic",
                            draggable: true,
                            onmousedown: move |event| {
                                let (x, y) = (event.coordinates().client().x, event.coordinates().client().y);
                                author_drag.set(Some((x, y)));
                            },
                            onmousemove: move |event| {
                                if let Some((start_x, start_y)) = author_drag() {
                                    let delta_x = event.coordinates().client().x - start_x;
                                    let delta_y = event.coordinates().client().y - start_y;

                                    author_position.set((delta_x, delta_y));
                                }
                            },
                            onmouseup: move |_| {
                                author_drag.set(None);
                            },
                            style: if author_position().1 != 0. || author_position().1 != 0. {
                                    format!(
                                        "top: {}px; left: {}px;",
                                        author_position().1,
                                        author_position().0
                                    )
                                } else {
                                    "".to_string()
                                },
                            "Author: {author()} | Site: {site_name()}"
                        },
                        div {
                            class: "absolute w-24 h-24 m-4",
                            draggable: true,
                            onmousedown: move |event| {
                                let (x, y) = (event.coordinates().client().x, event.coordinates().client().y);
                                img_drag.set(Some((x, y)));
                            },
                            onmousemove: move |event| {
                                if let Some((start_x, start_y)) = img_drag() {
                                    let delta_x = start_x - event.coordinates().client().x;
                                    let delta_y = event.coordinates().client().y - start_y;

                                    img_position.set((delta_x, delta_y));
                                }
                            },
                            onmouseup: move |_| {
                                img_drag.set(None);
                            },
                            style: format!(
                                "background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat; top: {}px; right: {}px;",
                                image_url(),
                                img_position().1,
                                img_position().0
                            ),
                        }
                    },
                    button {
                        class: "mt-4 px-4 py-2 bg-indigo-500 text-white rounded-md hover:bg-green-600 transition",
                        onclick: move |_| { download_preview_as_image() },
                        r#type: "button",
                        "Download as Image"
                    }
                    div {
                        class: "mt-6 bg-gray-900 text-white p-4 rounded-lg shadow-md",
                        h4 { class: "text-lg font-semibold mb-2", "Generated Meta Tags" },
                        pre {
                            class: "bg-gray-800 p-3 rounded text-sm overflow-x-auto",
                            "{generate_meta_tags(generated_metadata().unwrap_or_default())}"
                        },
                        button {
                            class: "mt-4 px-4 py-2 bg-indigo-500 text-white rounded-md hover:bg-green-600 transition",
                            onclick: copy_to_clipboard,
                            "Copy to Clipboard"
                        }
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

async fn generate_ai_title() -> Result<String, String> {
    let mut client = NanoAI::new();
    let system_prompt = format!(
        "
        **System Prompt (SP):** You are an expert in content generation for web metadata and SEO optimization.

        **Prompt (P):** Generate a unique, concise, and creative image title for an OG (Open Graph) metadata tag.
        The title should align with modern web standards, capture user attention, and concisely describe the associated website content.

        **Expected Format (EF):**
        - A short, unique title (maximum 20 characters) that is impactful and descriptive.

        **Roleplay (RP):** Act as an experienced SEO copywriter crafting metadata titles for websites.
        "
    );

    match client.create_session(None).await {
        Ok(_) => match client.send_prompt(&system_prompt).await {
            Ok(response) => Ok(response),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

async fn generate_ai_description() -> Result<String, String> {
    let mut client = NanoAI::new();
    let system_prompt = format!(
        "
        **System Prompt (SP):** You are an expert in content generation for web metadata and SEO optimization.

        **Prompt (P):** Generate a unique and concise description for an OG (Open Graph) metadata tag.
        The description should provide a brief, engaging summary of the associated content, optimized for search engines and user engagement.

        **Expected Format (EF):**
        - A short sentence (maximum 60 characters) that is compelling, informative, and aligned with modern web best practices.

        **Roleplay (RP):** Act as an experienced SEO copywriter crafting metadata descriptions for websites.
        "
    );

    match client.create_session(None).await {
        Ok(_) => match client.send_prompt(&system_prompt).await {
            Ok(response) => Ok(response),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}
