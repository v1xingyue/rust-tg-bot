use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "显示此帮助消息")]
    Help,
    #[command(description = "开始使用机器人")]
    Start,
    #[command(description = "向指定用户问候")]
    Greet(String),
}

impl Command {
    pub async fn execute(
        self,
        bot: &Bot,
        msg: &Message,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Command::Start => {
                log::info!("收到开始命令, from {}", msg.chat.id);
                bot.send_message(
                    msg.chat.id,
                    "👋 你好！我是一个 Telegram 机器人。\n输入 /help 查看可用命令。",
                )
                .await?;
            }
            Command::Greet(name) => {
                bot.send_message(msg.chat.id, format!("你好, {}!", name))
                    .await?;
            }
        }
        Ok(())
    }
}
