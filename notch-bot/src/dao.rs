use serenity::client::Context;
use crate::models::database::DBConnection;
use crate::models::argument::{Argument, ArgumentStatus, CreateArgumentParams, DBArgument};

pub async fn create_argument(context: &Context, params: CreateArgumentParams) -> Result<Argument, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = context.data.write().await;
    let database  = &*data.get_mut::<DBConnection>()
                          .expect("Unable to get db connection in command")
                          .clone();

    let status = ArgumentStatus::InProgress.as_str().to_string();
    let guild_id = i64::from(params.guild_id);
    let argument_starter_id = i64::from(params.argument_starter_id);
    let dissenter_id = i64::from(params.dissenter_id);
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"INSERT INTO arguments
        (guild_id, argument_starter_id, dissenter_id, description, status)
        values ($1, $2, $3, $4, $5) RETURNING *"#,
        guild_id,
        argument_starter_id,
        dissenter_id,
        params.description,
        status
    )
        .fetch_one(database)
        .await;

    match db_response {
        Ok(db_argument) => {
            match Argument::from_db(db_argument) {
                Ok(a) => Ok(a),
                Err(e) => Err(Box::new(e))
            }
        },
        Err(e)  => Err(Box::new(e))
    }
}