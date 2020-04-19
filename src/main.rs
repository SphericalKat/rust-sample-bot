use std::env;

use teloxide::{prelude::*, utils::command::BotCommand};

use admin::promote;
use dotenv::dotenv;
use lazy_static::lazy_static;

mod admin;

lazy_static! {
    static ref BOT_ID: String = env::var("BOT_ID").expect("BOT_ID must be defined");
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Starts the bot")]
    Start,
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Send a photo")]
    SendPhoto,
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn answer(
    cx: &DispatcherHandlerCx<Message>,
    command: &Command,
    args: &[String],
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions())
                .reply_to_message_id(cx.update.id)
                .send()
                .await?;
        }
        Command::Start => {
            cx.answer("Hi there! I'm a high performance bot written in Rust.")
                .reply_to_message_id(cx.update.id)
                .send()
                .await?;
        }
        Command::SendPhoto => {
            promote(cx, args).await?;
        }
    };

    Ok(())
}

async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(command_handler)
        .dispatch()
        .await;
}

async fn command_handler(rx: DispatcherHandlerRx<Message>) {
    rx.commands::<Command, &str>("rusty")
        .for_each_concurrent(None, |(cx, command, args)| async move {
            answer(&cx, &command, &args).await.log_on_error().await
        })
        .await;
}
