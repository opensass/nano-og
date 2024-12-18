use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::server::og::controller::get_og_for_user;
use crate::server::og::model::OG;
use crate::server::og::request::GetOGForUserRequest;
use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn ViewOGPanel(og_id: String, user_token: Signal<String>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut selected_og = use_signal(|| None::<OG>);
    let mut loading = use_signal(|| true);

    let _ = use_resource(move || {
        let og_id_cloned = og_id.clone();
        async move {
            if let Ok(response) = get_og_for_user(GetOGForUserRequest {
                og_id: og_id_cloned,
                token: user_token(),
            })
            .await
            {
                loading.set(false);
                selected_og.set(Some(response.data));
            } else {
                loading.set(false);
            }
        }
    });
    fn download_preview_as_image() {
        client! {
            document::eval(r#"
                const element = document.getElementById('preview-section');
                if (element) {
                    html2canvas(element).then((canvas) => {
                        const link = document.createElement('a');
                        link.download = 'og-preview.png';
                        link.href = canvas.toDataURL('image/png');
                        link.click();
                    });
                }
            "#);
        }
    }

    rsx! {
        div {
            class: format!("flex h-full {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            div {
                class: "flex-1 p-6 overflow-y-auto",
                if let Some(og) = selected_og() {
                    div {
                        id: "preview-section",
                        class: "relative bg-gradient-to-r from-purple-300 to-pink-300 p-4 rounded-lg shadow-md min-h-screen w-full aspect-w-16 aspect-h-9",
                        h1 {
                            class: "absolute top-4 left-4 text-4xl font-bold text-gray-900",
                            contenteditable: true,
                            "{og.title}"
                        },
                        div {
                            class: "absolute top-1/3 left-4 text-xl text-gray-900",
                            contenteditable: true,
                            "{og.description}"
                        },
                        h3 {
                            class: "absolute bottom-4 left-4 text-sm text-gray-900 italic",
                            contenteditable: true,
                            "Author: {og.author} | Site: {og.site_name}"
                        },
                        img {
                            class: "absolute top-0 right-0 w-24 h-24 shadow-lg m-4",
                            src: "{og.brand_url}",
                            alt: "Brand Logo"
                        }
                    }
                    button {
                        class: "mt-4 px-4 py-2 bg-indigo-500 text-white rounded-md hover:bg-green-600 transition",
                        onclick: move |_| { download_preview_as_image() },
                        r#type: "button",
                        "Download as Image"
                    }
                } else {
                    p {
                        class: "flex items-center space-x-2 px-4 py-2 rounded",
                        Spinner {
                            aria_label: "Loading spinner".to_string(),
                            size: SpinnerSize::Md,
                            dark_mode: true,
                        }
                        span { "Loading OG..." }
                    }
                }
            }
        }
    }
}
