use actix_web::{get, web, App, HttpServer, Responder};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
