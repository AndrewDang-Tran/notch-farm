use actix_web::{get, Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse<'a> {
    status: &'a str
}
const HEALTHY_STATUS: &str = "pass";
const HEALTH_RESPONSE: HealthResponse = HealthResponse {
    status: HEALTHY_STATUS
};

#[get("/health-check")]
async fn health_check() -> impl Responder {
    web::Json(HEALTH_RESPONSE)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
