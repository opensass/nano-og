pub(crate) mod grid;
pub(crate) mod item;

use crate::components::features::grid::Grid;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let dark_mode = *THEME.read();

    let features = vec![
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "fa-solid fa-brain text-indigo-500 group-hover:animate-bounce" } },
            title: "AI-Powered Generation",
            description: "Generate dynamic OG images in seconds, optimized by the power of Nano AI's cutting-edge algorithms.",
        },
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "fa-solid fa-palette text-pink-500 group-hover:animate-spin" } },
            title: "Customizable Designs",
            description: "Tailor your images to match your brand’s style with our fully customizable design tools.",
        },
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "text-yellow-500 group-hover:animate-ping fa-regular fa-clock" } },
            title: "Real-Time Previews",
            description: "See real-time previews of your images as you design, ensuring every detail is perfect.",
        },
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "fa-solid fa-bolt text-blue-500 group-hover:animate-pulse" } },
            title: "Lightning-Fast Output",
            description: "Experience unparalleled speed with our optimized Rust-powered backend.",
        },
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "fa-solid fa-code text-green-500 group-hover:animate-bounce" } },
            title: "Developer-Friendly API",
            description: "Seamlessly integrate Nano AI into your workflow with our robust developer tools.",
        },
        Feature {
            icon: rsx! { i { width: 30, height: 30, class: "fa-solid fa-shield-halved text-purple-500 group-hover:animate-ping" } },
            title: "Secure and Reliable",
            description: "Your data and designs are safe with our secure, privacy-focused platform.",
        },
    ];

    rsx! {
        section {
            id: "features",
            class: format!(
                "relative min-h-screen flex flex-col items-center justify-center px-6 overflow-hidden transition-colors duration-300 {}",
                if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-gray-50 text-gray-900" }
            ),

            div {
                class: "absolute top-0 left-0 w-96 h-96 rounded-full bg-gradient-to-r from-indigo-500 to-purple-600 opacity-10 blur-xl animate-float",
            }
            div {
                class: "absolute bottom-0 right-0 w-80 h-80 rounded-full bg-gradient-to-r from-pink-500 to-yellow-500 opacity-10 blur-2xl animate-float-slow",
            }

            div {
                class: "max-w-7xl mx-auto text-center space-y-12 relative z-10",

                div {
                    class: "relative mb-12 space-y-6",

                    h2 {
                        class: format!(
                            "text-3xl md:text-5xl font-extrabold tracking-tight {}",
                            if dark_mode == Theme::Dark { "text-white" } else { "text-gray-900" }
                        ),
                        "Why Choose OG Nano?"
                    }
                    p {
                        class: format!(
                            "text-lg md:text-xl {}",
                            if dark_mode == Theme::Dark { "text-gray-300" } else { "text-gray-700" }
                        ),
                        "Leverage the power of AI to craft stunning OG images for your websites in record time."
                    }
                }

                Grid { features: features }
            }
        }
    }
}
