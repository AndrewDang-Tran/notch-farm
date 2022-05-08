use actix_web::{post, Responder, web, HttpResponse};
use crate::AppData;

#[post("/arguments")]
async fn create_argument(_data: AppData) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_argument);
}
