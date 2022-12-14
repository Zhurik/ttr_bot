use teloxide::{prelude::*, utils::command::BotCommands};

const BOT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "ping-pong.")]
    Ping,
    #[command(description = "display current bot version")]
    Version,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Ping => bot.send_message(msg.chat.id, "Pong").await?,
        Command::Version => {
            bot.send_message(msg.chat.id, format!("My current version is {BOT_VERSION}"))
                .await?
        }
    };

    Ok(())
}
