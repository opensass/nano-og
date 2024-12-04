use pinata_sdk::PinataApi;
use tokio::sync::{Mutex, OnceCell};

static PINATA: OnceCell<Mutex<PinataApi>> = OnceCell::const_new();

async fn init_pinata() -> &'static Mutex<PinataApi> {
    PINATA
        .get_or_init(|| async {
            let api_key = std::env::var("PINATA_API_KEY").expect("PINATA_API_KEY must be set");
            let secret_api_key =
                std::env::var("PINATA_API_SECRET").expect("PINATA_API_SECRET must be set");
            let pinata = PinataApi::new(&api_key, &secret_api_key).unwrap();
            Mutex::new(pinata)
        })
        .await
}

pub async fn get_pinata_client() -> &'static Mutex<PinataApi> {
    init_pinata().await
}
