pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaStar;
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: &'static str,
    author_name: &'static str,
    author_title: &'static str,
    author_image: &'static str,
    company_logo: &'static str,
    star_images: Vec<Element>,
}

#[allow(unused_mut)]
#[component]
pub fn Testimonial() -> Element {
    let testimonials = vec![
        TestimonialData {
            quote: "I asked Nano OG to generate an OG image for my website... and somehow, I ended up with a neon spaceship and a llama in sunglasses. 10/10 would og again!",
            author_name: "Jeff Bezos",
            author_title: "Founder, Amazon",
            author_image: "./elon.webp",
            company_logo: "./spacex.webp",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
        TestimonialData {
            quote: "I told Nano OG I needed a website header. It gave me a flying cat riding a skateboard. Not what I expected, but I can't look away. Genius work!",
            author_name: "Mark Zuckerberg",
            author_title: "CEO, Meta",
            author_image: "./elon.webp",
            company_logo: "./spacex.webp",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
        TestimonialData {
            quote: "I wanted an OG image for my new e-commerce site. Nano OG delivered... but now my website is asking me to go to the moon? I'm intrigued, but slightly concerned.",
            author_name: "Elon Musk",
            author_title: "CEO, SpaceX",
            author_image: "./elon.webp",
            company_logo: "./spacex.webp",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
    ];

    let dark_mode = *THEME.read();
    let mut current_index = use_signal(|| 0);

    client! {
        let vec_len = testimonials.len();
        let mut eval = use_hook(|| {
            eval(
                r#"
                setInterval(() => {
                    dioxus.send("");
                }, 5000)
                "#,
            )
        });

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv().await;
                    current_index.set((current_index() + 1) % vec_len);
                }
            })
        });
    }

    rsx! {
        section {
            id: "testimonial",
            class: format!("flex flex-col items-center justify-center min-h-screen p-8 {}",
            if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }),

            div { class: "flex flex-col items-center mb-8",
                h2 { class: "text-4xl font-bold text-center",
                    "What People Are Saying about Nano OG"
                }

                p { class: format!("mt-2 text-lg {}", if dark_mode == Theme::Dark { "text-gray-300" } else { "text-gray-700" }),
                    "Nano OG: Where AI takes your website (and your imagination) on a wild ride."
                }
            }

            div { class: "flex flex-wrap justify-center items-center gap-8 p-4",
                for (i, testimonial) in testimonials.iter().enumerate() {
                    div { class: format!("transition-transform duration-500 transform {}, hover:scale-105 hover:shadow-xl",
                        if current_index() == i { "opacity-100 scale-100" } else { "opacity-50 scale-75 blur-sm" }),
                        div { class: format!("{} p-8 rounded-xl shadow-2xl text-center max-w-sm border",
                            if dark_mode == Theme::Dark { "border-gray-700 bg-gray-800" } else { "bg-white border-gray-300" }),
                            StarRating { star_images: testimonial.star_images.clone() }
                            blockquote {
                                class: format!("text-lg font-semibold italic {}",
                                    if dark_mode == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                                ),
                                "{testimonial.quote}"
                            }
                            AuthorInfo {
                                author_image: testimonial.author_image,
                                author_name: testimonial.author_name,
                                author_title: testimonial.author_title,
                                company_logo: testimonial.company_logo,
                            }
                        }
                    }
                }
            }

            div { class: "flex justify-center mt-6 space-x-2",
                for (i, _) in testimonials.iter().enumerate() {
                    div { class: format!("w-3 h-3 rounded-full {} transition-all duration-300",
                        if current_index() == i { "bg-gradient-to-r from-blue-400 to-indigo-500" } else { "bg-gray-400" })
                    }
                }
            }
        }
    }
}
