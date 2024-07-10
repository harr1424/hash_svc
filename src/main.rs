use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

mod lib {
    pub mod config;
    pub mod endpoints;
    pub mod hashing;
}

use lib::config::Config;
use lib::endpoints;
use lib::hashing::{self, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState {
        en_image_hash: Mutex::new(String::new()),
        en_p_image_hash: Mutex::new(String::new()),
        es_image_hash: Mutex::new(String::new()),
        es_p_image_hash: Mutex::new(String::new()),
        fr_image_hash: Mutex::new(String::new()),
        po_image_hash: Mutex::new(String::new()),
        it_image_hash: Mutex::new(String::new()),
        de_image_hash: Mutex::new(String::new()),
        notify: Notify::new(),
    });

    let app_state_clone = Arc::clone(&app_state);

    tokio::spawn(async move {
        let config = Config::load_from_file("Config.toml").unwrap_or_else(|e| {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        });

        hashing::download_and_hash_images(app_state_clone, config).await;
    });

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("certs/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("certs/cert.pem")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(endpoints::en_image_hash)
            .service(endpoints::en_p_image_hash)
            .service(endpoints::es_image_hash)
            .service(endpoints::es_p_image_hash)
            .service(endpoints::fr_image_hash)
            .service(endpoints::po_image_hash)
            .service(endpoints::it_image_hash)
            .service(endpoints::de_image_hash)
    })
    .bind_openssl("0.0.0.0:9191", builder)?
    .run()
    .await
}
