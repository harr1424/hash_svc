use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::prelude::*;
use sha2::{Digest, Sha256};
use tokio::sync::Notify;

use crate::config::Config;

const REFRESH_HASH_IN_SECONDS: u64 = 60;

pub(crate) struct AppState {
    pub(crate) en_image_hash: Mutex<String>,
    pub(crate) en_p_image_hash: Mutex<String>,
    pub(crate) es_image_hash: Mutex<String>,
    pub(crate) es_p_image_hash: Mutex<String>,
    pub(crate) fr_image_hash: Mutex<String>,
    pub(crate) po_image_hash: Mutex<String>,
    pub(crate) it_image_hash: Mutex<String>,
    pub(crate) de_image_hash: Mutex<String>,
    pub(crate) notify: Notify,
}

macro_rules! download_and_hash_image {
    ($state_mu:expr, $image:expr, $state_notify:expr) => {
        let image_data = match reqwest::get($image).await {
            Ok(response) => match response.bytes().await {
                Ok(data) => data,
                Err(e) => {
                    let now: DateTime<Utc> = Utc::now();
                    eprintln!("{} : Error reading response bytes: {}", now, e);
                    continue;
                }
            },
            Err(e) => {
                let now: DateTime<Utc> = Utc::now();
                eprintln!("{} : Error fetching image: {}", now, e);
                continue;
            }
        };
        let hash = format!("{:x}", Sha256::digest(&image_data));
        {
            let mut image_hash = $state_mu.lock().unwrap();
            *image_hash = hash.clone();
            $state_notify.notify_one();
        }
    };
}
pub(crate) async fn download_and_hash_images(state: Arc<AppState>, config: Config) {
    let en_image = config.secrets.en_image.clone();
    let en_p_image = config.secrets.en_image_p.clone();
    let es_image = config.secrets.es_image.clone();
    let es_p_image = config.secrets.es_image_p.clone();
    let fr_image = config.secrets.fr_image.clone();
    let po_image = config.secrets.po_image.clone();
    let it_image = config.secrets.it_image.clone();
    let de_image = config.secrets.de_image.clone();

    loop {
        download_and_hash_image!(state.en_image_hash, &en_image, state.notify);
        download_and_hash_image!(state.en_p_image_hash, &en_p_image, state.notify);
        download_and_hash_image!(state.es_image_hash, &es_image, state.notify);
        download_and_hash_image!(state.es_p_image_hash, &es_p_image, state.notify);
        download_and_hash_image!(state.fr_image_hash, &fr_image, state.notify);
        download_and_hash_image!(state.po_image_hash, &po_image, state.notify);
        download_and_hash_image!(state.it_image_hash, &it_image, state.notify);
        download_and_hash_image!(state.de_image_hash, &de_image, state.notify);

        tokio::time::sleep(Duration::from_secs(REFRESH_HASH_IN_SECONDS)).await;
    }
}
