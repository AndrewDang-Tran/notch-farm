use actix_web::{get, post, Responder, web, HttpResponse};
use crate::AppData;
use crate::models::{argument, Argument, DBArgument, CreateArgumentRequest};

#[post("/arguments")]
async fn create_argument(request_json: web::Json<CreateArgumentRequest>,
                         data: AppData) -> impl Responder {
    let request = request_json.into_inner();
    let status = argument::ArgumentStatus::InProgress.as_str().to_string();
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"INSERT INTO arguments
        (group_id, argument_starter, dissenter, description, status)
        values ($1, $2, $3, $4, $5) RETURNING *"#,
        request.group_id,
        request.argument_starter,
        request.dissenter,
        request.description,
        status
    )
        .fetch_one(&data.db_connection_pool)
        .await;
    db_to_responder(db_response)
}

#[get("/arguments/{argument_id}")]
async fn get_argument(path: web::Path<i64>,
                      data: AppData) -> impl Responder {
    let argument_id = path.into_inner();
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"SELECT
        argument_id, group_id, argument_starter, dissenter, description, status, notch_taker
        FROM arguments
        WHERE argument_id = $1"#,
        argument_id
    )
        .fetch_one(&data.db_connection_pool)
        .await;
    db_to_responder(db_response)
}

fn db_to_responder(result: Result<DBArgument, sqlx::Error>) -> impl Responder {
    match result {
        Ok(db_argument) => {
            let argument_result = Argument::from_db(db_argument);
            match argument_result {
                Ok(argument_response) => {
                    HttpResponse::Created().json(argument_response)
                },
                Err(e) => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(sqlx::Error::Database(err))  => {
            HttpResponse::InternalServerError().finish()
        },
        Err(e)  => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_argument);
    cfg.service(get_argument);
}