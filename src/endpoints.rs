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

create_hash_endpoint!(en_image_hash, "/en");
create_hash_endpoint!(en_p_image_hash, "/en_p");
create_hash_endpoint!(es_image_hash, "/es");
create_hash_endpoint!(es_p_image_hash, "/es_p");
create_hash_endpoint!(fr_image_hash, "/fr");
create_hash_endpoint!(po_image_hash, "/po");
create_hash_endpoint!(it_image_hash, "/it");
create_hash_endpoint!(de_image_hash, "/de");