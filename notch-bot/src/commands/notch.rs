use itertools::Itertools;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::dao;
use crate::models::argument::{CreateArgumentParams, UpdateNotchTakerParams};

#[command]
#[sub_commands(help, argument, list_arguments, take_your_notch, leaderboard)]
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
    // TODO: https://docs.rs/serenity/latest/serenity/utils/fn.parse_username.html
    let _discard = args.single::<String>().expect("Should have dissenter mentioned");
    let description = args.single_quoted::<String>().expect("should have description");
    let params = CreateArgumentParams {
        guild_id,
        argument_starter_id: argument_starter.id,
        dissenter_id: dissenter.id,
        description
    };
    let created_argument = dao::create_argument(context, params).await;

    match created_argument {
        Ok(argument) => {
            message.channel_id.say(&context.http,
                                   format!("argument created with id: {}", argument.argument_id))
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

#[command]
#[aliases("arguments")]
pub async fn list_arguments(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let guild_id= message.guild_id.expect("Should have guaranteed guild_id");
    let arguments_result = dao::get_open_arguments(context, guild_id).await;

    match arguments_result {
        Ok(arguments) => {
            let mut arguments_message = MessageBuilder::new()
                .push_bold_line("Arguments:")
                .build();
            for argument in arguments {
                let argument_starter = context.http
                                              .get_user(argument.argument_starter_id.unsigned_abs())
                                              .await
                                              .expect("Failed to get argument starter");
                let dissenter = context.http
                                       .get_user(argument.dissenter_id.unsigned_abs())
                                       .await
                                       .expect("Failed to get dissenter");
                let partial = MessageBuilder::new().push("Argument id: ")
                                                   .push(argument.argument_id)
                                                   .push(" between ")
                                                   .push(&argument_starter.name)
                                                   .push(" and ")
                                                   .push(&dissenter.name)
                                                   .push_line(" about")
                                                   .push_codeblock(&argument.description, None)
                                                   .build();
                arguments_message = arguments_message + &partial;
            }
            message.channel_id.say(&context.http,
                                   arguments_message)
                   .await?;
            Ok(())
        },
        Err(_e) => {
            message.channel_id.say(&context.http, "Failed to parse arguments")
                   .await?;
            Ok(())
        }
    }
}

const FAILED_TO_GET_ARGUMENT: &str = "Failed to find the argument";
const NOT_PART_OF_ARGUMENT: &str = "You can only give notches if you're part of the argument";
const NOTCH_ALREADY_TAKEN: &str = "Notch for this argument already taken";
#[command]
#[aliases("take-your-notch")]
pub async fn take_your_notch(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let notch_giver_id = message.author.id.as_u64();
    let argument_id = args.single::<i64>().expect("Must provide argument_id");
    let argument_result = dao::get_argument(context, argument_id).await;

    match argument_result {
        Err(_e) => {
            message.channel_id.say(&context.http, FAILED_TO_GET_ARGUMENT).await?;
            Ok(())
        },
        Ok(argument) => {
            let argument_taken = argument.notch_taker_id.is_some();
            if argument_taken {
                message.channel_id.say(&context.http, NOTCH_ALREADY_TAKEN).await?;
                Ok(())
            } else {
                let mut notch_taker_id: Option<u64> = None;
                if *notch_giver_id == argument.argument_starter_id as u64 {
                    notch_taker_id = Some(argument.dissenter_id as u64)
                } else if *notch_giver_id == argument.dissenter_id as u64 {
                    notch_taker_id = Some(argument.argument_starter_id as u64)
                }
                match notch_taker_id {
                    Some(id) => {
                        let params = UpdateNotchTakerParams {
                            argument_id,
                            notch_taker: UserId::from(id)
                        };
                        dao::update_notch_taker(context, params).await?;
                        let confirmation = MessageBuilder::new()
                            .user(UserId::from(*notch_giver_id))
                            .push(" has given the notch to ")
                            .user(UserId::from(id))
                            .build();
                        message.channel_id.say(&context.http, confirmation).await?;
                        Ok(())
                    },
                    None => {
                        message.channel_id.say(&context.http, NOT_PART_OF_ARGUMENT).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

#[command]
pub async fn leaderboard(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let guild_id = message.guild_id.expect("Command must be sent in guild");
    let taken_arguments = dao::get_taken_arguments(context, guild_id).await;
    match taken_arguments {
        Ok(arguments) => {
            let stats: Vec<(i64, usize)> = arguments.into_iter()
                                                    .into_group_map_by(|a| a.notch_taker_id)
                                                    .into_iter()
                                                    .filter(|(id, t_arguments)| id.is_some())
                                                    .map(|(id, t_arguments)| (id.unwrap(), t_arguments.len()))
                                                    .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
                                                    .collect::<Vec<(i64, usize)>>();

            let mut leaderboard = "Notch Leaderboard\n".to_string();
            for (user_id, notch_count) in stats {
                let user = UserId::from(user_id as u64).to_user(&context.http).await?;
                let row = MessageBuilder::new()
                    .push(user.name)
                    .push(" : ")
                    .push_line(notch_count.to_string())
                    .build();
                leaderboard += &row;
            }
            message.channel_id.say(&context.http, leaderboard).await?;
            Ok(())
        },
        Err(_e) => {
            message.channel_id.say(&context.http, "Failed to get taken arguments").await?;
            Ok(())
        }
    }
}
