use crate::models::argument::{
    Argument, ArgumentStatus, ArgumentStatusParseError, CreateArgumentParams, DBArgument,
    UpdateNotchTakerParams,
};
use crate::models::database::DBConnection;
use serenity::client::Context;
use serenity::model::id::GuildId;
use std::error::Error;

pub async fn create_argument(
    context: &Context,
    params: CreateArgumentParams,
) -> Result<Argument, Box<dyn Error + Send + Sync>> {
    let mut data = context.data.write().await;
    let database = &*data
        .get_mut::<DBConnection>()
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

    db_response_to_argument(db_response)
}

pub async fn get_argument(
    context: &Context,
    argument_id: i64,
) -> Result<Argument, Box<dyn Error + Send + Sync>> {
    let mut data = context.data.write().await;
    let database = &*data
        .get_mut::<DBConnection>()
        .expect("Unable to get db connection in command")
        .clone();

    let db_response = sqlx::query_as!(
        DBArgument,
        r#"SELECT
        argument_id, guild_id, argument_starter_id, dissenter_id, description, status, notch_taker_id
        FROM arguments
        WHERE argument_id = $1"#,
        argument_id
    )
        .fetch_one(database) // TODO: switch to getting optional and 404
        .await;

    db_response_to_argument(db_response)
}

pub async fn get_open_arguments(
    context: &Context,
    guild_id: GuildId,
) -> Result<Vec<Argument>, Box<dyn Error + Send + Sync>> {
    get_arguments(context, guild_id, ArgumentStatus::InProgress).await
}

pub async fn get_taken_arguments(
    context: &Context,
    guild_id: GuildId,
) -> Result<Vec<Argument>, Box<dyn Error + Send + Sync>> {
    get_arguments(context, guild_id, ArgumentStatus::NotchTaken).await
}

async fn get_arguments(
    context: &Context,
    guild_id: GuildId,
    status: ArgumentStatus,
) -> Result<Vec<Argument>, Box<dyn Error + Send + Sync>> {
    let mut data = context.data.write().await;
    let database = &*data
        .get_mut::<DBConnection>()
        .expect("Unable to get db connection in command")
        .clone();

    let string_status = status.as_str().to_string();
    let guild_id = i64::from(guild_id);
    let db_response: Result<Vec<DBArgument>, sqlx::Error> = sqlx::query_as!(
        DBArgument,
        r#"SELECT
        argument_id, guild_id, argument_starter_id, dissenter_id, description, status, notch_taker_id
        FROM arguments
        WHERE guild_id = $1 AND status = $2"#,
        guild_id,
        string_status
    )
        .fetch_all(database)
        .await;

    match db_response {
        Ok(db_arguments) => {
            let argument_results = db_arguments
                .into_iter()
                .map(Argument::from_db)
                .collect::<Result<Vec<Argument>, ArgumentStatusParseError>>();

            match argument_results {
                Ok(arguments) => Ok(arguments),
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn update_notch_taker(
    context: &Context,
    params: UpdateNotchTakerParams,
) -> Result<Argument, Box<dyn Error + Send + Sync>> {
    let mut data = context.data.write().await;
    let database = &*data
        .get_mut::<DBConnection>()
        .expect("Unable to get db connection in command")
        .clone();

    let notch_taken_status = ArgumentStatus::NotchTaken.as_str().to_string();
    let notch_taker = i64::from(params.notch_taker);
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"UPDATE arguments
        SET notch_taker_id = $1, status = $2
        WHERE argument_id = $3
        RETURNING *"#,
        notch_taker,
        notch_taken_status,
        params.argument_id,
    )
    .fetch_one(database) // TODO: switch to getting optional and 404
    .await;

    db_response_to_argument(db_response)
}

fn db_response_to_argument(
    db_response: Result<DBArgument, sqlx::Error>,
) -> Result<Argument, Box<dyn Error + Send + Sync>> {
    match db_response {
        Ok(db_argument) => match Argument::from_db(db_argument) {
            Ok(a) => Ok(a),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}
