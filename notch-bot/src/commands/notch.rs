use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::models::argument::{ArgumentStatus, DBArgument};
use crate::models::database::DBConnection;

#[command]
#[sub_commands(help, argument)]
pub async fn notch(context: &Context, message: &Message, args: Args) -> CommandResult {
    help(context, message, args).await
}


const HELP_MESSAGE: &str = r#"
Commands
`argument @user1 "description"` - Starts an internet argument between you and the mentioned user
`arguments` - Shows all open internet arguments
`take-your-notch {argument_id}` - gives the notch to the opposing party
`leaderboard` - Shows ordered list of user notch counts
"#;

#[command]
#[description("Generates help text")]
async fn help(context: &Context, message: &Message, _args: Args) -> CommandResult {
    message.channel_id.say(&context.http, HELP_MESSAGE).await?;
    Ok(())
}

const MISSING_DISSENTER: &str = "You can't earn a notch without mentioning someone to argue with";
const MISSING_GUILD_ID: &str = "notch-bot only works in discord servers";
#[command]
#[description("Starts an internet argument between you and the mentioned user")]
pub async fn argument(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if message.mentions.len() != 1 {
        message.channel_id.say(&context.http, MISSING_DISSENTER).await?;
        return Ok(())
    }
    let guild_id_option= message.guild_id;
    if let None = guild_id_option {
        message.channel_id.say(&context.http, MISSING_GUILD_ID).await?;
        return Ok(())
    }
    let guild_id = guild_id_option.expect("Should have guaranteed guild_id");

    let argument_starter : &User = &message.author;
    let dissenter : &User = message.mentions.get(0).expect("Should have guaranteed mention");
    let _discard = args.single::<String>().expect("Should have dissenter mentioned");
    let description = args.single_quoted::<String>().expect("should have description");
    let mut data = context.data.write().await;
    let database  = &*data.get_mut::<DBConnection>()
                         .expect("Unable to get db connection in command")
                         .clone();

    let status = ArgumentStatus::InProgress.as_str().to_string();
    let guild_id_u64 = i64::from(guild_id);
    let argument_starter_id = i64::from(argument_starter.id);
    let dissenter_id = i64::from(dissenter.id);
    let db_response = sqlx::query_as!(
        DBArgument,
        r#"INSERT INTO arguments
        (guild_id, argument_starter_id, dissenter_id, description, status)
        values ($1, $2, $3, $4, $5) RETURNING *"#,
        guild_id_u64,
        argument_starter_id,
        dissenter_id,
        description,
        status
    )
        .fetch_one(database)
        .await;

    match db_response {
        Ok(db_argument) => {
            message.channel_id.say(&context.http,
                                   format!("Argument created with id: {}", db_argument.argument_id))
                   .await?;
            Ok(())
        },
        Err(_e)  => {
            message.channel_id.say(&context.http, "Failed to create argument")
                   .await?;
            Ok(())
        }
    }
}
