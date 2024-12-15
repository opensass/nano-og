use crate::router::Route;
use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let dark_mode = use_context::<Signal<Theme>>();

    rsx! {
        section {
            class: format!(
                "relative min-h-screen flex flex-col items-center justify-center px-6 overflow-hidden transition-colors duration-300 {}",
                if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-gray-50 text-gray-900" }
            ),
            div {
                class: "absolute inset-0 bg-gradient-to-bl from-indigo-500 via-purple-500 to-transparent opacity-30 pointer-events-none"
            },
            div {
                class: "relative z-10 text-center space-y-6 max-w-5xl mx-auto",
                p {
                    class: "text-base md:text-lg font-medium tracking-widest uppercase text-transparent bg-clip-text bg-gradient-to-r from-pink-400 via-purple-500 to-blue-400 animate-pulse",
                    "Create OG Images with Precision and Style"
                },
                h1 {
                    class: "text-4xl md:text-7xl font-extrabold leading-tight tracking-wide animate-fade-in",
                    span { class: "text-indigo-500", "AI" },
                    " for Perfect",
                    span { class: "text-indigo-500", " OG Images" },
                },
                p {
                    class: format!(
                        "text-lg md:text-2xl leading-relaxed max-w-3xl mx-auto animate-fade-in delay-150 {}",
                        if dark_mode() == Theme::Dark { "text-gray-300" } else { "text-gray-400" }
                    ),
                    "Unleash the power of Gemini Nano AI to craft visually stunning OG images that make your website stand out. It's fast, intuitive, and cutting-edge."
                },
                div {
                    class: "flex justify-center gap-6 mt-8 animate-slide-up delay-200",
                    a {
                        href: "#features",
                        class: "bg-indigo-500 text-white py-3 px-8 rounded-full shadow-lg hover:bg-indigo-600 focus:outline-none transform hover:scale-105 transition-transform duration-150 font-semibold",
                        "Explore Features"
                    },
                    Link {
                        to: Route::Login {},
                        class: format!(
                            "py-3 px-8 rounded-full shadow-lg focus:outline-none transform hover:scale-105 transition-transform duration-150 font-semibold {}",
                            if dark_mode() == Theme::Dark {
                                "bg-gray-800 text-gray-100 hover:bg-gray-700"
                            } else {
                                "bg-gray-100 text-gray-800 hover:bg-gray-200"
                            }
                        ),
                        "Try Demo"
                    }
                }
            },
            div {
                class: "absolute top-12 left-8 w-24 h-24 bg-indigo-400 rounded-full opacity-40 animate-bounce-slow shadow-xl filter blur-xl"
            },
            div {
                class: "absolute bottom-12 right-16 w-36 h-36 bg-pink-500 rounded-full opacity-40 animate-bounce-slow animation-delay-300 shadow-xl filter blur-lg"
            },
            div {
                class: "absolute left-12 w-36 h-36 bg-blue-500 rounded-full opacity-40 animate-bounce-slow animation-delay-300 shadow-xl filter blur-lg"
            },
            div {
                class: "absolute top-20 right-1/4 w-12 h-12 bg-gradient-to-r from-indigo-300 to-purple-500 rounded-full shadow-md animate-bounce shadow-lg delay-700"
            },
            div {
                class: format!(
                    "absolute bottom-0 w-full h-64 bg-gradient-to-t from-gray-900 to-transparent opacity-80 pointer-events-none {}",
                    if dark_mode() == Theme::Dark { "opacity-60" } else { "" }
                )
            }
        }
    }
}
