use actix_web::{App, HttpServer, middleware};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::init_health_controller)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
