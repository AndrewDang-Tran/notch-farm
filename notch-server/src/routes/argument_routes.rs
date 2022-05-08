use actix_web::{post, Responder, web, HttpResponse};
use crate::AppData;
use crate::models::{argument, CreateArgumentRequest};

#[post("/arguments")]
async fn create_argument(request_json: web::Json<CreateArgumentRequest>,
                         data: AppData) -> impl Responder {
    let request = request_json.into_inner();
    let status = argument::ArgumentStatus::InProgress.as_str().to_string();
    let db_response = sqlx::query!(
        r#"INSERT INTO arguments
        (group_id, argument_starter, dissenter, description, status)
        values ($1, $2, $3, $4, $5)"#,
        request.group_id,
        request.argument_starter,
        request.dissenter,
        request.description,
        status
    )
        .execute(&data.db_connection_pool)
        .await;
    if let Err(sqlx::Error::Database(err)) = db_response {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().body("Hello world!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_argument);
}
