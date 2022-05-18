use actix_web::{get, HttpResponse, HttpResponseBuilder, post, put, Responder, web};
use sqlx::{FromRow, query};

use crate::AppData;
use crate::models::argument::{Argument, ArgumentStatus, ArgumentStatusParseError, CreateArgumentRequest, DBArgument, GetArgumentsParams, UpdateNotchTakerRequest};

#[post("/arguments")]
async fn create_argument(request_json: web::Json<CreateArgumentRequest>,
                         data: AppData) -> impl Responder {
    let request = request_json.into_inner();
    let status = ArgumentStatus::InProgress.as_str().to_string();
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
    db_to_responder(db_response, HttpResponse::Created())
}

#[get("/arguments")]
async fn get_arguments_by_group_id(query_params: web::Query<GetArgumentsParams>,
                                   data: AppData) -> impl Responder {
    let params = query_params.into_inner();
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"SELECT
        argument_id, group_id, argument_starter, dissenter, description, status, notch_taker
        FROM arguments
        WHERE group_id = $1"#,
        params.group_id
    )
        .fetch_all(&data.db_connection_pool)
        .await;

    match db_response {
        Ok(db_arguments) => {
            let argument_results = db_arguments.into_iter()
                .map(|db_a| Argument::from_db(db_a))
                .collect::<Result<Vec<Argument>, ArgumentStatusParseError>>();
            match argument_results {
                Ok(arguments) => HttpResponse::Created().json(arguments),
                Err(e) => HttpResponse::InternalServerError().finish()
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

#[get("/arguments/{argument_id}")]
async fn get_argument(path: web::Path<i64>,
                      data: AppData) -> impl Responder {
    let argument_id = path.into_inner();
    let argument_result = internal_get_argument(&data, argument_id).await;
    match argument_result {
        Ok(argument) => {
            HttpResponse::Ok().json(argument)
        },
        Err(_e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/arguments/{argument_id}/notch-taker")]
async fn update_argument_notch_taker(path: web::Path<i64>,
                                     request_json: web::Json<UpdateNotchTakerRequest>,
                                     data: AppData) -> impl Responder {
    let request = request_json.into_inner();
    let argument_id = path.into_inner();
    let argument_result = internal_get_argument(&data, argument_id).await;
    let notch_taken_status = ArgumentStatus::NotchTaken.as_str().to_string();

    match argument_result {
        Err(e) => HttpResponse::InternalServerError().finish(),
        Ok(argument) => {
            let argument_not_taken = argument.notch_taker.is_none();
            let valid_notch_taker = request.notch_taker == argument.argument_starter ||
                request.notch_taker == argument.dissenter;
            if argument_not_taken && valid_notch_taker {
                let db_response = sqlx::query_as!(
                    DBArgument,
                    r#"UPDATE arguments
                    SET notch_taker = $1, status = $2
                    WHERE argument_id = $3
                    RETURNING *"#,
                    request.notch_taker,
                    notch_taken_status,
                    argument_id,
                )
                    .fetch_one(&data.db_connection_pool) // TODO: switch to getting optional and 404
                    .await;
                db_to_responder(db_response, HttpResponse::Ok())
            } else {
                HttpResponse::BadRequest().finish()//.body("invalid notch_taker sent")
            }
        }
    }
}

async fn internal_get_argument(data: &AppData, argument_id: i64) -> Result<Argument, Box<dyn std::error::Error>> {
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"SELECT
        argument_id, group_id, argument_starter, dissenter, description, status, notch_taker
        FROM arguments
        WHERE argument_id = $1"#,
        argument_id
    )
        .fetch_one(&data.db_connection_pool) // TODO: switch to getting optional and 404
        .await;

    match db_response {
        Ok(db_argument) => {
            let argument_result = Argument::from_db(db_argument);
            match argument_result {
                Ok(a) => Ok(a),
                Err(e) => Err(Box::new(e))
            }
        },
        Err(e) => Err(Box::new(e))
    }
}

fn db_to_responder(result: Result<DBArgument, sqlx::Error>,
                   mut happy_response_builder: HttpResponseBuilder) -> HttpResponse {
    match result {
        Ok(db_argument) => {
            let argument_result = Argument::from_db(db_argument);
            match argument_result {
                Ok(argument_response) => {
                    happy_response_builder.json(argument_response)
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
    cfg.service(get_arguments_by_group_id);
    cfg.service(update_argument_notch_taker);
}
