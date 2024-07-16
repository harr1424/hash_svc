use actix_web::{get, web,HttpResponse};
use std::sync::Arc;
use crate::hashing;

macro_rules! create_hash_endpoint {
    ($state_field:ident, $route:expr) => {
        #[get($route)]
        async fn $state_field(state: web::Data<Arc<hashing::AppState>>) -> HttpResponse {
            let image_hash = state.$state_field.lock().unwrap();
            HttpResponse::Ok().body(image_hash.clone())
        }
    };
}