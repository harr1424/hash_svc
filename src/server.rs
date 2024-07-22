use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_rt::spawn;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::{Arc, Mutex};
use chrono::Duration;
use tokio::sync::Notify;

use actix_route_rate_limiter::{LimiterBuilder, RateLimiter};
use crate::{hashing, config::Config};

macro_rules! create_hash_endpoint {
    ($state_field:ident, $route:expr) => {
        #[actix_web::get($route)]
        async fn $state_field(state: web::Data<Arc<hashing::AppState>>) -> HttpResponse {
            let image_hash = state.$state_field.lock().unwrap();
            HttpResponse::Ok().body(image_hash.clone())
        }
    };
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    create_hash_endpoint!(en_image_hash, "/en");
    create_hash_endpoint!(en_p_image_hash, "/en_p");
    create_hash_endpoint!(es_image_hash, "/es");
    create_hash_endpoint!(es_p_image_hash, "/es_p");
    create_hash_endpoint!(fr_image_hash, "/fr");
    create_hash_endpoint!(po_image_hash, "/po");
    create_hash_endpoint!(it_image_hash, "/it");
    create_hash_endpoint!(de_image_hash, "/de");

    let app_state = Arc::new(hashing::AppState {
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

    let limiter = LimiterBuilder::new()
        .with_duration(Duration::seconds(20))
        .with_num_requests(2)
        .build();

    spawn(async move {
        let config = Config::load_from_file("Config.toml").unwrap_or_else(|e| {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        });

        hashing::download_and_hash_images(app_state_clone, config).await;
    });

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("certs/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder
    //     .set_certificate_chain_file("certs/cert.pem")
    //     .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(RateLimiter::new(Arc::clone(&limiter)))
            .service(en_image_hash)
            .service(en_p_image_hash)
            .service(es_image_hash)
            .service(es_p_image_hash)
            .service(fr_image_hash)
            .service(po_image_hash)
            .service(it_image_hash)
            .service(de_image_hash)
    })
        //.bind_openssl("0.0.0.0:9191", builder)?
        .bind(("0.0.0.0", 9191))?
        .run()
        .await
}
