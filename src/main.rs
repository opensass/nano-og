#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use nano_og::components::toast::provider::ToastProvider;
use nano_og::router::Route;
use nano_og::theme::ThemeProvider;

fn main() {
    #[cfg(feature = "web")]
    {
        dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
        tracing::info!("starting app");
        let config = dioxus_web::Config::new().hydrate(true);

        LaunchBuilder::new().with_cfg(config).launch(App);
    }

    #[cfg(feature = "server")]
    {
        use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
        use axum::http::Method;
        use axum::{Extension, Router};
        use dotenv::dotenv;
        use std::sync::Arc;
        use tower_http::cors::{Any, CorsLayer};

        dotenv().ok();
        dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cors = CorsLayer::new()
                    .allow_origin(Any)
                    // TODO
                    // .allow_origin("http://0.0.0.0:3000".parse::<HeaderValue>().unwrap())
                    // .allow_origin(
                    //     "https://generativelanguage.googleapis.com"
                    //         .parse::<HeaderValue>()
                    //         .unwrap(),
                    // )
                    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                    // .allow_credentials(true)
                    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

                let app = Router::new()
                    .layer(cors)
                    .serve_dioxus_application(ServeConfig::new().unwrap(), App);

                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    rsx! {
        ThemeProvider {
            ToastProvider {
                Router::<Route> {}
            }
        }
    }
}
