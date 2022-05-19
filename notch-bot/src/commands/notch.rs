use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

const HELP: &str = "help";
const ARGUMENT: &str = "argument";
enum NotchSubcommand {
    Help,
    Argument,
}

impl NotchSubcommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotchSubcommand::Help => HELP,
            NotchSubcommand::Argument => ARGUMENT
        }
    }

    pub fn as_command(s: &str) -> NotchSubcommand {
        let clean_s = &s.to_lowercase();
        match clean_s.as_str() {
            HELP => NotchSubcommand::Help,
            ARGUMENT => NotchSubcommand::Argument,
            _ => NotchSubcommand::Help
        }
    }
}

#[command]
pub async fn notch(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let subcommand_s = args.single::<String>()?;
    let subcommand = NotchSubcommand::as_command(&subcommand_s);
    match subcommand {
        NotchSubcommand::Help => help(context, message).await,
        NotchSubcommand::Argument => argument(context, message).await
    }
}


const HELP_MESSAGE: &str = r#"
Commands
`argument @user1 "description"` - Starts an internet argument between you and chosen other user
`arguments` - Shows all open internet arguments
`take-your-notch {argument_id}` - gives the notch to the opposing party
`leaderboard` - Shows ordered list of user notch counts
"#;

async fn help(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(&context.http, HELP_MESSAGE).await?;
    Ok(())
}

const MISSING_DISSENTER: &str = "You can't earn a notch without mentioning someone to argue with";
pub async fn argument(context: &Context, message: &Message) -> CommandResult {
    if message.mentions.len() != 1 {
        message.channel_id.say(&context.http, MISSING_DISSENTER).await?;
        return Ok(())
    }

    Ok(())
}
